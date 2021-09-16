use async_std::prelude::*;
use shellfish::{async_fn, Command, Shell};
use sqlx::postgres::PgPoolOptions;

use common::{
    model::{
        opportunity::{OpportunityQuery, OpportunityQueryOrdering},
        Opportunity, Pagination,
    },
    Database,
};

type DynError = Box<dyn std::error::Error>;

fn tokenize(
    chunks: impl Iterator<Item = String>,
    separators: &str,
) -> impl Iterator<Item = String> {
    let mut ret = Vec::new();

    for chunk in chunks {
        let mut begin = 0;

        for (end, sep) in chunk.match_indices(|c| separators.contains(c)) {
            if begin < end {
                ret.push(chunk[begin..end].to_string());
            }
            ret.push(sep.to_string());
            begin = end + sep.len();
        }

        if begin < chunk.len() {
            ret.push(chunk[begin..].to_string());
        }
    }

    ret.into_iter()
}

#[derive(Debug)]
enum Operator {
    Equal { attribute: String, value: String },
}

#[derive(Debug)]
enum BuildOperator {
    NeedAttribute,
    NeedOperator { attribute: String },
    NeedValue { attribute: String, operator: String },
}

fn operations(
    args: impl Iterator<Item = String>,
    operator_symbols: &str,
) -> Result<Vec<Operator>, DynError> {
    let mut operators = Vec::new();
    let mut state = BuildOperator::NeedAttribute;

    for token in tokenize(args, operator_symbols) {
        match state {
            BuildOperator::NeedAttribute => {
                if !operator_symbols.contains(&token) {
                    state = BuildOperator::NeedOperator { attribute: token };
                } else {
                    return Err(format!(
                        "Found an `{}` where we were expecting an attribute",
                        &token
                    )
                    .into());
                }
            }
            BuildOperator::NeedOperator { attribute } => {
                if operator_symbols.contains(&token) {
                    state = BuildOperator::NeedValue {
                        attribute,
                        operator: token,
                    };
                } else {
                    return Err(format!("Expected an operator, found `{}`", &token).into());
                }
            }
            BuildOperator::NeedValue {
                attribute,
                operator,
            } => {
                if !operator_symbols.contains(&token) {
                    match operator.as_ref() {
                        "=" => {
                            state = BuildOperator::NeedAttribute;
                            operators.push(Operator::Equal {
                                attribute,
                                value: token,
                            })
                        }
                        _ => {
                            return Err(format!("Unrecognized operator `{}`", &operator).into());
                        }
                    }
                } else {
                    return Err(format!("Found an `=` where we were expecting a value").into());
                }
            }
        }
    }

    Ok(operators)
}

#[derive(Debug)]
enum Table {
    Ambiguous,
    Opportunity,
}

#[derive(Debug)]
struct State {
    db: Database,
    opportunity_query: OpportunityQuery,
    table: Table,
}

impl State {
    async fn new() -> Result<Self, DynError> {
        let db = PgPoolOptions::new()
            .min_connections(1)
            .connect(&std::env::var("DATABASE_URL")?)
            .await?;

        common::migrate(&db).await?;

        let opportunity_query = OpportunityQuery::default();

        Ok(State {
            db,
            opportunity_query,
            table: Table::Opportunity,
        })
    }
}

fn reset_opportunity(state: &mut State) -> Result<(), DynError> {
    state.opportunity_query = OpportunityQuery::default();
    Ok(())
}

fn narrow_opportunity(
    state: &mut State,
    args: impl Iterator<Item = String>,
) -> Result<(), DynError> {
    for op in operations(args, "=")? {
        match op {
            Operator::Equal { attribute, value } => {
                todo!();
                println!("{} = {}", &attribute, &value);
            }
        }
    }

    Ok(())
}

async fn first_opportunity(state: &mut State) -> Result<(), DynError> {
    let opp = Opportunity::load_matching(
        &state.db,
        &state.opportunity_query,
        OpportunityQueryOrdering::Any,
        Pagination::One,
    )
    .await?
    .into_iter()
    .take(1)
    .next();

    if let Some(o) = opp {
        println!("{}", serde_json::to_string_pretty(&o)?);
    } else {
        println!("No results");
    }

    Ok(())
}

async fn update_opportunities<F: Fn(&mut Opportunity) -> Result<(), DynError>>(
    state: &mut State,
    update: F,
) -> Result<(), DynError> {
    let mut pagination = Pagination::Page {
        index: 0,
        size: 100,
    };

    loop {
        let opps = Opportunity::load_matching(
            &state.db,
            &state.opportunity_query,
            OpportunityQueryOrdering::Native,
            pagination,
        )
        .await?;

        if opps.is_empty() {
            break;
        }

        for mut opp in opps {
            update(&mut opp)?;
            opp.store(&state.db).await?;
        }

        if let Pagination::Page { index, size } = pagination {
            pagination = Pagination::Page {
                index: index + 1,
                size,
            };
            println!("Updated {}", (index + 1) * size);
        } else {
            panic!("pagination is not an instance of the Page variant");
        }
    }

    println!("Update finished.");

    Ok(())
}

async fn revalidate_opportunities(state: &mut State) -> Result<(), DynError> {
    update_opportunities(state, |_| Ok(())).await
}

async fn accept_opportunities(state: &mut State, accepted: bool) -> Result<(), DynError> {
    update_opportunities(state, |opp: &mut _| {
        opp.interior.accepted = Some(accepted);
        Ok(())
    })
    .await
}

async fn withdraw_opportunities(state: &mut State, withdrawn: bool) -> Result<(), DynError> {
    update_opportunities(state, |opp: &mut _| {
        opp.interior.withdrawn = withdrawn;
        Ok(())
    })
    .await
}

fn table(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    let mut args = args.into_iter().skip(1);

    let identifier = if let Some(ident) = args.next() {
        ident
    } else {
        return Err("table identifier expected".into());
    };

    match identifier.as_ref() {
        "opportunity" => state.table = Table::Opportunity,
        "opp" => state.table = Table::Opportunity,
        _ => {
            return Err("valid identifiers: opp|opportunity".into());
        }
    };

    Ok(())
}

fn reset(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => reset_opportunity(state)?,
        _ => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

fn narrow(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => narrow_opportunity(state, args.into_iter().skip(1))?,
        _ => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn first(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => first_opportunity(state).await?,
        _ => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn revalidate(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => revalidate_opportunities(state).await?,
        _ => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn accept(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    let mut args = args.into_iter().skip(1);

    let accepted = args.next().map(|s| s != "false").unwrap_or(true);

    match state.table {
        Table::Opportunity => accept_opportunities(state, accepted).await?,
        _ => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn withdraw(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    let mut args = args.into_iter().skip(1);

    let withdrawn = args.next().map(|s| s != "false").unwrap_or(true);

    match state.table {
        Table::Opportunity => withdraw_opportunities(state, withdrawn).await?,
        _ => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn db_meta(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    if args.len() < 1 {
        println!("Expected a database command: [analyze]");
        return Ok(());
    }

    match &*args[0] {
        "analyze" => {
            sqlx::query("analyze;").execute(&state.db).await?;
        }
        _ => {
            println!("Unrecognized database command: {}", &args[0]);
        }
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), DynError> {
    let mut shell = Shell::new_async(State::new().await?, "SNM Toolkit $ ");

    shell
        .commands
        .insert("table".into(), Command::new("select table".into(), table));

    shell.commands.insert(
        "reset".into(),
        Command::new("undo query narrowing".into(), reset),
    );

    shell.commands.insert(
        "narrow".into(),
        Command::new("narrow the query for the current table".into(), narrow),
    );

    shell.commands.insert(
        "first".into(),
        Command::new_async(
            "display the first result of the query".into(),
            async_fn!(State, first),
        )
        .await,
    );

    shell.commands.insert(
        "revalidate".into(),
        Command::new_async(
            "load, validate, and save rows matching the query".into(),
            async_fn!(State, revalidate),
        )
        .await,
    );

    shell.commands.insert(
        "accept".into(),
        Command::new_async(
            "mark matching rows accepted, or not accepted with `accept false`".into(),
            async_fn!(State, accept),
        )
        .await,
    );

    shell.commands.insert(
        "withdraw".into(),
        Command::new_async(
            "mark matching rows withdrawn, or not withdrawn with `withdraw false`".into(),
            async_fn!(State, withdraw),
        )
        .await,
    );

    shell.commands.insert(
        "db".into(),
        Command::new_async(
            "Database maintenance operations".into(),
            async_fn!(State, db_meta),
        )
        .await,
    );

    shell.run_async().await?;

    Ok(())
}

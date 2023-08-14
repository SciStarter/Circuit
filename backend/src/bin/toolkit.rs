use async_std::prelude::{Stream, StreamExt};
use clap::Parser;
use common::{geo::GeomQuery, model::Partner};
use counter::Counter;
use http_types::Method;
use serde::Deserialize;
use serde_json::json;
#[allow(unused_imports)]
use shellfish::{async_fn, handler::default, Command, Shell};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
#[allow(unused_imports)]
use std::io::Write;
use uuid::Uuid;

use common::{
    model::{
        opportunity::{OpportunityQuery, OpportunityQueryOrdering},
        Opportunity, Pagination, Person,
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

#[derive(Debug, Eq, PartialEq)]
enum Table {
    Ambiguous,
    Opportunity,
    Person,
    Partner,
}

#[derive(Debug, Default)]
enum PersonQuery {
    #[default]
    Any,
    _Email(String),
}

impl PersonQuery {
    pub fn load_matching<'db>(
        &self,
        db: &'db Database,
    ) -> Result<impl Stream<Item = Result<Person, sqlx::Error>> + 'db, DynError> {
        match self {
            PersonQuery::Any => Ok(sqlx::query("select * from c_person;")
                .map(|row: sqlx::postgres::PgRow| Person {
                    id: row.get("id"),
                    exterior: serde_json::from_value(row.get("exterior"))
                        .expect("Error decoding exterior"),
                    interior: serde_json::from_value(row.get("interior"))
                        .expect("Error decoding interior"),
                })
                .fetch(db)),
            PersonQuery::_Email(_) => todo!(),
        }
    }
}

#[derive(Debug, Default)]
enum PartnerQuery {
    #[default]
    Any,
}

impl PartnerQuery {
    pub fn load_matching<'db>(
        &self,
        db: &'db Database,
    ) -> Result<impl Stream<Item = Result<Partner, sqlx::Error>> + 'db, DynError> {
        match self {
            PartnerQuery::Any => Ok(sqlx::query("select * from c_partner;")
                .map(|row: sqlx::postgres::PgRow| Partner {
                    id: row.get("id"),
                    exterior: serde_json::from_value(row.get("exterior"))
                        .expect("Error decoding exterior"),
                    interior: serde_json::from_value(row.get("interior"))
                        .expect("Error decoding interior"),
                })
                .fetch(db)),
        }
    }
}

#[derive(Debug)]
struct State {
    db: Database,
    opportunity_query: OpportunityQuery,
    person_query: PersonQuery,
    partner_query: PartnerQuery,
    table: Table,
}

impl State {
    async fn new() -> Result<Self, DynError> {
        let db = PgPoolOptions::new()
            .min_connections(1)
            .connect(&std::env::var("DATABASE_URL")?)
            .await?;

        common::migrate(&db).await?;

        Ok(State {
            db,
            opportunity_query: Default::default(),
            person_query: Default::default(),
            partner_query: Default::default(),
            table: Table::Ambiguous,
        })
    }
}

fn reset_person(state: &mut State) -> Result<(), DynError> {
    state.person_query = PersonQuery::default();
    Ok(())
}

fn reset_partner(state: &mut State) -> Result<(), DynError> {
    state.partner_query = PartnerQuery::default();
    Ok(())
}

fn narrow_person(_state: &mut State, args: impl Iterator<Item = String>) -> Result<(), DynError> {
    for op in operations(args, "=")? {
        match op {
            Operator::Equal {
                attribute: _,
                value: _,
            } => {
                //println!("{} = {}", &attribute, &value);
                todo!();
            }
        }
    }

    Ok(())
}

fn narrow_partner(_state: &mut State, args: impl Iterator<Item = String>) -> Result<(), DynError> {
    for op in operations(args, "=")? {
        match op {
            Operator::Equal {
                attribute: _,
                value: _,
            } => {
                //println!("{} = {}", &attribute, &value);
                todo!();
            }
        }
    }

    Ok(())
}

async fn first_person(state: &mut State) -> Result<(), DynError> {
    let mut persons = state.person_query.load_matching(&state.db)?;

    if let Some(result) = persons.next().await {
        let person = result?;
        println!("{}", serde_json::to_string_pretty(&person)?);
    } else {
        println!("No results");
    }

    Ok(())
}

async fn first_partner(state: &mut State) -> Result<(), DynError> {
    let mut partners = state.partner_query.load_matching(&state.db)?;

    if let Some(result) = partners.next().await {
        let partner = result?;
        println!("{}", serde_json::to_string_pretty(&partner)?);
    } else {
        println!("No results");
    }

    Ok(())
}

async fn update_persons<F: Fn(&mut Person) -> Result<(), DynError>>(
    _state: &mut State,
    _update: F,
) -> Result<(), DynError> {
    todo!()
}

async fn update_partners<F: Fn(&mut Partner) -> Result<(), DynError>>(
    _state: &mut State,
    _update: F,
) -> Result<(), DynError> {
    todo!()
}

async fn revalidate_persons(state: &mut State) -> Result<(), DynError> {
    update_persons(state, |_| Ok(())).await
}

async fn revalidate_partners(state: &mut State) -> Result<(), DynError> {
    update_partners(state, |_| Ok(())).await
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
            Operator::Equal { attribute, value } => match attribute.as_ref() {
                "uid" => state.opportunity_query.uid = Some(Uuid::parse_str(&value)?),
                "slug" => state.opportunity_query.slug = Some(value),
                _ => println!("UNHANDLED ATTRIBUTE {} = {}", &attribute, &value),
            },
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
            panic!("Unexpected pagination variant");
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

async fn get_geo_opportunities(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    if state.table != Table::Opportunity {
        println!("Invalid table type");
        return Ok(());
    }

    update_opportunities(state, |opp: &mut _| {
        println!(
            "{}\n{:?}\n{:?}\n",
            opp.exterior.title, opp.exterior.location_point, opp.exterior.location_polygon
        );
        Ok(())
    })
    .await
}

async fn refresh_geo_opportunities(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    if state.table != Table::Opportunity {
        println!("Invalid table type");
        return Ok(());
    }

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
            let addr = format!(
                "{} {} {} {}",
                opp.exterior.address_street,
                opp.exterior.address_city,
                opp.exterior.address_state,
                opp.exterior.address_zip
            )
            .trim()
            .to_string();

            if addr.is_empty() {
                println!("Skipping empty address");
                continue;
            }

            println!("Looking for {} using geo::GeomQuery", &addr);

            let q = GeomQuery::new(addr.clone(), 0.5);

            let mut success = false;

            if let Ok(geo) = q.lookup(&state.db).await {
                if !geo.lon.is_empty() && !geo.lat.is_empty() {
                    let lon: f64 = geo.lon.parse()?;
                    let lat: f64 = geo.lat.parse()?;
                    opp.exterior.location_point = Some(json!({
                        "type": "Point",
                        "coordinates": [lon, lat]
                    }));
                    opp.exterior.location_polygon = geo.geojson;
                    opp.store(&state.db).await?;
                    success = true;
                }
            }

            if success {
                continue;
            }

            println!("Looking for {} using geo::Query", &addr);

            let q = common::geo::Query::new(addr, true);

            if let Some(geo) = q.lookup_one().await {
                opp.exterior.location_point = Some(json!({
                    "type": "Point",
                    "coordinates": [geo.geometry.longitude, geo.geometry.latitude]
                }));
                opp.store(&state.db).await?;
            }
        }

        if let Pagination::Page { index, size } = pagination {
            pagination = Pagination::Page {
                index: index + 1,
                size,
            };

            println!("Processed {}", (index + 1) * size);
        } else {
            panic!("Unexpected pagination");
        }
    }

    println!("Update finished.");

    Ok(())
}

async fn geoquery(_state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    let query = args.join(" ");

    println!(
        "{:?}",
        common::geo::Query::new(query, false,).lookup_one().await
    );

    Ok(())
}

async fn _geomquery(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    let query = args.join(" ");

    println!(
        "{:?}",
        common::geo::GeomQuery::new(query, 0.5)
            .lookup(&state.db)
            .await
    );

    Ok(())
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
        "person" => state.table = Table::Person,
        "partner" => state.table = Table::Partner,
        _ => {
            return Err("valid identifiers: opp|opportunity|person|partner".into());
        }
    };

    Ok(())
}

fn reset(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => reset_opportunity(state)?,
        Table::Person => reset_person(state)?,
        Table::Partner => reset_partner(state)?,
        Table::Ambiguous => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

fn narrow(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => narrow_opportunity(state, args.into_iter().skip(1))?,
        Table::Person => narrow_person(state, args.into_iter().skip(1))?,
        Table::Partner => narrow_partner(state, args.into_iter().skip(1))?,
        Table::Ambiguous => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn first(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => first_opportunity(state).await?,
        Table::Person => first_person(state).await?,
        Table::Partner => first_partner(state).await?,
        Table::Ambiguous => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn revalidate(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity => revalidate_opportunities(state).await?,
        Table::Person => revalidate_persons(state).await?,
        Table::Partner => revalidate_partners(state).await?,
        Table::Ambiguous => {
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
        Table::Person | Table::Partner => {
            return Err("not a meaningful operation".into());
        }
        Table::Ambiguous => {
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
        Table::Person | Table::Partner => {
            return Err("not a meaningful operation".into());
        }
        Table::Ambiguous => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn update_place(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    match state.table {
        Table::Opportunity | Table::Partner => {
            return Err("not a meaningful operation".into());
        }
        Table::Person => {
            let mut people = state.person_query.load_matching(&state.db)?;
            while let Some(Ok(person)) = people.next().await {
                person.update_state_and_metro(&state.db).await?;
            }
        }
        Table::Ambiguous => {
            return Err("select a table before trying this".into());
        }
    };

    Ok(())
}

async fn db_meta(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    let mut args = args.into_iter().skip(1);

    let command = args.next().unwrap_or_else(|| "help".into());

    match command.as_ref() {
        "help" => {
            println!("help         -- Print out this help text");
            println!("analyze      -- Have the database collect statistics to improve query execution time");
        }
        "analyze" => {
            sqlx::query("analyze;").execute(&state.db).await?;
            println!("Analysis completed");
        }
        _ => {
            println!("Unrecognized database command: {}", &command);
        }
    }

    Ok(())
}

async fn hashpassword(_state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    if args.len() < 2 {
        println!("Expected password argument");
    }

    println!("{}", djangohashers::make_password(&args[1]));

    Ok(())
}

async fn checkpassword(_state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    if args.len() < 3 {
        println!("Expected password and hashed arguments");
    }

    match djangohashers::check_password(&args[1], &args[2]) {
        Ok(matching) => {
            if matching {
                println!("matching");
            } else {
                println!("NOT matching");
            }
        }
        Err(err) => println!("{:?}", err),
    };

    Ok(())
}

async fn send(state: &mut State, args: Vec<String>) -> Result<(), DynError> {
    if args.len() < 2 {
        println!("Expected test or template keyword");
        return Ok(());
    }

    match args[1].as_str() {
        "test" => {
            if args.len() == 5 {
                let to = &args[2];
                let subject = &args[3];
                let body = &args[4];

                common::emails::send(
                    to,
                    "Science Near Me <info@sciencenearme.org>",
                    subject,
                    body,
                )
                .await;

                println!("Sent");
            }
        }
        "template" => {
            if args.len() == 3 {
                let slug = &args[2];

                let email = common::emails::EmailMessage::load(&state.db, slug)
                    .await
                    .unwrap();

                let mut persons = state.person_query.load_matching(&state.db)?;

                while let Some(result) = persons.next().await {
                    let person = result.unwrap();
                    println!("{}", &person.interior.email);
                    common::emails::send(
                        &person.interior.email,
                        "Science Near Me <info@sciencenearme.org>",
                        &email.subject,
                        &email.body,
                    )
                    .await;
                }
                println!("Sent");
            } else if args.len() > 3 {
                let slug = &args[2];

                let email = common::emails::EmailMessage::load(&state.db, slug)
                    .await
                    .unwrap();

                for address in args.iter().skip(3) {
                    println!("{address}");
                    common::emails::send(
                        address,
                        "Science Near Me <info@sciencenearme.org>",
                        &email.subject,
                        &email.body,
                    )
                    .await;
                }
                println!("Sent");
            }
        }
        unknown => {
            println!("unknown keyword {unknown}");
        }
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct UnsubscribeItem {
    _address: String,
    _code: String,
    _error: String,
    created_at: String,
}

#[derive(Deserialize, Debug)]
struct UnsubscribePaging {
    _first: String,
    next: String,
    _previous: String,
    _last: String,
}

#[derive(Deserialize, Debug)]
struct UnsubscribePage {
    items: Vec<UnsubscribeItem>,
    paging: UnsubscribePaging,
}

async fn load_unsubscribes(kind: &str) -> Counter<String> {
    let auth =
        surf::http::auth::BasicAuth::new("api", std::env::var("MAILGUN_PRIVATE_KEY").expect("This command should be run in an evironment with access to the Mailgun private key. Run it locally after setting the MAILGUN_PRIVAT_KEY environment variable."));
    let base = "https://api.mailgun.net/v3/mail.sciencenearme.org";

    let mut counts = Counter::new();
    let mut url = surf::Url::parse(&format!("{base}/{kind}?limit=100")).unwrap();

    loop {
        let data: UnsubscribePage = surf::Request::builder(Method::Get, url)
            .header(auth.name(), auth.value())
            .recv_json()
            .await
            .unwrap();

        url = surf::Url::parse(&data.paging.next).unwrap();

        if data.items.len() == 0 {
            break;
        }

        for item in data.items {
            let stamp =
                chrono::DateTime::parse_from_rfc2822(&item.created_at.replace("UTC", "+0000"))
                    .unwrap();
            counts[&stamp.format("%Y-%m").to_string()] += 1;
        }
    }

    counts
}

async fn unsubscribes(_state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    let unsubscribes = load_unsubscribes("unsubscribes").await;
    let bounces = load_unsubscribes("bounces").await;
    let complaints = load_unsubscribes("complaints").await;

    let cumulative = unsubscribes + bounces + complaints;

    let mut rows: Vec<_> = cumulative.iter().collect();

    rows.sort_unstable();

    println!(r#""month","unsubscribes""#);

    for row in rows {
        println!(r#""{}",{}"#, row.0, row.1);
    }

    Ok(())
}

#[allow(unused, unreachable_code)]
async fn new_accounts(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    #[cfg(not(feature = "container"))]
    panic!("Run this command in a production environment");

    let mut joins: Counter<String> = Counter::new();

    let mut stream = sqlx::query!(
        "SELECT substring(interior ->> 'joined_at' FROM 0 FOR 8) AS month FROM c_person"
    )
    .fetch(&state.db);

    while let Some(rec) = stream.next().await {
        if let Some(month) = rec?.month {
            joins[&month] += 1;
        }
    }

    let mut rows: Vec<_> = joins.iter().collect();

    rows.sort_unstable();

    println!(r#""month","new accounts""#);

    for row in rows {
        println!(r#""{}",{}"#, row.0, row.1);
    }

    Ok(())
}

#[allow(unused, unreachable_code)]
async fn new_opportunities(state: &mut State, _args: Vec<String>) -> Result<(), DynError> {
    #[cfg(not(feature = "container"))]
    panic!("Run this command in a production environment");

    let mut opps: Counter<String> = Counter::new();

    let mut stream =
        sqlx::query!("SELECT substring(created::text from 0 for 8) AS month FROM c_opportunity")
            .fetch(&state.db);

    while let Some(rec) = stream.next().await {
        if let Some(month) = rec?.month {
            opps[&month] += 1;
        }
    }

    let mut rows: Vec<_> = opps.iter().collect();

    rows.sort_unstable();

    println!(r#""month","new opportunities""#);

    for row in rows {
        println!(r#""{}",{}"#, row.0, row.1);
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TimezoneItem {
    _zone_name: String,
}

#[derive(Deserialize, Debug)]
struct TimezoneResponse {
    _status: String,
    _message: String,
    _zones: Vec<TimezoneItem>,
}

async fn run_shell(state: State) -> Result<(), DynError> {
    let mut shell = Shell::new_async(state, "SNM Toolkit $ ");

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
        ),
    );

    shell.commands.insert(
        "revalidate".into(),
        Command::new_async(
            "load, validate, and save rows matching the query".into(),
            async_fn!(State, revalidate),
        ),
    );

    shell.commands.insert(
        "accept".into(),
        Command::new_async(
            "mark matching rows accepted, or not accepted with `accept false`".into(),
            async_fn!(State, accept),
        ),
    );

    shell.commands.insert(
        "withdraw".into(),
        Command::new_async(
            "mark matching rows withdrawn, or not withdrawn with `withdraw false`".into(),
            async_fn!(State, withdraw),
        ),
    );

    shell.commands.insert(
        "db".into(),
        Command::new_async(
            "Database maintenance operations".into(),
            async_fn!(State, db_meta),
        ),
    );

    shell.commands.insert(
        "hashpassword".into(),
        Command::new_async(
            "Generate a password hash".into(),
            async_fn!(State, hashpassword),
        ),
    );

    shell.commands.insert(
        "checkpassword".into(),
        Command::new_async(
            "Check whether a password matches a hash".into(),
            async_fn!(State, checkpassword),
        ),
    );

    shell.commands.insert(
        "send".into(),
        Command::new_async("send an email message".into(), async_fn!(State, send)),
    );

    shell.commands.insert(
        "unsubscribes".into(),
        Command::new_async(
            "print unsubscribe CSV".into(),
            async_fn!(State, unsubscribes),
        ),
    );

    shell.commands.insert(
        "getgeo".into(),
        Command::new_async(
            "Get geo data from an opportunity".into(),
            async_fn!(State, get_geo_opportunities),
        ),
    );

    shell.commands.insert(
        "refreshgeo".into(),
        Command::new_async(
            "Update geo data for an opportunity based on address".into(),
            async_fn!(State, refresh_geo_opportunities),
        ),
    );

    shell.commands.insert(
        "geoquery".into(),
        Command::new_async(
            "Update geo data for an opportunity based on address".into(),
            async_fn!(State, geoquery),
        ),
    );

    shell.commands.insert(
        "update_place".into(),
        Command::new_async(
            "Update state and metro fields for selected people".into(),
            async_fn!(State, update_place),
        ),
    );

    shell.run_async().await?;

    Ok(())
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Shell,
    Unsubscribes,
    Joins,
    Opportunities,
}

#[derive(Parser, Debug)]
struct Options {
    #[clap(subcommand)]
    command: Option<Action>,
}

#[async_std::main]
async fn main() -> Result<(), DynError> {
    let options = Options::parse();

    let mut state = State::new().await?;

    match options.command.unwrap_or(Action::Shell) {
        Action::Unsubscribes => {
            unsubscribes(&mut state, Vec::new()).await?;
        }
        Action::Joins => {
            new_accounts(&mut state, Vec::new()).await?;
        }
        Action::Opportunities => {
            new_opportunities(&mut state, Vec::new()).await?;
        }
        Action::Shell => run_shell(state).await?,
    }

    Ok(())
}

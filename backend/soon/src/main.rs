use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tide::prelude::*;
use tide::Request;
use tide::Response;

#[derive(Debug, Deserialize)]
struct Form {
    email: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let mut app = tide::with_state(pool);

    app.at("/add").post(add_email);
    app.at("/").serve_file("static/index.html")?;
    app.at("/").serve_dir("static/")?;

    app.listen("[::]:8000").await?;

    Ok(())
}

async fn add_email(mut req: Request<PgPool>) -> tide::Result {
    let Form { email } = req.body_form().await?;

    sqlx::query(
        r#"insert into c_comingsoon ("category", "email", "when") values ('SNM pre', $1, now())"#,
    )
    .bind(email)
    .execute(req.state())
    .await?;

    Ok(Response::builder(303)
        .header("Location", "/added.html")
        .build())
}

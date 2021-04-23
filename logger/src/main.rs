use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("api/log/").get(order_shoes);
    app.listen("0.0.0.0:9000").await?;
    Ok(())
}

async fn order_shoes(_req: Request<()>) -> tide::Result {
    Ok("webly".into())
}

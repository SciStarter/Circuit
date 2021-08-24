use tide::Request;

async fn from_frontend(_req: Request<()>) -> tide::Result {
    Ok("logged".into())
}

async fn from_backend(_req: Request<()>) -> tide::Result {
    Ok("logged".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("api/log").post(from_frontend);

    // Not exposed by the ingress, so only accessible from within the
    // kubernetes cluster.
    app.at("internal/log").post(from_backend);

    app.listen("0.0.0.0:9000").await?;
    Ok(())
}

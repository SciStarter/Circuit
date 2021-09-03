use tide::Request;

async fn from_frontend(mut req: Request<()>) -> tide::Result {
    let content = req.body_string().await?;

    println!("FRONTEND {}", content);

    Ok("logged".into())
}

async fn from_backend(mut req: Request<()>) -> tide::Result {
    let content = req.body_string().await?;

    println!("BACKEND {}", content);

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

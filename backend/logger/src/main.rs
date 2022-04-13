use async_std::{
    channel::{bounded, Receiver, Sender},
    task,
};

use chrono::{DateTime, Datelike, Duration, FixedOffset, Timelike, Utc};
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use surf::http::Method;
use tide::Request;

const NEW_FILE_AFTER: i64 = 10; // minutes

type Sink = Sender<String>;

async fn log_item(mode: &'static str, content: String, sink: Sink) {
    let entry = format!(
        "{{\"mode\": \"{}\", \"when\": \"{:?}\", \"content\": {}}}",
        mode,
        Utc::now(),
        content
    );

    sink.send(entry).await.unwrap();
}

async fn from_client(mut req: Request<(Sink, Sink)>) -> tide::Result {
    async_std::task::spawn(log_item(
        "client",
        req.body_string().await?,
        req.state().0.clone(),
    ));
    Ok("logged".into())
}

async fn from_server(mut req: Request<(Sink, Sink)>) -> tide::Result {
    async_std::task::spawn(log_item(
        "server",
        req.body_string().await?,
        req.state().0.clone(),
    ));
    Ok("logged".into())
}

async fn click_client(mut req: Request<(Sink, Sink)>) -> tide::Result {
    async_std::task::spawn(log_item(
        "client",
        req.body_string().await?,
        req.state().1.clone(),
    ));
    Ok("logged".into())
}

async fn click_server(mut req: Request<(Sink, Sink)>) -> tide::Result {
    async_std::task::spawn(log_item(
        "server",
        req.body_string().await?,
        req.state().1.clone(),
    ));
    Ok("logged".into())
}

async fn writer(source: Receiver<String>, endpoint: String, access_key: String, secret: String) {
    let threshold = Duration::minutes(NEW_FILE_AFTER);

    let bucket = Bucket::new(
        url::Url::parse(&endpoint).unwrap(),
        UrlStyle::Path,
        "".to_string(),
        "".to_string(),
    )
    .unwrap();

    let credentials = Credentials::new(access_key, secret);

    let mut last_frame: DateTime<FixedOffset> = Utc::now().into();
    let mut buffer = String::new();

    loop {
        if let Ok(Ok(entry)) =
            async_std::future::timeout(threshold.to_std().unwrap(), source.recv()).await
        {
            buffer.push_str(&entry);
            buffer.push('\n');
            println!("{}", &entry);
        }

        let now: DateTime<FixedOffset> = Utc::now().into();

        if now - last_frame > threshold && !buffer.is_empty() {
            let fname = format!(
                "log_{:04}-{:02}-{:02}_{:02}{:02}.jsonl",
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute()
            );

            let call = bucket.put_object(Some(&credentials), &fname);

            surf::Request::builder(
                Method::Put,
                call.sign(Duration::minutes(1).to_std().unwrap()),
            )
            .body(buffer.as_bytes())
            .send()
            .await
            .unwrap();

            buffer.clear();
            last_frame = now;
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let (log_xmit, log_recv) = bounded(1000);
    let (stream_xmit, stream_recv) = bounded(1000);

    task::spawn(writer(
        log_recv,
        std::env::var("LOGGER_ENDPOINT").unwrap(),
        std::env::var("LOGGER_ACCESS_KEY").unwrap(),
        std::env::var("LOGGER_SECRET").unwrap(),
    ));

    task::spawn(writer(
        stream_recv,
        std::env::var("CLICK_ENDPOINT").unwrap(),
        std::env::var("CLICK_ACCESS_KEY").unwrap(),
        std::env::var("CLICK_SECRET").unwrap(),
    ));

    let mut app = tide::with_state((log_xmit, stream_xmit));

    app.at("api/log").post(from_client);
    app.at("api/click").post(click_client);

    // Not exposed by the ingress, so only accessible from within the
    // kubernetes cluster.
    app.at("internal/log").post(from_server);
    app.at("internal/click").post(click_server);

    app.listen("[::]:9000").await?;
    Ok(())
}

use std::time::{Duration, SystemTime};

use async_std::{
    channel::{unbounded, Receiver, Sender},
    task,
};
use serde::Deserialize;
use surf::{http::Method, StatusCode};
use tide::Request;

const TEXT_WIDTH: usize = 60;

#[derive(Deserialize, Debug)]
struct Email {
    to: String,
    from: String,
    subject: String,
    body: String,
}

async fn enqueue(mut req: Request<Sender<Email>>) -> tide::Result {
    let email: Email = req.body_json().await?;
    let now = chrono::Utc::now().to_rfc3339();
    println!("[{now}] Queuing email to {}", &email.to);
    req.state().send(email).await?;
    Ok("queued".into())
}

async fn send(recv: Receiver<Email>, xmit: Sender<Email>) {
    let rate = Duration::from_millis(200);
    let rate_limited = Duration::from_secs(60);
    let auth = surf::http::auth::BasicAuth::new("api", std::env::var("MAILGUN_SECRET").unwrap());
    let endpoint =
        url::Url::parse("https://api.mailgun.net/v3/mail.sciencenearme.org/messages").unwrap();

    loop {
        task::sleep(rate).await;

        if let Ok(email) = recv.recv().await {
            let now = chrono::Utc::now().to_rfc3339();

            let mut body = String::new();

            {
                let mut encode = form_urlencoded::Serializer::new(&mut body);
                encode.append_pair("to", &email.to);
                encode.append_pair("from", &email.from);
                encode.append_pair("subject", &email.subject);
                encode.append_pair("html", &email.body);
                encode.append_pair(
                    "text",
                    &html2text::from_read(email.body.as_bytes(), TEXT_WIDTH),
                );
            }

            match surf::Request::builder(Method::Post, endpoint.clone())
                .header(auth.name(), auth.value())
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body_string(body)
                .send()
                .await
            {
                Ok(mut x) => match x.status() {
                    StatusCode::Ok => println!("[{now}] Sent email to {}", &email.to),
                    StatusCode::TooManyRequests => {
                        xmit.send(email).await.ok();
                        task::sleep(rate_limited).await;
                    }
                    other => {
                        eprintln!("[{now}] {other:?} {:?}", x.body_string().await)
                    }
                },
                Err(err) => {
                    eprintln!("[{now}] Error during send: {err:?}")
                }
            }
        } else {
            panic!("channel was closed");
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let (xmit, recv) = unbounded();

    task::spawn(send(recv, xmit.clone()));

    let mut app = tide::with_state(xmit);
    app.at("enqueue").post(enqueue);
    app.listen("[::]:9100").await?;

    Ok(())
}

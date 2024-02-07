use std::{convert::Infallible, net::SocketAddr, path::PathBuf};

use bytes::{Buf, Bytes, BytesMut};
use chrono::Duration;
use futures_lite::prelude::*;
use once_cell::sync::Lazy;
use rusty_s3::{Bucket, BucketError, Credentials, S3Action, UrlStyle};
use tokio::{sync::Mutex, task};
use warp::{
    filters::multipart,
    hyper::StatusCode,
    reject::{self, Reject},
    reply::{json, with_status, Reply},
    Filter, Rejection,
};

static UI_AUDIENCE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("0be35cad-2b1f-4a45-a6da-b1051643c6f6").unwrap());

static INTERNAL_UID: Lazy<uuid::Uuid> = Lazy::new(|| {
    uuid::Uuid::parse_str(
        &std::env::var("INTERNAL_UID").expect("INTERNAL_UID should be set in the environmnet"),
    )
    .expect("INTERNAL_UID environment variable should contain a UUID")
});

const MIB: usize = 1024 * 1024;

const MAX_SIZE: usize = 10 * MIB;

#[derive(Debug)]
struct TooBig;

impl Reject for TooBig {}

#[derive(Debug)]
struct InvalidToken;

impl Reject for InvalidToken {}

#[derive(Debug)]
struct RejectBucketError(BucketError);

impl Reject for RejectBucketError {}

#[derive(Debug)]
struct RejectReqwestError(reqwest::Error);

impl Reject for RejectReqwestError {}

#[derive(Debug)]
struct RejectHTTPError(Mutex<Option<reqwest::Response>>);

impl Reject for RejectHTTPError {}

#[derive(Debug)]
struct RejectAuthError(String);

impl Reject for RejectAuthError {}

#[derive(serde::Serialize)]
struct Response {
    result: String,
    message: Option<String>,
    url: Option<String>,
}

async fn store<F>(
    endpoint: String,
    fname: String,
    data: Bytes,
    preprocess: F,
) -> Result<(), Rejection>
where
    F: Clone + Send + FnOnce(Bytes) -> Result<Bytes, Rejection> + 'static,
{
    let access_key = std::env::var("UPLOADER_ACCESS_KEY").unwrap();
    let secret = std::env::var("UPLOADER_SECRET").unwrap();

    let bucket = match Bucket::new(
        url::Url::parse(&endpoint).expect("UPLOADER_ENDPOINT env variable should contain a URL"),
        UrlStyle::Path,
        String::new(),
        String::new(),
    ) {
        Ok(bucket) => bucket,
        Err(e) => return Err(reject::custom(RejectBucketError(e))),
    };

    let credentials = Credentials::new(access_key, secret);

    let mut call = bucket.put_object(Some(&credentials), &fname);
    call.headers_mut().insert("x-amz-acl", "public-read");

    let data = task::block_in_place(move || preprocess(data))?;

    let client = reqwest::Client::new();

    match client
        .put(call.sign(Duration::minutes(1).to_std().unwrap()))
        .header("x-amz-acl", "public-read")
        .body(data)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status() != StatusCode::OK {
                return Err(reject::custom(RejectHTTPError(Mutex::new(Some(dbg!(
                    resp
                ))))));
            };
        }
        Err(err) => return Err(reject::custom(RejectReqwestError(dbg!(err)))),
    };

    Ok(())
}

/// Helper function for upload endpoint handlers
async fn upload_all_with<F>(
    token: String,
    mut form: multipart::FormData,
    preprocess: F,
) -> Result<impl Reply, Rejection>
where
    F: Clone + Send + FnOnce(Bytes) -> Result<Bytes, Rejection> + 'static,
{
    let endpoint =
        std::env::var("UPLOADER_ENDPOINT").expect("UPLOADER_ENDPOINT env variable should be set");

    let secret = std::env::var("UPLOADER_AUTH_SECRET")
        .expect("UPLOADER_AUTH_SECRET env variable should be set");

    let uid = if token == secret {
        INTERNAL_UID.clone()
    } else {
        match common::jwt::check_jwt(&token, &UI_AUDIENCE) {
            Ok(uid) => uid,
            Err(_) => return Err(reject::custom(InvalidToken)),
        }
    };

    let mut urls = Vec::new();

    while let Some(Ok(part)) = form.next().await {
        let original_fname = PathBuf::from(part.filename().unwrap_or("missing-filename"));
        let fname = format!(
            "{}/{}.{}",
            uid.as_hyphenated().to_string(),
            ulid::Ulid::new(),
            original_fname
                .extension()
                .and_then(|x| x.to_str())
                .unwrap_or("data")
        );
        urls.push(format!("{}{}", &endpoint, &fname));

        let mut stream = part.stream();
        let mut data = BytesMut::new();

        while let Some(Ok(mut buf)) = stream.next().await {
            while buf.has_remaining() {
                let chunk = buf.chunk();
                let length = chunk.len();
                data.extend_from_slice(chunk);
                buf.advance(length);
                if data.len() > MAX_SIZE {
                    return Err(reject::custom(TooBig));
                }
            }
            task::yield_now().await;
        }

        // Could spawn this as a task, if we start seeing timeout
        // errors. The client side would need to get a little smarter
        // to account for the fact that upload might not be completed
        // when the response arrives.
        let _ = store(
            endpoint.clone(),
            fname,
            Bytes::from(data),
            preprocess.clone(),
        )
        .await;
    }

    Ok(json(&urls))
}

/// Uploads files, without doing any intermediate processing
async fn generic(
    authorization: String,
    form: multipart::FormData,
) -> Result<impl Reply, Rejection> {
    upload_all_with(extract_token(authorization)?, form, |data| Ok(data)).await
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_method("POST")
        .allow_method("OPTIONS")
        .allow_header("Authorization")
        .allow_header("Content-Type")
        .allow_origin("https://sciencenearme.org")
        .allow_origin("https://www.sciencenearme.org")
        .allow_origin("https://beta.sciencenearme.org")
        .allow_credentials(true)
        .build();

    let app = warp::post()
        .and(warp::path("api"))
        .and(warp::path("upload"))
        //.and(warp::cookie("__Host-token"))
        .and(warp::header("Authorization"))
        .and(multipart::form())
        .and_then(generic)
        .recover(report_errors)
        .with(cors);

    warp::serve(app)
        .run(
            "[::]:9001"
                .parse::<SocketAddr>()
                .expect("bind address should parse"),
        )
        .await;
}

fn extract_token(authorization: String) -> Result<String, Rejection> {
    if let Some((scheme, token)) = authorization.split_once(' ') {
        if scheme == "Bearer" {
            Ok(token.to_string())
        } else {
            Err(reject::custom(RejectAuthError(String::from(
                "Incorrect authorization scheme",
            ))))
        }
    } else {
        Ok(authorization) // Assume it's a secret-based authorization

        // Err(reject::custom(RejectAuthError(String::from(
        //     "Improperly formatted authorization header",
        // ))))
    }
}

async fn report_errors(err: Rejection) -> Result<impl Reply, Infallible> {
    let mut out = Response {
        result: String::from("error"),
        message: None,
        url: None,
    };

    let mut code = StatusCode::BAD_REQUEST;

    if let Some(_) = err.find::<TooBig>() {
        code = StatusCode::PAYLOAD_TOO_LARGE;
        out.message = Some(format!("File size greater than {} MiB", MAX_SIZE / MIB));
    } else if let Some(_) = err.find::<InvalidToken>() {
        code = StatusCode::FORBIDDEN;
        out.message = Some(String::from("Invalid access token"));
    } else if let Some(e) = err.find::<RejectBucketError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        out.message = Some(format!("Communication with file storage failed: {}", e.0));
    } else if let Some(e) = err.find::<RejectReqwestError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        out.message = Some(format!("Communication with file storage failed: {}", e.0));
    } else if let Some(e) = err.find::<RejectHTTPError>() {
        if let Some(resp) = e.0.lock().await.take() {
            code = resp.status();
            out.message = Some(
                resp.text()
                    .await
                    .unwrap_or_else(|_| String::from("File storage error")),
            );
        } else {
            code = StatusCode::INTERNAL_SERVER_ERROR;
            out.message = Some(String::from("No response from file storage"));
        }
    } else if let Some(e) = err.find::<RejectAuthError>() {
        code = StatusCode::FORBIDDEN;
        out.message = Some(e.0.clone());
    }

    dbg!(err);

    Ok(with_status(json(&out), code))
}

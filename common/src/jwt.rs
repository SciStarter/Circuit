use ::jwt::SignWithKey;
use ::jwt::VerifyWithKey;
use chrono::Utc;
use hmac::{Hmac, NewMac};
use once_cell::sync::Lazy;
use sha2::Sha256;
use uuid::Uuid;

use crate::{model, Error};

pub static JWT_SIGNING_KEY: Lazy<Hmac<Sha256>> =
    Lazy::new(|| Hmac::new_varkey(std::env::var("JWT_SIGNING_KEY").unwrap().as_bytes()).unwrap());

pub fn issue_jwt(uid: &Uuid) -> Result<String, Error> {
    let now = Utc::now().timestamp() as ::jwt::claims::SecondsSinceEpoch;

    let mut claims = ::jwt::RegisteredClaims::default();
    claims.subject = Some(uid.to_string());
    claims.audience = Some(model::ROOT_NAMESPACE.to_string());
    claims.issuer = Some(model::ROOT_NAMESPACE.to_string());
    claims.issued_at = Some(now);
    claims.expiration = Some(now + (6 * 60 * 60));

    Ok(claims.sign_with_key(&*JWT_SIGNING_KEY)?)
}

pub fn check_jwt(token: &str) -> Result<Uuid, Error> {
    let claims: ::jwt::RegisteredClaims =
        token.verify_with_key(&*JWT_SIGNING_KEY).map_err(|e| {
            dbg!(e);
            Error::Auth("Invalid signature".to_string())
        })?;

    let now = Utc::now().timestamp() as ::jwt::claims::SecondsSinceEpoch;

    if claims.expiration.unwrap_or(u64::MAX) < now
        || claims.audience != Some(model::ROOT_NAMESPACE.to_string())
        || claims.issuer != Some(model::ROOT_NAMESPACE.to_string())
    {
        return Err(Error::Auth(
            "Incorrect expiration, issuer, or audience".to_string(),
        ));
    }

    Ok(Uuid::parse_str(
        claims
            .subject
            .ok_or_else(|| Error::Auth("Subject claim is missing".to_string()))?
            .as_ref(),
    )?)
}

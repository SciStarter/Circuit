use bytes::Bytes;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::{self, gen_keypair, gen_nonce, PublicKey, SecretKey};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Secret key is not known, but is required for this operation")]
    SecretKeyRequired,
    #[error("The box was sealed with different key than expected")]
    UnexpectedKey,
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("Decryption failed; maybe the key is wrong or the message was tampered with")]
    Decrypt,
    #[error("Couldn't initialize KeyPair from env var because the var is not set")]
    EnvVar(#[from] std::env::VarError),
}

#[readonly::make]
#[derive(Deserialize, Serialize)]
pub struct KeyPair {
    pub public: PublicKey,
    pub secret: Option<SecretKey>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sealed {
    source: PublicKey,
    nonce: box_::Nonce,
    ciphertext: Bytes,
}

impl KeyPair {
    pub fn generate() -> Self {
        let (public, secret) = gen_keypair();
        KeyPair {
            public,
            secret: Some(secret),
        }
    }

    pub fn from_env(var: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(&std::env::var(var)?)?)
    }

    pub fn seal<D: Serialize>(&self, data: &D, dest: &KeyPair) -> Result<Sealed, Error> {
        if let Some(secret) = &self.secret {
            let nonce = gen_nonce();
            let plaintext = serde_json::to_string(data)?;
            let ciphertext = Bytes::from(box_::seal(
                plaintext.as_bytes(),
                &nonce,
                &dest.public,
                secret,
            ));

            Ok(Sealed {
                source: self.public.clone(),
                nonce,
                ciphertext,
            })
        } else {
            Err(Error::SecretKeyRequired)
        }
    }

    pub fn open<D>(&self, sealed: Sealed, source: Option<&KeyPair>) -> Result<D, Error>
    where
        D: for<'d> Deserialize<'d>,
    {
        if let Some(secret) = &self.secret {
            if let Some(src) = source {
                if sealed.source != src.public {
                    return Err(Error::UnexpectedKey);
                }
            }

            let plaintext = box_::open(&sealed.ciphertext, &sealed.nonce, &sealed.source, secret)
                .map_err(|_| Error::Decrypt)?;

            let data: D = serde_json::from_slice(&plaintext)?;

            Ok(data)
        } else {
            Err(Error::SecretKeyRequired)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KeyPair;

    #[test]
    #[ignore]
    fn keypair_from_env_secret() {
        KeyPair::from_env("SNM_PAIR").unwrap();
    }

    #[test]
    #[ignore]
    fn keypair_from_env_public() {
        KeyPair::from_env("SCI_PUB").unwrap();
    }
}

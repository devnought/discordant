use std::{borrow::Cow, collections::HashMap, num::ParseIntError};

use ed25519_dalek::{PublicKey, Signature, Verifier};
use http::HeaderMap;
use tracing::debug;

pub mod handler;

pub trait DiscordState<'a> {
    fn public_key(&self) -> Cow<'a, str>;
    fn application_id(&self) -> Cow<'a, str>;
}

#[derive(Debug, Clone)]
pub struct State<'a> {
    pub public_key: Cow<'a, str>,
    pub application_id: Cow<'a, str>,
}

impl<'a> DiscordState<'a> for State<'a> {
    fn public_key(&self) -> Cow<'a, str> {
        self.public_key.clone()
    }

    fn application_id(&self) -> Cow<'a, str> {
        self.application_id.clone()
    }
}

#[derive(Debug)]
pub enum DiscordVerify {
    Valid,
    Invalid,
}

pub fn discord_verify<'a, S>(state: &S, body: &str, headers: HeaderMap) -> DiscordVerify
where
    S: DiscordState<'a>,
{
    let headers = headers
        .iter()
        .filter_map(|(name, value)| {
            let v = value.to_str().ok()?;
            match name.as_str() {
                "x-signature-ed25519" | "x-signature-timestamp" => Some((name.as_str(), v)),
                _ => None,
            }
        })
        .collect::<HashMap<_, _>>();

    let public_key = state.public_key();
    let valid = verify_signature(&headers, body, &public_key);

    debug!("discord_verify: {valid:?}");

    match valid {
        Err(_) | Ok(false) => DiscordVerify::Invalid,
        _ => DiscordVerify::Valid,
    }
}

#[derive(Debug)]
pub enum VerifyError {
    PublicKeyDecode,
    PublicKeyFromBytes,
    SignatureDecode,
    SignatureFromBytes,
}

pub fn verify_signature(
    headers: &HashMap<&str, &str>,
    body: &str,
    discord_key: &str,
) -> Result<bool, VerifyError> {
    const TIMESTAMP: &str = "x-signature-timestamp";
    const ED25519: &str = "x-signature-ed25519";

    if !headers.contains_key(TIMESTAMP) || !headers.contains_key(ED25519) {
        return Ok(false);
    }

    let timestamp_body = {
        let timestamp = headers.get(TIMESTAMP).unwrap();
        format!("{}{}", timestamp, body)
    };

    let signature = {
        let signature = headers.get(ED25519).unwrap();
        let signature_vec = decode_hex(signature).map_err(|_| VerifyError::SignatureDecode)?;

        Signature::from_bytes(&signature_vec).map_err(|_| VerifyError::SignatureFromBytes)?
    };

    let public_key = {
        let public_key_vec = decode_hex(discord_key).map_err(|_| VerifyError::PublicKeyDecode)?;

        PublicKey::from_bytes(&public_key_vec).map_err(|_| VerifyError::PublicKeyFromBytes)?
    };

    let verify = public_key.verify(timestamp_body.as_bytes(), &signature);

    match verify {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

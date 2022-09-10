use std::{collections::HashMap, fmt::Display, num::ParseIntError};

use ed25519_dalek::{PublicKey, Signature, Verifier};

mod application_command;
pub use application_command::*;

mod application;
pub use application::*;

mod channel;
pub use channel::*;

mod emoji;
pub use emoji::*;

mod guild;
pub use guild::*;

mod interaction_response;
pub use interaction_response::*;

mod interaction;
pub use interaction::*;

mod permission;
pub use permission::*;

mod sticker;
use serde::{de::Visitor, Deserialize, Serialize};
pub use sticker::*;

mod team;
pub use team::*;

mod user;
pub use user::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Snowflake(u64);

impl Snowflake {
    pub fn timestamp(&self) -> i64 {
        (self.0 as i64 >> 22) + 1420070400000
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Snowflake {
    fn from(s: &str) -> Self {
        let value = s.parse().unwrap();
        Self(value)
    }
}

struct SnowflakeVisitor;

impl<'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a str to convert to a i64")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let value = v
            .parse::<u64>()
            .map_err(|e| E::custom(format!("Could not parse `{v}`: {}", e)))?;

        Ok(Snowflake(value))
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("Snowflake", &self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SnowflakeVisitor)
    }
}

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

    match public_key.verify(timestamp_body.as_bytes(), &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn pong() -> InteractionResponse<'static> {
    InteractionResponse {
        response_type: InteractionCallbackType::Pong,
        data: None,
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

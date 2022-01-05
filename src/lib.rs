use std::{borrow::Cow, collections::HashMap, convert::TryInto, num::ParseIntError};

use sodiumoxide::crypto::sign::{self, PublicKey, Signature};

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
pub use sticker::*;

mod team;
pub use team::*;

mod user;
pub use user::*;

pub type Snowflake<'a> = Cow<'a, str>;

pub enum VerifyError {
    PublicKeyDecode,
    PublicKeyArray,
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

    let signature_type = {
        let signature = headers.get(ED25519).unwrap();
        let signature_vec = decode_hex(signature).map_err(|_| VerifyError::SignatureDecode)?;

        Signature::from_bytes(&signature_vec).map_err(|_| VerifyError::SignatureFromBytes)?
    };

    let public_key_type = {
        let public_key_vec = decode_hex(discord_key).map_err(|_| VerifyError::PublicKeyDecode)?;
        let public_key_array = public_key_vec
            .try_into()
            .map_err(|_| VerifyError::PublicKeyArray)?;
        PublicKey(public_key_array)
    };

    Ok(sign::verify_detached(
        &signature_type,
        timestamp_body.as_bytes(),
        &public_key_type,
    ))
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

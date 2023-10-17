use serde::{Deserialize, Serialize};

use super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration: Option<String>,
}

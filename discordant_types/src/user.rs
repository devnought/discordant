use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct User<'a> {
    pub id: Snowflake,
    pub username: Cow<'a, str>,
    pub discriminator: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration: Option<Cow<'a, str>>,
}

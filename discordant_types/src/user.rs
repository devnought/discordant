use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct User<'a> {
    pub id: Snowflake,
    pub username: Cow<'a, str>,
    pub discriminator: Cow<'a, str>,
    pub global_name: Option<Cow<'a, str>>,
    pub avatar: Option<Cow<'a, str>>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<Cow<'a, str>>,
    pub accent_color: Option<u64>,
    pub locale: Option<Cow<'a, str>>,
    pub verified: Option<bool>,
    pub email: Option<Cow<'a, str>>,
    pub flags: Option<u64>,
    pub premium_type: Option<u64>,
    pub public_flags: Option<u64>,
}

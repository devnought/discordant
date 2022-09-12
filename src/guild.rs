use std::{borrow::Cow, collections::HashSet};

use serde::{Deserialize, Serialize};

use super::{Snowflake, User};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GuildMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub roles: HashSet<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Cow<'a, str>>,
}

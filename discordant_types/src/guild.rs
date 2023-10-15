use std::{borrow::Cow, collections::HashSet};

use serde::{Deserialize, Serialize};

use super::{Snowflake, User};

#[derive(Debug, Default, Deserialize)]
pub struct GuildMember<'a> {
    pub user: Option<User<'a>>,
    pub nick: Option<Cow<'a, str>>,
    pub avatar: Option<Cow<'a, str>>,
    pub roles: HashSet<Snowflake>,
    pub joined_at: Cow<'a, str>,
    pub premium_since: Option<Cow<'a, str>>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub flags: Option<i64>,
    pub pending: Option<bool>,
    pub permissions: Option<Cow<'a, str>>,
    pub communication_disabled_until: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GuildMemberUpdate<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub roles: HashSet<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i64>,
}

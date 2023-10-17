use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{Snowflake, User};

#[derive(Debug, Default, Deserialize)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: HashSet<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub flags: Option<i64>,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GuildMemberUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub roles: HashSet<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i64>,
}

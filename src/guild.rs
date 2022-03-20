use std::{borrow::Cow, collections::HashSet};

use serde::Deserialize;

use super::{Snowflake, User};

#[derive(Debug, Deserialize)]
pub struct GuildMember<'a> {
    pub user: Option<User<'a>>,
    pub nick: Option<Cow<'a, str>>,
    pub avatar: Option<Cow<'a, str>>,
    pub roles: HashSet<Snowflake<'a>>,
    pub joined_at: Option<Cow<'a, str>>,
    pub premium_since: Option<Cow<'a, str>>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub permissions: Option<Cow<'a, str>>,
    pub communication_disabled_until: Option<Cow<'a, str>>,
}

use std::borrow::Cow;

use serde::Deserialize;

use super::{Snowflake, User};

#[derive(Debug, Deserialize)]
pub struct GuildMember<'a> {
    pub user: Option<User<'a>>,
    pub nick: Option<Cow<'a, str>>,
    pub roles: Vec<Snowflake<'a>>,
    pub joined_at: Cow<'a, str>,
    pub premium_since: Option<Cow<'a, str>>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<Cow<'a, str>>,
}

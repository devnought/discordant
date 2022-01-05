use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct Role<'a> {
    pub id: Snowflake<'a>,
    pub name: Cow<'a, str>,
    pub color: u64,
    pub hoist: bool,
    pub position: u64,
    pub permissions: Cow<'a, str>,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleTags<'a> {
    pub bot_id: Option<Snowflake<'a>>,
    pub integration_id: Option<Snowflake<'a>>,
    pub premium_subscriber: Option<()>,
}

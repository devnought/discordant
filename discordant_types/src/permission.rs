use serde::{Deserialize, Serialize};

use super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: u64,
    pub hoist: bool,
    pub position: u64,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleTags {
    pub bot_id: Option<Snowflake>,
    pub integration_id: Option<Snowflake>,
    pub premium_subscriber: Option<()>,
}

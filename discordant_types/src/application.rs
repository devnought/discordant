use serde::Deserialize;

use super::{Snowflake, Team, User};

#[derive(Debug, Deserialize)]
pub struct Application {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    #[serde(default)]
    pub rpc_origins: Vec<String>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<User>,
    pub summary: String,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<Snowflake>,
    pub primary_sku_id: Option<Snowflake>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<u64>,
}

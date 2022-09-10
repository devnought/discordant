use std::borrow::Cow;

use serde::Deserialize;

use super::{Snowflake, Team, User};

#[derive(Debug, Deserialize)]
pub struct Application<'a> {
    pub id: Snowflake,
    pub name: Cow<'a, str>,
    pub icon: Option<Cow<'a, str>>,
    pub description: Cow<'a, str>,
    #[serde(default)]
    pub rpc_origins: Vec<Cow<'a, str>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<Cow<'a, str>>,
    pub privacy_policy_url: Option<Cow<'a, str>>,
    pub owner: Option<User<'a>>,
    pub summary: Cow<'a, str>,
    pub verify_key: Cow<'a, str>,
    pub team: Option<Team<'a>>,
    pub guild_id: Option<Snowflake>,
    pub primary_sku_id: Option<Snowflake>,
    pub slug: Option<Cow<'a, str>>,
    pub cover_image: Option<Cow<'a, str>>,
    pub flags: Option<u64>,
}

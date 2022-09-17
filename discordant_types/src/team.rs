use std::borrow::Cow;

use serde::Deserialize;

use super::{Snowflake, User};

#[derive(Debug, Deserialize)]
pub struct Team<'a> {
    pub icon: Option<Cow<'a, str>>,
    pub id: Snowflake,
    pub members: Vec<TeamMember<'a>>,
    pub name: Cow<'a, str>,
    pub owner_user_id: Snowflake,
}

#[derive(Debug, Deserialize)]
pub struct TeamMember<'a> {
    pub membership_state: u64,
    pub permissions: Vec<Cow<'a, str>>,
    pub team_id: Snowflake,
    pub user: User<'a>,
}

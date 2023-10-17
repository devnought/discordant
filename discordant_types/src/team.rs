use serde::Deserialize;

use super::{Snowflake, User};

#[derive(Debug, Deserialize)]
pub struct Team {
    pub icon: Option<String>,
    pub id: Snowflake,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Snowflake,
}

#[derive(Debug, Deserialize)]
pub struct TeamMember {
    pub membership_state: u64,
    pub permissions: Vec<String>,
    pub team_id: Snowflake,
    pub user: User,
}

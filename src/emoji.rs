use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{Role, Snowflake, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Emoji<'a> {
    pub id: Option<Snowflake<'a>>,
    pub name: Option<Cow<'a, str>>,
    pub roles: Option<Vec<Role<'a>>>,
    pub user: Option<User<'a>>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

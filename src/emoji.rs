use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{Role, Snowflake, User};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Emoji<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Role<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_colons: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

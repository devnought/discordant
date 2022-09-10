use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ChannelType;

use super::Snowflake;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationCommand<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub application_command_type: Option<ApplicationCommandTypeDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    pub name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandOption<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Snowflake>,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandTypeDef {
    #[default]
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationCommandOption<'a> {
    #[serde(rename = "type")]
    pub application_command_option_type: ApplicationCommandOptionType,
    pub name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<ApplicationCommandOptionChoice<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandOption<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channel_type: Vec<ChannelType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<MinMaxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<MinMaxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    #[default]
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice<'a> {
    pub name: Cow<'a, str>,
    pub value: ApplicationCommandOptionChoiceValue<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationCommandOptionChoiceValue<'a> {
    String(Cow<'a, str>),
    Integer(i64),
    Double(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MinMaxValue {
    Integer(i64),
    Number(f64),
}

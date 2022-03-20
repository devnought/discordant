use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ChannelType;

use super::Snowflake;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationCommand<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake<'a>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub application_command_type: Option<ApplicationCommandType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Snowflake<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake<'a>>,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Snowflake<'a>>,
}

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

impl Default for ApplicationCommandType {
    fn default() -> Self {
        Self::ChatInput
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationCommandOption<'a> {
    #[serde(rename = "type")]
    pub application_command_option_type: ApplicationCommandOptionType,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<ApplicationCommandOptionChoice<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<Vec<ChannelType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<MinMaxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<MinMaxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
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
}

impl Default for ApplicationCommandOptionType {
    fn default() -> Self {
        Self::SubCommand
    }
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

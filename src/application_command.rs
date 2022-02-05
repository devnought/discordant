use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ChannelType;

use super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommand<'a> {
    pub id: Snowflake<'a>,
    #[serde(rename = "type")]
    pub application_command_type: Option<ApplicationCommandType>,
    pub application_id: Snowflake<'a>,
    pub guild_id: Option<Snowflake<'a>>,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub options: Option<Vec<ApplicationCommandOption<'a>>>,
    pub default_permissions: Option<bool>,
    pub version: Snowflake<'a>,
}

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommandOption<'a> {
    pub application_command_option_type: ApplicationCommandOptionType,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub required: Option<bool>,
    pub choices: Option<Vec<ApplicationCommandOptionChoice<'a>>>,
    pub options: Option<Vec<ApplicationCommandOption<'a>>>,
    pub channel_type: Option<Vec<ChannelType>>,
    pub min_value: Option<MinMaxValue>,
    pub max_value: Option<MinMaxValue>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice<'a> {
    pub name: Cow<'a, str>,
    pub value: ApplicationCommandOptionChoiceValue<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
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

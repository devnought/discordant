use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ChannelType;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApplicationCommand<'a> {
    pub name: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,

    pub description: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandOption<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub application_command_type: Option<ApplicationCommandTypeDef>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl<'a> ApplicationCommand<'a> {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.name = value.into();
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.description = value.into();
        self
    }

    pub fn description_localizations(mut self, value: HashMap<String, String>) -> Self {
        self.description_localizations = Some(value);
        self
    }

    pub fn option(mut self, value: ApplicationCommandOption<'a>) -> Self {
        self.options.push(value);
        self
    }

    pub fn default_member_permissions<T>(mut self, value: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.default_member_permissions = Some(value.into());
        self
    }

    pub fn dm_permission(mut self, value: bool) -> Self {
        self.dm_permission = Some(value);
        self
    }

    pub fn default_permission(mut self, value: bool) -> Self {
        self.default_permission = Some(value);
        self
    }

    pub fn application_command_type(mut self, value: ApplicationCommandTypeDef) -> Self {
        self.application_command_type = Some(value);
        self
    }

    pub fn nsfw(mut self, value: bool) -> Self {
        self.nsfw = Some(value);
        self
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandTypeDef {
    #[default]
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    pub channel_types: Vec<ChannelType>,

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

impl<'a> ApplicationCommandOption<'a> {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn application_command_option_type(mut self, value: ApplicationCommandOptionType) -> Self {
        self.application_command_option_type = value;
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.name = value.into();
        self
    }

    pub fn name_localizations(mut self, value: HashMap<String, String>) -> Self {
        self.name_localizations = Some(value);
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.description = value.into();
        self
    }

    pub fn description_localizations(mut self, value: HashMap<String, String>) -> Self {
        self.description_localizations = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn choice(mut self, value: ApplicationCommandOptionChoice<'a>) -> Self {
        self.choices.push(value);
        self
    }

    pub fn option(mut self, value: ApplicationCommandOption<'a>) -> Self {
        self.options.push(value);
        self
    }

    pub fn channel_type(mut self, value: ChannelType) -> Self {
        self.channel_types.push(value);
        self
    }

    pub fn min_value(mut self, value: MinMaxValue) -> Self {
        self.min_value = Some(value);
        self
    }

    pub fn max_value(mut self, value: MinMaxValue) -> Self {
        self.max_value = Some(value);
        self
    }

    pub fn min_length(mut self, value: i64) -> Self {
        self.min_length = Some(value);
        self
    }

    pub fn max_length(mut self, value: i64) -> Self {
        self.max_length = Some(value);
        self
    }

    pub fn autocomplete(mut self, value: bool) -> Self {
        self.autocomplete = Some(value);
        self
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice<'a> {
    pub name: Cow<'a, str>,
    pub value: ApplicationCommandOptionChoiceValue<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationCommandOptionChoiceValue<'a> {
    String(Cow<'a, str>),
    Integer(i64),
    Double(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinMaxValue {
    Integer(i64),
    Number(f64),
}

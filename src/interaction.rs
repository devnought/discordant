use std::{borrow::Cow, collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{Channel, Emoji, GuildMember, Message, Role, Snowflake, User};

#[derive(Debug, Deserialize)]
pub struct Interaction<'a> {
    pub id: Snowflake<'a>,
    pub application_id: Snowflake<'a>,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData<'a>>,
    pub guild_id: Option<Snowflake<'a>>,
    pub channel_id: Option<Snowflake<'a>>,
    pub member: Option<GuildMember<'a>>,
    pub user: Option<User<'a>>,
    pub token: Cow<'a, str>,
    pub version: u64,
    pub message: Option<Message<'a>>,
    pub app_permissions: Option<Cow<'a, str>>,
    pub locale: Option<Cow<'a, str>>,
    pub guild_locale: Option<Cow<'a, str>>,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Debug, Deserialize)]
pub struct InteractionData<'a> {
    pub id: Option<Snowflake<'a>>,
    pub name: Option<Cow<'a, str>>,
    #[serde(rename = "type")]
    pub data_type: Option<ApplicationCommandType>,
    pub resolved: Option<ResolvedData<'a>>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    pub custom_id: Option<Cow<'a, str>>,
    pub component_type: Option<ComponentType>,
    pub values: Option<Vec<SelectOption<'a>>>,
    pub target_id: Option<Snowflake<'a>>,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Debug, Deserialize)]
pub struct ResolvedData<'a> {
    pub users: Option<HashMap<Snowflake<'a>, User<'a>>>,
    pub members: Option<HashMap<Snowflake<'a>, GuildMember<'a>>>,
    pub roles: Option<HashMap<Snowflake<'a>, Role<'a>>>,
    pub channels: Option<HashMap<Snowflake<'a>, Channel<'a>>>,
    pub messages: Option<HashMap<Snowflake<'a>, Message<'a>>>,
}

#[derive(Debug, Deserialize)]
pub struct MessageInteraction<'a> {
    pub id: Snowflake<'a>,
    #[serde(rename = "type")]
    pub message_interaction_type: InteractionType,
    pub name: Cow<'a, str>,
    pub user: User<'a>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Component<'a> {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SelectOption<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component<'a>>>,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ComponentType {
    #[default]
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
}

#[derive(Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectOption<'a> {
    pub label: Cow<'a, str>,
    pub value: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug)]
// #[serde(tag = "type")]
pub enum ApplicationCommandInteractionDataOption<'a> {
    // #[serde(rename = 1)]
    SubCommand {
        name: Cow<'a, str>,
        options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    },
    // #[serde(rename = 3)]
    String {
        name: Cow<'a, str>,
        value: Option<Cow<'a, str>>,
        options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    },
    // #[serde(rename = 4)]
    Integer {
        name: Cow<'a, str>,
        value: Option<i64>,
        options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    },
    // #[serde(rename = 5)]
    Boolean {
        name: Cow<'a, str>,
        value: Option<bool>,
        options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    },
    // #[serde(rename = 6)]
    User {
        name: Cow<'a, str>,
        value: Option<Cow<'a, str>>,
        options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    },
    // #[serde(rename = 7)]
    Channel(Box<ChannelVariant<'a>>),
    // #[serde(rename = 8)]
    Role {
        name: Cow<'a, str>,
        value: Option<Cow<'a, str>>,
        options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    },
    // #[serde(rename = 9)]
    // Mentionable {
    //     name: Cow<'a, str>,
    //     value: Option<Mentionable>,
    //     options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    // },
    // #[serde(rename = 10)]
    Number {
        name: Cow<'a, str>,
        value: Option<f64>,
        options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
    },
}

impl<'a> Display for ApplicationCommandInteractionDataOption<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationCommandInteractionDataOption::String { name, value, .. } => {
                write!(f, "{name}-{value:?}")
            }
            ApplicationCommandInteractionDataOption::Integer { name, value, .. } => {
                write!(f, "{name}-{value:?}")
            }
            ApplicationCommandInteractionDataOption::Boolean { name, value, .. } => {
                write!(f, "{name}-{value:?}")
            }
            _ => panic!("to string not implement for type"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChannelVariant<'a> {
    pub name: Cow<'a, str>,
    pub value: Option<Channel<'a>>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption<'a>>>,
}

impl<'de, 'a> serde::Deserialize<'de> for ApplicationCommandInteractionDataOption<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        let d_value = Value::deserialize(deserializer)?;

        let message_type = d_value
            .get("type")
            .and_then(Value::as_u64)
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;

        let name = d_value
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| serde::de::Error::missing_field("name"))?
            .to_string()
            .into();

        let options = {
            let options_raw = d_value.get("options").and_then(Value::as_array);

            if let Some(o) = options_raw {
                let mut new_options = Vec::with_capacity(o.len());

                for item in o {
                    let new_item = ApplicationCommandInteractionDataOption::deserialize(item)
                        .map_err(|e| {
                            println!("{e:?}");
                            serde::de::Error::custom("sub option")
                        })?;
                    new_options.push(new_item);
                }

                Some(new_options)
            } else {
                None
            }
        };

        let value_raw = d_value.get("value");

        Ok(match message_type {
            1 => ApplicationCommandInteractionDataOption::SubCommand { name, options },
            3 => ApplicationCommandInteractionDataOption::String {
                name,
                options,
                value: value_raw
                    .and_then(Value::as_str)
                    .map(|x| x.to_string().into()),
            },
            4 => ApplicationCommandInteractionDataOption::Integer {
                name,
                options,
                value: value_raw.and_then(Value::as_i64),
            },
            5 => ApplicationCommandInteractionDataOption::Boolean {
                name,
                options,
                value: value_raw.and_then(Value::as_bool),
            },
            6 => ApplicationCommandInteractionDataOption::User {
                name,
                options,
                value: value_raw
                    .and_then(Value::as_str)
                    .map(|x| x.to_string().into()),
            },
            7 => ApplicationCommandInteractionDataOption::Channel(Box::new(ChannelVariant {
                name,
                options,
                value: if let Some(data) = value_raw {
                    let channel = Channel::deserialize(data)
                        .map_err(|_| serde::de::Error::custom("bad channel"))?;
                    Some(channel)
                } else {
                    None
                },
            })),
            8 => ApplicationCommandInteractionDataOption::Role {
                name,
                options,
                value: value_raw
                    .and_then(Value::as_str)
                    .map(|x| x.to_string().into()),
            },
            10 => ApplicationCommandInteractionDataOption::Number {
                name,
                options,
                value: value_raw.and_then(Value::as_f64),
            },
            type_ => panic!("unsupported type {:?}", type_),
        })
    }
}

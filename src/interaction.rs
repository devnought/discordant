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
}

#[derive(Debug, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
}

#[derive(Debug, Deserialize)]
pub struct InteractionData<'a> {
    pub id: Snowflake<'a>,
    pub name: Cow<'a, str>,
    #[serde(rename = "type")]
    pub data_type: ApplicationCommandType,
    pub resolved: Option<ResolvedData<'a>>,
    pub options: Option<Vec<ApplicationCommandOption<'a>>>,
    pub custom_id: Option<Cow<'a, str>>,
    pub component_type: Option<ComponentType>,
    pub values: Option<Vec<SelectOption<'a>>>,
    pub target_id: Option<Snowflake<'a>>,
}

#[derive(Debug, PartialEq, Deserialize_repr)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Component<'a> {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub custom_id: Option<Cow<'a, str>>,
    pub disabled: Option<bool>,
    pub style: Option<ButtonStyle>,
    pub label: Option<Cow<'a, str>>,
    pub emoji: Option<Emoji<'a>>,
    pub url: Option<Cow<'a, str>>,
    pub options: Vec<SelectOption<'a>>,
    pub placeholder: Option<Cow<'a, str>>,
    pub min_values: Option<u64>,
    pub max_values: Option<u64>,
    pub components: Option<Vec<Component<'a>>>,
}

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
}

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
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
    pub description: Option<Cow<'a, str>>,
    pub emoji: Option<Emoji<'a>>,
    pub default: Option<bool>,
}

#[derive(Debug)]
// #[serde(tag = "type")]
pub enum ApplicationCommandOption<'a> {
    // #[serde(rename = 3)]
    String {
        name: Cow<'a, str>,
        value: Option<Cow<'a, str>>,
        options: Option<Vec<ApplicationCommandOption<'a>>>,
    },
    // #[serde(rename = 4)]
    Integer {
        name: Cow<'a, str>,
        value: Option<i64>,
        options: Option<Vec<ApplicationCommandOption<'a>>>,
    },
    // #[serde(rename = 5)]
    Boolean {
        name: Cow<'a, str>,
        value: Option<bool>,
        options: Option<Vec<ApplicationCommandOption<'a>>>,
    },
    // #[serde(rename = 6)]
    User(Box<UserVariant<'a>>),
    // #[serde(rename = 7)]
    Channel(Box<ChannelVariant<'a>>),
    // #[serde(rename = 8)]
    Role {
        name: Cow<'a, str>,
        value: Option<Role<'a>>,
        options: Option<Vec<ApplicationCommandOption<'a>>>,
    },
    // #[serde(rename = 10)]
    Number {
        name: Cow<'a, str>,
        value: Option<f64>,
        options: Option<Vec<ApplicationCommandOption<'a>>>,
    },
}

impl<'a> Display for ApplicationCommandOption<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationCommandOption::String { name, value, .. } => {
                write!(f, "{}-{:?}", name, value)
            }
            ApplicationCommandOption::Integer { name, value, .. } => {
                write!(f, "{}-{:?}", name, value)
            }
            ApplicationCommandOption::Boolean { name, value, .. } => {
                write!(f, "{}-{:?}", name, value)
            }
            _ => panic!("to string not implement for type"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserVariant<'a> {
    pub name: Cow<'a, str>,
    pub value: Option<User<'a>>,
    pub options: Option<Vec<ApplicationCommandOption<'a>>>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelVariant<'a> {
    pub name: Cow<'a, str>,
    pub value: Option<Channel<'a>>,
    pub options: Option<Vec<ApplicationCommandOption<'a>>>,
}

impl<'de, 'a> serde::Deserialize<'de> for ApplicationCommandOption<'a> {
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
                    let new_item = ApplicationCommandOption::deserialize(item)
                        .map_err(|_| serde::de::Error::custom("sub option"))?;
                    new_options.push(new_item);
                }

                Some(new_options)
            } else {
                None
            }
        };

        let value_raw = d_value.get("value");

        Ok(match message_type {
            3 => ApplicationCommandOption::String {
                name,
                options,
                value: value_raw
                    .and_then(Value::as_str)
                    .map(|x| x.to_string().into()),
            },
            4 => ApplicationCommandOption::Integer {
                name,
                options,
                value: value_raw.and_then(Value::as_i64),
            },
            5 => ApplicationCommandOption::Boolean {
                name,
                options,
                value: value_raw.and_then(Value::as_bool),
            },
            6 => ApplicationCommandOption::User(Box::new(UserVariant {
                name,
                options,
                value: if let Some(data) = value_raw {
                    let user = User::deserialize(data)
                        .map_err(|_| serde::de::Error::custom("bad user"))?;
                    Some(user)
                } else {
                    None
                },
            })),
            7 => ApplicationCommandOption::Channel(Box::new(ChannelVariant {
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
            8 => ApplicationCommandOption::Role {
                name,
                options,
                value: if let Some(data) = value_raw {
                    let role = Role::deserialize(data)
                        .map_err(|_| serde::de::Error::custom("bad role"))?;
                    Some(role)
                } else {
                    None
                },
            },
            10 => ApplicationCommandOption::Number {
                name,
                options,
                value: value_raw.and_then(Value::as_f64),
            },
            type_ => panic!("unsupported type {:?}", type_),
        })
    }
}

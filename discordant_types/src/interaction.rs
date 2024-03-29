use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{Channel, Emoji, GuildMember, Message, Role, Snowflake, User};

#[derive(Debug, Deserialize)]
pub struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub token: String,
    pub version: u64,
    pub message: Option<Message>,
    pub app_permissions: Option<String>,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
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
pub struct InteractionData {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub data_type: Option<ApplicationCommandType>,
    pub resolved: Option<ResolvedData>,
    #[serde(default)]
    pub options: Vec<ApplicationCommandInteractionDataOption>,
    pub custom_id: Option<String>,
    pub component_type: Option<ComponentType>,
    #[serde(default)]
    pub values: Vec<SelectOption>,
    pub target_id: Option<Snowflake>,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Debug, Deserialize)]
pub struct ResolvedData {
    pub users: Option<HashMap<Snowflake, User>>,
    pub members: Option<HashMap<Snowflake, GuildMember>>,
    pub roles: Option<HashMap<Snowflake, Role>>,
    pub channels: Option<HashMap<Snowflake, Channel>>,
    pub messages: Option<HashMap<Snowflake, Message>>,
}

#[derive(Debug, Deserialize)]
pub struct MessageInteraction {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub message_interaction_type: InteractionType,
    pub name: String,
    pub user: User,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: ComponentType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<SelectOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<i64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
}

impl Component {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn component_type(mut self, value: ComponentType) -> Self {
        self.component_type = value;
        self
    }

    pub fn custom_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_id = Some(value.into());
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn style(mut self, value: ButtonStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn label<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(value.into());
        self
    }

    pub fn emoji(mut self, value: Emoji) -> Self {
        self.emoji = Some(value);
        self
    }

    pub fn url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.url = Some(value.into());
        self
    }

    pub fn option(mut self, value: SelectOption) -> Self {
        self.options.push(value);
        self
    }

    pub fn placeholder<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.placeholder = Some(value.into());
        self
    }

    pub fn min_values(mut self, value: i64) -> Self {
        self.min_values = Some(value);
        self
    }

    pub fn max_values(mut self, value: i64) -> Self {
        self.max_values = Some(value);
        self
    }

    pub fn component(mut self, value: Component) -> Self {
        self.components.push(value);
        self
    }
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
pub struct SelectOption {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug)]
// #[serde(tag = "type")]
pub enum ApplicationCommandInteractionDataOption {
    // #[serde(rename = 1)]
    SubCommand {
        name: String,
        options: Vec<ApplicationCommandInteractionDataOption>,
    },
    // #[serde(rename = 2)]
    SubCommandGroup {
        name: String,
        options: Vec<ApplicationCommandInteractionDataOption>,
    },
    // #[serde(rename = 3)]
    String {
        name: String,
        value: Option<String>,
    },
    // #[serde(rename = 4)]
    Integer {
        name: String,
        value: Option<i64>,
    },
    // #[serde(rename = 5)]
    Boolean {
        name: String,
        value: Option<bool>,
    },
    // #[serde(rename = 6)]
    User {
        name: String,
        value: Option<Snowflake>,
    },
    // #[serde(rename = 7)]
    Channel(Box<ChannelVariant>),
    // #[serde(rename = 8)]
    Role {
        name: String,
        value: Option<String>,
    },
    // #[serde(rename = 9)]
    // Mentionable {
    //     name: String,
    //     value: Option<Mentionable>,
    // },
    // #[serde(rename = 10)]
    Number {
        name: String,
        value: Option<f64>,
    },
    // #[serde(rename = 11)]
    // Attachment {
    //     name: String,
    //     value: Option<Mentionable>,
    // },
}

impl Display for ApplicationCommandInteractionDataOption {
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
pub struct ChannelVariant {
    pub name: String,
    pub value: Option<Channel>,
    #[serde(default)]
    pub options: Vec<ApplicationCommandInteractionDataOption>,
}

impl<'de, 'a> serde::Deserialize<'de> for ApplicationCommandInteractionDataOption {
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

                new_options
            } else {
                Vec::with_capacity(0)
            }
        };

        let value_raw = d_value.get("value");

        Ok(match message_type {
            1 => ApplicationCommandInteractionDataOption::SubCommand { name, options },
            3 => ApplicationCommandInteractionDataOption::String {
                name,
                value: value_raw
                    .and_then(Value::as_str)
                    .map(|x| x.to_string().into()),
            },
            4 => ApplicationCommandInteractionDataOption::Integer {
                name,
                value: value_raw.and_then(Value::as_i64),
            },
            5 => ApplicationCommandInteractionDataOption::Boolean {
                name,
                value: value_raw.and_then(Value::as_bool),
            },
            6 => ApplicationCommandInteractionDataOption::User {
                name,
                value: value_raw
                    .and_then(Value::as_str)
                    .map(|x| Snowflake::from(x)),
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
                value: value_raw
                    .and_then(Value::as_str)
                    .map(|x| x.to_string().into()),
            },
            10 => ApplicationCommandInteractionDataOption::Number {
                name,
                value: value_raw.and_then(Value::as_f64),
            },
            type_ => panic!("unsupported type {:?}", type_),
        })
    }
}

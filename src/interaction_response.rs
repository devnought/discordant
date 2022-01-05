use std::borrow::Cow;

use serde::Serialize;
use serde_repr::Serialize_repr;

use super::{AllowedMentions, Component, Embed};

#[derive(Debug, Serialize)]
pub struct InteractionResponse<'a> {
    #[serde(rename = "type")]
    pub response_type: InteractionCallbackType,
    pub data: Option<InteractionCallbackData<'a>>,
}

#[derive(Debug, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
}

#[derive(Debug, Default, Serialize)]
pub struct InteractionCallbackData<'a> {
    pub tts: Option<bool>,
    pub content: Option<Cow<'a, str>>,
    pub embeds: Option<Vec<Embed<'a>>>,
    pub allowed_mentions: Option<AllowedMentions<'a>>,
    pub flags: Option<u64>,
    pub components: Option<Vec<Component<'a>>>,
}

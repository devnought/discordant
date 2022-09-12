use std::borrow::Cow;

use serde::Serialize;
use serde_repr::Serialize_repr;

use crate::Attachment;

use super::{AllowedMentions, Component, Embed};

#[derive(Debug, Serialize)]
pub struct InteractionResponse<'a> {
    #[serde(rename = "type")]
    pub response_type: InteractionCallbackType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData<'a>>,
}

#[derive(Debug, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
}

#[derive(Debug, Default, Serialize)]
pub struct InteractionCallbackData<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<InteractionCallbackDataFlags>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment<'a>>,
}

#[derive(Debug, Eq, PartialEq, Serialize_repr)]
#[repr(u16)]
pub enum InteractionCallbackDataFlags {
    Crossposted = 1 << 0,
    IsCrosspost = 1 << 1,
    SuppressEmbeds = 1 << 2,
    SourceMessageDeleted = 1 << 3,
    Urgent = 1 << 4,
    HasThread = 1 << 5,
    Ephemeral = 1 << 6,
    Loading = 1 << 7,
    FailedToMentionSomeRolesInThread = 1 << 8,
}

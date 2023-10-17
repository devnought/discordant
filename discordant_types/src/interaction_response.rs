use serde::Serialize;
use serde_repr::Serialize_repr;

use crate::Attachment;

use super::{AllowedMentions, Component, Embed};

#[derive(Debug, Default, Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub response_type: InteractionCallbackType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData>,
}

impl InteractionResponse {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn response_type(mut self, value: InteractionCallbackType) -> Self {
        self.response_type = value;
        self
    }

    pub fn data(mut self, value: InteractionCallbackData) -> Self {
        self.data = Some(value);
        self
    }
}

#[derive(Debug, Default, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    #[default]
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
}

#[derive(Debug, Default, Serialize)]
pub struct InteractionCallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<InteractionCallbackDataFlags>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,
}

impl InteractionCallbackData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn tts(mut self, value: bool) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn content<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.content = Some(value.into());
        self
    }

    pub fn embed(mut self, value: Embed) -> Self {
        self.embeds.push(value);
        self
    }

    pub fn allowed_mentions(mut self, value: AllowedMentions) -> Self {
        self.allowed_mentions = Some(value);
        self
    }

    pub fn flags(mut self, value: InteractionCallbackDataFlags) -> Self {
        self.flags = Some(value);
        self
    }

    pub fn component(mut self, value: Component) -> Self {
        self.components.push(value);
        self
    }

    pub fn attachments(mut self, value: Attachment) -> Self {
        self.attachments.push(value);
        self
    }
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

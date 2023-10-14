use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{
    Application, Component, Emoji, GuildMember, MessageInteraction, Role, Snowflake, Sticker,
    StickerItem, User,
};

#[derive(Debug, Deserialize)]
pub struct Channel<'a> {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub guild_id: Option<Snowflake>,
    pub position: Option<u64>,
    #[serde(default)]
    pub permission_overwrites: Vec<Overwrite<'a>>,
    pub name: Option<Cow<'a, str>>,
    pub topic: Option<Cow<'a, str>>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    #[serde(default)]
    pub recipients: Vec<User<'a>>,
    pub icon: Option<Cow<'a, str>>,
    pub owner_id: Option<Snowflake>,
    pub applicaiton_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<Cow<'a, str>>,
    pub rtc_region: Option<Cow<'a, str>>,
    pub video_quality_mode: Option<u64>,
    pub message_count: Option<u64>,
    pub member_count: Option<u64>,
    pub thread_metadata: Option<ThreadMetadata<'a>>,
    pub member: Option<ThreadMember<'a>>,
    pub default_auto_archive_duration: Option<u64>,
    pub permissions: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
}

#[derive(Debug, Deserialize)]
pub struct Overwrite<'a> {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub overwrite_type: OverwriteType,
    pub allow: Cow<'a, str>,
    pub deny: Cow<'a, str>,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

#[derive(Debug, Deserialize)]
pub struct ThreadMetadata<'a> {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: Cow<'a, str>,
    pub locked: bool,
    pub invitable: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadMember<'a> {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,
    pub join_timestamp: Cow<'a, str>,
    pub flags: u64,
}

#[derive(Debug, Deserialize)]
pub struct Message<'a> {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: User<'a>,
    pub member: Option<GuildMember<'a>>,
    pub content: Cow<'a, str>,
    pub timestamp: Cow<'a, str>,
    pub edited_timestamp: Option<Cow<'a, str>>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User<'a>>,
    pub mention_roles: Vec<Role<'a>>,
    #[serde(default)]
    pub mention_channels: Vec<ChannelMention<'a>>,
    pub attachments: Vec<Attachment<'a>>,
    pub embeds: Vec<Embed<'a>>,
    #[serde(default)]
    pub reactions: Vec<Reaction<'a>>,
    pub nonce: Option<Nonce<'a>>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub activity: Option<MessageActivity<'a>>,
    pub application: Option<Application<'a>>,
    pub application_id: Option<Snowflake>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<Message<'a>>>,
    pub interaction: Option<MessageInteraction<'a>>,
    pub thread: Option<Channel<'a>>,
    #[serde(default)]
    pub components: Vec<Component<'a>>,
    #[serde(default)]
    pub sticker_items: Vec<StickerItem<'a>>,
    #[serde(default)]
    pub stickers: Vec<Sticker<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelMention<'a> {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_mention_type: ChannelType,
    pub name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment<'a> {
    pub id: Snowflake,
    pub filename: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<Cow<'a, str>>,
    pub size: u64,
    pub url: Cow<'a, str>,
    pub proxy_url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Embed<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub embed_type: Option<EmbedType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    Gifv,
    Article,
    Link,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EmbedFooter<'a> {
    pub text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImage<'a> {
    pub url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EmbedThumbnail<'a> {
    pub url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedVideo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProvider<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EmbedAuthor<'a> {
    pub name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedField<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Reaction<'a> {
    pub count: u64,
    pub me: bool,
    pub emoji: Emoji<'a>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Nonce<'a> {
    Integer(u64),
    String(Cow<'a, str>),
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
}

#[derive(Debug, Deserialize)]
pub struct MessageActivity<'a> {
    #[serde(rename = "type")]
    pub message_activity_type: MessageActivityType,
    pub party_id: Option<Cow<'a, str>>,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Debug, Deserialize)]
pub struct MessageReference {
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AllowedMentions<'a> {
    pub parse: Vec<Cow<'a, str>>,
    pub roles: Vec<Snowflake>,
    pub users: Vec<Snowflake>,
    pub replied_user: bool,
}

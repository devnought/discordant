use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{
    Application, Component, Emoji, GuildMember, MessageInteraction, Role, Snowflake, Sticker,
    StickerItem, User,
};

#[derive(Debug, Deserialize)]
pub struct Channel<'a> {
    pub id: Snowflake<'a>,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub guild_id: Option<Snowflake<'a>>,
    pub position: Option<u64>,
    pub permission_overwrites: Option<Vec<Overwrite<'a>>>,
    pub name: Option<Cow<'a, str>>,
    pub topic: Option<Cow<'a, str>>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake<'a>>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub recipients: Option<Vec<User<'a>>>,
    pub icon: Option<Cow<'a, str>>,
    pub owner_id: Option<Snowflake<'a>>,
    pub applicaiton_id: Option<Snowflake<'a>>,
    pub parent_id: Option<Snowflake<'a>>,
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

#[derive(Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
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
    pub id: Snowflake<'a>,
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
    pub id: Option<Snowflake<'a>>,
    pub user_id: Option<Snowflake<'a>>,
    pub join_timestamp: Cow<'a, str>,
    pub flags: u64,
}

#[derive(Debug, Deserialize)]
pub struct Message<'a> {
    pub id: Snowflake<'a>,
    pub channel_id: Snowflake<'a>,
    pub guild_id: Option<Snowflake<'a>>,
    pub author: User<'a>,
    pub member: Option<GuildMember<'a>>,
    pub content: Cow<'a, str>,
    pub timestamp: Cow<'a, str>,
    pub edited_timestamp: Option<Cow<'a, str>>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User<'a>>,
    pub mention_roles: Vec<Role<'a>>,
    pub mention_channels: Option<Vec<ChannelMention<'a>>>,
    pub attachments: Vec<Attachment<'a>>,
    pub embeds: Vec<Embed<'a>>,
    pub reactions: Option<Vec<Reaction<'a>>>,
    pub nonce: Option<Nonce<'a>>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake<'a>>,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub activity: Option<MessageActivity<'a>>,
    pub application: Option<Application<'a>>,
    pub application_id: Option<Snowflake<'a>>,
    pub message_reference: Option<MessageReference<'a>>,
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<Message<'a>>>,
    pub interaction: Option<MessageInteraction<'a>>,
    pub thread: Option<Channel<'a>>,
    pub components: Option<Vec<Component<'a>>>,
    pub sticker_items: Option<Vec<StickerItem<'a>>>,
    pub stickers: Option<Vec<Sticker<'a>>>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelMention<'a> {
    pub id: Snowflake<'a>,
    pub guild_id: Snowflake<'a>,
    pub channel_mention_type: ChannelType,
    pub name: Cow<'a, str>,
}

#[derive(Debug, Deserialize)]
pub struct Attachment<'a> {
    pub id: Snowflake<'a>,
    pub filename: Cow<'a, str>,
    pub content_type: Option<Cow<'a, str>>,
    pub size: u64,
    pub url: Cow<'a, str>,
    pub proxy_url: Cow<'a, str>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub ephemeral: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed<'a> {
    pub title: Option<Cow<'a, str>>,
    #[serde(rename = "type")]
    pub embed_type: Option<EmbedType>,
    pub description: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub timestamp: Option<Cow<'a, str>>,
    pub color: Option<u64>,
    pub footer: Option<EmbedFooter<'a>>,
    pub image: Option<EmbedImage<'a>>,
    pub thumbnail: Option<EmbedThumbnail<'a>>,
    pub video: Option<EmbedVideo<'a>>,
    pub provider: Option<EmbedProvider<'a>>,
    pub author: Option<EmbedAuthor<'a>>,
    pub fields: Option<Vec<EmbedField<'a>>>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooter<'a> {
    pub text: Cow<'a, str>,
    pub icon_url: Option<Cow<'a, str>>,
    pub proxy_icon_url: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImage<'a> {
    pub url: Cow<'a, str>,
    pub proxy_url: Option<Cow<'a, str>>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail<'a> {
    pub url: Cow<'a, str>,
    pub proxy_url: Option<Cow<'a, str>>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedVideo<'a> {
    pub url: Option<Cow<'a, str>>,
    pub proxy_url: Option<Cow<'a, str>>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProvider<'a> {
    pub name: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedAuthor<'a> {
    pub name: Cow<'a, str>,
    pub url: Option<Cow<'a, str>>,
    pub icon_url: Option<Cow<'a, str>>,
    pub proxy_icon_url: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedField<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
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
pub struct MessageReference<'a> {
    pub message_id: Option<Snowflake<'a>>,
    pub channel_id: Option<Snowflake<'a>>,
    pub guild_id: Option<Snowflake<'a>>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AllowedMentions<'a> {
    pub parse: Vec<Cow<'a, str>>,
    pub roles: Vec<Snowflake<'a>>,
    pub users: Vec<Snowflake<'a>>,
    pub replied_user: bool,
}

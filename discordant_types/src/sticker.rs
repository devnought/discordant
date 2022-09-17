use std::borrow::Cow;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

use super::{Snowflake, User};

#[derive(Debug, Deserialize)]
pub struct StickerItem<'a> {
    pub id: Snowflake,
    pub name: Cow<'a, str>,
    pub format_type: StickerFormat,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum StickerFormat {
    Png = 1,
    Apng = 2,
    Lottie = 3,
}

#[derive(Debug, Deserialize)]
pub struct Sticker<'a> {
    pub id: Snowflake,
    pub pack_id: Option<Snowflake>,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub tags: Cow<'a, str>,
    pub asset: Cow<'a, str>,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub format_type: StickerFormat,
    pub available: Option<bool>,
    pub guild_id: Option<Snowflake>,
    pub user: Option<User<'a>>,
    pub sort_value: Option<u64>,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum StickerType {
    Standard = 1,
    Guild = 2,
}

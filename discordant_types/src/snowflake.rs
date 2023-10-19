use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Snowflake(u64);

impl Snowflake {
    pub fn timestamp(&self) -> i64 {
        (self.0 as i64 >> 22) + 1420070400000
    }

    pub fn bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Snowflake {
    fn from(s: &str) -> Self {
        let value = s.parse().unwrap();
        Self(value)
    }
}

impl From<Snowflake> for String {
    fn from(Snowflake(value): Snowflake) -> Self {
        value.to_string()
    }
}

struct SnowflakeVisitor;

impl<'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a str to convert to a i64")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let value = v
            .parse::<u64>()
            .map_err(|e| E::custom(format!("Could not parse `{v}`: {}", e)))?;

        Ok(Snowflake(value))
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("Snowflake", &self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SnowflakeVisitor)
    }
}

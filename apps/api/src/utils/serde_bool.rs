use serde::{de, Deserializer};
use std::fmt;

pub struct PermissiveBoolVisitor;

impl<'de> de::Visitor<'de> for PermissiveBoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean or a string/number that can be interpreted as a boolean")
    }

    fn visit_bool<E>(self, value: bool) -> Result<bool, E> {
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<bool, E>
    where
        E: de::Error,
    {
        match value.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "on" | "1" => Ok(true),
            _ => Ok(false),
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<bool, E>
    where
        E: de::Error,
    {
        match value {
            1 => Ok(true),
            _ => Ok(false),
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<bool, E>
    where
        E: de::Error,
    {
        match value {
            1 => Ok(true),
            _ => Ok(false),
        }
    }

    fn visit_unit<E>(self) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_none<E>(self) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PermissiveBoolVisitor)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PermissiveBoolVisitor)
    }
}

pub fn deserialize_permissive_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(PermissiveBoolVisitor)
}

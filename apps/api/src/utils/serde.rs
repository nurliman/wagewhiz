pub mod uuid_string {
    use std::str::FromStr;

    use serde::{Deserializer, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&uuid.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid: &str = serde::Deserialize::deserialize(deserializer)?;
        Ok(Uuid::from_str(uuid).map_err(serde::de::Error::custom)?)
    }
}

pub mod uuid_string_option {
    use std::str::FromStr;

    use serde::{Deserializer, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(uuid: &Option<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match uuid {
            Some(uuid) => serializer.serialize_str(&uuid.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid: Option<&str> = serde::Deserialize::deserialize(deserializer)?;
        match uuid {
            Some(uuid) => Ok(Some(
                Uuid::from_str(uuid).map_err(serde::de::Error::custom)?,
            )),
            None => Ok(None),
        }
    }
}

pub mod string {
    use serde::{de, Deserialize, Deserializer};
    use std::{fmt, str::FromStr};

    pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: fmt::Display,
    {
        let opt = Option::<String>::deserialize(de)?;
        match opt.as_deref() {
            None | Some("") => Ok(None),
            Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
        }
    }
}

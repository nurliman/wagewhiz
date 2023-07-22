pub mod uuid_option {
    use serde::{Deserializer, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(uuid: &Option<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match uuid {
            Some(uuid) => serde::Serialize::serialize(uuid.as_bytes(), serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid: Option<[u8; 16]> = serde::Deserialize::deserialize(deserializer)?;
        
        if let Some(uuid) = uuid {
            return Ok(Some(Uuid::from_bytes(uuid)));
        }

        Ok(None)
    }
}

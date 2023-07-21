use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasetoError {
    #[error("Invalid key (string). Key must start with the specified magic string")]
    InvalidKeyString,

    #[error("Invalid key. Key must be of specified length")]
    InvalidKeyLength,
}

static KEY_MAGIC_STRINGS: Lazy<HashMap<&'static str, HashMap<&'static str, &'static str>>> =
    Lazy::new(|| {
        let mut v4 = HashMap::new();
        v4.insert("local", "k4.local.");
        v4.insert("secret", "k4.secret.");
        v4.insert("public", "k4.public.");

        let mut map = HashMap::new();
        map.insert("v4", v4);

        map
    });

static KEY_LENGTHS: Lazy<HashMap<&'static str, HashMap<&'static str, usize>>> = Lazy::new(|| {
    let mut v4 = HashMap::new();
    v4.insert("local", 32);
    v4.insert("secret", 64);
    v4.insert("public", 32);

    let mut map = HashMap::new();
    map.insert("v4", v4);

    map
});

pub enum KeyPurpose {
    Local,
    Secret,
    Public,
}

pub enum Version {
    V4,
}

pub fn parse_key_data(
    purpose: KeyPurpose,
    key: &str,
    version: Option<Version>,
) -> Result<Vec<u8>, PasetoError> {
    let version = version.unwrap_or(Version::V4);

    let magic_string = match version {
        Version::V4 => match purpose {
            KeyPurpose::Local => KEY_MAGIC_STRINGS.get("v4").unwrap().get("local").unwrap(),
            KeyPurpose::Secret => KEY_MAGIC_STRINGS.get("v4").unwrap().get("secret").unwrap(),
            KeyPurpose::Public => KEY_MAGIC_STRINGS.get("v4").unwrap().get("public").unwrap(),
        },
    };

    if !key.starts_with(magic_string) {
        return Err(PasetoError::InvalidKeyString);
    }

    // remove magic string
    let mut parts: Vec<&str> = key.split('.').collect();
    let key_data_base64 = match parts.pop() {
        Some(key_data) => key_data,
        None => return Err(PasetoError::InvalidKeyString),
    };

    // decode base64
    let key_data =
        base64_url::decode(key_data_base64).map_err(|_| PasetoError::InvalidKeyString)?;

    let key_length = match version {
        Version::V4 => match purpose {
            KeyPurpose::Local => KEY_LENGTHS.get("v4").unwrap().get("local").unwrap(),
            KeyPurpose::Secret => KEY_LENGTHS.get("v4").unwrap().get("secret").unwrap(),
            KeyPurpose::Public => KEY_LENGTHS.get("v4").unwrap().get("public").unwrap(),
        },
    };

    // check that key length is correct
    if key_data.len() != *key_length {
        return Err(PasetoError::InvalidKeyLength);
    }

    Ok(key_data)
}

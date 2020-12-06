use crate::*;
use serde::{
    de::{Error as DeErr, *},
    ser::{Error as SerErr, *},
};
use serde_with::{DeserializeAs, SerializeAs};

/// `serde_as` notation from `serde_with` crate for [`std::time::Duration`]
/// and [`chrono::Duration`] with optional `chrono` feature
pub struct DurationWrapper;

impl SerializeAs<std::time::Duration> for DurationWrapper {
    fn serialize_as<S>(duration: &std::time::Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&to_iso8601(duration))
    }
}

impl<'de> DeserializeAs<'de, std::time::Duration> for DurationWrapper {
    fn deserialize_as<D>(deserializer: D) -> Result<std::time::Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        from_iso8601(&value)
            .ok_or_else(|| D::Error::invalid_value(Unexpected::Str(&value), &"PdDThHmMsS"))
    }
}

#[cfg(feature = "chrono")]
impl SerializeAs<chrono::Duration> for DurationWrapper {
    fn serialize_as<S>(duration: &chrono::Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = duration.to_std().map_err(|_| {
            S::Error::custom(format!(
                "only positive duration supported for now but got {}",
                duration
            ))
        })?;
        serializer.serialize_str(&to_iso8601(&duration))
    }
}

#[cfg(feature = "chrono")]
impl<'de> DeserializeAs<'de, chrono::Duration> for DurationWrapper {
    fn deserialize_as<D>(deserializer: D) -> Result<chrono::Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match from_iso8601(&value) {
            None => Err(D::Error::invalid_value(
                Unexpected::Str(&value),
                &"PdDThHmMsS",
            )),
            Some(duration) => chrono::Duration::from_std(duration)
                .map_err(|_| D::Error::invalid_value(Unexpected::Str(&value), &"PdDThHmMsS")),
        }
    }
}

use serde::{de::*, ser::*};
use std::time::Duration;

use crate::*;

/// ISO 8601 serialization format for `std::time::Duration`.
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(to_iso8601(duration).as_str())
}

/// ISO 8601 deserialization format for `std::time::Duration`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    from_iso8601(&value)
        .ok_or_else(|| D::Error::invalid_value(Unexpected::Str(&value), &"PdDThHmMsS"))
}

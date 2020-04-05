use serde::{de::*, ser::*, Deserialize, Serialize};
use std::time::Duration;

use crate::direct_serde;

#[derive(Serialize, Deserialize)]
struct DurationWrapper(#[serde(with = "direct_serde")] Duration);

/// ISO 8601 serialization format for `std::time::Duration`.
pub fn serialize<S>(opt: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(duration) = opt {
        serializer.serialize_some(&DurationWrapper(*duration))
    } else {
        serializer.serialize_none()
    }
}

/// ISO 8601 deserialization format for `std::time::Duration`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<DurationWrapper>::deserialize(deserializer)
        .map(|opt_duration| opt_duration.map(|DurationWrapper(duration)| duration))
}

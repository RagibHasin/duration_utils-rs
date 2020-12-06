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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Tester(#[serde(with = "super")] Option<Duration>);

    #[test]
    fn ser_opt_dur() {
        let dur = serde_json::to_string(&Tester(Some(Duration::from_secs(1)))).unwrap();
        assert_eq!(dur, "\"PT1S\"".to_string());

        let dur = serde_json::to_string(&Tester(None)).unwrap();
        assert_eq!(dur, "null".to_string());
    }

    #[test]
    fn de_opt_dur() {
        let Tester(opt_dur) = serde_json::from_str("\"PT1S\"").unwrap();
        assert_eq!(opt_dur, Some(Duration::from_secs(1)));

        let Tester(opt_dur) = serde_json::from_str("null").unwrap();
        assert_eq!(opt_dur, None);
    }
}

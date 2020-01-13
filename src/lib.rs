use regex::Regex;
use serde::{de::*, ser::*};
use std::ops::Range;
use std::time::Duration;

const VALID_SECS: Range<u32> = 0..60;

/// Optionally creates a new `std::time::Duration` from the specified number of
/// hours, minutes and seconds.
///
/// Returns `None` if minutes or seconds is invalid.
#[inline]
pub fn from_hms_opt(h: u32, m: u32, s: u32) -> Option<Duration> {
    if VALID_SECS.contains(&s) && VALID_SECS.contains(&m) {
        Some(Duration::from_secs((h * 3600 + m * 60 + s) as u64))
    } else {
        None
    }
}

/// Creates a new `std::time::Duration` from the specified number of hours,
/// minutes and seconds.
///
/// Panics if minutes or seconds is invalid.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use duration_utils;
///
/// let duration = duration_utils::from_hms(2, 30, 45);
///
/// assert_eq!(9045, duration.as_secs());
/// assert_eq!(0, duration.subsec_nanos());
/// ```
#[inline]
pub fn from_hms(h: u32, m: u32, s: u32) -> Duration {
    from_hms_opt(h, m, s).unwrap()
}

/// Optionally creates a new `std::time::Duration` from the specified number of
/// hours, minutes, seconds and milliseconds.
///
/// Returns `None` if minutes or seconds is invalid.
#[inline]
pub fn from_hms_milli_opt(h: u32, m: u32, s: u32, milli: u32) -> Option<Duration> {
    if VALID_SECS.contains(&s) && VALID_SECS.contains(&m) && (0..1000).contains(&milli) {
        Some(Duration::new(
            (h * 3600 + m * 60 + s) as u64,
            milli * 1000_000,
        ))
    } else {
        None
    }
}

/// Creates a new `std::time::Duration` from the specified number of hours,
/// minutes, seconds and milliseconds.
///
/// Panics if minutes or seconds is invalid.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use duration_utils;
///
/// let duration = duration_utils::from_hms_milli(2, 30, 45, 50);
///
/// assert_eq!(9045, duration.as_secs());
/// assert_eq!(50, duration.subsec_millis());
/// ```
#[inline]
pub fn from_hms_milli(h: u32, m: u32, s: u32, milli: u32) -> Duration {
    from_hms_milli_opt(h, m, s, milli).unwrap()
}

/// Optionally creates a new `std::time::Duration` from the specified number of
/// hours, minutes, seconds and microseconds.
///
/// Returns `None` if minutes or seconds is invalid.
#[inline]
pub fn from_hms_micro_opt(h: u32, m: u32, s: u32, micro: u32) -> Option<Duration> {
    if VALID_SECS.contains(&s) && VALID_SECS.contains(&m) && (0..1000_000).contains(&micro) {
        Some(Duration::new((h * 3600 + m * 60 + s) as u64, micro * 1000))
    } else {
        None
    }
}

/// Creates a new `std::time::Duration` from the specified number of hours,
/// minutes, seconds and milliseconds.
///
/// Panics if minutes or seconds is invalid.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use duration_utils;
///
/// let duration = duration_utils::from_hms_micro(2, 30, 45, 50);
///
/// assert_eq!(9045, duration.as_secs());
/// assert_eq!(50, duration.subsec_micros());
/// ```
#[inline]
pub fn from_hms_micro(h: u32, m: u32, s: u32, micro: u32) -> Duration {
    from_hms_micro_opt(h, m, s, micro).unwrap()
}

/// Optionally creates a new `std::time::Duration` from the specified number of
/// hours, minutes, seconds and nanoseconds.
///
/// Returns `None` if minutes or seconds is invalid.
#[inline]
pub fn from_hms_nano_opt(h: u32, m: u32, s: u32, nano: u32) -> Option<Duration> {
    if VALID_SECS.contains(&s) && VALID_SECS.contains(&m) && (0..1000_000_000).contains(&nano) {
        Some(Duration::new((h * 3600 + m * 60 + s) as u64, nano))
    } else {
        None
    }
}

/// Creates a new `std::time::Duration` from the specified number of hours,
/// minutes, seconds and milliseconds.
///
/// Panics if minutes or seconds is invalid.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use duration_utils;
///
/// let duration = duration_utils::from_hms_nano(2, 30, 45, 50);
///
/// assert_eq!(9045, duration.as_secs());
/// assert_eq!(50, duration.subsec_nanos());
/// ```
#[inline]
pub fn from_hms_nano(h: u32, m: u32, s: u32, nano: u32) -> Duration {
    from_hms_nano_opt(h, m, s, nano).unwrap()
}

/// Converts given value to a `String` according to ISO 8601 standard with day
/// as largest unit.
pub fn to_iso8601(duration: &Duration) -> String {
    let secs = duration.as_secs_f64();

    if secs == 0.0 {
        "PT0S".to_string()
    } else {
        let d = secs as i64 / 86400;
        let h = secs as i64 % 86400 / 3600;
        let n = secs as i64 % 3600 / 60;
        let secs = secs.rem_euclid(60.0);

        let mut out = "P".to_string();
        if d != 0 {
            out += &d.to_string();
            out += "D";
        }
        if h != 0 || n != 0 || secs != 0.0 {
            out.push('T');
        }
        if h != 0 {
            out += &h.to_string();
            out += "H";
        }
        if n != 0 {
            out += &n.to_string();
            out += "M";
        }
        if secs != 0.0 {
            out += &secs.to_string();
            out += "S";
        }
        out
    }
}

/// Tries to parse a string according to ISO 8601 standard with day as largest unit.
///
/// Returns `None` on failure.
pub fn from_iso8601(value: &str) -> Option<Duration> {
    let pattern = Regex::new(
        r"([-+]?)P(?:([-+]?[0-9]+)D)?(T(?:([-+]?[0-9]+)H)?(?:([-+]?[0-9]+)M)?(?:([-+]?[0-9]+)(?:[.,]([0-9]{0,9}))?S)?)?").unwrap();

    if let Some(caps) = pattern.captures(value) {
        if caps.get(3).map_or(true, |t| t.as_str() != "T") {
            let _negate = caps.get(1).map_or(false, |neg| neg.as_str() == "-");
            let secs_in_d = if let Some(days) = caps.get(2) {
                days.as_str().parse::<f64>().unwrap() * 86400.0
            } else {
                0.0
            };
            let secs_in_h = if let Some(hours) = caps.get(4) {
                hours.as_str().parse::<f64>().unwrap() * 3600.0
            } else {
                0.0
            };
            let secs_in_m = if let Some(minutes) = caps.get(5) {
                minutes.as_str().parse::<f64>().unwrap() * 60.0
            } else {
                0.0
            };
            let secs_in_s = if let Some(seconds) = caps.get(6) {
                seconds.as_str().parse::<f64>().unwrap()
            } else {
                0.0
            };
            let secs_in_f = if let Some(fraction) = caps.get(7) {
                fraction.as_str().parse::<f64>().unwrap()
                    / 10i32.pow(fraction.as_str().len() as u32) as f64
            } else {
                0.0
            };

            let secs = secs_in_d + secs_in_h + secs_in_m + secs_in_s + secs_in_f;

            Some(Duration::from_secs_f64(secs))
        } else {
            None
        }
    } else {
        None
    }
}

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

    from_iso8601(&value).ok_or(D::Error::invalid_value(
        Unexpected::Str(&value),
        &"PdDThHmMsS",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hms_opt_s() {
        assert_eq!(from_hms_opt(2, 30, 60), None)
    }

    #[test]
    fn from_hms_opt_m() {
        assert_eq!(from_hms_opt(2, 60, 0), None)
    }

    #[test]
    fn from_hms_milli_opt_milli() {
        assert_eq!(from_hms_milli_opt(2, 30, 0, 1500), None)
    }

    #[test]
    fn from_hms_micro_opt_micro() {
        assert_eq!(from_hms_micro_opt(2, 30, 0, 1500_000), None)
    }

    #[test]
    fn from_hms_nano_opt_nano() {
        assert_eq!(from_hms_nano_opt(2, 30, 0, 1500_000_000), None)
    }

    #[test]
    fn serialize0() {
        assert_eq!(to_iso8601(&Duration::from_secs(0)), "PT0S");
    }

    #[test]
    fn serialize10() {
        assert_eq!(to_iso8601(&Duration::from_secs(10)), "PT10S");
    }

    #[test]
    fn serialize10_1() {
        assert_eq!(to_iso8601(&Duration::from_secs_f64(10.1)), "PT10.1S");
    }

    #[test]
    fn serialize100() {
        assert_eq!(to_iso8601(&Duration::from_secs(100)), "PT1M40S");
    }

    #[test]
    fn serialize10000() {
        assert_eq!(to_iso8601(&Duration::from_secs(10000)), "PT2H46M40S");
    }

    #[test]
    fn serialize1000000() {
        assert_eq!(to_iso8601(&Duration::from_secs(1000000)), "P11DT13H46M40S");
    }

    #[test]
    fn deserialize0() {
        assert_eq!(Some(Duration::from_secs(0)), from_iso8601("PT0S"));
    }

    #[test]
    fn deserialize10() {
        assert_eq!(Some(Duration::from_secs(10)), from_iso8601("PT10S"));
    }

    #[test]
    fn deserialize10_1() {
        assert_eq!(Some(Duration::from_secs_f64(10.1)), from_iso8601("PT10.1S"));
    }

    #[test]
    fn deserialize100() {
        assert_eq!(Some(Duration::from_secs(100)), from_iso8601("PT1M40S"));
    }

    #[test]
    fn deserialize10000() {
        assert_eq!(Some(Duration::from_secs(10000)), from_iso8601("PT2H46M40S"));
    }

    #[test]
    fn deserialize1000000() {
        assert_eq!(
            Some(Duration::from_secs(1000000)),
            from_iso8601("P11DT13H46M40S")
        );
    }
}

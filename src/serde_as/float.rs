use std::fmt;

use serde::de::{Error, Visitor};
use serde::{Deserializer, Serializer};
use tz::{DateTime, UtcDateTime};

use super::common::{deserialize_date_time, nanos_to_utc, project_utc, serialize_date_time};
use super::serde_with;

/// (De)serialize a (Utc)DateTime as an f64 with millisecond resolution
///
/// The nanoseconds of the input are rounded down to the next millisecond.
///
/// * For [UtcDateTime] a single `f64` is emitted, the Unix time.
/// * For [DateTime] a tuple `(f64, (i32, bool, &str))`, the Unix time,
///   and the time zone information (offset, DST, name).
///
/// Annotate a struct/enum with [`#[serde_with::serde_as]`][serde_with::serde_as],
/// and a [UtcDateTime]/[DateTime] field with [`#[serde_as(as = "Float")]`]
/// to make it [serde] serializable/deserializable.
///
/// ```
/// # #[cfg(all(feature = "serde-as-v1", not(feature = "serde-as-v2")))]
/// # pub use ::serde_with_v1 as serde_with;
/// # #[cfg(feature = "serde-as-v2")]
/// # pub use ::serde_with_v2 as serde_with;
/// #
/// use serde::{Deserialize, Serialize};
/// use serde_with::serde_as;
/// use tz::UtcDateTime;
/// use tzdb::serde_as::Float;
///
/// # #[serde_as(crate = "serde_with")]
/// # /*
/// #[serde_as]
/// # */
/// #[derive(Deserialize, Serialize)]
/// struct Foo {
///     #[serde_as(as = "Float")]
///     now: UtcDateTime,
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Float;

fn serialize_nanos(nanos: i128) -> f64 {
    // into seconds
    let nanos = nanos as f64 / 1_000_000_000_f64;
    // reduce to millisecond resolution
    (nanos * 1_000_f64).floor() / 1_000_f64
}

impl<'de> serde_with::DeserializeAs<'de, UtcDateTime> for Float {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<UtcDateTime, D::Error> {
        struct UnixTpl;

        impl Visitor<'_> for UnixTpl {
            type Value = f64;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("UnixTime")
            }

            fn visit_f64<E: Error>(self, value: f64) -> Result<Self::Value, E> {
                Ok(value)
            }

            fn visit_f32<E: Error>(self, value: f32) -> Result<Self::Value, E> {
                Ok(value as f64)
            }

            fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(value as f64)
            }

            fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(value as f64)
            }
        }

        nanos_to_utc(deserializer.deserialize_f64(UnixTpl)?)
    }
}

impl serde_with::SerializeAs<UtcDateTime> for Float {
    fn serialize_as<S: Serializer>(source: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(serialize_nanos(source.total_nanoseconds()))
    }
}

impl<'de> serde_with::DeserializeAs<'de, DateTime> for Float {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let (value, tz) = deserialize_date_time(deserializer)?;
        let utc = nanos_to_utc(value)?;
        project_utc(utc, tz)
    }
}

impl serde_with::SerializeAs<DateTime> for Float {
    fn serialize_as<S: Serializer>(source: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_date_time(
            serializer,
            source,
            serialize_nanos(source.total_nanoseconds()),
        )
    }
}

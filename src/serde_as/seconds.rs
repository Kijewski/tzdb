use std::fmt;

use serde::de::{Error, Visitor};
use serde::{Deserializer, Serializer};
use tz::{DateTime, UtcDateTime};

use super::common::{deserialize_date_time, project_utc, serialize_date_time};
use super::serde_with;

/// (De)serialize only the seconds of a (Utc)DateTime as an i64
///
/// * For [UtcDateTime] a single `i64` is emitted, the Unix time.
/// * For [DateTime] a tuple `(i64, (i32, bool, &str))`, the Unix time,
///   and the time zone information (offset, DST, name).
///
/// Annotate a struct/enum with [`#[serde_with::serde_as]`][serde_with::serde_as],
/// and a [UtcDateTime]/[DateTime] field with [`#[serde_as(as = "Seconds")]`]
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
/// use tzdb::serde_as::Seconds;
///
/// # #[serde_as(crate = "serde_with")]
/// # /*
/// #[serde_as]
/// # */
/// #[derive(Deserialize, Serialize)]
/// struct Foo {
///     #[serde_as(as = "Seconds")]
///     now: UtcDateTime,
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Seconds;

impl<'de> serde_with::DeserializeAs<'de, UtcDateTime> for Seconds {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<UtcDateTime, D::Error> {
        struct UnixTpl;

        impl Visitor<'_> for UnixTpl {
            type Value = i64;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("UnixTime")
            }

            fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(value)
            }

            fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
                value.try_into().map_err(E::custom)
            }
        }

        let secs = deserializer.deserialize_i64(UnixTpl)?;
        UtcDateTime::from_timespec(secs, 0).map_err(D::Error::custom)
    }
}

impl serde_with::SerializeAs<UtcDateTime> for Seconds {
    fn serialize_as<S: Serializer>(source: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(source.unix_time())
    }
}

impl<'de> serde_with::DeserializeAs<'de, DateTime> for Seconds {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let (secs, tz) = deserialize_date_time(deserializer)?;
        let utc = UtcDateTime::from_timespec(secs, 0).map_err(D::Error::custom)?;
        project_utc(utc, tz)
    }
}

impl serde_with::SerializeAs<DateTime> for Seconds {
    fn serialize_as<S: Serializer>(source: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_date_time(serializer, source, source.unix_time())
    }
}

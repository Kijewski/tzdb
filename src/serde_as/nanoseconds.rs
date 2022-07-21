use std::fmt;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserializer, Serializer};
use tz::{DateTime, UtcDateTime};

use super::common::{deserialize_date_time, project_utc, serialize_date_time};
use super::serde_with;

/// (De)serialize a (Utc)DateTime as a tuple with nanosecond resolution
///
/// * For [UtcDateTime] a tuple `(i64, u32)` is emitted, the Unix time and its nano seconds.
/// * For [DateTime] a tuple `((i64, u32), (i32, bool, &str))`, the Unix time and its nano seconds,
///   and the time zone information (offset, DST, name).
///
/// The nanoseconds cannot be stored as an i128, because e.g. in JavaScript the [biggest
/// integer][MAX_SAFE_INTEGER] is only 2^53 - 1.
///
/// [MAX_SAFE_INTEGER]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER
///
/// Annotate a struct/enum with [`#[serde_with::serde_as]`][serde_with::serde_as],
/// and a [UtcDateTime]/[DateTime] field with [`#[serde_as(as = "Nanoseconds")]`]
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
/// use tzdb::serde_as::Nanoseconds;
///
/// # #[serde_as(crate = "serde_with")]
/// # /*
/// #[serde_as]
/// # */
/// #[derive(Deserialize, Serialize)]
/// struct Foo {
///     #[serde_as(as = "Nanoseconds")]
///     now: UtcDateTime,
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Nanoseconds;

impl<'de> serde_with::DeserializeAs<'de, UtcDateTime> for Nanoseconds {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<UtcDateTime, D::Error> {
        struct UnixTpl;

        impl<'de> Visitor<'de> for UnixTpl {
            type Value = (i64, u32);

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("UnixTime")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let secs = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("expected seconds"))?;
                let nanos = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("expected nanoseconds"))?;
                Ok((secs, nanos))
            }
        }

        let (secs, nanos) = deserializer.deserialize_tuple(2, UnixTpl)?;
        UtcDateTime::from_timespec(secs, nanos).map_err(D::Error::custom)
    }
}

impl serde_with::SerializeAs<UtcDateTime> for Nanoseconds {
    fn serialize_as<S: Serializer>(source: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&source.unix_time())?;
        seq.serialize_element(&source.nanoseconds())?;
        seq.end()
    }
}

impl<'de> serde_with::DeserializeAs<'de, DateTime> for Nanoseconds {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let ((secs, nanos), tz) = deserialize_date_time(deserializer)?;
        let utc = UtcDateTime::from_timespec(secs, nanos).map_err(D::Error::custom)?;
        project_utc(utc, tz)
    }
}

impl serde_with::SerializeAs<DateTime> for Nanoseconds {
    fn serialize_as<S: Serializer>(source: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_date_time(
            serializer,
            source,
            (source.unix_time(), source.nanoseconds()),
        )
    }
}

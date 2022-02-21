use std::fmt;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use tz::{DateTime, UtcDateTime};

use super::common::{deserialize_date_time, serialize_date_time};
use crate::serde_as::common::project_utc;

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
/// use serde::{Deserialize, Serialize};
/// use serde_with::serde_as;
/// use tz::UtcDateTime;
/// use tzdb::serde_as::Nanoseconds;
///
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct Foo {
///     #[serde_as(as = "Nanoseconds")]
///     now: UtcDateTime,
/// }
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "serde-as")))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Nanoseconds;

impl<'de> DeserializeAs<'de, UtcDateTime> for Nanoseconds {
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

impl SerializeAs<UtcDateTime> for Nanoseconds {
    fn serialize_as<S: Serializer>(source: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&source.unix_time())?;
        seq.serialize_element(&source.nanoseconds())?;
        seq.end()
    }
}

impl<'de> DeserializeAs<'de, DateTime> for Nanoseconds {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let ((secs, nanos), tz) = deserialize_date_time(deserializer)?;
        let utc = UtcDateTime::from_timespec(secs, nanos).map_err(D::Error::custom)?;
        project_utc(utc, tz)
    }
}

impl SerializeAs<DateTime> for Nanoseconds {
    fn serialize_as<S: Serializer>(source: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_date_time(
            serializer,
            source,
            (source.unix_time(), source.nanoseconds()),
        )
    }
}

#[cfg(test)]
#[test]
fn test_seconds_nanos_tuple() {
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string};
    use serde_with::serde_as;

    use crate::time_zone::europe::BERLIN;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct UtcStruct(#[serde_as(as = "Nanoseconds")] UtcDateTime);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct DtStruct(#[serde_as(as = "Nanoseconds")] DateTime);

    #[cfg(not(feature = "testing"))]
    compile_error!("When testing, use: --features testing");

    let utc = UtcDateTime::new(2022, 3, 1, 15, 20, 37, 730296742).unwrap();
    let dt = utc.project(BERLIN).unwrap();
    assert_eq!(utc.total_nanoseconds(), 1_646_148_037_730_296_742);
    assert_eq!(dt.local_time_type().ut_offset(), 3600);

    assert_eq!(
        to_string(&UtcStruct(utc)).unwrap(),
        "[1646148037,730296742]",
    );
    assert_eq!(
        from_str::<UtcStruct>("[1646148037,730296742]").unwrap(),
        UtcStruct(utc),
    );
    assert_eq!(
        to_string(&DtStruct(dt)).unwrap(),
        r#"[[1646148037,730296742],[3600,false,"CET"]]"#,
    );
    assert_eq!(
        from_str::<DtStruct>(r#"[[1646148037,730296742],[3600,false,"CET"]]"#).unwrap(),
        DtStruct(dt),
    );
}

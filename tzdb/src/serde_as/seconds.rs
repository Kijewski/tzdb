use std::fmt;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use tz::{DateTime, UtcDateTime};

use super::common::serialize_tz;
use crate::serde_as::common::{deserialize_tz, TzTuple};

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
/// use serde::{Deserialize, Serialize};
/// use serde_with::serde_as;
/// use tz::UtcDateTime;
/// use tzdb::serde_as::Seconds;
///
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct Foo {
///     #[serde_as(as = "Seconds")]
///     now: UtcDateTime,
/// }
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "serde-as")))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Seconds;

impl<'de> DeserializeAs<'de, UtcDateTime> for Seconds {
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

impl SerializeAs<UtcDateTime> for Seconds {
    fn serialize_as<S: Serializer>(source: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(source.unix_time())
    }
}

impl<'de> DeserializeAs<'de, DateTime> for Seconds {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        struct UnixTpl;

        impl<'de> Visitor<'de> for UnixTpl {
            type Value = (i64, TzTuple<'de>);

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("UnixTime")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let ut = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("expected UnixTime"))?;
                let tz = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("expected LocalTimeType"))?;
                Ok((ut, tz))
            }
        }

        let (secs, tz) = deserializer.deserialize_tuple(2, UnixTpl)?;
        let utc = UtcDateTime::from_timespec(secs, 0).map_err(D::Error::custom)?;
        let tz = deserialize_tz(tz)?;
        utc.project(tz.as_ref()).map_err(D::Error::custom)
    }
}

impl SerializeAs<DateTime> for Seconds {
    fn serialize_as<S: Serializer>(source: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&source.unix_time())?;
        serialize_tz::<S>(source, seq)
    }
}

#[cfg(test)]
#[test]
fn test_seconds_tuple() {
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string};
    use serde_with::serde_as;

    use crate::time_zone::europe::BERLIN;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct UtcStruct(#[serde_as(as = "Seconds")] UtcDateTime);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct DtStruct(#[serde_as(as = "Seconds")] DateTime);

    #[cfg(not(feature = "testing"))]
    compile_error!("When testing, use: --features testing");

    let utc = UtcDateTime::new(2022, 3, 1, 15, 20, 37, 0).unwrap();
    let dt = utc.project(BERLIN).unwrap();
    assert_eq!(utc.unix_time(), 1_646_148_037);
    assert_eq!(dt.local_time_type().ut_offset(), 3600);

    assert_eq!(to_string(&UtcStruct(utc.clone())).unwrap(), "1646148037",);
    assert_eq!(from_str::<UtcStruct>("1646148037").unwrap(), UtcStruct(utc),);
    assert_eq!(
        to_string(&DtStruct(dt.clone())).unwrap(),
        r#"[1646148037,[3600,false,"CET"]]"#,
    );
    assert_eq!(
        from_str::<DtStruct>(r#"[1646148037,[3600,false,"CET"]]"#).unwrap(),
        DtStruct(dt),
    );
}

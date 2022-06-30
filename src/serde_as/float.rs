use std::fmt;
use std::num::FpCategory;

use serde::de::{Error, Visitor};
use serde::{Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use tz::{DateTime, UtcDateTime};

use super::common::{deserialize_date_time, serialize_date_time};
use crate::serde_as::common::project_utc;

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
/// use serde::{Deserialize, Serialize};
/// use serde_with::serde_as;
/// use tz::UtcDateTime;
/// use tzdb::serde_as::Float;
///
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct Foo {
///     #[serde_as(as = "Float")]
///     now: UtcDateTime,
/// }
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "serde-as")))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Float;

fn nanos_to_utc<E: Error>(value: f64) -> Result<UtcDateTime, E> {
    match value.classify() {
        FpCategory::Nan | FpCategory::Infinite => Err(E::custom("illegal Unix time")),
        FpCategory::Zero | FpCategory::Subnormal => {
            UtcDateTime::from_timespec(0, 0).map_err(E::custom)
        },
        FpCategory::Normal => {
            let secs = value.floor();
            let nanos = ((value - secs) * 1_000_000_000_f64) as u32;
            let nanos = nanos - nanos % 1_000_000;
            UtcDateTime::from_timespec(secs as _, nanos as _).map_err(E::custom)
        },
    }
}

fn serialize_nanos(nanos: i128) -> f64 {
    // into seconds
    let nanos = nanos as f64 / 1_000_000_000_f64;
    // reduce to millisecond resolution
    (nanos * 1_000_f64).floor() / 1_000_f64
}

impl<'de> DeserializeAs<'de, UtcDateTime> for Float {
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

impl SerializeAs<UtcDateTime> for Float {
    fn serialize_as<S: Serializer>(source: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(serialize_nanos(source.total_nanoseconds()))
    }
}

impl<'de> DeserializeAs<'de, DateTime> for Float {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let (value, tz) = deserialize_date_time(deserializer)?;
        let utc = nanos_to_utc(value)?;
        project_utc(utc, tz)
    }
}

impl SerializeAs<DateTime> for Float {
    fn serialize_as<S: Serializer>(source: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_date_time(
            serializer,
            source,
            serialize_nanos(source.total_nanoseconds()),
        )
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
    struct UtcStruct(#[serde_as(as = "Float")] UtcDateTime);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct DtStruct(#[serde_as(as = "Float")] DateTime);

    #[cfg(not(feature = "testing"))]
    compile_error!("When testing, use: --features testing");

    let utc_full = UtcDateTime::new(2022, 3, 1, 15, 20, 37, 730_296_742).unwrap();
    let utc_reduced = UtcDateTime::new(2022, 3, 1, 15, 20, 37, 730_000_000).unwrap();
    let dt_full = utc_full.project(BERLIN).unwrap();
    let dt_reduced = utc_reduced.project(BERLIN).unwrap();

    assert_eq!(to_string(&UtcStruct(utc_full)).unwrap(), "1646148037.73");
    assert_eq!(
        from_str::<UtcStruct>("1646148037.730296742").unwrap(),
        UtcStruct(utc_reduced),
    );
    assert_eq!(
        to_string(&DtStruct(dt_full)).unwrap(),
        r#"[1646148037.73,[3600,false,"CET"]]"#,
    );
    assert_eq!(
        from_str::<DtStruct>(r#"[1646148037.730296742,[3600,false,"CET"]]"#).unwrap(),
        DtStruct(dt_reduced),
    );
}

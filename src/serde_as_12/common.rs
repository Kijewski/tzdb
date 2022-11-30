use std::fmt;
use std::marker::PhantomData;

use tz::timezone::LocalTimeType;
use tz::{DateTime, TimeZoneRef, UtcDateTime};

use self::de::Error;
use self::ser::SerializeTuple;
use super::serde::{de, ser};

pub(super) type TzTuple<'de> = (i32, bool, &'de str);

pub(super) fn project_utc<E: de::Error>(
    utc: UtcDateTime,
    value: TzTuple<'_>,
) -> Result<DateTime, E> {
    let (ut_offset, is_dst, tz) = value;
    let tz = match tz {
        "" => None,
        tz => Some(tz.as_bytes()),
    };
    let tz = LocalTimeType::new(ut_offset, is_dst, tz).map_err(E::custom)?;
    let tz = &[tz];
    let tz = TimeZoneRef::new(&[], tz, &[], &None).map_err(E::custom)?;
    utc.project(tz).map_err(E::custom)
}

pub(super) fn deserialize_date_time<'de, D: de::Deserializer<'de>, T: de::Deserialize<'de>>(
    deserializer: D,
) -> Result<(T, TzTuple<'de>), D::Error> {
    struct UnixTpl<T>(PhantomData<T>);

    impl<'de, T: de::Deserialize<'de>> de::Visitor<'de> for UnixTpl<T> {
        type Value = (T, TzTuple<'de>);

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("UnixTime tuple")
        }

        fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let ut = seq
                .next_element()?
                .ok_or_else(|| A::Error::custom("expected UnixTime"))?;
            let tz = seq
                .next_element()?
                .ok_or_else(|| A::Error::custom("expected LocalTimeType"))?;
            Ok((ut, tz))
        }
    }

    deserializer.deserialize_tuple(2, UnixTpl(PhantomData))
}

pub(super) fn serialize_date_time<S: ser::Serializer, T: ser::Serialize>(
    serializer: S,
    source: &DateTime,
    date_time: T,
) -> Result<S::Ok, S::Error> {
    let local_time_type = source.local_time_type();
    let local_time_type = (
        local_time_type.ut_offset(),
        local_time_type.is_dst(),
        local_time_type.time_zone_designation(),
    );

    let mut seq = serializer.serialize_tuple(2)?;
    seq.serialize_element(&date_time)?;
    seq.serialize_element(&local_time_type)?;
    seq.end()
}

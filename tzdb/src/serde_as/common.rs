use serde::de::Error;
use serde::ser::SerializeTuple;
use serde::Serializer;
use tz::timezone::LocalTimeType;
use tz::{DateTime, TimeZone};

pub(super) type TzTuple<'de> = (i32, bool, &'de str);

pub(super) fn deserialize_tz<E: Error>(value: TzTuple<'_>) -> Result<TimeZone, E> {
    let (ut_offset, is_dst, tz) = value;
    let tz = match tz {
        "" => None,
        tz => Some(tz.as_bytes()),
    };
    let tz = LocalTimeType::new(ut_offset, is_dst, tz).map_err(E::custom)?;
    TimeZone::new(vec![], vec![tz], vec![], None).map_err(E::custom)
}

pub(super) fn serialize_tz<S: Serializer>(
    source: &DateTime,
    mut seq: <S as Serializer>::SerializeTuple,
) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
    let local_time_type = source.local_time_type();
    seq.serialize_element(&(
        local_time_type.ut_offset(),
        local_time_type.is_dst(),
        local_time_type.time_zone_designation(),
    ))?;
    seq.end()
}

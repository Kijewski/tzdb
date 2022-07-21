use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tz::{DateTime, UtcDateTime};

use super::serde_with;
use crate::time_zone::europe::BERLIN;

#[test]
fn test_seconds() {
    use super::Seconds;

    #[serde_with::serde_as(crate = "serde_with")]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct UtcStruct(#[serde_as(as = "Seconds")] UtcDateTime);

    #[serde_with::serde_as(crate = "serde_with")]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct DtStruct(#[serde_as(as = "Seconds")] DateTime);

    let utc = UtcDateTime::new(2022, 3, 1, 15, 20, 37, 0).unwrap();
    let dt = utc.project(BERLIN).unwrap();
    assert_eq!(utc.unix_time(), 1_646_148_037);
    assert_eq!(dt.local_time_type().ut_offset(), 3600);

    assert_eq!(to_string(&UtcStruct(utc)).unwrap(), "1646148037");
    assert_eq!(from_str::<UtcStruct>("1646148037").unwrap(), UtcStruct(utc));
    assert_eq!(
        to_string(&DtStruct(dt)).unwrap(),
        r#"[1646148037,[3600,false,"CET"]]"#,
    );
    assert_eq!(
        from_str::<DtStruct>(r#"[1646148037,[3600,false,"CET"]]"#).unwrap(),
        DtStruct(dt),
    );
}

#[test]
fn test_nanoseconds() {
    use super::Nanoseconds;

    #[serde_with::serde_as(crate = "serde_with")]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct UtcStruct(#[serde_as(as = "Nanoseconds")] UtcDateTime);

    #[serde_with::serde_as(crate = "serde_with")]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct DtStruct(#[serde_as(as = "Nanoseconds")] DateTime);

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

#[test]
fn test_float() {
    use super::Float;

    #[serde_with::serde_as(crate = "serde_with")]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct UtcStruct(#[serde_as(as = "Float")] UtcDateTime);

    #[serde_with::serde_as(crate = "serde_with")]
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    struct DtStruct(#[serde_as(as = "Float")] DateTime);

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

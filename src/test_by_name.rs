use crate::{time_zone, tz_by_name};

#[test]
fn test_by_name() {
    let _: tz::TimeZoneRef<'static> = tz_by_name("Europe/Berlin").unwrap();
    let _: tz::TimeZoneRef<'static> = tz_by_name("America/Dominica").unwrap();
}

#[test]
fn test_by_absent_name() {
    assert_eq!(tz_by_name("Berlin/Steglitz-Zehlendorf"), None);
}

#[test]
fn test_name_empty() {
    assert_eq!(tz_by_name(""), None);
}

#[test]
fn test_name_too_long() {
    assert_eq!(
        tz_by_name(
            "Pacific/Taumatawhakatangihangakoauauotamateaturipukakapikimaungahoronukupokaiwhenuakitanatahu"
        ),
        None,
    );
}

#[test]
fn test_static() {
    assert_eq!(
        time_zone::pacific::NAURU,
        tz_by_name("Pacific/Nauru").unwrap()
    );
}

#[test]
fn test_raw_static() {
    assert_eq!(
        time_zone::pacific::RAW_NAURU,
        crate::raw_tz_by_name("Pacific/Nauru").unwrap()
    );
}

#[test]
fn test_issue_49() {
    assert_eq!(
        time_zone::asia::HO_CHI_MINH,
        tz_by_name("Asia/Ho_Chi_Minh").unwrap()
    );
    assert_eq!(
        time_zone::asia::HO_CHI_MINH,
        tz_by_name("asia/ho_chi_minh").unwrap()
    );
    assert_eq!(
        time_zone::asia::HO_CHI_MINH,
        tz_by_name("ASIA/HO_CHI_MINH").unwrap()
    );
    assert_eq!(
        time_zone::asia::HO_CHI_MINH,
        tz_by_name("aSIA/hO_cHI_mINH").unwrap()
    );
}

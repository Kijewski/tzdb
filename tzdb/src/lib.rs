// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2022 René Kijewski <crates.io@k6i.de>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![forbid(unsafe_code)]
#![allow(unused_attributes)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(noop_method_call)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(unused_extern_crates)]
#![warn(unused_lifetimes)]
#![warn(unused_results)]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]

//! # tzdb — Time Zone Database
//!
//! [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
//! ![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.57+-important?logo=rust "Minimum Supported Rust Version")
//! [![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)
//!
//! Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).
//!
//! This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
//! currently in the version 2022d (released 2022-09-23).
//!
//! See the documentation for a full list the the contained time zones:
//! <https://docs.rs/tzdb/latest/tzdb/time_zone/index.html>
//!
//! ## Usage examples
//!
//! ```
//! use tz::{DateTime, TimeZone};
//! use tzdb::{time_zone, tz_by_name};
//!
//! // access by identifier
//! DateTime::now(time_zone::europe::KYIV);
//! // access by name
//! DateTime::now(tz_by_name("Europe/Berlin").unwrap());
//! // names are case insensitive
//! DateTime::now(tz_by_name("ArCtIc/LongYeArByEn").unwrap());
//!
//! ```
//!
//! ## Feature flags
//!
//! * `by-name` *(enabled by default)* — enables [`tz_by_name()`] to get a time zone at runtime by name
//!
//! * `list` *(enabled by default)* — enables [`TZ_NAMES`] to get a list of all shipped time zones
//!
//! * `local` *(enabled by default)* — enables [`local_tz()`] to get the system time zone
//!
//! * `serde-as` — enables the module [`serde_as`] to (de)serialize (Utc)DateTimes with [serde]
//!
//! * `binary` – make the unparsed, binary tzdata of a time zone available
//!

mod generated;
#[cfg(feature = "by-name")]
mod lower;
#[cfg(feature = "serde-as")]
pub mod serde_as;

#[cfg(feature = "docsrs")]
use serde;
#[cfg(feature = "by-name")]
use tz::TimeZoneRef;

pub use crate::generated::time_zone;

#[cfg(feature = "docsrs")]
pub mod changelog {
    #![doc = include_str!("../CHANGELOG.md")]
}

/// The version of the source Time Zone Database
pub const VERSION: &str = "2022b";

/// The SHA512 hash of the source Time Zone Database (using the "Complete Distribution")
pub const VERSION_HASH: &str = "b5bd93401c9586047e865a49b9f7fd0bdf451973be24a433786633f41a1ecc2f7eacdeb78d9a906c3526bfd3b236edab72682e5bc09add42156f8dcc2497694a";

/// Find a time zone by name, e.g. `"Europe/Berlin"` (case-insensitive)
#[cfg(feature = "by-name")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "by-name", feature = "local")))
)]
pub fn tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<TimeZoneRef<'static>> {
    let s = s.as_ref();
    if s.len() > crate::lower::FULL_TO_LOWER_MAX_LEN {
        return None;
    }
    Some(**generated::TIME_ZONES_BY_NAME.get(&crate::lower::full_to_lower(s))?)
}

/// Find the raw, unparsed time zone data by name, e.g. `"Europe/Berlin"` (case-insensitive)
#[cfg(all(feature = "binary", feature = "by-name"))]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(all(feature = "binary", any(feature = "by-name", feature = "local"),)))
)]
pub fn raw_tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<&'static [u8]> {
    let s = s.as_ref();
    if s.len() > crate::lower::FULL_TO_LOWER_MAX_LEN {
        return None;
    }
    Some(*generated::RAW_TIME_ZONES_BY_NAME.get(&crate::lower::full_to_lower(s))?)
}

/// A list of all known time zones
#[cfg(feature = "list")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "list")))]
pub const TZ_NAMES: &[&str] = &crate::generated::TIME_ZONES_LIST;

/// Find the time zone of the current system
///
/// This function uses [iana_time_zone::get_timezone()] in the background.
/// You may want to cache the output to avoid repeated filesystem accesses by get_timezone().
#[cfg(feature = "local")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "local")))]
pub fn local_tz() -> Option<TimeZoneRef<'static>> {
    tz_by_name(&iana_time_zone::get_timezone().ok()?)
}

#[cfg(all(test, feature = "by-name"))]
mod test_by_name {
    use super::*;

    #[test]
    fn test_by_name() {
        let _ = tz_by_name("Europe/Berlin").unwrap();
        let _ = tz_by_name("America/Dominica").unwrap();
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

    #[cfg(all(feature = "binary", feature = "by-name"))]
    #[test]
    fn test_raw_static() {
        assert_eq!(
            time_zone::pacific::RAW_NAURU,
            raw_tz_by_name("Pacific/Nauru").unwrap()
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

    #[test]
    fn test_trailing_spaces() {
        assert_eq!(None, tz_by_name("Pacific/Nauru "));
    }
}

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
//! currently in the version 2022e (released 2022-03-15).
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
//! DateTime::now(time_zone::europe::KIEV);
//! // access by name
//! DateTime::now(tz_by_name("Europe/Berlin").unwrap());
//! // names are case insensitive
//! DateTime::now(tz_by_name("ArCtIc/LongYeArByEn").unwrap());
//! ```
//!
//! ## Feature flags
#![cfg_attr(feature = "docsrs", doc = ::document_features::document_features!())]

mod generated;
#[cfg(feature = "serde-as")]
pub mod serde_as;

use tz::TimeZoneRef;

pub use crate::generated::time_zone;

#[cfg(feature = "docsrs")]
pub mod changelog {
    #![doc = include_str!("../CHANGELOG.md")]
}

/// Find a time zone by name, e.g. `"Europe/Berlin"` (case-insensitive)
#[cfg(feature = "by-name")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "by-name", feature = "local")))
)]
pub fn tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<TimeZoneRef<'static>> {
    use std::str::from_utf8;

    use byte_slice_cast::{AsByteSlice, AsMutByteSlice};

    let s = s.as_ref();
    let mut lower = [0u128; 2];
    lower
        .as_mut_byte_slice()
        .get_mut(..s.len())?
        .copy_from_slice(s);
    lower[0] |= 0x2020_2020_2020_2020_2020_2020_2020_2020_u128;
    lower[1] |= 0x2020_2020_2020_2020_2020_2020_2020_2020_u128;
    let lower = from_utf8(lower.as_byte_slice()).ok()?.get(..s.len())?;

    Some(**generated::TIME_ZONES_BY_NAME.get(lower)?)
}

/// Find the raw, unparsed time zone data by name, e.g. `"Europe/Berlin"` (case-insensitive)
#[cfg(all(feature = "binary", feature = "by-name"))]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(all(feature = "binary", any(feature = "by-name", feature = "local"),)))
)]
pub fn raw_tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<&'static [u8]> {
    use std::str::from_utf8;

    use byte_slice_cast::{AsByteSlice, AsMutByteSlice};

    let s = s.as_ref();
    let mut lower = [0u128; 2];
    lower
        .as_mut_byte_slice()
        .get_mut(..s.len())?
        .copy_from_slice(s);
    lower[0] |= 0x2020_2020_2020_2020_2020_2020_2020_2020_u128;
    lower[1] |= 0x2020_2020_2020_2020_2020_2020_2020_2020_u128;
    let lower = from_utf8(lower.as_byte_slice()).ok()?.get(..s.len())?;

    Some(*generated::RAW_TIME_ZONES_BY_NAME.get(lower)?)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "by-name")]
    #[test]
    fn test_by_name() {
        let _ = tz_by_name("Europe/Berlin").unwrap();
        let _ = tz_by_name("America/Dominica").unwrap();
    }

    #[cfg(feature = "by-name")]
    #[test]
    #[should_panic]
    fn test_by_absent_name() {
        let _ = tz_by_name("Berlin/Steglitz-Zehlendorf").unwrap();
    }

    #[cfg(feature = "by-name")]
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
}

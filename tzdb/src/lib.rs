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
//! use tzdb::TimeZoneExt;
//!
//! // access by identifier
//! DateTime::now(tzdb::time_zone::europe::KIEV);
//! // access by name
//! DateTime::now(TimeZone::from_db("Europe/Berlin").unwrap());
//! // names are case insensitive
//! DateTime::now(TimeZone::from_db("ArCtIc/LongYeArByEn").unwrap());
//! ```
//!
//! ## Feature flags
#![cfg_attr(feature = "docsrs", doc = ::document_features::document_features!())]

use tz::{TimeZone, TimeZoneRef};
#[cfg(feature = "serde-as")]
pub use tzdb_0_2::serde_as;
pub use tzdb_0_2::time_zone;

/// Import this trait to extend [tz::TimeZone]'s functionality
pub trait TimeZoneExt {
    /// Find a time zone by name, e.g. `"Europe/Berlin"` (case-insensitive)
    #[cfg(feature = "by-name")]
    #[cfg_attr(
        feature = "docsrs",
        doc(cfg(any(feature = "by-name", feature = "local")))
    )]
    #[inline(always)]
    fn from_db(s: &str) -> Option<TimeZoneRef<'static>> {
        tzdb_0_2::tz_by_name(s)
    }

    /// A list of all known time zones
    #[cfg(feature = "list")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "list")))]
    #[inline(always)]
    fn names_in_db() -> &'static [&'static str] {
        tzdb_0_2::TZ_NAMES
    }

    /// Find the time zone of the current system
    ///
    /// This function uses [iana_time_zone::get_timezone()](https://docs.rs/iana-time-zone/0.1.31/iana_time_zone/fn.get_timezone.html) in the background.
    /// You may want to cache the output to avoid repeated filesystem accesses by get_timezone().
    #[cfg(feature = "local")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "local")))]
    #[inline(always)]
    fn local_from_db() -> Option<TimeZoneRef<'static>> {
        tzdb_0_2::local_tz()
    }
}

impl TimeZoneExt for TimeZone {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_name() {
        let _ = TimeZone::from_db("Europe/Berlin").unwrap();
        let _ = TimeZone::from_db("America/Dominica").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_by_absent_name() {
        let _ = TimeZone::from_db("Berlin/Steglitz-Zehlendorf").unwrap();
    }

    #[test]
    fn test_static() {
        assert_eq!(
            time_zone::pacific::NAURU,
            TimeZone::from_db("Pacific/Nauru").unwrap()
        );
    }
}

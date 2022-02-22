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
//! ![Minimum supported Rust version](https://img.shields.io/badge/msrv-1.57-informational?logo=rust)
//! [![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)
//!
//! Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).
//!
//! This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
//! currently in the version 2021e (released 2021-10-21).
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
//! let access_by_identifier = DateTime::now(tzdb::time_zone::europe::KIEV);
//! let access_by_name = DateTime::now(TimeZone::from_db("Europe/Berlin").unwrap());
//! let names_are_case_insensitive = DateTime::now(TimeZone::from_db("ArCtIc/LongYeArByEn").unwrap());
//! ```
//!
//! ## Feature flags
#![cfg_attr(feature = "docsrs", doc = ::document_features::document_features!())]

mod generated;

use tz::TimeZone;

pub use crate::generated::time_zone;

/// Import this trait to extend [tz::TimeZone]'s functionality
pub trait TimeZoneExt {
    /// Find a time zone by name, e.g. `"Europe/Berlin"` (case-insensitive)
    #[cfg(feature = "by-name")]
    #[cfg_attr(
        feature = "docsrs",
        doc(cfg(any(feature = "by-name", feature = "local")))
    )]
    #[inline]
    fn from_db(s: &str) -> Option<&'static tz::statics::StaticTimeZone> {
        Some(&*crate::generated::tz_by_name(s)?)
    }

    /// A list of all known time zones
    #[cfg(feature = "list")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "list")))]
    #[inline]
    fn names_in_db() -> &'static [(&'static str, &'static tz::statics::StaticTimeZone)] {
        &crate::generated::TIME_ZONES_LIST[..]
    }

    /// Find the time zone of the current system
    #[cfg(feature = "local")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "local")))]
    #[inline]
    fn local_from_db() -> Option<&'static tz::statics::StaticTimeZone> {
        Some(&*crate::generated::tz_by_name(
            &iana_time_zone::get_timezone().ok()?,
        )?)
    }
}

impl TimeZoneExt for TimeZone {}

#[cfg(feature = "by-name")]
struct Lower32([u128; 2]);

#[cfg(feature = "by-name")]
impl Lower32 {
    #[inline]
    fn for_str<'a>(&'a mut self, s: &str) -> Option<&'a str> {
        use byte_slice_cast::{AsByteSlice, AsMutByteSlice};

        self.0
            .as_mut_byte_slice()
            .get_mut(..s.len())?
            .copy_from_slice(s.as_bytes());

        self.0[0] |= 0x2020_2020_2020_2020_2020_2020_2020_2020_u128;
        self.0[1] |= 0x2020_2020_2020_2020_2020_2020_2020_2020_u128;

        std::str::from_utf8(self.0.as_byte_slice())
            .ok()?
            .get(..s.len())
    }
}

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

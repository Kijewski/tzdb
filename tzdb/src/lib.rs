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

//! # tzdb — Time Zone Database
//!
//! [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/tzdb)](https://crates.io/crates/tzdb)
//! [![License](https://img.shields.io/crates/l/tzdb?color=informational)](/LICENSES)
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
//! let access_by_identifier = DateTime::now(tzdb::time_zone::EuropeKiev);
//! let access_by_name = DateTime::now(TimeZone::from_db("Europe/Berlin").unwrap());
//! let names_are_caseless = DateTime::now(TimeZone::from_db("ArCtIc/LongYeArByEn").unwrap());
//! ```

mod generated;

use std::fmt;
use std::ops::Deref;

use byte_slice_cast::{AsByteSlice, AsMutByteSlice};
use once_cell::race::OnceBox;
use tz::TimeZone;

pub use crate::generated::time_zone;
use crate::generated::{tz_by_name, TIME_ZONES_LIST};

/// A time zone
#[derive(Clone, Copy)]
pub struct DbTimeZone {
    index: usize,
    name: &'static str,
    debug_name: &'static str,
    bytes: &'static [u8],
    parsed: &'static OnceBox<TimeZone>,
}

impl PartialEq for DbTimeZone {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for DbTimeZone {}

impl PartialOrd for DbTimeZone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for DbTimeZone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl fmt::Display for DbTimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

impl fmt::Debug for DbTimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.debug_name)
    }
}

impl Deref for DbTimeZone {
    type Target = TimeZone;

    fn deref(&self) -> &Self::Target {
        self.parsed.get_or_init(|| {
            let tz = TimeZone::from_tz_data(self.bytes)
                .expect("could not parse time zone data, this should be impossible");
            Box::new(tz)
        })
    }
}

/// Import this trait to extend [tz::TimeZone]'s functionality
pub trait TimeZoneExt {
    /// Find a time zone by name, e.g. `"Europe/Berlin"` (caseless)
    fn from_db(s: &str) -> Option<&'static TimeZone> {
        Some(&*tz_by_name(s)?)
    }

    /// A list of all known time zones
    fn names_in_db() -> &'static [(&'static str, &'static DbTimeZone)] {
        &TIME_ZONES_LIST[..]
    }
}

impl TimeZoneExt for TimeZone {}

#[repr(align(32))]
struct Lower32([u64; 4]);

impl Lower32 {
    #[inline]
    fn for_str<'a>(&'a mut self, s: &str) -> Option<&'a str> {
        self.0
            .as_mut_byte_slice()
            .get_mut(..s.len())?
            .copy_from_slice(s.as_bytes());

        self.0[0] |= 0x2020_2020_2020_2020_u64;
        self.0[1] |= 0x2020_2020_2020_2020_u64;
        self.0[2] |= 0x2020_2020_2020_2020_u64;
        self.0[3] |= 0x2020_2020_2020_2020_u64;

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
            time_zone::PacificNauru.deref(),
            TimeZone::from_db("Pacific/Nauru").unwrap()
        );
    }

    #[test]
    fn test_sync_send() {
        trait AssertSyncSend: 'static + Sync + Send {}
        impl AssertSyncSend for DbTimeZone {}
    }
}

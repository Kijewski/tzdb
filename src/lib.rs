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

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
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
//! [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
//! ![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.55+-important?logo=rust "Minimum Supported Rust Version")
//! [![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)
//!
//! Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).
//!
//! This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
//! currently in the version 2022g (released 2022-11-29).
//!
//! See the documentation for a full list the the contained time zones:
//! <https://docs.rs/tzdb/latest/tzdb/time_zone/index.html>
//!
//! ## Usage examples
//!
//! ```
//! # #[cfg(feature = "by-name")] let _: () = {
//! use tz::{DateTime, TimeZone};
//! use tzdb::{time_zone, tz_by_name};
//!
//! // access by identifier
//! DateTime::now(time_zone::europe::KYIV);
//! // access by name
//! DateTime::now(tz_by_name("Europe/Berlin").unwrap());
//! // names are case insensitive
//! DateTime::now(tz_by_name("ArCtIc/LongYeArByEn").unwrap());
//! # };
//! ```
//!
//! ## Feature flags
//!
//! * `by-name` <sup>*(enabled by default, enabled by* `local`*)*</sup> — enables [`tz_by_name()`] to get a time zone at runtime by name
//!
//! * `list` <sup>*(enabled by default)*</sup> — enables [`TZ_NAMES`] to get a list of all shipped time zones
//!
//! * `local` <sup>*(enabled by default)*</sup> — enables [`local_tz()`] to get the system time zone
//!
//! * `binary` — make the unparsed, binary tzdata of a time zone available
//!
//! * `std` <sup>*(enabled by default)*</sup> — enable features that need the standard library [`std`]
//!
//! * `alloc` <sup>*(enabled by default, enabled by* `std`*)*</sup> — enable features that need the standard library [`alloc`]
//!
//! * `fallback` <sup>*(enabled by default)*</sup> — compile for unknown target platforms, too
//!

#[cfg(docsrs)]
extern crate alloc;
#[cfg(docsrs)]
extern crate std;

mod generated;
#[cfg(feature = "by-name")]
mod lower;
#[cfg(all(test, feature = "by-name"))]
mod test_by_name;
#[cfg(all(test, not(miri), feature = "by-name"))]
mod test_proptest;

#[cfg(feature = "local")]
use iana_time_zone::get_timezone;

pub use crate::generated::time_zone;

#[cfg(docsrs)]
pub mod changelog {
    #![doc = include_str!("../CHANGELOG.md")]
}

/// The version of the source Time Zone Database
pub const VERSION: &str = "2022g";

/// The SHA512 hash of the source Time Zone Database (using the "Complete Distribution")
pub const VERSION_HASH: &str = "f471046189f519de5735ac2d8c3edb27cbe925247b06f44634e700e5e4453ec5f715d85256fc74d300bcdaa070a7600fcc054327f2dfe743ab3c0fe404ff83c1";

/// Find a time zone by name, e.g. `"Europe/Berlin"` (case-insensitive)
///
/// # Example
///
/// ```
/// # #[cfg(feature = "by-name")] let _: () = {
/// assert_eq!(
///     tzdb::time_zone::europe::BERLIN,
///     tzdb::tz_by_name("Europe/Berlin").unwrap(),
/// );
/// # };
/// ```
#[inline]
#[cfg(feature = "by-name")]
#[cfg_attr(docsrs, doc(cfg(feature = "by-name")))]
pub fn tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<tz::TimeZoneRef<'static>> {
    fn tz_by_name(s: &[u8]) -> Option<tz::TimeZoneRef<'static>> {
        if s.len() > crate::lower::FULL_TO_LOWER_MAX_LEN {
            return None;
        }
        Some(**generated::TIME_ZONES_BY_NAME.get(&crate::lower::full_to_lower(s))?)
    }

    tz_by_name(s.as_ref())
}

/// Find the raw, unparsed time zone data by name, e.g. `"Europe/Berlin"` (case-insensitive)
///
/// # Example
///
/// ```
/// # #[cfg(all(feature = "binary", feature = "by-name"))] let _: () = {
/// assert_eq!(
///     tzdb::time_zone::europe::RAW_BERLIN,
///     tzdb::raw_tz_by_name("Europe/Berlin").unwrap(),
/// );
/// # };
/// ```
#[inline]
#[cfg(all(feature = "binary", feature = "by-name"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "binary", feature = "by-name"))))]
pub fn raw_tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<&'static [u8]> {
    fn raw_tz_by_name(s: &[u8]) -> Option<&'static [u8]> {
        if s.len() > crate::lower::FULL_TO_LOWER_MAX_LEN {
            return None;
        }
        Some(*generated::RAW_TIME_ZONES_BY_NAME.get(&crate::lower::full_to_lower(s))?)
    }

    raw_tz_by_name(s.as_ref())
}

/// A list of all known time zones
#[cfg(feature = "list")]
#[cfg_attr(docsrs, doc(cfg(feature = "list")))]
pub const TZ_NAMES: &[&str] = &crate::generated::TIME_ZONES_LIST;

/// Find the time zone of the current system
///
/// This function uses [`iana_time_zone::get_timezone()`](get_timezone) in the background.
/// You may want to cache the output to avoid repeated filesystem accesses by `get_timezone()`.
///
/// # Example
///
/// ```rust
/// # #[cfg(feature = "local")] let _: () = {
/// // Query the time zone of the local system:
/// let time_zone = tzdb::local_tz().unwrap();
/// # };
/// ```
///
/// Most likely you will want to fallback to a default time zone,
/// if the system time zone could not be determined or was not found in the database:
///
/// ```rust
/// # #[cfg(feature = "local")] let _: () = {
/// // Query the time zone of the local system:
/// let time_zone = tzdb::local_tz().unwrap_or(tzdb::time_zone::GMT);
/// # };
/// ```
#[cfg(feature = "local")]
#[cfg_attr(docsrs, doc(cfg(feature = "local")))]
#[must_use]
pub fn local_tz() -> Option<tz::TimeZoneRef<'static>> {
    tz_by_name(get_timezone().ok()?)
}

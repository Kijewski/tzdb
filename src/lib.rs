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
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/tzdb/ci.yml?branch=v0.4.x)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
//! ![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.60+-important?logo=rust "Minimum Supported Rust Version")
//! [![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)
//!
//! Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).
//!
//! This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones).
//!
//! ## Usage examples
//!
//! ```rust
//! # #[cfg(all(feature = "local", feature = "now"))] let _: () = {
//! // get the system time zone
//! let time_zone = tzdb::local_tz().unwrap();       // tz::TimeZoneRef<'_>
//! let current_time = tzdb::now::local().unwrap();  // tz::DateTime
//!
//! // access by identifier
//! let time_zone = tzdb::time_zone::europe::KYIV;
//! let current_time = tzdb::now::in_tz(tzdb::time_zone::europe::KYIV).unwrap();
//!
//! // access by name
//! let time_zone = tzdb::tz_by_name("Europe/Berlin").unwrap();
//! let current_time = tzdb::now::in_named("Europe/Berlin").unwrap();
//!
//! // names are case insensitive
//! let time_zone = tzdb::tz_by_name("ArCtIc/LongYeArByEn").unwrap();
//! let current_time = tzdb::now::in_named("ArCtIc/LongYeArByEn").unwrap();
//!
//! // provide a default time zone
//! let current_time = tzdb::now::local_or(tzdb::time_zone::GMT).unwrap();
//! let current_time = tzdb::now::in_named_or(tzdb::time_zone::GMT, "Some/City").unwrap();
//! # };
//! ```
//!
//! ## Feature flags
//!
//! * `now` <sup>*(enabled by default)*</sup> — enables the module [`now`] to get the current time
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

#[cfg(docsrs)]
pub mod changelog;
#[cfg(feature = "now")]
#[cfg_attr(docsrs, doc(cfg(feature = "now")))]
pub mod now;
#[cfg(test)]
mod test_by_name;

#[cfg(feature = "local")]
use iana_time_zone::get_timezone;
#[cfg_attr(docsrs, doc(no_inline))]
pub use tzdb_data::{time_zone, TZ_NAMES, VERSION, VERSION_HASH};

/// Find a time zone by name, e.g. `"Europe/Berlin"` (case-insensitive)
///
/// # Example
///
/// ```
/// assert_eq!(
///     tzdb::time_zone::europe::BERLIN,
///     tzdb::tz_by_name("Europe/Berlin").unwrap(),
/// );
/// ```
#[inline]
pub fn tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<tz::TimeZoneRef<'static>> {
    Some(*tzdb_data::find_tz(s.as_ref())?)
}

/// Find the raw, unparsed time zone data by name, e.g. `"Europe/Berlin"` (case-insensitive)
///
/// # Example
///
/// ```
/// assert_eq!(
///     tzdb::time_zone::europe::RAW_BERLIN,
///     tzdb::raw_tz_by_name("Europe/Berlin").unwrap(),
/// );
/// ```
#[inline]
pub fn raw_tz_by_name<S: AsRef<[u8]>>(s: S) -> Option<&'static [u8]> {
    tzdb_data::find_raw(s.as_ref())
}

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

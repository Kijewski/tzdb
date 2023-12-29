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

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unknown_lints)]
#![allow(unused_attributes)]
#![forbid(unsafe_code)]
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
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/tzdb/ci.yml?branch=v0.5.x)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
//! ![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.56+-important?logo=rust "Minimum Supported Rust Version: 1.56")
//! [![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-informational?logo=apache)](/LICENSE.md "License: Apache-2.0")
//!
//! Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).
//!
//! This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
//! currently in the version 2023d (released 2023-12-22).
//!
//! See the documentation for a full list the the contained time zones:
//! <https://docs.rs/tzdb/latest/tzdb/time_zone/index.html>
//!
//! ## Usage examples
//!
//! ```rust
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
//! ```
//!
//! ## Feature flags
//!
//! * `fallback` <sup>*(enabled by default)*</sup> — compile for unknown target platforms, too
//!

#[cfg(docsrs)]
pub mod changelog;
pub mod now;

#[cfg_attr(docsrs, doc(no_inline))]
pub use tzdb_06::{raw_tz_by_name, time_zone, tz_by_name, TZ_NAMES, VERSION, VERSION_HASH};

/// Find the time zone of the current system
///
/// This function uses [`iana_time_zone::get_timezone()`] in the background.
/// You may want to cache the output to avoid repeated filesystem accesses by `get_timezone()`.
///
/// # Example
///
/// ```rust
/// // Query the time zone of the local system:
/// let time_zone = tzdb::local_tz().unwrap();
/// ```
///
/// Most likely you will want to fallback to a default time zone,
/// if the system time zone could not be determined or was not found in the database:
///
/// ```rust
/// // Query the time zone of the local system:
/// let time_zone = tzdb::local_tz().unwrap_or(tzdb::time_zone::GMT);
/// ```
#[must_use]
pub fn local_tz() -> Option<tz::TimeZoneRef<'static>> {
    tz_by_name(iana_time_zone::get_timezone().ok()?)
}

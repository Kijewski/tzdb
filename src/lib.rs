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
//! <div style="text-align:center; font-size: larger;">tzdb v0.2.14 and higher re-export tzdb <a href="https://docs.rs/tzdb/0.5">v0.5</a></div>
//!
//!  ---
//!
//! [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
//! ![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.57+-important?logo=rust "Minimum Supported Rust Version")
//! [![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)
//!
//!
//! Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).
//!
//! This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones).
//!
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
//! ```
//!
//! ## Feature flags
//!
//! * `serde-as` — alias for `serde-as-1`
//! * `serde-as-1` — enables the module [`serde_as`] to (de)serialize (Utc)DateTimes with `serde` and `serde_with (v1)`
//! * `serde-as-2` — enables the module [`serde_as`] to (de)serialize (Utc)DateTimes with `serde` and `serde_with (v2)`
//!

#[cfg(feature = "docsrs")]
pub mod changelog;
#[cfg(any(feature = "serde_with_1", feature = "serde_with_2"))]
pub mod serde_as;
#[cfg(feature = "serde_with_1")]
pub mod serde_as_1;
#[cfg(feature = "serde_with_2")]
pub mod serde_as_2;

pub use tzdb_05::{
    local_tz, raw_tz_by_name, time_zone, tz_by_name, TZ_NAMES, VERSION, VERSION_HASH,
};

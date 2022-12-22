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
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/tzdb/ci.yml?branch=v0.3.x)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
//! ![Minimum supported Rust version: 1.55](https://img.shields.io/badge/rustc-1.55+-important?logo=rust "Minimum Supported Rust Version: 1.55")
//! [![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-informational?logo=apache)](/LICENSE.md "License: Apache-2.0")
//!
//! Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).
//!
//! See the documentation for a full list the the contained time zones:
//! <https://docs.rs/tzdb/0.3/tzdb/time_zone/index.html>
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
//! * `fallback` <sup>*(enabled by default)*</sup> — compile for unknown target platforms, too
//!

#[cfg(docsrs)]
pub mod changelog;
#[cfg(test)]
mod test_by_name;
#[cfg(test)]
mod test_proptest;

#[cfg_attr(docsrs, doc(inline))]
pub use tzdb_05::{
    local_tz, raw_tz_by_name, time_zone, tz_by_name, TZ_NAMES, VERSION, VERSION_HASH,
};

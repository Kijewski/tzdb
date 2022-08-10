// SPDX-License-Identifier: MIT-0
//
// GENERATED FILE
// ALL CHANGES MADE IN THIS FOLDER WILL BE LOST!
//
// MIT No Attribution
//
// Copyright 2022 René Kijewski <crates.io@k6i.de>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![allow(clippy::pedantic)]

#[cfg(feature = "by-name")]
use crate::lower::Lower;

/// All defined time zones statically accessible
pub mod time_zone;

#[cfg(all(feature = "tz-rs", feature = "by-name"))]
pub(crate) const TIME_ZONES_BY_NAME: phf::Map<Lower, &'static tz::TimeZoneRef<'static>> =
    include!("time_zones_by_name.inc.rs");

#[cfg(all(feature = "binary", feature = "by-name"))]
pub(crate) const RAW_TIME_ZONES_BY_NAME: phf::Map<Lower, &'static [u8]> =
    include!("raw_time_zones_by_name.inc.rs");

#[cfg(all(feature = "tz-rs", feature = "list"))]
pub(crate) const TIME_ZONES_LIST: [&str; 571] = include!("time_zones_list.inc.rs");

#[cfg(feature = "tz-rs")]
mod tzdata;

#[cfg(all(feature = "binary", feature = "list"))]
mod raw_tzdata;

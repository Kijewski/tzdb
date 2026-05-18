// GENERATED FILE
// ALL CHANGES MADE IN THIS FOLDER WILL BE LOST!

// SPDX-License-Identifier: MIT-0 OR MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2022-2026 René Kijewski <crates.io@k6i.de>

#![allow(unknown_lints)]
#![allow(clippy::pedantic)]

#[cfg(all(test, not(miri)))]
mod test_all_names;

pub(crate) mod by_name;
mod raw_tzdata;
mod tz_names;
mod tzdata;

/// All defined time zones statically accessible
pub mod time_zone;

/// The version of the source Time Zone Database
pub const VERSION: &str = "2026b";

/// The SHA512 hash of the source Time Zone Database (using the "Complete Distribution")
pub const VERSION_HASH: &str = "5ec7f74f14cd2c70a0730e3690e82bd0ba889ac26c96397c16aa08005473c2c86feb47958b52e0301810c8eb908e6d8faf998ffae75b2337a912cc9e52c0f9e9";

#[allow(unreachable_pub)] // false positive
pub use self::tz_names::TZ_NAMES;

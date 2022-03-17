# tzdb — Time Zone Database

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.57+-important?logo=rust "Minimum Supported Rust Version")
[![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)

Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).

This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
currently in the version 2022e (released 2022-03-15).

See the documentation for a full list the the contained time zones:
<https://docs.rs/tzdb/latest/tzdb/time_zone/index.html>

## Usage examples

```rust
use tz::{DateTime, TimeZone};
use tzdb::{time_zone, tz_by_name};

// access by identifier
DateTime::now(time_zone::europe::KIEV);
// access by name
DateTime::now(tz_by_name("Europe/Berlin").unwrap());
// names are case insensitive
DateTime::now(tz_by_name("ArCtIc/LongYeArByEn").unwrap());
```

## Feature flags

* `by-name` *(enabled by default)* — enables TimeZoneExt::from_db() to get a time zone at runtime by name
* `list` *(enabled by default)* — enables TimeZoneExt::names_in_db() to get a list of all shipped time zones
* `local` *(enabled by default)* — enables TimeZoneExt::local_from_db() to get the system time zone
* `serde-as` — enables the module `serde_as` to (de)serialize (Utc)DateTimes with serde
* `binary` – make the unparsed, binary tzdata of a time zone available

## Git Cloning

To clone / fork the Git repo you need to have [git-lfs](https://git-lfs.github.com/) installed.

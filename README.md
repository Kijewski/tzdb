# tzdb — Time Zone Database

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.57+-important?logo=rust "Minimum Supported Rust Version")
[![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)

Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).

This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
currently in the version 2022a (released 2022-03-15).

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

* `by-name` *(enabled by default)* — enables tz_by_name() to get a time zone at runtime by name
* `list` *(enabled by default)* — enables TZ_NAMES to get a list of all shipped time zones
* `local` *(enabled by default)* — enables local_tz() to get the system time zone
* `serde-as` — enables the module `serde_as` to (de)serialize (Utc)DateTimes with serde
* `binary` – make the unparsed, binary tzdata of a time zone available

## Git cloning

The `main` branch gets squashed regularily to keep the size of the repository at a maintainable size.
To get the history until then, please refer to the id in the initial commit.

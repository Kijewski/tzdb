# tzdb — Time Zone Database

<div style="text-align:center; font-size: larger;">tzdb v0.2.14 and higher re-export tzdb <a href="https://docs.rs/tzdb/0.5">v0.5</a></div>

 ---

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.57+-important?logo=rust "Minimum Supported Rust Version")
[![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)


Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).

This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones).


## Usage examples

```
use tz::{DateTime, TimeZone};
use tzdb::{time_zone, tz_by_name};

// access by identifier
DateTime::now(time_zone::europe::KYIV);
// access by name
DateTime::now(tz_by_name("Europe/Berlin").unwrap());
// names are case insensitive
DateTime::now(tz_by_name("ArCtIc/LongYeArByEn").unwrap());
```

## Feature flags

* `serde-as` — alias for `serde-as-1`
* `serde-as-1` — enables the module [`serde_as`] to (de)serialize (Utc)DateTimes with `serde` and `serde_with (v1)`
* `serde-as-2` — enables the module [`serde_as`] to (de)serialize (Utc)DateTimes with `serde` and `serde_with (v2)`

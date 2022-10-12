# tzdb — Time Zone Database

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.55+-important?logo=rust "Minimum Supported Rust Version")
[![License](https://img.shields.io/crates/l/tzdb?color=informational&logo=apache)](/LICENSES)

Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).

This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
currently in the version 2022d (released 2022-09-23).

See the documentation for a full list the the contained time zones:
<https://docs.rs/tzdb/latest/tzdb/time_zone/index.html>

## Usage examples

```rust
let time_zone = tzdb::local_tz()?;       // tz::TimeZoneRef<'_>
let current_time = tzdb::now::local()?;  // tz::DateTime

// access by identifier
let time_zone = tzdb::time_zone::europe::KYIV;
let current_time = tzdb::now::in_tz(tzdb::time_zone::europe::KYIV)?;

// access by name
let time_zone = tzdb::tz_by_name("Europe/Berlin")?;
let current_time = tzdb::now::in_named("Europe/Berlin")?;

// names are case insensitive
let time_zone = tzdb::tz_by_name("ArCtIc/LongYeArByEn")?;
let current_time = tzdb::now::in_named("ArCtIc/LongYeArByEn")?;

// provide a default time zone
let current_time = tzdb::now::local_or(tzdb::time_zone::GMT)?;
let current_time = tzdb::now::in_named_or(tzdb::time_zone::GMT, "Some/City")?;
```

## Feature flags

* `fallback` <sup>(enabled by default)</sup> — compile for unknown target platforms, too

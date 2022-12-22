# tzdb — Time Zone Database

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/tzdb/ci.yml?branch=v0.3.x)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tzdb?logo=rust)](https://crates.io/crates/tzdb)
![Minimum supported Rust version: 1.55](https://img.shields.io/badge/rustc-1.55+-important?logo=rust "Minimum Supported Rust Version: 1.55")
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-informational?logo=apache)](/LICENSE.md "License: Apache-2.0")

Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).

See the documentation for a full list the the contained time zones:
<https://docs.rs/tzdb/latest/0.3/time_zone/index.html>

## Usage examples

```rust
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

* `fallback` <sup>*(enabled by default)*</sup> — compile for unknown target platforms, too

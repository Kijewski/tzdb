# tzdb — Time Zone Database

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/tzdb/CI)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tzdb)](https://crates.io/crates/tzdb)
[![License](https://img.shields.io/crates/l/tzdb?color=informational)](/LICENSES)

Static time zone information for [tz-rs](https://crates.io/crates/tz-rs).

This crate provides all time zones found in the [Time Zone Database](https://www.iana.org/time-zones),
currently in the version 2021e (released 2021-10-21).

See the documentation for a full list the the contained time zones:
<https://docs.rs/tzdb/latest/tzdb/time_zone/index.html>

## Usage examples

```rs
use tz::{DateTime, TimeZone};
use tzdb::TimeZoneExt;

let access_by_identifier = DateTime::now(tzdb::time_zone::EuropeKiev);
let access_by_name = DateTime::now(TimeZone::from_db("Europe/Berlin").unwrap());
let names_are_caseless = DateTime::now(TimeZone::from_db("ArCtIc/LongYeArByEn").unwrap());
```

## Git Cloning

To clone / fork the Git repo you need to have [git-lfs](https://git-lfs.github.com/) installed.

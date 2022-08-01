## Changes between the versions

### 0.3.4 (2022-08-02)

* Fix endianess issues for PowerPCs

### 0.3.3 (2022-08-01)

* Update [tz-rs](https://crates.io/crates/tz-rs) to v0.6.12 to work in a no-std context
  ([#33](https://github.com/x-hgg-x/tz-rs/pull/33))
* Expand documentation
* Add features `std`, `alloc`, and `fallback` (unused until the next breaking change)

### 0.3.2 (2022-07-30)

* Update [iana-time-zone](https://crates.io/crates/iana-time-zone) to implement
  [`local_tz()`](https://docs.rs/tzdb/0.3.2/tzdb/fn.local_tz.html) for
  Illumos ([#44](https://github.com/strawlab/iana-time-zone/pull/44)) and
  Android ([#45](https://github.com/strawlab/iana-time-zone/pull/45))

### 0.3.1 (2022-07-23)

* Update [iana-time-zone](https://crates.io/crates/iana-time-zone) to implement
  [`local_tz()`](https://docs.rs/tzdb/0.2.6/tzdb/fn.local_tz.html) for
  iOS ([#41](https://github.com/strawlab/iana-time-zone/pull/41))

### 0.3.0 (2022-07-21)

* Remove serde-as feature. The feature is very unrelated to goals of the crate, so it should be
  moved somewhere else
* Split up `generated.rs` to speed up compilation if not all features are selected
* Reduce msrv to 1.55

### 0.2.7 (2022-06-30)

* Fix error if build and target platform have different pointer widths

### 0.2.6 (2022-06-29)

* Update [iana-time-zone](https://crates.io/crates/iana-time-zone) to implement
  [`local_tz()`](https://docs.rs/tzdb/0.2.6/tzdb/fn.local_tz.html) for
  Wasm ([#38](https://github.com/strawlab/iana-time-zone/pull/38)), and
  {Free,Net,Open,Dragonfly}BSD ([#39](https://github.com/strawlab/iana-time-zone/pull/39))

### 0.2.5 (2022-06-26)

* Ensure `-Zminimal-versions` works

### 0.2.4 (2022-06-08)

* Fix missing import if the project is used with `default-features = false`

### 0.2.3 (2022-04-15)

* Fix lookup error for names containing underscores

### 0.2.2 (2022-03-27)

* Bump dependency versions

### 0.2.1 (2022-03-27)

* Fix typos
* Introduce `VERSION` and `VERSION_HASH`

### 0.2.0 (2022-03-17)

* Update to 2022a
* Make the unparsed binary time zone data available
* Simplify the library by removing the trait TimeZoneExt:

   * `TimeZoneExt::from_db()` is now `tz_by_name()`
   * `TimeZoneExt::local_from_db()` is now `local_tz()`
   * `TimeZoneExt::names_in_db()` is now `TZ_NAMES`

### 0.1.4 (2022-03-17)

* Re-export v0.2 with old names and default features

### 0.1.3 (2022-03-03)

* Optimize `DateTime` deserialization to work without dynamic allocation
  ([tz-rs#22](https://github.com/x-hgg-x/tz-rs/pull/22))

### 0.1.2 (2022-03-02)

* Include “backzone” data to include pre-1970 information for some more time zones

### 0.1.1 (2022-03-01)

* Make `UtcDateTime`/`DateTime` serializable with `serde` using `serde_with`

### 0.1.0 (2022-02-28)

* Initial release

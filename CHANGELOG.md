## Changes between the versions

### 0.2.7

* Fix error if build and target platform have different pointer widths

### 0.2.6

* Update [iana-time-zone](https://crates.io/crates/iana-time-zone) to implement
  [`local_tz()`](https://docs.rs/tzdb/0.2.6/tzdb/fn.local_tz.html) for
  Wasm ([#38](https://github.com/strawlab/iana-time-zone/pull/38)), and
  {Free,Net,Open,Dragonfly}BSD ([#39](https://github.com/strawlab/iana-time-zone/pull/39))

### 0.2.5

* Ensure `-Zminimal-versions` works

### 0.2.4

* Fix missing import if the project is used with `default-features = false`

### 0.2.3

* Fix lookup error for names containing underscores

### 0.2.2

* Bump dependency versions

### 0.2.1

* Fix typos
* Introduce VERSION and VERSION_HASH

### 0.2.0

* Update to 2022a
* Make the unparsed binary time zone data available
* Simplify the library by removing the trait TimeZoneExt:

   * TimeZoneExt::from_db() is now tz_by_name()
   * TimeZoneExt::local_from_db() is now local_tz()
   * TimeZoneExt::names_in_db() is now TZ_NAMES

### 0.1.3

* Optimize DateTime deserialization to work without dynamic allocation
  ([tz-rs#22](https://github.com/x-hgg-x/tz-rs/pull/22))

### 0.1.2

* Include “backzone” data to include pre-1970 information for some more time zones

### 0.1.1

* Make UtcDateTime/DateTime serializable with serde using serde_with

### 0.1.0

* Initial release

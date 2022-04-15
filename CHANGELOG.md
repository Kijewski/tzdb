## Changes between the versions

### 0.2.1

* Bump dependency version numbers

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

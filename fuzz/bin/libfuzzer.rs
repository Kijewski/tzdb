#![no_main]

libfuzzer_sys::fuzz_target!(|name: &[u8]| {
    let _ = tzdb::tz_by_name(name);
});

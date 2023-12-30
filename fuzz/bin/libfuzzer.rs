#![no_main]

libfuzzer_sys::fuzz_target!(|name: &[u8]| {
    let _ = tzdb_data::find_tz(name);
});

fn main() {
    afl::fuzz!(|name: &[u8]| {
        let _ = tzdb::tz_by_name(name);
    });
}

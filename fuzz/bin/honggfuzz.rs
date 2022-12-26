fn main() {
    loop {
        honggfuzz::fuzz!(|name: &[u8]| {
            let _ = tzdb::tz_by_name(name);
        });
    }
}

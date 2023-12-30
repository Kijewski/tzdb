fn main() {
    loop {
        honggfuzz::fuzz!(|name: &[u8]| {
            let _ = tzdb_data::find_tz(name);
        });
    }
}

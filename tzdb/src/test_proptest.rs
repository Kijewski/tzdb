use proptest::collection::{vec, SizeRange};
use proptest::prelude::*;
use test_strategy::proptest;

fn ascii_string(size: impl Into<SizeRange>) -> impl Strategy<Value = String> {
    vec(proptest::char::range('\0', '\x7f'), size).prop_map(|v| v.into_iter().collect())
}

fn random_string(size: impl Into<SizeRange>) -> impl Strategy<Value = String> {
    vec(any::<char>(), size).prop_map(|v| v.into_iter().collect())
}

fn random_bytes(size: impl Into<SizeRange>) -> impl Strategy<Value = Vec<u8>> {
    vec(any::<u8>(), size)
}

#[proptest]
fn test_short_ascii_string(#[strategy(ascii_string(0..8))] s: String) {
    let _ = crate::tz_by_name(&s);
    #[cfg(feature = "binary")]
    let _ = crate::raw_tz_by_name(&s);
}

#[proptest]
fn test_ascii_string(#[strategy(ascii_string(8..40))] s: String) {
    let _ = crate::tz_by_name(&s);
    #[cfg(feature = "binary")]
    let _ = crate::raw_tz_by_name(&s);
}

#[proptest]
fn test_short_string(#[strategy(random_string(0..8))] s: String) {
    let _ = crate::tz_by_name(&s);
    #[cfg(feature = "binary")]
    let _ = crate::raw_tz_by_name(&s);
}

#[proptest]
fn test_string(#[strategy(random_string(8..40))] s: String) {
    let _ = crate::tz_by_name(&s);
    #[cfg(feature = "binary")]
    let _ = crate::raw_tz_by_name(&s);
}

#[proptest]
fn test_short_bytes(#[strategy(random_bytes(0..8))] s: Vec<u8>) {
    let _ = crate::tz_by_name(&s);
    #[cfg(feature = "binary")]
    let _ = crate::raw_tz_by_name(&s);
}

#[proptest]
fn test_bytes(#[strategy(random_bytes(8..40))] s: Vec<u8>) {
    let _ = crate::tz_by_name(&s);
    #[cfg(feature = "binary")]
    let _ = crate::raw_tz_by_name(&s);
}

#![cfg_attr(feature = "docsrs", doc(cfg(feature = "serde-as-v2")))]

//! (De)serialization helpers for [`serde`][::serde] / [`serde_with` (v2)][serde_with]

mod common;
pub(crate) mod float;
pub(crate) mod nanoseconds;
pub(crate) mod seconds;
#[cfg(test)]
mod test;

pub use float::Float;
pub use nanoseconds::Nanoseconds;
pub use seconds::Seconds;
pub(crate) use serde_with_v2 as serde_with;

#![cfg_attr(feature = "docsrs", doc(cfg(feature = "serde-as-v1")))]

//! (De)serialization helpers for [`serde`][::serde] / [`serde_with` (v1)][serde_with]

mod common;
pub(crate) mod float;
pub(crate) mod nanoseconds;
pub(crate) mod seconds;
#[cfg(test)]
mod test;

pub use float::Float;
pub use nanoseconds::Nanoseconds;
pub use seconds::Seconds;
pub(crate) use serde_with_v1 as serde_with;

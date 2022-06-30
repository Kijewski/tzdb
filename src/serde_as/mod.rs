#![cfg_attr(feature = "docsrs", doc(cfg(feature = "serde-as")))]

//! Helper functions to be used with [serde][::serde]

mod common;
mod float;
mod nanoseconds;
mod seconds;

pub use float::Float;
pub use nanoseconds::Nanoseconds;
pub use seconds::Seconds;

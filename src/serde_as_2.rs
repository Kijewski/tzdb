#![cfg_attr(feature = "docsrs", doc(cfg(feature = "serde-as-2")))]

//! Helper functions to be used with [`serde`](https://docs.rs/serde/1/serde/) and [`serde_with (v2.x.y)`](https://docs.rs/serde_with/2/serde_with/)

#[path = "./serde_as_12/common.rs"]
mod common;
#[path = "./serde_as_12/float.rs"]
mod float;
#[path = "./serde_as_12/nanoseconds.rs"]
mod nanoseconds;
#[path = "./serde_as_12/seconds.rs"]
mod seconds;

pub use float::Float;
pub use nanoseconds::Nanoseconds;
pub use seconds::Seconds;
pub(crate) use serde_with::serde;
pub(crate) use serde_with_2 as serde_with;

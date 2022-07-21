#![cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "serde-as", feature = "serde-as-v1", feature = "serde-as-v2")))
)]
#![allow(unused_imports)]

//! (De)serialization helpers for [`serde`][::serde] / [`serde_with`][serde_with]
//!
//! This mod uses the **oldest** feature selected `serde_with` version.
//! I.e. if you use the library with
//! ```toml
//! tzdb = { version = "…", features = ["serde-as-v1", "serde-as-v2"] }
//! ```
//! then [`tzdb::serde_as::Float`](crate::serde_as::Float) will be compatible to [`serde_with` v1][::serde_with_v1].
//!
//! In [`tzdb` (v0.2)](https://docs.rs/tzdb/0.2/tzdb/) the feature `serde-as` is an alias for
//! `serde-as-v1`. In version incompatible updates (`v0.3`, or `v1`) this will be changed to the
//! (currently) newest major version of `serde_with`.
//!
//! In future releases the alias will stay the
//! same beween all version compatible releases (all `0.x.*`, and `y.*.*` [y ≥ 1]).
//!

pub use serde_as::float::Float;
pub use serde_as::nanoseconds::Nanoseconds;
pub use serde_as::seconds::Seconds;
#[cfg(feature = "serde-as-v1")]
use {crate::serde_as_v1 as serde_as, serde_with_v1 as serde_with};
#[cfg(all(feature = "serde-as-v2", not(feature = "serde-as-v1")))]
use {crate::serde_as_v2 as serde_as, serde_with_v2 as serde_with};

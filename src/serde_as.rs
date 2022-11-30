#![cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "serde-as-1", feature = "serde-as-2")))
)]

//! Helper functions to be used with [`serde`](https://docs.rs/serde/1/serde/) and [`serde_with`](https://docs.rs/serde_with/)
//!
//! * If `feature = "serde-as-1"` is used, [`serde_with v1`](https://docs.rs/serde_with/1/)
//!   compatible trait are used.
//!
//! * If `feature = "serde-as-2"` is used, [`serde_with v2`](https://docs.rs/serde_with/1/)
//!   compatible trait are used.
//!
//! * If both features are used, `feature = "serde-as-2"` wins.
//!
//! * `feature = "serde-as"` is an alias for `feature = "serde-as-1"`.
//!

#[cfg(all(feature = "serde_with_1", not(feature = "serde_with_2")))]
pub use crate::serde_as_1::*;
#[cfg(feature = "serde_with_2")]
pub use crate::serde_as_2::*;

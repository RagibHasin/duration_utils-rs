//! Utils for easy creation of `std::time::Duration`.

#![deny(nonstandard_style, unused, future_incompatible, missing_docs)]

mod from;
pub use from::*;

mod direct_serde;
pub use direct_serde::*;

/// serde for `Option<Duration>`
pub mod opt;

#[cfg(feature = "serde_with")]
/// `serde_as` notation from `serde_with` crate
pub mod serde_as;

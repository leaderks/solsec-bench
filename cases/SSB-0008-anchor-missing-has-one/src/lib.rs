//! SSB-0008: an Anchor vault program. Each vault account records the
//! authority allowed to withdraw from it, and `withdraw` moves lamports out.
//!
//! The crate builds the vulnerable program by default. Enable the `fixed`
//! feature to build the reference fix instead.

#[cfg(not(feature = "fixed"))]
include!("vulnerable.rs");

#[cfg(feature = "fixed")]
include!("fixed.rs");

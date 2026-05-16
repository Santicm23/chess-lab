//! Variant-specific wrappers (Standard, Chess960).
//!

mod chess960;
mod standard;
mod three_check;

pub use chess960::*;
pub use standard::*;
pub use three_check::*;

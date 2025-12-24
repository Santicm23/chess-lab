//! # Chess Lab
//! A comprehensive Rust library for chess game development, including core logic, game rules, parsing, and support for various chess variants.
//!
//! # Modules
//! - `core`: Core chess logic and data structures.
//! - `logic`: Game rules and mechanics.
//! - `parsing`: Parsing chess notations and formats.
//! - `variants`: Different chess variants implementations.
//! - `errors`: Error handling utilities.
//!

mod common;
pub mod core;
pub mod logic;
pub mod parsing;
pub mod variants;

pub use common::errors;
pub(crate) use common::utils;

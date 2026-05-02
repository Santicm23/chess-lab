//! # Chess Lab
//! A comprehensive Rust library for chess game development, including core logic, game rules, parsing, and support for various chess variants.
//!

mod common;
pub mod core;
/// Core game logic types and gameplay APIs.
pub mod logic;
/// PGN and FEN parsing helpers.
pub mod parsing;
/// Variant-specific wrappers (Standard, Chess960).
pub mod variants;

pub use common::errors;
pub(crate) use common::utils;

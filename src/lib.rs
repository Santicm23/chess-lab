//! # Chess Lab
//! A comprehensive Rust library for chess game development, including core logic, game rules, parsing, and support for various chess variants.
//!

mod common;
pub mod core;
pub mod logic;
pub mod parsing;
pub mod variants;

pub use common::errors;
pub(crate) use common::utils;

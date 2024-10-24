//! Rust-Analyzer friendly macro utilities.
//!
//! # Backgrounds
//!
//! As of 2024, IDEs and function-like procedural macros have bad chemistry.
//! Followings are common problems targeted by this module.
//!
//! * Code completion does not work
//! * Error report ranges become ambiguous

pub mod errors;
pub use parser::*;
pub use soft_result::*;
pub use with_error::*;

mod parser;
mod soft_result;
mod with_error;

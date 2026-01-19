#![warn(clippy::pedantic)]
#![warn(missing_docs, rustdoc::all)]
#![warn(clippy::cargo)]
//! # Alphabetic
//!
//! "alphabetic" is simple crate featuring [`AlphabeticLetter`] struct, that represents a letter in Latin-script alphabet.
//!
//! Provides [`shift()`](AlphabeticLetter::shift()) method.

mod alphabetic;
pub use crate::alphabetic::*;

//! Comprehensive fretboard system for MuTheoRS
//!
//! This module provides abstract representations of musical instrument playing surfaces,
//! automatic chord fingering generation, and fingering optimization for multiple
//! instrument types including guitars, pianos, bass guitars, and violin family instruments.

#[cfg(feature = "bindgen")]
use uniffi;

pub mod errors;
pub mod traits;
pub mod types;

// Re-export specific items to avoid naming conflicts
pub use errors::{FretboardError, FretboardResult};
pub use traits::*;
pub use types::*;

//! Comprehensive fretboard system for MuTheoRS
//!
//! This module provides abstract representations of musical instrument playing surfaces,
//! automatic chord fingering generation, and fingering optimization for multiple
//! instrument types including guitars, pianos, bass guitars, and violin family instruments.

#[cfg(feature = "bindgen")]
use uniffi;

pub mod errors;
pub mod fingering;
pub mod keyboard;
pub mod presets;
pub mod stringed;
pub mod traits;
pub mod types;
pub mod visualization;
pub mod voice_leading;

// Re-export specific items to avoid naming conflicts
pub use errors::{FretboardError, FretboardResult};
pub use fingering::{
    ChordFingeringConfig, ChordFingeringGenerator, DifficultyEvaluator, DifficultyWeights,
};
pub use keyboard::KeyboardFretboard;
pub use presets::InstrumentPresets;
pub use stringed::StringedFretboard;
pub use traits::*;
pub use types::*;
pub use visualization::{DiagramConfig, FretboardDiagramGenerator};
pub use voice_leading::{SequenceAnalysis, VoiceLeadingOptimizer};

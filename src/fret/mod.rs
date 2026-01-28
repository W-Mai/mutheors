//! Comprehensive fretboard system for MuTheoRS
//!
//! This module provides abstract representations of musical instrument playing surfaces,
//! automatic chord fingering generation, and fingering optimization for multiple
//! instrument types including guitars, pianos, bass guitars, and violin family instruments.

#[cfg(feature = "bindgen")]
use uniffi;

pub mod continuous;
pub mod errors;
pub mod extensions;
pub mod fingering;
#[cfg(test)]
mod integration_tests;
pub mod keyboard;
pub mod presets;
pub mod stringed;
pub mod traits;
pub mod types;
pub mod visualization;
pub mod voice_leading;

// Re-export specific items to avoid naming conflicts
pub use continuous::ContinuousFretboard;
pub use errors::{FretboardError, FretboardResult};
pub use extensions::{DefaultExtensionRegistry, InstrumentConfigValidator};
pub use fingering::{
    ChordFingeringConfig, ChordFingeringGenerator, DifficultyEvaluator, DifficultyWeights,
};
pub use keyboard::KeyboardFretboard;
pub use presets::InstrumentPresets;
pub use stringed::StringedFretboard;
pub use traits::*;
pub use types::*;
pub use visualization::{
    DiagramConfig, DiagramData, DiagramOrientation, ExportConfig, ExportFormat,
    FingeringExportData, FingeringMetadata, FretRange, FretboardDiagramGenerator, PositionInfo,
    StringInfo,
};
pub use voice_leading::{SequenceAnalysis, VoiceLeadingOptimizer};

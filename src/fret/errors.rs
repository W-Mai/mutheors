//! Error types for the fretboard system

use crate::{Chord, Tuning};
use thiserror::Error;

#[cfg(feature = "bindgen")]
use uniffi;

/// Comprehensive error types for fretboard operations
#[derive(Debug, Error)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Error))]
pub enum FretboardError {
    /// Invalid position coordinates for the instrument
    #[error("Invalid position: {position}. Context: {context}")]
    InvalidPosition { position: String, context: String },

    /// Tuning is outside the instrument's playable range
    #[error("Tuning out of range: {tuning}. Valid range: {valid_range}")]
    TuningOutOfRange { tuning: String, valid_range: String },

    /// No valid fingerings could be generated for the given chord
    #[error("No valid fingerings found for chord: {chord}. Suggestions: {suggestions}")]
    NoValidFingerings { chord: String, suggestions: String },

    /// Invalid instrument configuration parameters
    #[error("Invalid instrument configuration: {reason}. Fix: {fix_suggestion}")]
    InvalidConfiguration {
        reason: String,
        fix_suggestion: String,
    },

    /// Requested fingering is physically impossible to play
    #[error("Fingering physically impossible: {reason}. Alternative: {alternative}")]
    ImpossibleFingering { reason: String, alternative: String },

    /// Error in tuning system calculations
    #[error("Tuning system error: {reason}. Recovery: {recovery_action}")]
    TuningSystemError {
        reason: String,
        recovery_action: String,
    },

    /// Position calculation failed
    #[error("Position calculation failed: {reason}. Fallback: {fallback}")]
    PositionCalculationError { reason: String, fallback: String },

    /// Instrument type not supported for this operation
    #[error("Unsupported instrument type: {instrument_type}. Supported types: {supported_types}")]
    UnsupportedInstrument {
        instrument_type: String,
        supported_types: String,
    },

    /// Partial chord voicing - some notes could not be fingered
    #[error("Partial chord voicing: {missing_notes} notes missing. Available voicing: {available_notes}")]
    PartialVoicing {
        missing_notes: String,
        available_notes: String,
    },

    /// Skill level mismatch - fingering too difficult for specified skill level
    #[error("Fingering too difficult for {skill_level} level. Difficulty: {difficulty:.2}. Suggestion: {suggestion}")]
    SkillLevelMismatch {
        skill_level: String,
        difficulty: f32,
        suggestion: String,
    },

    /// Cache operation failed
    #[error("Cache operation failed: {operation}. Reason: {reason}")]
    CacheError { operation: String, reason: String },

    /// Memory allocation or optimization failed
    #[error("Memory operation failed: {operation}. Available memory: {available_memory}")]
    MemoryError {
        operation: String,
        available_memory: String,
    },

    /// Extension system error
    #[error("Extension error: {extension_name}. Error: {error_details}")]
    ExtensionError {
        extension_name: String,
        error_details: String,
    },

    /// Validation failed for custom configuration
    #[error("Validation failed: {validation_type}. Issues: {issues}")]
    ValidationError {
        validation_type: String,
        issues: String,
    },
}

/// Result type for fretboard operations
pub type FretboardResult<T> = Result<T, FretboardError>;

/// Error recovery strategies and suggestions
pub struct ErrorRecovery;

impl ErrorRecovery {
    /// Generate suggestions for chord fingering failures
    pub fn chord_fingering_suggestions(chord: &Chord) -> String {
        format!(
            "Try: 1) Use a capo to transpose to easier key, 2) Try partial voicing (omit 5th), 3) Use different inversion, 4) Consider alternate tuning for chord: {}",
            chord
        )
    }

    /// Generate alternative fingering suggestions
    pub fn alternative_fingering_suggestions(reason: &str) -> String {
        if reason.contains("stretch") {
            "Try using a barre chord or different fingering pattern".to_string()
        } else if reason.contains("fret") {
            "Try playing in a different position on the neck".to_string()
        } else if reason.contains("string") {
            "Try using open strings or different string combinations".to_string()
        } else {
            "Try simplifying the chord or using a different voicing".to_string()
        }
    }

    /// Generate recovery actions for tuning system errors
    pub fn tuning_recovery_actions(reason: &str) -> String {
        if reason.contains("microtonal") {
            "Fall back to equal temperament tuning".to_string()
        } else if reason.contains("range") {
            "Transpose to playable octave".to_string()
        } else {
            "Use standard tuning as fallback".to_string()
        }
    }

    /// Generate fallback strategies for position calculations
    pub fn position_calculation_fallback(reason: &str) -> String {
        if reason.contains("cache") {
            "Recalculate without cache".to_string()
        } else if reason.contains("optimization") {
            "Use brute force calculation".to_string()
        } else {
            "Use simplified position mapping".to_string()
        }
    }

    /// Generate skill level suggestions
    pub fn skill_level_suggestions(current_level: &str, difficulty: f32) -> String {
        if difficulty > 0.8 {
            format!("This fingering is advanced level. Consider: 1) Practice easier versions first, 2) Use open chord alternatives, 3) Break down into smaller parts")
        } else if difficulty > 0.6 {
            format!("This fingering is intermediate level. Try: 1) Practice chord transitions slowly, 2) Use metronome for timing")
        } else {
            format!(
                "This fingering should be manageable for {} level",
                current_level
            )
        }
    }

    /// Generate supported instrument types list
    pub fn supported_instrument_types() -> String {
        "Guitar (6-string, 7-string, 12-string), Bass (4-string, 5-string, 6-string), Piano/Keyboard (25-88 keys), Violin family (violin, viola, cello, double bass), Mandolin, Banjo, Ukulele".to_string()
    }

    /// Generate configuration fix suggestions
    pub fn configuration_fix_suggestions(reason: &str) -> String {
        if reason.contains("string") {
            "Check string count and tuning configuration".to_string()
        } else if reason.contains("fret") {
            "Verify fret count is within valid range (1-24)".to_string()
        } else if reason.contains("tuning") {
            "Ensure all tuning values are valid musical notes".to_string()
        } else {
            "Review instrument configuration parameters".to_string()
        }
    }
}

impl FretboardError {
    /// Create an invalid position error with context
    pub fn invalid_position_with_context(
        position: impl std::fmt::Display,
        context: impl Into<String>,
    ) -> Self {
        Self::InvalidPosition {
            position: position.to_string(),
            context: context.into(),
        }
    }

    /// Create an invalid position error
    pub fn invalid_position(position: impl std::fmt::Display) -> Self {
        Self::InvalidPosition {
            position: position.to_string(),
            context: "Position coordinates are outside valid range".to_string(),
        }
    }

    /// Create a tuning out of range error with valid range info
    pub fn tuning_out_of_range_with_range(tuning: &Tuning, valid_range: impl Into<String>) -> Self {
        Self::TuningOutOfRange {
            tuning: tuning.to_string(),
            valid_range: valid_range.into(),
        }
    }

    /// Create a tuning out of range error
    pub fn tuning_out_of_range(tuning: &Tuning) -> Self {
        Self::TuningOutOfRange {
            tuning: tuning.to_string(),
            valid_range: "Check instrument's playable range".to_string(),
        }
    }

    /// Create a no valid fingerings error with suggestions
    pub fn no_valid_fingerings_with_suggestions(chord: &Chord) -> Self {
        Self::NoValidFingerings {
            chord: chord.to_string(),
            suggestions: ErrorRecovery::chord_fingering_suggestions(chord),
        }
    }

    /// Create a no valid fingerings error
    pub fn no_valid_fingerings(chord: impl std::fmt::Display) -> Self {
        Self::NoValidFingerings {
            chord: chord.to_string(),
            suggestions: "Try different chord voicing or instrument position".to_string(),
        }
    }

    /// Create an invalid configuration error with fix suggestion
    pub fn invalid_configuration_with_fix(reason: impl Into<String>) -> Self {
        let reason_str = reason.into();
        Self::InvalidConfiguration {
            fix_suggestion: ErrorRecovery::configuration_fix_suggestions(&reason_str),
            reason: reason_str,
        }
    }

    /// Create an invalid configuration error
    pub fn invalid_configuration(reason: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            reason: reason.into(),
            fix_suggestion: "Review configuration parameters".to_string(),
        }
    }

    /// Create an impossible fingering error with alternative
    pub fn impossible_fingering_with_alternative(reason: impl Into<String>) -> Self {
        let reason_str = reason.into();
        Self::ImpossibleFingering {
            alternative: ErrorRecovery::alternative_fingering_suggestions(&reason_str),
            reason: reason_str,
        }
    }

    /// Create an impossible fingering error
    pub fn impossible_fingering(reason: impl Into<String>) -> Self {
        Self::ImpossibleFingering {
            reason: reason.into(),
            alternative: "Try different fingering pattern".to_string(),
        }
    }

    /// Create a tuning system error with recovery action
    pub fn tuning_system_error_with_recovery(reason: impl Into<String>) -> Self {
        let reason_str = reason.into();
        Self::TuningSystemError {
            recovery_action: ErrorRecovery::tuning_recovery_actions(&reason_str),
            reason: reason_str,
        }
    }

    /// Create a tuning system error
    pub fn tuning_system_error(reason: impl Into<String>) -> Self {
        Self::TuningSystemError {
            reason: reason.into(),
            recovery_action: "Use standard tuning".to_string(),
        }
    }

    /// Create a position calculation error with fallback
    pub fn position_calculation_error_with_fallback(reason: impl Into<String>) -> Self {
        let reason_str = reason.into();
        Self::PositionCalculationError {
            fallback: ErrorRecovery::position_calculation_fallback(&reason_str),
            reason: reason_str,
        }
    }

    /// Create a position calculation error
    pub fn position_calculation_error(reason: impl Into<String>) -> Self {
        Self::PositionCalculationError {
            reason: reason.into(),
            fallback: "Use simplified calculation".to_string(),
        }
    }

    /// Create an unsupported instrument error
    pub fn unsupported_instrument(instrument_type: impl Into<String>) -> Self {
        Self::UnsupportedInstrument {
            instrument_type: instrument_type.into(),
            supported_types: ErrorRecovery::supported_instrument_types(),
        }
    }

    /// Create a partial voicing error
    pub fn partial_voicing(
        missing_notes: impl Into<String>,
        available_notes: impl Into<String>,
    ) -> Self {
        Self::PartialVoicing {
            missing_notes: missing_notes.into(),
            available_notes: available_notes.into(),
        }
    }

    /// Create a skill level mismatch error
    pub fn skill_level_mismatch(skill_level: impl Into<String>, difficulty: f32) -> Self {
        let skill_level_str = skill_level.into();
        Self::SkillLevelMismatch {
            suggestion: ErrorRecovery::skill_level_suggestions(&skill_level_str, difficulty),
            skill_level: skill_level_str,
            difficulty,
        }
    }

    /// Create a cache error
    pub fn cache_error(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::CacheError {
            operation: operation.into(),
            reason: reason.into(),
        }
    }

    /// Create a memory error
    pub fn memory_error(operation: impl Into<String>, available_memory: impl Into<String>) -> Self {
        Self::MemoryError {
            operation: operation.into(),
            available_memory: available_memory.into(),
        }
    }

    /// Create an extension error
    pub fn extension_error(
        extension_name: impl Into<String>,
        error_details: impl Into<String>,
    ) -> Self {
        Self::ExtensionError {
            extension_name: extension_name.into(),
            error_details: error_details.into(),
        }
    }

    /// Create a validation error
    pub fn validation_error(validation_type: impl Into<String>, issues: impl Into<String>) -> Self {
        Self::ValidationError {
            validation_type: validation_type.into(),
            issues: issues.into(),
        }
    }
}

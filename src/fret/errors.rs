//! Error types for the fretboard system

use crate::Tuning;
use thiserror::Error;

#[cfg(feature = "bindgen")]
use uniffi;

/// Comprehensive error types for fretboard operations
#[derive(Debug, Error)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Error))]
pub enum FretboardError {
    /// Invalid position coordinates for the instrument
    #[error("Invalid position: {position}")]
    InvalidPosition { position: String },

    /// Tuning is outside the instrument's playable range
    #[error("Tuning out of range: {tuning}")]
    TuningOutOfRange { tuning: String },

    /// No valid fingerings could be generated for the given chord
    #[error("No valid fingerings found for chord: {chord}")]
    NoValidFingerings { chord: String },

    /// Invalid instrument configuration parameters
    #[error("Invalid instrument configuration: {reason}")]
    InvalidConfiguration { reason: String },

    /// Requested fingering is physically impossible to play
    #[error("Fingering physically impossible: {reason}")]
    ImpossibleFingering { reason: String },

    /// Error in tuning system calculations
    #[error("Tuning system error: {reason}")]
    TuningSystemError { reason: String },

    /// Position calculation failed
    #[error("Position calculation failed: {reason}")]
    PositionCalculationError { reason: String },

    /// Instrument type not supported for this operation
    #[error("Unsupported instrument type: {instrument_type}")]
    UnsupportedInstrument { instrument_type: String },
}

/// Result type for fretboard operations
pub type FretboardResult<T> = Result<T, FretboardError>;

impl FretboardError {
    /// Create an invalid position error
    pub fn invalid_position(position: impl std::fmt::Display) -> Self {
        Self::InvalidPosition {
            position: position.to_string(),
        }
    }

    /// Create a tuning out of range error
    pub fn tuning_out_of_range(tuning: &Tuning) -> Self {
        Self::TuningOutOfRange {
            tuning: tuning.to_string(),
        }
    }

    /// Create a no valid fingerings error
    pub fn no_valid_fingerings(chord: impl std::fmt::Display) -> Self {
        Self::NoValidFingerings {
            chord: chord.to_string(),
        }
    }

    /// Create an invalid configuration error
    pub fn invalid_configuration(reason: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            reason: reason.into(),
        }
    }

    /// Create an impossible fingering error
    pub fn impossible_fingering(reason: impl Into<String>) -> Self {
        Self::ImpossibleFingering {
            reason: reason.into(),
        }
    }

    /// Create a tuning system error
    pub fn tuning_system_error(reason: impl Into<String>) -> Self {
        Self::TuningSystemError {
            reason: reason.into(),
        }
    }

    /// Create a position calculation error
    pub fn position_calculation_error(reason: impl Into<String>) -> Self {
        Self::PositionCalculationError {
            reason: reason.into(),
        }
    }

    /// Create an unsupported instrument error
    pub fn unsupported_instrument(instrument_type: impl Into<String>) -> Self {
        Self::UnsupportedInstrument {
            instrument_type: instrument_type.into(),
        }
    }
}

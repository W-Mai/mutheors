//! Core trait definitions for the fretboard system

use super::{Fingering, FretboardResult};
use crate::{Chord, Tuning};
use std::fmt::Debug;

#[cfg(feature = "bindgen")]
use uniffi;

/// Core trait for all fretboard implementations
///
/// This trait provides a unified interface for different instrument types,
/// allowing common operations while supporting instrument-specific position systems.
pub trait Fretboard {
    /// Position type specific to this instrument (e.g., StringedPosition, KeyboardPosition)
    type Position: Clone + Debug + PartialEq + Send + Sync;

    /// Configuration type for this instrument
    type Config: Clone + Debug + Send + Sync;

    /// Get the tuning produced at a specific position
    ///
    /// # Arguments
    /// * `position` - The position to query
    ///
    /// # Returns
    /// * `Some(Tuning)` if the position is valid and produces a note
    /// * `None` if the position is invalid or doesn't produce a note
    fn tuning_at_position(&self, position: &Self::Position) -> Option<Tuning>;

    /// Find all positions where a specific tuning can be played
    ///
    /// # Arguments
    /// * `tuning` - The tuning to find positions for
    ///
    /// # Returns
    /// * Vector of all valid positions that produce the given tuning
    fn positions_for_tuning(&self, tuning: &Tuning) -> Vec<Self::Position>;

    /// Check if a position is valid for this instrument
    ///
    /// # Arguments
    /// * `position` - The position to validate
    ///
    /// # Returns
    /// * `true` if the position is within the instrument's range and physically possible
    /// * `false` otherwise
    fn is_position_valid(&self, position: &Self::Position) -> bool;

    /// Get the playable range of this instrument
    ///
    /// # Returns
    /// * Tuple of (minimum_position, maximum_position) defining the instrument's range
    fn get_range(&self) -> (Self::Position, Self::Position);

    /// Get the instrument configuration
    ///
    /// # Returns
    /// * Reference to the instrument's configuration
    fn get_config(&self) -> &Self::Config;

    /// Get all valid positions within the instrument's range
    ///
    /// # Returns
    /// * Vector of all valid positions for this instrument
    fn get_all_positions(&self) -> Vec<Self::Position>;

    /// Calculate the distance between two positions (instrument-specific metric)
    ///
    /// # Arguments
    /// * `pos1` - First position
    /// * `pos2` - Second position
    ///
    /// # Returns
    /// * Distance metric between positions (lower values indicate closer positions)
    fn position_distance(&self, pos1: &Self::Position, pos2: &Self::Position) -> f32;
}

/// Trait for generating chord fingerings on a fretboard
///
/// This trait defines the interface for algorithms that can generate
/// fingering patterns for chords on specific instrument types.
pub trait FingeringGenerator<F: Fretboard> {
    /// Generate all possible fingerings for a chord
    ///
    /// # Arguments
    /// * `fretboard` - The fretboard to generate fingerings for
    /// * `chord` - The chord to generate fingerings for
    ///
    /// # Returns
    /// * Vector of all valid fingerings for the chord
    /// * Empty vector if no valid fingerings exist
    fn generate_chord_fingerings(
        &self,
        fretboard: &F,
        chord: &Chord,
    ) -> FretboardResult<Vec<Fingering<F::Position>>>;

    /// Optimize a set of fingerings by ranking them by difficulty and practicality
    ///
    /// # Arguments
    /// * `fingerings` - Vector of fingerings to optimize
    ///
    /// # Returns
    /// * Vector of fingerings sorted by preference (best first)
    fn optimize_fingerings(
        &self,
        fingerings: Vec<Fingering<F::Position>>,
    ) -> Vec<Fingering<F::Position>>;

    /// Generate fingerings within a specific range constraint
    ///
    /// # Arguments
    /// * `fretboard` - The fretboard to generate fingerings for
    /// * `chord` - The chord to generate fingerings for
    /// * `min_position` - Minimum position constraint
    /// * `max_position` - Maximum position constraint
    ///
    /// # Returns
    /// * Vector of valid fingerings within the specified range
    fn generate_chord_fingerings_in_range(
        &self,
        fretboard: &F,
        chord: &Chord,
        min_position: &F::Position,
        max_position: &F::Position,
    ) -> FretboardResult<Vec<Fingering<F::Position>>>;
}

/// Trait for evaluating fingering quality and difficulty
///
/// This trait provides methods to assess fingering patterns for
/// difficulty, physical possibility, and musical appropriateness.
pub trait FingeringEvaluator<F: Fretboard> {
    /// Evaluate the difficulty of a fingering on a scale from 0.0 (easiest) to 1.0 (hardest)
    ///
    /// # Arguments
    /// * `fingering` - The fingering to evaluate
    ///
    /// # Returns
    /// * Difficulty score between 0.0 and 1.0
    fn evaluate_difficulty(&self, fingering: &Fingering<F::Position>) -> f32;

    /// Check if a fingering is physically possible to play
    ///
    /// # Arguments
    /// * `fingering` - The fingering to check
    ///
    /// # Returns
    /// * `true` if the fingering is physically possible
    /// * `false` if the fingering requires impossible finger positions or stretches
    fn is_physically_possible(&self, fingering: &Fingering<F::Position>) -> bool;

    /// Evaluate the musical quality of a fingering (voice leading, note clarity, etc.)
    ///
    /// # Arguments
    /// * `fingering` - The fingering to evaluate
    ///
    /// # Returns
    /// * Musical quality score between 0.0 (poor) and 1.0 (excellent)
    fn evaluate_musical_quality(&self, fingering: &Fingering<F::Position>) -> f32;

    /// Calculate the transition cost between two fingerings
    ///
    /// # Arguments
    /// * `from_fingering` - Starting fingering
    /// * `to_fingering` - Target fingering
    ///
    /// # Returns
    /// * Transition cost (lower values indicate easier transitions)
    fn calculate_transition_cost(
        &self,
        from_fingering: &Fingering<F::Position>,
        to_fingering: &Fingering<F::Position>,
    ) -> f32;
}

/// Trait for instruments that support barre techniques
///
/// This trait extends the basic fretboard functionality for instruments
/// where multiple strings/keys can be pressed simultaneously with one finger.
pub trait BarreCapable<F: Fretboard> {
    /// Check if a barre is possible at the given position
    ///
    /// # Arguments
    /// * `fretboard` - The fretboard to check
    /// * `start_position` - Starting position of the barre
    /// * `end_position` - Ending position of the barre
    ///
    /// # Returns
    /// * `true` if a barre is possible between the positions
    fn can_barre(
        &self,
        fretboard: &F,
        start_position: &F::Position,
        end_position: &F::Position,
    ) -> bool;

    /// Generate barre fingerings for a chord
    ///
    /// # Arguments
    /// * `fretboard` - The fretboard to generate fingerings for
    /// * `chord` - The chord to generate barre fingerings for
    ///
    /// # Returns
    /// * Vector of fingerings that use barre techniques
    fn generate_barre_fingerings(
        &self,
        fretboard: &F,
        chord: &Chord,
    ) -> FretboardResult<Vec<Fingering<F::Position>>>;
}

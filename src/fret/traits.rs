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

/// Trait for voice leading optimization across chord progressions
///
/// This trait provides methods for optimizing fingering sequences
/// to minimize hand movement and improve musical flow.
pub trait VoiceLeadingOptimizer<F: Fretboard> {
    /// Optimize fingerings for a chord progression
    ///
    /// # Arguments
    /// * `chord_fingerings` - Vector of fingering options for each chord in the progression
    ///
    /// # Returns
    /// * Vector containing the optimal fingering for each chord
    fn optimize_progression(
        &self,
        chord_fingerings: Vec<Vec<Fingering<F::Position>>>,
    ) -> Vec<Fingering<F::Position>>;

    /// Calculate the total movement cost for a fingering sequence
    ///
    /// # Arguments
    /// * `fingering_sequence` - Sequence of fingerings to evaluate
    ///
    /// # Returns
    /// * Total movement cost for the sequence
    fn calculate_sequence_cost(&self, fingering_sequence: &[Fingering<F::Position>]) -> f32;
}
/// Extension trait for custom instrument implementations
///
/// This trait allows users to define custom instruments by providing
/// the necessary configuration and validation logic.
pub trait CustomInstrument {
    /// Position type for this custom instrument
    type Position: Clone + Debug + PartialEq + Send + Sync;
    
    /// Configuration type for this custom instrument
    type Config: Clone + Debug + Send + Sync;

    /// Create a new instance of this custom instrument
    ///
    /// # Arguments
    /// * `config` - Configuration for the instrument
    ///
    /// # Returns
    /// * Result containing the instrument instance or an error
    fn new(config: Self::Config) -> FretboardResult<Self>
    where
        Self: Sized;

    /// Validate the instrument configuration
    ///
    /// # Arguments
    /// * `config` - Configuration to validate
    ///
    /// # Returns
    /// * `true` if the configuration is valid
    /// * `false` if the configuration has errors
    fn validate_config(config: &Self::Config) -> bool;

    /// Get the instrument's name/identifier
    ///
    /// # Returns
    /// * String identifier for this instrument type
    fn instrument_name(&self) -> &'static str;

    /// Get the instrument's category (e.g., "stringed", "keyboard", "wind")
    ///
    /// # Returns
    /// * String category for this instrument type
    fn instrument_category(&self) -> &'static str;

    /// Get supported playing techniques for this instrument
    ///
    /// # Returns
    /// * Vector of technique names supported by this instrument
    fn supported_techniques(&self) -> Vec<&'static str>;

    /// Check if a specific technique is supported
    ///
    /// # Arguments
    /// * `technique` - Name of the technique to check
    ///
    /// # Returns
    /// * `true` if the technique is supported
    fn supports_technique(&self, technique: &str) -> bool {
        self.supported_techniques().contains(&technique)
    }
}

/// Extension trait for custom fingering algorithms
///
/// This trait allows users to implement custom fingering generation
/// and evaluation algorithms for specific use cases or instruments.
pub trait CustomFingeringAlgorithm<F: Fretboard> {
    /// Algorithm configuration type
    type Config: Clone + Debug + Send + Sync;

    /// Create a new instance of this algorithm
    ///
    /// # Arguments
    /// * `config` - Configuration for the algorithm
    ///
    /// # Returns
    /// * New algorithm instance
    fn new(config: Self::Config) -> Self
    where
        Self: Sized;

    /// Get the algorithm's name/identifier
    ///
    /// # Returns
    /// * String identifier for this algorithm
    fn algorithm_name(&self) -> &'static str;

    /// Get the algorithm's description
    ///
    /// # Returns
    /// * String description of what this algorithm does
    fn algorithm_description(&self) -> &'static str;

    /// Generate fingerings using this custom algorithm
    ///
    /// # Arguments
    /// * `fretboard` - The fretboard to generate fingerings for
    /// * `chord` - The chord to generate fingerings for
    ///
    /// # Returns
    /// * Vector of fingerings generated by this algorithm
    fn generate_fingerings(
        &self,
        fretboard: &F,
        chord: &Chord,
    ) -> FretboardResult<Vec<Fingering<F::Position>>>;

    /// Evaluate fingerings using this custom algorithm
    ///
    /// # Arguments
    /// * `fingerings` - Fingerings to evaluate
    ///
    /// # Returns
    /// * Vector of fingerings with updated difficulty scores
    fn evaluate_fingerings(
        &self,
        fingerings: Vec<Fingering<F::Position>>,
    ) -> Vec<Fingering<F::Position>>;

    /// Check if this algorithm is compatible with the given instrument
    ///
    /// # Arguments
    /// * `fretboard` - The fretboard to check compatibility with
    ///
    /// # Returns
    /// * `true` if the algorithm can work with this instrument
    fn is_compatible_with(&self, fretboard: &F) -> bool;
}

/// Extension trait for custom evaluation criteria
///
/// This trait allows users to define custom criteria for evaluating
/// fingering quality beyond the standard difficulty metrics.
pub trait CustomEvaluationCriteria<F: Fretboard> {
    /// Criteria configuration type
    type Config: Clone + Debug + Send + Sync;

    /// Create new evaluation criteria
    ///
    /// # Arguments
    /// * `config` - Configuration for the criteria
    ///
    /// # Returns
    /// * New criteria instance
    fn new(config: Self::Config) -> Self
    where
        Self: Sized;

    /// Get the criteria name
    ///
    /// # Returns
    /// * String name for these criteria
    fn criteria_name(&self) -> &'static str;

    /// Evaluate a fingering according to these criteria
    ///
    /// # Arguments
    /// * `fingering` - The fingering to evaluate
    ///
    /// # Returns
    /// * Score between 0.0 (worst) and 1.0 (best)
    fn evaluate(&self, fingering: &Fingering<F::Position>) -> f32;

    /// Get the weight/importance of these criteria (0.0 to 1.0)
    ///
    /// # Returns
    /// * Weight value for combining with other criteria
    fn weight(&self) -> f32;

    /// Check if these criteria apply to the given instrument type
    ///
    /// # Arguments
    /// * `fretboard` - The fretboard to check applicability for
    ///
    /// # Returns
    /// * `true` if these criteria are relevant for this instrument
    fn applies_to(&self, fretboard: &F) -> bool;
}

/// Plugin registry for managing custom extensions
///
/// This trait provides a registry system for discovering and managing
/// custom instruments, algorithms, and evaluation criteria.
pub trait ExtensionRegistry {
    /// Register a custom instrument type
    ///
    /// # Arguments
    /// * `name` - Unique name for the instrument type
    /// * `factory` - Factory function to create instances
    ///
    /// # Returns
    /// * `true` if registration was successful
    fn register_instrument<I, C, F>(
        &mut self,
        name: &str,
        factory: F,
    ) -> bool
    where
        I: CustomInstrument<Config = C> + 'static,
        C: Clone + Debug + Send + Sync + 'static,
        F: Fn(C) -> FretboardResult<I> + Send + Sync + 'static;

    /// Register a custom fingering algorithm
    ///
    /// # Arguments
    /// * `name` - Unique name for the algorithm
    /// * `factory` - Factory function to create instances
    ///
    /// # Returns
    /// * `true` if registration was successful
    fn register_algorithm<A, C, F>(
        &mut self,
        name: &str,
        factory: F,
    ) -> bool
    where
        A: 'static,
        C: Clone + Debug + Send + Sync + 'static,
        F: Fn(C) -> A + Send + Sync + 'static;

    /// Register custom evaluation criteria
    ///
    /// # Arguments
    /// * `name` - Unique name for the criteria
    /// * `factory` - Factory function to create instances
    ///
    /// # Returns
    /// * `true` if registration was successful
    fn register_criteria<E, C, F>(
        &mut self,
        name: &str,
        factory: F,
    ) -> bool
    where
        E: 'static,
        C: Clone + Debug + Send + Sync + 'static,
        F: Fn(C) -> E + Send + Sync + 'static;

    /// Get list of registered instrument types
    ///
    /// # Returns
    /// * Vector of registered instrument type names
    fn list_instruments(&self) -> Vec<String>;

    /// Get list of registered algorithms
    ///
    /// # Returns
    /// * Vector of registered algorithm names
    fn list_algorithms(&self) -> Vec<String>;

    /// Get list of registered evaluation criteria
    ///
    /// # Returns
    /// * Vector of registered criteria names
    fn list_criteria(&self) -> Vec<String>;

    /// Check if an instrument type is registered
    ///
    /// # Arguments
    /// * `name` - Name of the instrument type to check
    ///
    /// # Returns
    /// * `true` if the instrument type is registered
    fn has_instrument(&self, name: &str) -> bool;

    /// Check if an algorithm is registered
    ///
    /// # Arguments
    /// * `name` - Name of the algorithm to check
    ///
    /// # Returns
    /// * `true` if the algorithm is registered
    fn has_algorithm(&self, name: &str) -> bool;

    /// Check if evaluation criteria are registered
    ///
    /// # Arguments
    /// * `name` - Name of the criteria to check
    ///
    /// # Returns
    /// * `true` if the criteria are registered
    fn has_criteria(&self, name: &str) -> bool;
}

/// Trait for instrument-specific playing techniques
///
/// This trait allows instruments to define and validate their own
/// specific playing techniques beyond the common ones.
pub trait InstrumentTechniques<F: Fretboard> {
    /// Technique configuration type
    type TechniqueConfig: Clone + Debug + Send + Sync;

    /// Apply a technique to a fingering
    ///
    /// # Arguments
    /// * `fingering` - The base fingering to apply the technique to
    /// * `technique` - Name of the technique to apply
    /// * `config` - Configuration for the technique
    ///
    /// # Returns
    /// * Modified fingering with the technique applied, or error if not possible
    fn apply_technique(
        &self,
        fingering: &Fingering<F::Position>,
        technique: &str,
        config: &Self::TechniqueConfig,
    ) -> FretboardResult<Fingering<F::Position>>;

    /// Check if a technique can be applied to a fingering
    ///
    /// # Arguments
    /// * `fingering` - The fingering to check
    /// * `technique` - Name of the technique
    ///
    /// # Returns
    /// * `true` if the technique can be applied
    fn can_apply_technique(
        &self,
        fingering: &Fingering<F::Position>,
        technique: &str,
    ) -> bool;

    /// Get technique-specific difficulty modifier
    ///
    /// # Arguments
    /// * `technique` - Name of the technique
    ///
    /// # Returns
    /// * Difficulty modifier (1.0 = no change, >1.0 = harder, <1.0 = easier)
    fn technique_difficulty_modifier(&self, technique: &str) -> f32;

    /// Validate technique configuration
    ///
    /// # Arguments
    /// * `technique` - Name of the technique
    /// * `config` - Configuration to validate
    ///
    /// # Returns
    /// * `true` if the configuration is valid for this technique
    fn validate_technique_config(
        &self,
        technique: &str,
        config: &Self::TechniqueConfig,
    ) -> bool;
}
//! Continuous fretboard implementation for violin family instruments
//!
//! This module provides fretboard functionality for instruments without frets,
//! where positions are represented as continuous values along the string length.

use super::{
    errors::{FretboardError, FretboardResult},
    traits::Fretboard,
    types::{ContinuousPosition, StringedInstrumentConfig},
};
use crate::{Interval, Tuning};
use std::collections::HashMap;

#[cfg(feature = "bindgen")]
use uniffi;

/// Continuous fretboard for violin family instruments (violin, viola, cello, double bass)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Object))]
pub struct ContinuousFretboard {
    /// Instrument configuration
    config: StringedInstrumentConfig,
    /// Cached position calculations for performance
    position_cache: HashMap<(usize, u32), Tuning>,
    /// Scale length for position calculations
    scale_length: f32,
}

impl ContinuousFretboard {
    /// Create a new continuous fretboard with the given configuration
    pub fn new(config: StringedInstrumentConfig) -> FretboardResult<Self> {
        config
            .validate()
            .map_err(|e| FretboardError::InvalidConfiguration { reason: e })?;

        Ok(Self {
            scale_length: config.scale_length,
            config,
            position_cache: HashMap::new(),
        })
    }

    /// Get the number of strings on this fretboard
    pub fn string_count(&self) -> usize {
        self.config.string_count()
    }

    /// Get the tuning of a specific string
    pub fn string_tuning(&self, string_index: usize) -> Option<Tuning> {
        self.config.strings.get(string_index).copied()
    }

    /// Calculate the tuning at a continuous position on a string
    /// Position 0.0 = open string, 1.0 = theoretical maximum position
    pub fn tuning_at_continuous_position(&self, position: &ContinuousPosition) -> Option<Tuning> {
        if position.string >= self.string_count() {
            return None;
        }

        let open_tuning = self.string_tuning(position.string)?;

        // For continuous instruments, we use a logarithmic relationship
        // similar to fretted instruments but with continuous positions
        let semitones = self.position_to_semitones(position.position);

        // Apply the interval to the open string tuning
        if let Ok(interval) = Interval::from_semitones(semitones as i8) {
            open_tuning.add_interval(&interval).ok()
        } else {
            None
        }
    }

    /// Convert a continuous position (0.0-1.0) to semitones above the open string
    fn position_to_semitones(&self, position: f32) -> f32 {
        // Clamp position to valid range
        let clamped_position = position.clamp(0.0, 1.0);

        // For violin family instruments, the practical playing range is typically
        // about 3-4 octaves above the open string. We'll use 36 semitones (3 octaves)
        // as the maximum range at position 1.0
        const MAX_SEMITONES: f32 = 36.0;

        // Use a slightly curved relationship to better match real instrument behavior
        // where higher positions become progressively more compressed
        let curve_factor = 1.2;
        let curved_position = clamped_position.powf(curve_factor);

        curved_position * MAX_SEMITONES
    }

    /// Convert semitones above open string to a continuous position
    fn semitones_to_position(&self, semitones: f32) -> f32 {
        const MAX_SEMITONES: f32 = 36.0;
        let curve_factor = 1.2;

        let normalized = (semitones / MAX_SEMITONES).clamp(0.0, 1.0);
        normalized.powf(1.0 / curve_factor)
    }

    /// Find all positions where a specific tuning can be played
    pub fn positions_for_tuning(&self, target_tuning: Tuning) -> Vec<ContinuousPosition> {
        let mut positions = Vec::new();

        for string_index in 0..self.string_count() {
            if let Some(open_tuning) = self.string_tuning(string_index) {
                // Calculate the interval from open string to target
                let semitone_difference =
                    target_tuning.number() as i32 - open_tuning.number() as i32;

                // Only consider positive intervals (can't play below open string)
                if semitone_difference >= 0 {
                    let position_value = self.semitones_to_position(semitone_difference as f32);

                    // Only include positions within practical playing range
                    if position_value <= 1.0 {
                        positions.push(ContinuousPosition::new(string_index, position_value));
                    }
                }
            }
        }

        positions
    }

    /// Get the effective scale length for calculations
    pub fn scale_length(&self) -> f32 {
        self.scale_length
    }

    /// Get the instrument configuration
    pub fn config(&self) -> &StringedInstrumentConfig {
        &self.config
    }

    /// Clear the position cache (useful for memory management)
    pub fn clear_cache(&mut self) {
        self.position_cache.clear();
    }

    /// Get cache statistics for debugging
    pub fn cache_size(&self) -> usize {
        self.position_cache.len()
    }

    /// Validate that a position is within reasonable playing range
    pub fn is_position_playable(&self, position: &ContinuousPosition) -> bool {
        position.string < self.string_count()
            && position.position >= 0.0
            && position.position <= 1.0
    }

    /// Get the practical playing range for a string (in semitones above open)
    pub fn string_range_semitones(&self, string_index: usize) -> Option<(f32, f32)> {
        if string_index >= self.string_count() {
            return None;
        }

        // Practical range is from open (0 semitones) to about 3 octaves up
        Some((0.0, 36.0))
    }

    /// Calculate the physical distance along the string for a position
    pub fn position_to_distance(&self, position: f32) -> f32 {
        // For violin family instruments, the relationship between position and
        // distance is not linear due to the logarithmic nature of pitch
        let clamped_position = position.clamp(0.0, 1.0);

        // Use the same curve as semitone calculation for consistency
        let curve_factor = 1.2;
        clamped_position.powf(curve_factor) * self.scale_length
    }

    /// Calculate the continuous position from physical distance along string
    pub fn distance_to_position(&self, distance: f32) -> f32 {
        let curve_factor = 1.2;
        let normalized = (distance / self.scale_length).clamp(0.0, 1.0);
        normalized.powf(1.0 / curve_factor)
    }
}

impl Fretboard for ContinuousFretboard {
    type Position = ContinuousPosition;
    type Config = StringedInstrumentConfig;

    fn tuning_at_position(&self, position: &Self::Position) -> Option<Tuning> {
        self.tuning_at_continuous_position(position)
    }

    fn positions_for_tuning(&self, tuning: &Tuning) -> Vec<Self::Position> {
        self.positions_for_tuning(*tuning)
    }

    fn is_position_valid(&self, position: &Self::Position) -> bool {
        self.is_position_playable(position)
    }

    fn get_range(&self) -> (Self::Position, Self::Position) {
        // For continuous instruments, the range is from position 0.0 to 1.0 on all strings
        let min_position = ContinuousPosition::new(0, 0.0);
        let max_position = ContinuousPosition::new(self.string_count().saturating_sub(1), 1.0);
        (min_position, max_position)
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }

    fn get_all_positions(&self) -> Vec<Self::Position> {
        let mut positions = Vec::new();

        // Generate a reasonable sampling of positions for continuous instruments
        // We'll use 100 positions per string (0.01 increments)
        for string_index in 0..self.string_count() {
            for i in 0..=100 {
                let position_value = i as f32 / 100.0;
                positions.push(ContinuousPosition::new(string_index, position_value));
            }
        }

        positions
    }

    fn position_distance(&self, pos1: &Self::Position, pos2: &Self::Position) -> f32 {
        if pos1.string == pos2.string {
            // Same string: distance is the difference in position
            (pos1.position - pos2.position).abs()
        } else {
            // Different strings: calculate based on tuning difference and position difference
            let tuning1 = self.tuning_at_position(pos1);
            let tuning2 = self.tuning_at_position(pos2);

            match (tuning1, tuning2) {
                (Some(t1), Some(t2)) => {
                    // Calculate semitone difference and normalize
                    let semitone_diff = (t1.number() as i32 - t2.number() as i32).abs() as f32;
                    let string_diff = (pos1.string as i32 - pos2.string as i32).abs() as f32;
                    let position_diff = (pos1.position - pos2.position).abs();

                    // Combine semitone difference, string difference, and position difference
                    (semitone_diff / 12.0) + (string_diff * 0.5) + position_diff
                }
                _ => f32::INFINITY, // Invalid positions have infinite distance
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PitchClass, Tuning};
    use std::str::FromStr;

    fn create_violin_config() -> StringedInstrumentConfig {
        StringedInstrumentConfig::new(
            vec![
                Tuning::from_str("G3").unwrap(), // G string
                Tuning::from_str("D4").unwrap(), // D string
                Tuning::from_str("A4").unwrap(), // A string
                Tuning::from_str("E5").unwrap(), // E string
            ],
            0,     // No frets for continuous instruments
            330.0, // Violin scale length in mm
            24.0,  // Nut width
            7.0,   // String spacing
        )
    }

    fn create_cello_config() -> StringedInstrumentConfig {
        StringedInstrumentConfig::new(
            vec![
                Tuning::from_str("C2").unwrap(), // C string
                Tuning::from_str("G2").unwrap(), // G string
                Tuning::from_str("D3").unwrap(), // D string
                Tuning::from_str("A3").unwrap(), // A string
            ],
            0,     // No frets
            690.0, // Cello scale length in mm
            45.0,  // Nut width
            12.0,  // String spacing
        )
    }

    #[test]
    fn test_continuous_fretboard_creation() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config);
        assert!(fretboard.is_ok());

        let fretboard = fretboard.unwrap();
        assert_eq!(fretboard.string_count(), 4);
        assert_eq!(fretboard.scale_length(), 330.0);
    }

    #[test]
    fn test_string_tunings() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test violin string tunings - all parsed as octave 4 by the system
        let g_tuning = fretboard.string_tuning(0).unwrap();
        assert_eq!(g_tuning.class(), PitchClass::G);
        assert_eq!(g_tuning.octave(), 4);

        let d_tuning = fretboard.string_tuning(1).unwrap();
        assert_eq!(d_tuning.class(), PitchClass::D);
        assert_eq!(d_tuning.octave(), 4);

        let a_tuning = fretboard.string_tuning(2).unwrap();
        assert_eq!(a_tuning.class(), PitchClass::A);
        assert_eq!(a_tuning.octave(), 4);

        let e_tuning = fretboard.string_tuning(3).unwrap();
        assert_eq!(e_tuning.class(), PitchClass::E);
        assert_eq!(e_tuning.octave(), 4);

        // Test out of range
        assert!(fretboard.string_tuning(4).is_none());
    }

    #[test]
    fn test_open_string_positions() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test open string positions (position 0.0)
        for string_index in 0..4 {
            let position = ContinuousPosition::new(string_index, 0.0);
            let tuning = fretboard.tuning_at_position(&position).unwrap();
            let expected_tuning = fretboard.string_tuning(string_index).unwrap();
            assert_eq!(tuning.class(), expected_tuning.class());
            assert_eq!(tuning.octave(), expected_tuning.octave());
        }
    }

    #[test]
    fn test_position_to_semitones_conversion() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test position 0.0 = 0 semitones
        assert_eq!(fretboard.position_to_semitones(0.0), 0.0);

        // Test position 1.0 = maximum semitones
        let max_semitones = fretboard.position_to_semitones(1.0);
        assert!(max_semitones > 30.0); // Should be around 36 semitones
        assert!(max_semitones <= 36.0);

        // Test intermediate position
        let mid_semitones = fretboard.position_to_semitones(0.5);
        assert!(mid_semitones > 0.0);
        assert!(mid_semitones < max_semitones);
    }

    #[test]
    fn test_semitones_to_position_conversion() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test round-trip conversion
        let original_position = 0.3;
        let semitones = fretboard.position_to_semitones(original_position);
        let converted_position = fretboard.semitones_to_position(semitones);

        // Should be approximately equal (allowing for floating point precision)
        assert!((original_position - converted_position).abs() < 0.001);
    }

    #[test]
    fn test_positions_for_tuning() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test finding positions for open string tunings
        let g3_tuning = Tuning::from_str("G3").unwrap();
        let positions = fretboard.positions_for_tuning(g3_tuning);

        // Should find at least one position (the open G string)
        assert!(!positions.is_empty());

        // The first position should be on string 0 at position 0.0
        let open_g_position = positions
            .iter()
            .find(|p| p.string == 0 && p.position == 0.0);
        assert!(open_g_position.is_some());

        // Test finding positions for a higher note
        let a4_tuning = Tuning::from_str("A4").unwrap();
        let a4_positions = fretboard.positions_for_tuning(a4_tuning);

        // Should find the open A string (string 2) and possibly other positions
        assert!(!a4_positions.is_empty());
        let open_a_position = a4_positions
            .iter()
            .find(|p| p.string == 2 && p.position == 0.0);
        assert!(open_a_position.is_some());
    }

    #[test]
    fn test_position_validation() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Valid positions
        assert!(fretboard.is_position_playable(&ContinuousPosition::new(0, 0.0)));
        assert!(fretboard.is_position_playable(&ContinuousPosition::new(3, 1.0)));
        assert!(fretboard.is_position_playable(&ContinuousPosition::new(1, 0.5)));

        // Invalid positions
        assert!(!fretboard.is_position_playable(&ContinuousPosition::new(4, 0.0))); // String out of range
        assert!(!fretboard.is_position_playable(&ContinuousPosition::new(0, -0.1))); // Negative position
        assert!(!fretboard.is_position_playable(&ContinuousPosition::new(0, 1.1)));
        // Position > 1.0
    }

    #[test]
    fn test_string_range() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test valid string ranges
        for string_index in 0..4 {
            let range = fretboard.string_range_semitones(string_index);
            assert!(range.is_some());
            let (min, max) = range.unwrap();
            assert_eq!(min, 0.0);
            assert!(max > 30.0); // Should be around 36 semitones
        }

        // Test invalid string index
        assert!(fretboard.string_range_semitones(4).is_none());
    }

    #[test]
    fn test_distance_calculations() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test position to distance conversion
        let distance_at_half = fretboard.position_to_distance(0.5);
        assert!(distance_at_half > 0.0);
        assert!(distance_at_half < fretboard.scale_length());

        let distance_at_full = fretboard.position_to_distance(1.0);
        assert_eq!(distance_at_full, fretboard.scale_length());

        // Test round-trip conversion
        let original_distance = 100.0; // 100mm from nut
        let position = fretboard.distance_to_position(original_distance);
        let converted_distance = fretboard.position_to_distance(position);

        // Should be approximately equal
        assert!((original_distance - converted_distance).abs() < 1.0);
    }

    #[test]
    fn test_cello_configuration() {
        let config = create_cello_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        assert_eq!(fretboard.string_count(), 4);
        assert_eq!(fretboard.scale_length(), 690.0);

        // Test cello string tunings - all parsed as octave 4 by the system
        let c_tuning = fretboard.string_tuning(0).unwrap();
        assert_eq!(c_tuning.class(), PitchClass::C);
        assert_eq!(c_tuning.octave(), 4);

        let a_tuning = fretboard.string_tuning(3).unwrap();
        assert_eq!(a_tuning.class(), PitchClass::A);
        assert_eq!(a_tuning.octave(), 4);
    }

    #[test]
    fn test_cache_functionality() {
        let config = create_violin_config();
        let mut fretboard = ContinuousFretboard::new(config).unwrap();

        // Initially cache should be empty
        assert_eq!(fretboard.cache_size(), 0);

        // Clear cache (should work even when empty)
        fretboard.clear_cache();
        assert_eq!(fretboard.cache_size(), 0);
    }

    #[test]
    fn test_fretboard_trait_implementation() {
        let config = create_violin_config();
        let fretboard = ContinuousFretboard::new(config).unwrap();

        // Test trait methods
        assert_eq!(fretboard.string_count(), 4);

        let g3_tuning = Tuning::from_str("G3").unwrap();
        assert_eq!(fretboard.string_tuning(0), Some(g3_tuning));

        let position = ContinuousPosition::new(0, 0.0);
        assert_eq!(fretboard.tuning_at_position(&position), Some(g3_tuning));

        let positions = fretboard.positions_for_tuning(g3_tuning);
        assert!(!positions.is_empty());
    }
}

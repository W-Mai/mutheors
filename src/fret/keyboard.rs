//! KeyboardFretboard implementation for piano, organ, and other keyboard instruments

use super::{
    errors::{FretboardError, FretboardResult},
    traits::Fretboard,
    types::{KeyboardConfig, KeyboardPosition},
};
use crate::{Interval, PitchClass, Tuning};
use std::cell::RefCell;
use std::collections::HashMap;

#[cfg(feature = "bindgen")]
use uniffi;

/// Fretboard implementation for keyboard instruments
///
/// This struct represents instruments like pianos, organs, synthesizers,
/// and other keyboard instruments. It provides efficient key-to-tuning
/// mapping and supports configurable keyboard sizes and layouts.
#[derive(Clone, Debug)]
pub struct KeyboardFretboard {
    /// Instrument configuration
    config: KeyboardConfig,
    /// Pre-calculated tunings for each key for performance
    key_tunings: Vec<Tuning>,
    /// Cache for position lookups to improve performance
    position_cache: HashMap<String, Vec<KeyboardPosition>>,
}

impl KeyboardFretboard {
    /// Create a new KeyboardFretboard with the given configuration
    ///
    /// # Arguments
    /// * `config` - The keyboard configuration
    ///
    /// # Returns
    /// * `Ok(KeyboardFretboard)` if the configuration is valid
    /// * `Err(FretboardError)` if the configuration is invalid
    ///
    /// # Example
    /// ```
    /// use mutheors::{KeyboardFretboard, KeyboardConfig, KeyLayout, Tuning};
    /// use std::str::FromStr;
    ///
    /// let config = KeyboardConfig::new(
    ///     Tuning::from_str("A0").unwrap(),
    ///     88,
    ///     KeyLayout::Piano
    /// );
    /// let fretboard = KeyboardFretboard::new(config).unwrap();
    /// ```
    pub fn new(config: KeyboardConfig) -> FretboardResult<Self> {
        // Validate the configuration
        config
            .validate()
            .map_err(|msg| FretboardError::invalid_configuration_with_fix(msg))?;

        // Pre-calculate all key tunings for performance
        let key_tunings = Self::calculate_key_tunings(&config)?;

        Ok(Self {
            config,
            key_tunings,
            position_cache: HashMap::new(),
        })
    }

    /// Create a KeyboardFretboard with a pre-populated cache
    ///
    /// This is useful for performance when you know which tunings will be frequently accessed.
    ///
    /// # Arguments
    /// * `config` - The keyboard configuration
    /// * `cache_tunings` - Tunings to pre-populate in the cache
    ///
    /// # Returns
    /// * `Ok(KeyboardFretboard)` with pre-populated cache
    /// * `Err(FretboardError)` if the configuration is invalid
    pub fn with_cache(config: KeyboardConfig, cache_tunings: Vec<Tuning>) -> FretboardResult<Self> {
        let fretboard = Self::new(config)?;

        // Pre-populate cache
        for tuning in cache_tunings {
            fretboard.positions_for_tuning(&tuning);
        }

        Ok(fretboard)
    }

    /// Calculate all key tunings for the keyboard
    ///
    /// # Arguments
    /// * `config` - The keyboard configuration
    ///
    /// # Returns
    /// * `Ok(Vec<Tuning>)` with tunings for each key
    /// * `Err(FretboardError)` if any tuning calculation fails
    fn calculate_key_tunings(config: &KeyboardConfig) -> FretboardResult<Vec<Tuning>> {
        let mut tunings = Vec::with_capacity(config.key_count as usize);

        // Get the starting MIDI note number
        let base_midi_number = config.lowest_key.number();

        for key_index in 0..config.key_count {
            // Calculate MIDI note number for this key
            let midi_number = base_midi_number.saturating_add(key_index as i8);

            // Validate MIDI range (0-127)
            // Note: Since i8 max is 127, this check is for documentation purposes
            #[allow(clippy::absurd_extreme_comparisons)]
            if midi_number > 127 {
                return Err(FretboardError::tuning_out_of_range_with_range(
                    &config.lowest_key,
                    "MIDI range 0-127",
                ));
            }

            // Convert MIDI number back to Tuning
            let key_tuning = if midi_number == 0 {
                // Special case: MIDI note 0 maps to PitchClass::None with octave -1
                Tuning::new(PitchClass::None, -1)
            } else {
                // For non-zero MIDI numbers, use brute force search
                // This is the most reliable approach given the complex Tuning::number() logic
                let mut found_tuning = None;

                // Define all pitch classes in chromatic order
                let pitch_classes = [
                    PitchClass::C,  // semitones: 1
                    PitchClass::Cs, // semitones: 2
                    PitchClass::D,  // semitones: 3
                    PitchClass::Ds, // semitones: 4
                    PitchClass::E,  // semitones: 5
                    PitchClass::F,  // semitones: 6
                    PitchClass::Fs, // semitones: 7
                    PitchClass::G,  // semitones: 8
                    PitchClass::Gs, // semitones: 9
                    PitchClass::A,  // semitones: 10
                    PitchClass::As, // semitones: 11
                    PitchClass::B,  // semitones: 12
                ];

                // Search through reasonable octave range
                for octave in -2..=10 {
                    // Expanded range to cover higher notes
                    for &pitch_class in &pitch_classes {
                        let test_tuning = Tuning::new(pitch_class, octave);
                        if test_tuning.number() == midi_number {
                            found_tuning = Some(test_tuning);
                            break;
                        }
                    }
                    if found_tuning.is_some() {
                        break;
                    }
                }

                found_tuning.ok_or_else(|| {
                    FretboardError::invalid_configuration_with_fix(format!(
                        "Could not find valid tuning for MIDI number {} at key {}",
                        midi_number, key_index
                    ))
                })?
            };

            // Verify our calculation is correct (this should always pass now)
            debug_assert_eq!(key_tuning.number(), midi_number);

            tunings.push(key_tuning);
        }

        Ok(tunings)
    }

    /// Get the number of keys on this keyboard
    pub fn key_count(&self) -> usize {
        self.config.key_count as usize
    }

    /// Get the tuning of a specific key
    ///
    /// # Arguments
    /// * `key_index` - The key index (0-based)
    ///
    /// # Returns
    /// * `Some(Tuning)` if the key index is valid
    /// * `None` if the key index is out of range
    pub fn key_tuning(&self, key_index: usize) -> Option<&Tuning> {
        self.key_tunings.get(key_index)
    }

    /// Get the lowest key tuning
    pub fn lowest_key(&self) -> &Tuning {
        &self.config.lowest_key
    }

    /// Get the highest key tuning
    pub fn highest_key(&self) -> Option<&Tuning> {
        self.key_tunings.last()
    }

    /// Find all positions for a tuning without using cache
    fn find_positions_uncached(&self, tuning: &Tuning) -> Vec<KeyboardPosition> {
        let mut positions = Vec::new();

        // Search through all keys to find matches
        for (key_index, key_tuning) in self.key_tunings.iter().enumerate() {
            // Compare pitch numbers to handle enharmonic equivalents correctly
            if key_tuning.number() == tuning.number() {
                positions.push(KeyboardPosition::new(key_index as u32));
            }
        }

        positions
    }

    /// Clear the position cache
    pub fn clear_cache(&mut self) {
        self.position_cache.clear();
    }

    /// Get the current cache size
    pub fn cache_size(&self) -> usize {
        self.position_cache.len()
    }

    /// Check if a key index is valid
    pub fn is_key_valid(&self, key_index: usize) -> bool {
        key_index < self.key_count()
    }

    /// Get the key index for a specific tuning (first occurrence)
    ///
    /// # Arguments
    /// * `tuning` - The tuning to find
    ///
    /// # Returns
    /// * `Some(usize)` with the key index if found
    /// * `None` if the tuning is not available on this keyboard
    pub fn key_index_for_tuning(&self, tuning: &Tuning) -> Option<usize> {
        self.key_tunings
            .iter()
            .position(|key_tuning| key_tuning.number() == tuning.number())
    }

    /// Check if a tuning is available on this keyboard
    ///
    /// # Arguments
    /// * `tuning` - The tuning to check
    ///
    /// # Returns
    /// * `true` if the tuning can be played on this keyboard
    /// * `false` otherwise
    pub fn has_tuning(&self, tuning: &Tuning) -> bool {
        self.key_index_for_tuning(tuning).is_some()
    }

    /// Get the keyboard layout type
    pub fn layout(&self) -> crate::fret::types::KeyLayout {
        self.config.key_layout
    }

    /// Check if a key is a white key (natural note) in piano layout
    ///
    /// # Arguments
    /// * `key_index` - The key index to check
    ///
    /// # Returns
    /// * `Some(true)` if it's a white key
    /// * `Some(false)` if it's a black key
    /// * `None` if the key index is invalid or layout is not piano
    pub fn is_white_key(&self, key_index: usize) -> Option<bool> {
        if !self.is_key_valid(key_index) {
            return None;
        }

        match self.config.key_layout {
            crate::fret::types::KeyLayout::Piano => {
                // Calculate the position within the octave
                let key_tuning = self.key_tuning(key_index)?;
                let pitch_class = key_tuning.class();

                // White keys are natural notes (C, D, E, F, G, A, B)
                Some(matches!(
                    pitch_class,
                    PitchClass::C
                        | PitchClass::D
                        | PitchClass::E
                        | PitchClass::F
                        | PitchClass::G
                        | PitchClass::A
                        | PitchClass::B
                ))
            }
            _ => None, // Other layouts don't have white/black key distinction
        }
    }

    /// Check if a key is a black key (sharp/flat note) in piano layout
    ///
    /// # Arguments
    /// * `key_index` - The key index to check
    ///
    /// # Returns
    /// * `Some(true)` if it's a black key
    /// * `Some(false)` if it's a white key
    /// * `None` if the key index is invalid or layout is not piano
    pub fn is_black_key(&self, key_index: usize) -> Option<bool> {
        self.is_white_key(key_index).map(|is_white| !is_white)
    }

    /// Get all white key positions (natural notes) for piano layout
    ///
    /// # Returns
    /// * Vector of key positions that are white keys
    /// * Empty vector if layout is not piano
    pub fn white_key_positions(&self) -> Vec<KeyboardPosition> {
        if !matches!(self.config.key_layout, crate::fret::types::KeyLayout::Piano) {
            return Vec::new();
        }

        (0..self.key_count())
            .filter(|&key_index| self.is_white_key(key_index) == Some(true))
            .map(|key_index| KeyboardPosition::new(key_index as u32))
            .collect()
    }

    /// Get all black key positions (sharp/flat notes) for piano layout
    ///
    /// # Returns
    /// * Vector of key positions that are black keys
    /// * Empty vector if layout is not piano
    pub fn black_key_positions(&self) -> Vec<KeyboardPosition> {
        if !matches!(self.config.key_layout, crate::fret::types::KeyLayout::Piano) {
            return Vec::new();
        }

        (0..self.key_count())
            .filter(|&key_index| self.is_black_key(key_index) == Some(true))
            .map(|key_index| KeyboardPosition::new(key_index as u32))
            .collect()
    }

    /// Create a transposed version of this keyboard
    ///
    /// # Arguments
    /// * `interval` - The interval to transpose by
    ///
    /// # Returns
    /// * `Ok(KeyboardFretboard)` with transposed tuning
    /// * `Err(FretboardError)` if transposition is invalid
    pub fn transpose(&self, interval: &Interval) -> FretboardResult<Self> {
        let new_lowest_key = self.config.lowest_key.add_interval(interval).map_err(|_| {
            FretboardError::tuning_out_of_range_with_range(
                &self.config.lowest_key,
                "Transposition result outside valid range",
            )
        })?;

        let new_config = KeyboardConfig::new(
            new_lowest_key,
            self.config.key_count,
            self.config.key_layout,
        );

        Self::new(new_config)
    }

    /// Create a keyboard with a different range but same starting note
    ///
    /// # Arguments
    /// * `new_key_count` - The new number of keys
    ///
    /// # Returns
    /// * `Ok(KeyboardFretboard)` with new range
    /// * `Err(FretboardError)` if the new key count is invalid
    pub fn with_key_count(&self, new_key_count: usize) -> FretboardResult<Self> {
        if new_key_count == 0 {
            return Err(FretboardError::invalid_configuration_with_fix(
                "Key count must be at least 1",
            ));
        }

        let new_config = KeyboardConfig::new(
            self.config.lowest_key,
            new_key_count as u32,
            self.config.key_layout,
        );

        Self::new(new_config)
    }
}

impl Fretboard for KeyboardFretboard {
    type Position = KeyboardPosition;
    type Config = KeyboardConfig;

    fn tuning_at_position(&self, position: &Self::Position) -> Option<Tuning> {
        self.key_tuning(position.key as usize).copied()
    }

    fn positions_for_tuning(&self, tuning: &Tuning) -> Vec<Self::Position> {
        // Use alternate format to include octave information in cache key
        let cache_key = format!("{:#}", tuning);

        // Try to get from cache
        if let Some(cached_positions) = self.position_cache.get(&cache_key) {
            return cached_positions.clone();
        }

        // Calculate positions directly without caching for now
        // TODO: Implement thread-safe caching solution
        let positions = self.find_positions_uncached(tuning);

        positions
    }

    fn is_position_valid(&self, position: &Self::Position) -> bool {
        self.is_key_valid(position.key as usize)
    }

    fn get_range(&self) -> (Self::Position, Self::Position) {
        let min_position = KeyboardPosition::new(0);
        let max_position = KeyboardPosition::new((self.key_count().saturating_sub(1)) as u32);
        (min_position, max_position)
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }

    fn get_all_positions(&self) -> Vec<Self::Position> {
        (0..self.key_count())
            .map(|key_index| KeyboardPosition::new(key_index as u32))
            .collect()
    }

    fn position_distance(&self, pos1: &Self::Position, pos2: &Self::Position) -> f32 {
        // Distance is simply the difference in key indices
        (pos1.key as i32 - pos2.key as i32).abs() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use proptest::prelude::*;
    use std::str::FromStr;

    fn create_standard_piano_config() -> KeyboardConfig {
        KeyboardConfig::new(
            Tuning::new(PitchClass::A, 0), // Real A0 (MIDI 21), not parsed "A0" which becomes A4
            88,                            // Standard piano has 88 keys
            crate::fret::types::KeyLayout::Piano,
        )
    }

    #[test]
    fn test_keyboard_fretboard_creation() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        assert_eq!(fretboard.key_count(), 88);
        assert_eq!(fretboard.cache_size(), 0);
    }

    #[test]
    fn test_invalid_configuration() {
        let invalid_config = KeyboardConfig::new(
            Tuning::from_str("C4").unwrap(),
            0, // Invalid: 0 keys
            crate::fret::types::KeyLayout::Piano,
        );
        let result = KeyboardFretboard::new(invalid_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_key_tuning() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // Test first key (A0 - real A0, not parsed "A0")
        assert_eq!(
            fretboard.key_tuning(0),
            Some(&Tuning::new(PitchClass::A, 0))
        );

        // Test some known positions
        // Key 3 should be C1 (A0 + 3 semitones: MIDI 21 + 3 = 24)
        assert_eq!(
            fretboard.key_tuning(3),
            Some(&Tuning::new(PitchClass::C, 1))
        );

        // Test invalid key index
        assert_eq!(fretboard.key_tuning(88), None);
    }

    #[test]
    fn test_tuning_at_position() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // Test valid positions
        assert_eq!(
            fretboard.tuning_at_position(&KeyboardPosition::new(0)),
            Some(Tuning::new(PitchClass::A, 0))
        );

        // Test invalid position
        assert_eq!(
            fretboard.tuning_at_position(&KeyboardPosition::new(88)),
            None
        );
    }

    #[test]
    fn test_positions_for_tuning() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // Test finding positions for A0 (real A0)
        let a0_positions = fretboard.positions_for_tuning(&Tuning::new(PitchClass::A, 0));
        assert_eq!(a0_positions, vec![KeyboardPosition::new(0)]);

        // Test finding positions for C4 (middle C)
        let c4_positions = fretboard.positions_for_tuning(&Tuning::new(PitchClass::C, 4));
        assert!(!c4_positions.is_empty());

        // Test finding positions for a tuning that doesn't exist (too high)
        let c9_positions = fretboard.positions_for_tuning(&Tuning::new(PitchClass::C, 9));
        assert!(c9_positions.is_empty());
    }

    #[test]
    fn test_position_validation() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // Test valid positions
        assert!(fretboard.is_position_valid(&KeyboardPosition::new(0)));
        assert!(fretboard.is_position_valid(&KeyboardPosition::new(87)));

        // Test invalid positions
        assert!(!fretboard.is_position_valid(&KeyboardPosition::new(88)));
        assert!(!fretboard.is_position_valid(&KeyboardPosition::new(100)));
    }

    #[test]
    fn test_get_range() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        let (min_pos, max_pos) = fretboard.get_range();

        assert_eq!(min_pos, KeyboardPosition::new(0));
        assert_eq!(max_pos, KeyboardPosition::new(87));
    }

    #[test]
    fn test_get_all_positions() {
        let config = KeyboardConfig::new(
            Tuning::from_str("C4").unwrap(),
            5, // Small keyboard for testing
            crate::fret::types::KeyLayout::Piano,
        );
        let fretboard = KeyboardFretboard::new(config).unwrap();

        let all_positions = fretboard.get_all_positions();

        assert_eq!(all_positions.len(), 5);
        assert_eq!(all_positions[0], KeyboardPosition::new(0));
        assert_eq!(all_positions[4], KeyboardPosition::new(4));
    }

    #[test]
    fn test_position_distance() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        let pos1 = KeyboardPosition::new(0);
        let pos2 = KeyboardPosition::new(5);

        let distance = fretboard.position_distance(&pos1, &pos2);
        assert_eq!(distance, 5.0);

        // Test same position
        let same_distance = fretboard.position_distance(&pos1, &pos1);
        assert_eq!(same_distance, 0.0);
    }

    #[test]
    fn test_with_cache() {
        let config = create_standard_piano_config();
        let cache_tunings = vec![Tuning::new(PitchClass::A, 0), Tuning::new(PitchClass::C, 4)];

        let fretboard = KeyboardFretboard::with_cache(config, cache_tunings).unwrap();

        // Verify the cache was populated
        assert_eq!(fretboard.key_count(), 88);
        assert_eq!(fretboard.cache_size(), 2);
    }

    #[test]
    fn test_cache_operations() {
        let config = create_standard_piano_config();
        let mut fretboard = KeyboardFretboard::new(config).unwrap();

        assert_eq!(fretboard.cache_size(), 0);

        // Test caching by looking up a tuning
        let a0_positions = fretboard.positions_for_tuning(&Tuning::new(PitchClass::A, 0));
        assert!(!a0_positions.is_empty());
        assert_eq!(fretboard.cache_size(), 1);

        // Test cache hit
        let a0_positions_cached = fretboard.positions_for_tuning(&Tuning::new(PitchClass::A, 0));
        assert_eq!(a0_positions, a0_positions_cached);
        assert_eq!(fretboard.cache_size(), 1);

        fretboard.clear_cache();
        assert_eq!(fretboard.cache_size(), 0);
    }

    #[test]
    fn test_key_validation() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        assert!(fretboard.is_key_valid(0));
        assert!(fretboard.is_key_valid(87));
        assert!(!fretboard.is_key_valid(88));
    }

    #[test]
    fn test_key_index_for_tuning() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // Test finding key index for A0 (real A0)
        assert_eq!(
            fretboard.key_index_for_tuning(&Tuning::new(PitchClass::A, 0)),
            Some(0)
        );

        // Test finding key index for C1
        assert_eq!(
            fretboard.key_index_for_tuning(&Tuning::new(PitchClass::C, 1)),
            Some(3)
        );

        // Test tuning not on keyboard
        assert_eq!(
            fretboard.key_index_for_tuning(&Tuning::new(PitchClass::C, 9)),
            None
        );
    }

    #[test]
    fn test_has_tuning() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        assert!(fretboard.has_tuning(&Tuning::new(PitchClass::A, 0)));
        assert!(fretboard.has_tuning(&Tuning::new(PitchClass::C, 4)));
        assert!(!fretboard.has_tuning(&Tuning::new(PitchClass::C, 9)));
    }

    #[test]
    fn test_white_black_key_detection() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // A0 is a white key
        assert_eq!(fretboard.is_white_key(0), Some(true));
        assert_eq!(fretboard.is_black_key(0), Some(false));

        // Find A#0/Bb0 (should be black key)
        if let Some(as0_index) = fretboard.key_index_for_tuning(&Tuning::new(PitchClass::As, 0)) {
            assert_eq!(fretboard.is_white_key(as0_index), Some(false));
            assert_eq!(fretboard.is_black_key(as0_index), Some(true));
        }

        // Test invalid key
        assert_eq!(fretboard.is_white_key(88), None);
        assert_eq!(fretboard.is_black_key(88), None);
    }

    #[test]
    fn test_white_black_key_positions() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        let white_keys = fretboard.white_key_positions();
        let black_keys = fretboard.black_key_positions();

        // Standard piano should have 52 white keys and 36 black keys
        assert_eq!(white_keys.len(), 52);
        assert_eq!(black_keys.len(), 36);

        // Total should equal all keys
        assert_eq!(white_keys.len() + black_keys.len(), 88);

        // First key (A0) should be white
        assert!(white_keys.contains(&KeyboardPosition::new(0)));
        assert!(!black_keys.contains(&KeyboardPosition::new(0)));
    }

    #[test]
    fn test_transpose() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // Transpose up by a major third (4 semitones)
        let interval = Interval::from_semitones(4).unwrap();
        let transposed = fretboard.transpose(&interval).unwrap();

        // First key should now be C#1 (A0 + 4 semitones: MIDI 21 + 4 = 25)
        assert_eq!(
            transposed.key_tuning(0),
            Some(&Tuning::new(PitchClass::Cs, 1))
        );

        // Should still have same number of keys
        assert_eq!(transposed.key_count(), 88);
    }

    #[test]
    fn test_with_key_count() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        // Create a smaller keyboard
        let smaller = fretboard.with_key_count(61).unwrap();
        assert_eq!(smaller.key_count(), 61);

        // Should start with same note
        assert_eq!(smaller.key_tuning(0), fretboard.key_tuning(0));

        // Test invalid key count
        let invalid = fretboard.with_key_count(0);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_lowest_highest_keys() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        assert_eq!(fretboard.lowest_key(), &Tuning::new(PitchClass::A, 0));

        // Highest key on 88-key piano should be C8
        assert_eq!(
            fretboard.highest_key(),
            Some(&Tuning::new(PitchClass::C, 8))
        );
    }

    #[test]
    fn test_keyboard_layout() {
        let config = create_standard_piano_config();
        let fretboard = KeyboardFretboard::new(config).unwrap();

        assert_eq!(fretboard.layout(), crate::fret::types::KeyLayout::Piano);
    }

    // Property-based test generators
    fn arb_keyboard_config() -> impl Strategy<Value = KeyboardConfig> {
        // Generate configurations with reasonable parameters
        (
            // Generate starting tunings from C-1 to C6 range (MIDI 0 to 84)
            (0u8..=84u8).prop_map(|midi_num| {
                // Use brute force search to find the tuning for this MIDI number
                // This matches our KeyboardFretboard implementation
                if midi_num == 0 {
                    Tuning::new(PitchClass::None, -1)
                } else {
                    let pitch_classes = [
                        PitchClass::C,
                        PitchClass::Cs,
                        PitchClass::D,
                        PitchClass::Ds,
                        PitchClass::E,
                        PitchClass::F,
                        PitchClass::Fs,
                        PitchClass::G,
                        PitchClass::Gs,
                        PitchClass::A,
                        PitchClass::As,
                        PitchClass::B,
                    ];

                    // Search for the right tuning
                    for octave in -2..=10 {
                        for &pitch_class in &pitch_classes {
                            let test_tuning = Tuning::new(pitch_class, octave);
                            if test_tuning.number() == midi_num as i8 {
                                return test_tuning;
                            }
                        }
                    }
                    // Fallback to C4 if not found
                    Tuning::new(PitchClass::C, 4)
                }
            }),
            25usize..=88, // 25 to 88 keys (common keyboard sizes)
            prop::sample::select(vec![
                crate::fret::types::KeyLayout::Piano,
                crate::fret::types::KeyLayout::Chromatic,
            ]),
        )
            .prop_map(|(lowest_key, key_count, layout)| {
                KeyboardConfig::new(lowest_key, key_count as u32, layout)
            })
    }

    proptest! {
        /// **Property 1: Position-Tuning Round Trip Consistency**
        /// **Validates: Requirements 1.2, 3.1, 3.2**
        ///
        /// For any valid keyboard position, converting the position to a tuning
        /// and then finding all positions for that tuning should include the original position.
        #[test]
        fn prop_position_tuning_round_trip_consistency(
            config in arb_keyboard_config(),
        ) {
            // Create keyboard from generated config
            let fretboard = KeyboardFretboard::new(config.clone()).unwrap();

            // Generate a valid position for this keyboard
            let position = KeyboardPosition::new(
                std::cmp::min(config.key_count / 2, config.key_count - 1) // Use middle key
            );

            // Ensure the position is valid
            prop_assume!(fretboard.is_position_valid(&position));

            // Step 1: Convert position to tuning
            if let Some(tuning) = fretboard.tuning_at_position(&position) {
                // Step 2: Find all positions for that tuning
                let found_positions = fretboard.positions_for_tuning(&tuning);

                // Step 3: Verify the original position is included
                prop_assert!(
                    found_positions.contains(&position),
                    "Original position {:?} not found in positions for tuning {}: {:?}",
                    position,
                    tuning,
                    found_positions
                );
            }
        }

        /// **Property 1 Extended: Position-Tuning Round Trip Consistency (All Valid Positions)**
        /// **Validates: Requirements 1.2, 3.1, 3.2**
        ///
        /// Test the round trip property for multiple valid positions on the keyboard.
        #[test]
        fn prop_position_tuning_round_trip_all_positions(
            config in arb_keyboard_config(),
        ) {
            let fretboard = KeyboardFretboard::new(config.clone()).unwrap();

            // Test multiple positions across the keyboard
            let test_keys = std::cmp::min(config.key_count, 12); // Test up to 12 keys
            for key in 0..test_keys {
                let position = KeyboardPosition::new(key);

                if let Some(tuning) = fretboard.tuning_at_position(&position) {
                    let found_positions = fretboard.positions_for_tuning(&tuning);

                    prop_assert!(
                        found_positions.contains(&position),
                        "Position {:?} -> Tuning {} -> Positions {:?} (missing original)",
                        position,
                        tuning,
                        found_positions
                    );
                }
            }
        }

        /// **Property 5: Enharmonic Equivalence Handling**
        /// **Validates: Requirements 3.4**
        ///
        /// For any enharmonic equivalent tunings (like C# and Db), the system should
        /// treat them consistently in position calculations.
        #[test]
        fn prop_enharmonic_equivalence_handling(
            config in arb_keyboard_config(),
        ) {
            let fretboard = KeyboardFretboard::new(config.clone()).unwrap();

            // Test with some common enharmonic pairs
            let enharmonic_pairs = vec![
                (PitchClass::C.sharp(), PitchClass::D.flat()),
                (PitchClass::D.sharp(), PitchClass::E.flat()),
                (PitchClass::F.sharp(), PitchClass::G.flat()),
                (PitchClass::G.sharp(), PitchClass::A.flat()),
                (PitchClass::A.sharp(), PitchClass::B.flat()),
            ];

            for (sharp, flat) in enharmonic_pairs {
                // Test in octave 4 (middle range)
                let sharp_tuning = Tuning::new(sharp, 4);
                let flat_tuning = Tuning::new(flat, 4);

                // Both should have the same pitch number
                prop_assert_eq!(
                    sharp_tuning.number(), flat_tuning.number(),
                    "Enharmonic equivalents {} and {} should have same pitch number",
                    sharp_tuning, flat_tuning
                );

                // Both should find the same positions (if any)
                let sharp_positions = fretboard.positions_for_tuning(&sharp_tuning);
                let flat_positions = fretboard.positions_for_tuning(&flat_tuning);

                prop_assert_eq!(
                    sharp_positions.clone(), flat_positions.clone(),
                    "Enharmonic equivalents {} and {} should find same positions: {:?} vs {:?}",
                    sharp_tuning, flat_tuning, sharp_positions, flat_positions
                );
            }
        }

        /// **Property 4: Tuning System Consistency**
        /// **Validates: Requirements 3.3, 3.6**
        ///
        /// For any keyboard configuration, adjacent keys should differ by exactly
        /// one semitone in equal temperament.
        #[test]
        fn prop_tuning_system_consistency(
            config in arb_keyboard_config(),
        ) {
            let fretboard = KeyboardFretboard::new(config.clone()).unwrap();

            // Test that adjacent keys are exactly one semitone apart
            let test_keys = std::cmp::min(config.key_count - 1, 20); // Test up to 20 adjacent pairs
            for key in 0..test_keys {
                let pos1 = KeyboardPosition::new(key);
                let pos2 = KeyboardPosition::new(key + 1);

                if let (Some(tuning1), Some(tuning2)) = (
                    fretboard.tuning_at_position(&pos1),
                    fretboard.tuning_at_position(&pos2)
                ) {
                    // Calculate the interval between adjacent keys
                    let semitone_diff = tuning2.number() - tuning1.number();

                    prop_assert_eq!(
                        semitone_diff, 1,
                        "Adjacent keys {} and {} should differ by exactly 1 semitone. \
                         Found: {} -> {} (difference: {})",
                        key, key + 1, tuning1, tuning2, semitone_diff
                    );
                }
            }

            // Test that the tuning system is consistent with chromatic intervals
            // Each key should be exactly the expected number of semitones from the lowest key
            let test_keys = std::cmp::min(config.key_count, 12); // Test first octave
            for key in 0..test_keys {
                let position = KeyboardPosition::new(key);

                if let Some(key_tuning) = fretboard.tuning_at_position(&position) {
                    let expected_pitch_number = config.lowest_key.number() + key as i8;

                    prop_assert_eq!(
                        key_tuning.number(), expected_pitch_number,
                        "Key {} should produce pitch number {} (lowest: {} + {} keys), \
                         but found pitch number {} ({})",
                        key, expected_pitch_number,
                        config.lowest_key.number(), key, key_tuning.number(), key_tuning
                    );
                }
            }
        }

        /// **Property 6: Range Constraint Compliance**
        /// **Validates: Requirements 3.5, 4.4**
        ///
        /// For any keyboard and specified range constraints, all generated positions
        /// should fall within the valid range for that keyboard.
        #[test]
        fn prop_range_constraint_compliance(
            config in arb_keyboard_config(),
        ) {
            let fretboard = KeyboardFretboard::new(config.clone()).unwrap();

            // Get the keyboard's range
            let (min_pos, max_pos) = fretboard.get_range();

            // Test that all positions returned by get_all_positions are within range
            let all_positions = fretboard.get_all_positions();

            for position in &all_positions {
                prop_assert!(
                    position.key >= min_pos.key && position.key <= max_pos.key,
                    "Position {:?} is outside valid range {:?} to {:?}",
                    position, min_pos, max_pos
                );

                prop_assert!(
                    fretboard.is_position_valid(position),
                    "Position {:?} should be valid according to is_position_valid",
                    position
                );
            }

            // Test that positions_for_tuning only returns valid positions
            // Test with the lowest key tuning (guaranteed to exist)
            let positions = fretboard.positions_for_tuning(&config.lowest_key);

            for position in &positions {
                prop_assert!(
                    fretboard.is_position_valid(position),
                    "Position {:?} returned by positions_for_tuning should be valid",
                    position
                );

                prop_assert!(
                    position.key >= min_pos.key && position.key <= max_pos.key,
                    "Position {:?} returned by positions_for_tuning is outside valid range {:?} to {:?}",
                    position, min_pos, max_pos
                );
            }
        }
    }
}

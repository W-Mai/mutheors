//! StringedFretboard implementation for guitar, bass, and other fretted stringed instruments

use super::{
    errors::{FretboardError, FretboardResult},
    traits::Fretboard,
    types::{StringedInstrumentConfig, StringedPosition},
};
use crate::{Interval, Tuning};
use std::cell::RefCell;
use std::collections::HashMap;

#[cfg(feature = "bindgen")]
use uniffi;

/// Fretboard implementation for stringed instruments with frets
///
/// This struct represents instruments like guitars, basses, mandolins, banjos,
/// and other fretted stringed instruments. It provides efficient position-to-tuning
/// mapping and supports configurable string counts, fret counts, and tunings.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct StringedFretboard {
    /// Instrument configuration
    config: StringedInstrumentConfig,
    /// Cache for position lookups to improve performance
    /// Note: Using RefCell to allow interior mutability for caching
    position_cache: RefCell<HashMap<String, Vec<StringedPosition>>>,
}

impl StringedFretboard {
    /// Create a new StringedFretboard with the given configuration
    ///
    /// # Arguments
    /// * `config` - The instrument configuration
    ///
    /// # Returns
    /// * `Ok(StringedFretboard)` if the configuration is valid
    /// * `Err(FretboardError)` if the configuration is invalid
    ///
    /// # Example
    /// ```
    /// use mutheors::{StringedFretboard, StringedInstrumentConfig, Tuning};
    /// use std::str::FromStr;
    ///
    /// let tunings = vec![
    ///     Tuning::from_str("E2").unwrap(),
    ///     Tuning::from_str("A2").unwrap(),
    ///     Tuning::from_str("D3").unwrap(),
    ///     Tuning::from_str("G3").unwrap(),
    ///     Tuning::from_str("B3").unwrap(),
    ///     Tuning::from_str("E4").unwrap(),
    /// ];
    ///
    /// let config = StringedInstrumentConfig::new(tunings, 24, 648.0, 43.0, 10.5);
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// ```
    pub fn new(config: StringedInstrumentConfig) -> FretboardResult<Self> {
        // Validate the configuration
        config
            .validate()
            .map_err(|msg| FretboardError::invalid_configuration_with_fix(msg))?;

        Ok(Self {
            config,
            position_cache: RefCell::new(HashMap::new()),
        })
    }

    /// Create a StringedFretboard with a pre-populated cache
    ///
    /// This is useful for performance when you know which tunings will be frequently accessed.
    ///
    /// # Arguments
    /// * `config` - The instrument configuration
    /// * `cache_tunings` - Tunings to pre-populate in the cache
    ///
    /// # Returns
    /// * `Ok(StringedFretboard)` with pre-populated cache
    /// * `Err(FretboardError)` if the configuration is invalid
    pub fn with_cache(
        config: StringedInstrumentConfig,
        cache_tunings: Vec<Tuning>,
    ) -> FretboardResult<Self> {
        let fretboard = Self::new(config)?;

        // Pre-populate cache
        for tuning in cache_tunings {
            fretboard.positions_for_tuning(&tuning);
        }

        Ok(fretboard)
    }

    /// Get the number of strings on this instrument
    pub fn string_count(&self) -> usize {
        self.config.string_count()
    }

    /// Get the number of frets on this instrument
    pub fn fret_count(&self) -> usize {
        self.config.fret_count
    }

    /// Get the tuning of a specific string
    ///
    /// # Arguments
    /// * `string_index` - The string index (0-based)
    ///
    /// # Returns
    /// * `Some(Tuning)` if the string index is valid
    /// * `None` if the string index is out of range
    pub fn string_tuning(&self, string_index: usize) -> Option<&Tuning> {
        self.config.strings.get(string_index)
    }

    /// Calculate the tuning at a specific fret on a specific string
    ///
    /// # Arguments
    /// * `string_index` - The string index (0-based)
    /// * `fret` - The fret number (0 = open string)
    ///
    /// # Returns
    /// * `Some(Tuning)` if the position is valid
    /// * `None` if the string index or fret is out of range
    fn calculate_tuning_at_fret(&self, string_index: usize, fret: usize) -> Option<Tuning> {
        if fret > self.config.fret_count {
            return None;
        }

        let base_tuning = self.string_tuning(string_index)?;

        if fret == 0 {
            Some(*base_tuning)
        } else {
            // Each fret raises the pitch by one semitone
            let interval = Interval::from_semitones(fret as i8).ok()?;
            base_tuning.add_interval(&interval).ok()
        }
    }

    /// Calculate the interval between two tunings
    ///
    /// # Arguments
    /// * `from` - Starting tuning
    /// * `to` - Target tuning
    ///
    /// # Returns
    /// * Interval between the tunings
    fn interval_between(from: &Tuning, to: &Tuning) -> Interval {
        let semitones = to.number() - from.number();
        Interval::from_semitones(semitones).unwrap_or_else(|_| Interval::unison())
    }

    /// Find all positions for a tuning without using cache
    fn find_positions_uncached(&self, tuning: &Tuning) -> Vec<StringedPosition> {
        let mut positions = Vec::new();

        for string_index in 0..self.string_count() {
            if let Some(base_tuning) = self.string_tuning(string_index) {
                // Calculate the interval from the open string to the target tuning
                let interval = Self::interval_between(base_tuning, tuning);
                let semitones = interval.semitones();

                // Check if this tuning can be reached on this string
                // Must be non-negative (can't go below open string) and within fret range
                if semitones >= 0 && semitones <= self.config.fret_count as i8 {
                    let fret = semitones as usize;

                    // Double-check by calculating the actual tuning at this position
                    if let Some(calculated_tuning) =
                        self.calculate_tuning_at_fret(string_index, fret)
                    {
                        // Compare pitch numbers to handle enharmonic equivalents correctly
                        if calculated_tuning.number() == tuning.number() {
                            positions.push(StringedPosition::new(string_index, fret));
                        }
                    }
                }
            }
        }

        positions
    }

    /// Clear the position cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.position_cache.try_borrow_mut() {
            cache.clear();
        }
    }

    /// Get the current cache size
    pub fn cache_size(&self) -> usize {
        self.position_cache
            .try_borrow()
            .map_or(0, |cache| cache.len())
    }

    /// Check if a string index is valid
    pub fn is_string_valid(&self, string_index: usize) -> bool {
        string_index < self.string_count()
    }

    /// Check if a fret number is valid
    pub fn is_fret_valid(&self, fret: usize) -> bool {
        fret <= self.config.fret_count
    }

    /// Apply a capo at the specified fret
    ///
    /// This creates a new fretboard configuration where all open strings
    /// are effectively raised by the capo fret amount.
    ///
    /// # Arguments
    /// * `capo_fret` - The fret where the capo is placed
    ///
    /// # Returns
    /// * `Ok(StringedFretboard)` with capo applied
    /// * `Err(FretboardError)` if capo fret is invalid
    pub fn with_capo(&self, capo_fret: usize) -> FretboardResult<Self> {
        if capo_fret > self.config.fret_count {
            return Err(FretboardError::invalid_position_with_context(
                format!("Capo fret {} exceeds fret count {}", capo_fret, self.config.fret_count),
                "Capo position is outside valid range"
            ));
        }

        if capo_fret == 0 {
            return Ok(self.clone());
        }

        // Calculate new string tunings with capo
        let mut new_strings = Vec::new();
        for &base_tuning in &self.config.strings {
            let interval = Interval::from_semitones(capo_fret as i8).map_err(|_| {
                FretboardError::invalid_configuration_with_fix(
                    format!("Invalid capo fret: {}", capo_fret)
                )
            })?;

            let new_tuning = base_tuning.add_interval(&interval).map_err(|_| {
                FretboardError::tuning_out_of_range_with_range(
                    &base_tuning,
                    "Check instrument's playable range with capo"
                )
            })?;

            new_strings.push(new_tuning);
        }

        // Create new config with adjusted fret count
        let new_config = StringedInstrumentConfig::new(
            new_strings,
            self.config.fret_count - capo_fret,
            self.config.scale_length,
            self.config.nut_width,
            self.config.string_spacing,
        );

        Self::new(new_config)
    }

    /// Apply scordatura (alternative tuning) to specific strings
    ///
    /// # Arguments
    /// * `string_tunings` - Vector of (string_index, new_tuning) pairs
    ///
    /// # Returns
    /// * `Ok(StringedFretboard)` with new tuning applied
    /// * `Err(FretboardError)` if any string index is invalid
    pub fn with_scordatura(&self, string_tunings: Vec<(usize, Tuning)>) -> FretboardResult<Self> {
        let mut new_strings = self.config.strings.clone();

        for (string_index, new_tuning) in string_tunings {
            if string_index >= new_strings.len() {
                return Err(FretboardError::invalid_position_with_context(
                    format!("String index {} exceeds string count {}", string_index, new_strings.len()),
                    "String index is outside valid range"
                ));
            }
            new_strings[string_index] = new_tuning;
        }

        let new_config = StringedInstrumentConfig::new(
            new_strings,
            self.config.fret_count,
            self.config.scale_length,
            self.config.nut_width,
            self.config.string_spacing,
        );

        Self::new(new_config)
    }

    /// Get the effective fret number accounting for capo
    ///
    /// This is useful when working with tablature or fingering patterns
    /// that need to account for capo position.
    ///
    /// # Arguments
    /// * `actual_fret` - The physical fret being pressed
    /// * `capo_fret` - The fret where capo is placed (0 if no capo)
    ///
    /// # Returns
    /// * The effective fret number relative to the capo
    pub fn effective_fret(actual_fret: usize, capo_fret: usize) -> usize {
        actual_fret.saturating_sub(capo_fret)
    }
}

impl Fretboard for StringedFretboard {
    type Position = StringedPosition;
    type Config = StringedInstrumentConfig;

    fn tuning_at_position(&self, position: &Self::Position) -> Option<Tuning> {
        self.calculate_tuning_at_fret(position.string, position.fret)
    }

    fn positions_for_tuning(&self, tuning: &Tuning) -> Vec<Self::Position> {
        // Use alternate format to include octave information in cache key
        let cache_key = format!("{:#}", tuning);

        // Try to get from cache
        if let Ok(cache) = self.position_cache.try_borrow() {
            if let Some(cached_positions) = cache.get(&cache_key) {
                return cached_positions.clone();
            }
        }

        // Calculate positions if not in cache
        let positions = self.find_positions_uncached(tuning);

        // Try to cache the result
        if let Ok(mut cache) = self.position_cache.try_borrow_mut() {
            cache.insert(cache_key, positions.clone());
        }

        positions
    }

    fn is_position_valid(&self, position: &Self::Position) -> bool {
        self.is_string_valid(position.string) && self.is_fret_valid(position.fret)
    }

    fn get_range(&self) -> (Self::Position, Self::Position) {
        let min_position = StringedPosition::new(0, 0);
        let max_position = StringedPosition::new(
            self.string_count().saturating_sub(1),
            self.config.fret_count,
        );
        (min_position, max_position)
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }

    fn get_all_positions(&self) -> Vec<Self::Position> {
        let mut positions = Vec::new();

        for string in 0..self.string_count() {
            for fret in 0..=self.config.fret_count {
                positions.push(StringedPosition::new(string, fret));
            }
        }

        positions
    }

    fn position_distance(&self, pos1: &Self::Position, pos2: &Self::Position) -> f32 {
        // Calculate distance based on string and fret differences
        let string_diff = (pos1.string as i32 - pos2.string as i32).abs() as f32;
        let fret_diff = (pos1.fret as i32 - pos2.fret as i32).abs() as f32;

        // Weight fret distance more heavily than string distance
        // as fret changes require more hand movement
        string_diff + (fret_diff * 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use proptest::prelude::*;
    use std::str::FromStr;

    fn create_standard_guitar_config() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::from_str("E2").unwrap(), // Low E
            Tuning::from_str("A2").unwrap(), // A
            Tuning::from_str("D3").unwrap(), // D
            Tuning::from_str("G3").unwrap(), // G
            Tuning::from_str("B3").unwrap(), // B
            Tuning::from_str("E4").unwrap(), // High E
        ];

        StringedInstrumentConfig::new(tunings, 24, 648.0, 43.0, 10.5)
    }

    #[test]
    fn test_stringed_fretboard_creation() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        assert_eq!(fretboard.string_count(), 6);
        assert_eq!(fretboard.fret_count(), 24);
        assert_eq!(fretboard.cache_size(), 0);
    }

    #[test]
    fn test_invalid_configuration() {
        let invalid_config = StringedInstrumentConfig::new(vec![], 0, -1.0, 0.0, 0.0);
        let result = StringedFretboard::new(invalid_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_tuning() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        // Test valid string indices
        assert_eq!(
            fretboard.string_tuning(0),
            Some(&Tuning::from_str("E2").unwrap())
        );
        assert_eq!(
            fretboard.string_tuning(5),
            Some(&Tuning::from_str("E4").unwrap())
        );

        // Test invalid string index
        assert_eq!(fretboard.string_tuning(6), None);
    }

    #[test]
    fn test_tuning_at_position() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        // Test open strings
        assert_eq!(
            fretboard.tuning_at_position(&StringedPosition::new(0, 0)),
            Some(Tuning::from_str("E2").unwrap())
        );

        // Test fretted positions
        // 5th fret of low E string should be A2
        assert_eq!(
            fretboard.tuning_at_position(&StringedPosition::new(0, 5)),
            Some(Tuning::from_str("A2").unwrap())
        );

        // Test invalid positions
        assert_eq!(
            fretboard.tuning_at_position(&StringedPosition::new(6, 0)),
            None
        );
        assert_eq!(
            fretboard.tuning_at_position(&StringedPosition::new(0, 25)),
            None
        );
    }

    #[test]
    fn test_positions_for_tuning() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        // Test finding positions for A2
        let a2_positions = fretboard.positions_for_tuning(&Tuning::from_str("A2").unwrap());

        // A2 should be found at:
        // - String 0 (low E), fret 5
        // - String 1 (A), fret 0 (open)
        assert!(a2_positions.contains(&StringedPosition::new(0, 5)));
        assert!(a2_positions.contains(&StringedPosition::new(1, 0)));

        // Test finding positions for a tuning that doesn't exist in range
        let c8_positions = fretboard.positions_for_tuning(&Tuning::from_str("C8").unwrap());
        assert!(c8_positions.is_empty());
    }

    #[test]
    fn test_position_validation() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        // Test valid positions
        assert!(fretboard.is_position_valid(&StringedPosition::new(0, 0)));
        assert!(fretboard.is_position_valid(&StringedPosition::new(5, 24)));

        // Test invalid positions
        assert!(!fretboard.is_position_valid(&StringedPosition::new(6, 0))); // Invalid string
        assert!(!fretboard.is_position_valid(&StringedPosition::new(0, 25))); // Invalid fret
    }

    #[test]
    fn test_get_range() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        let (min_pos, max_pos) = fretboard.get_range();

        assert_eq!(min_pos, StringedPosition::new(0, 0));
        assert_eq!(max_pos, StringedPosition::new(5, 24));
    }

    #[test]
    fn test_get_all_positions() {
        let config = StringedInstrumentConfig::new(
            vec![
                Tuning::from_str("E2").unwrap(),
                Tuning::from_str("A2").unwrap(),
            ],
            2,
            648.0,
            43.0,
            10.5,
        );
        let fretboard = StringedFretboard::new(config).unwrap();

        let all_positions = fretboard.get_all_positions();

        // Should have 2 strings * 3 frets (0, 1, 2) = 6 positions
        assert_eq!(all_positions.len(), 6);

        // Check that all expected positions are present
        assert!(all_positions.contains(&StringedPosition::new(0, 0)));
        assert!(all_positions.contains(&StringedPosition::new(0, 1)));
        assert!(all_positions.contains(&StringedPosition::new(0, 2)));
        assert!(all_positions.contains(&StringedPosition::new(1, 0)));
        assert!(all_positions.contains(&StringedPosition::new(1, 1)));
        assert!(all_positions.contains(&StringedPosition::new(1, 2)));
    }

    #[test]
    fn test_position_distance() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        let pos1 = StringedPosition::new(0, 0);
        let pos2 = StringedPosition::new(1, 2);

        let distance = fretboard.position_distance(&pos1, &pos2);

        // Distance should be: 1 (string diff) + 2*2 (fret diff * weight) = 5.0
        assert_eq!(distance, 5.0);

        // Test same position
        let same_distance = fretboard.position_distance(&pos1, &pos1);
        assert_eq!(same_distance, 0.0);
    }

    #[test]
    fn test_with_cache() {
        let config = create_standard_guitar_config();
        let cache_tunings = vec![
            Tuning::from_str("A2").unwrap(),
            Tuning::from_str("E4").unwrap(),
        ];

        let fretboard = StringedFretboard::with_cache(config, cache_tunings).unwrap();

        // Verify the cache was populated
        assert_eq!(fretboard.string_count(), 6);
        assert_eq!(fretboard.cache_size(), 2); // Should have cached 2 tunings
    }

    #[test]
    fn test_cache_operations() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        assert_eq!(fretboard.cache_size(), 0);

        // Test caching by looking up a tuning
        let a2_positions = fretboard.positions_for_tuning(&Tuning::from_str("A2").unwrap());
        assert!(!a2_positions.is_empty());
        assert_eq!(fretboard.cache_size(), 1);

        // Test cache hit
        let a2_positions_cached = fretboard.positions_for_tuning(&Tuning::from_str("A2").unwrap());
        assert_eq!(a2_positions, a2_positions_cached);
        assert_eq!(fretboard.cache_size(), 1); // Should still be 1

        fretboard.clear_cache();
        assert_eq!(fretboard.cache_size(), 0);
    }

    #[test]
    fn test_string_and_fret_validation() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        // Test string validation
        assert!(fretboard.is_string_valid(0));
        assert!(fretboard.is_string_valid(5));
        assert!(!fretboard.is_string_valid(6));

        // Test fret validation
        assert!(fretboard.is_fret_valid(0));
        assert!(fretboard.is_fret_valid(24));
        assert!(!fretboard.is_fret_valid(25));
    }

    #[test]
    fn test_capo_functionality() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        // Test capo at 2nd fret
        let capo_fretboard = fretboard.with_capo(2).unwrap();

        // Open strings should now be 2 semitones higher
        let original_low_e = fretboard.string_tuning(0).unwrap();
        let capo_low_e = capo_fretboard.string_tuning(0).unwrap();

        assert_eq!(capo_low_e.number(), original_low_e.number() + 2);

        // Fret count should be reduced
        assert_eq!(capo_fretboard.fret_count(), fretboard.fret_count() - 2);

        // Test invalid capo position
        let invalid_capo = fretboard.with_capo(25);
        assert!(invalid_capo.is_err());

        // Test capo at fret 0 (no change)
        let no_capo = fretboard.with_capo(0).unwrap();
        assert_eq!(no_capo.string_tuning(0), fretboard.string_tuning(0));
    }

    #[test]
    fn test_scordatura() {
        let config = create_standard_guitar_config();
        let fretboard = StringedFretboard::new(config).unwrap();

        // Apply drop D tuning (lower the low E string to D)
        let drop_d_tuning = vec![(0, Tuning::from_str("D2").unwrap())];
        let drop_d_fretboard = fretboard.with_scordatura(drop_d_tuning).unwrap();

        // Check that the first string is now D2
        assert_eq!(
            drop_d_fretboard.string_tuning(0),
            Some(&Tuning::from_str("D2").unwrap())
        );

        // Other strings should remain unchanged
        assert_eq!(
            drop_d_fretboard.string_tuning(1),
            fretboard.string_tuning(1)
        );

        // Test invalid string index
        let invalid_scordatura = vec![(6, Tuning::from_str("C4").unwrap())];
        let result = fretboard.with_scordatura(invalid_scordatura);
        assert!(result.is_err());
    }

    #[test]
    fn test_effective_fret_calculation() {
        // Test with no capo
        assert_eq!(StringedFretboard::effective_fret(5, 0), 5);

        // Test with capo at 2nd fret
        assert_eq!(StringedFretboard::effective_fret(5, 2), 3);
        assert_eq!(StringedFretboard::effective_fret(2, 2), 0);

        // Test edge case where actual fret is less than capo fret
        assert_eq!(StringedFretboard::effective_fret(1, 2), 0);
    }

    // Property-based test generators
    fn arb_stringed_fretboard_config() -> impl Strategy<Value = StringedInstrumentConfig> {
        // Generate configurations with reasonable parameters
        prop::collection::vec(
            // Generate tunings from C2 to C6 range
            (24u8..=84u8).prop_map(|midi_num| {
                let octave = (midi_num / 12) as i8 - 1;
                let pitch_class_index = midi_num % 12;
                let pitch_class = match pitch_class_index {
                    0 => PitchClass::C,
                    1 => PitchClass::C.sharp(),
                    2 => PitchClass::D,
                    3 => PitchClass::D.sharp(),
                    4 => PitchClass::E,
                    5 => PitchClass::F,
                    6 => PitchClass::F.sharp(),
                    7 => PitchClass::G,
                    8 => PitchClass::G.sharp(),
                    9 => PitchClass::A,
                    10 => PitchClass::A.sharp(),
                    11 => PitchClass::B,
                    _ => unreachable!(),
                };
                Tuning::new(pitch_class, octave)
            }),
            3..=8, // 3 to 8 strings
        )
        .prop_flat_map(|strings| {
            let _string_count = strings.len();
            (
                Just(strings),
                12usize..=24,       // 12 to 24 frets
                600.0f32..700.0f32, // Scale length in mm
                35.0f32..50.0f32,   // Nut width in mm
                8.0f32..12.0f32,    // String spacing in mm
            )
        })
        .prop_map(
            |(strings, fret_count, scale_length, nut_width, string_spacing)| {
                StringedInstrumentConfig::new(
                    strings,
                    fret_count,
                    scale_length,
                    nut_width,
                    string_spacing,
                )
            },
        )
    }

    proptest! {
        /// **Property 1: Position-Tuning Round Trip Consistency**
        /// **Validates: Requirements 1.2, 3.1, 3.2**
        ///
        /// For any valid fretboard position, converting the position to a tuning
        /// and then finding all positions for that tuning should include the original position.
        #[test]
        fn prop_position_tuning_round_trip_consistency(
            config in arb_stringed_fretboard_config(),
        ) {
            // Create fretboard from generated config
            let fretboard = StringedFretboard::new(config.clone()).unwrap();

            // Generate a valid position for this fretboard
            let position = StringedPosition::new(
                0, // Use first string to ensure validity
                std::cmp::min(5, config.fret_count) // Use a reasonable fret within range
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
        /// Test the round trip property for multiple valid positions on the fretboard.
        #[test]
        fn prop_position_tuning_round_trip_all_positions(
            config in arb_stringed_fretboard_config(),
        ) {
            let fretboard = StringedFretboard::new(config.clone()).unwrap();

            // Test multiple positions across the fretboard
            for string in 0..std::cmp::min(config.strings.len(), 3) { // Test first 3 strings
                for fret in 0..=std::cmp::min(config.fret_count, 12) { // Test first 12 frets
                    let position = StringedPosition::new(string, fret);

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
        }

        /// **Property 1 Inverse: Tuning-Position Round Trip Consistency**
        /// **Validates: Requirements 1.2, 3.1, 3.2**
        ///
        /// For any tuning that can be played on the fretboard, all returned positions
        /// should produce the same tuning when queried (accounting for enharmonic equivalents).
        #[test]
        fn prop_tuning_position_round_trip_consistency(
            config in arb_stringed_fretboard_config(),
        ) {
            let fretboard = StringedFretboard::new(config.clone()).unwrap();

            // Test with the open string tunings (guaranteed to be playable)
            for (_string_idx, &open_tuning) in config.strings.iter().enumerate().take(3) {
                let positions = fretboard.positions_for_tuning(&open_tuning);

                // Every returned position should produce the same pitch (accounting for enharmonic equivalents)
                for position in positions {
                    if let Some(found_tuning) = fretboard.tuning_at_position(&position) {
                        prop_assert_eq!(
                            found_tuning.number(), open_tuning.number(),
                            "Position {:?} produced tuning {} (pitch number {}) but expected {} (pitch number {})",
                            position, found_tuning, found_tuning.number(), open_tuning, open_tuning.number()
                        );
                    }
                }
            }
        }

        /// **Property 4: Tuning System Consistency**
        /// **Validates: Requirements 3.3, 3.6**
        ///
        /// For any tuning system and instrument configuration, all position-to-tuning
        /// calculations should be consistent with the specified tuning system.
        /// This means that the interval between adjacent frets should always be exactly
        /// one semitone (100 cents) in equal temperament.
        #[test]
        fn prop_tuning_system_consistency(
            config in arb_stringed_fretboard_config(),
        ) {
            let fretboard = StringedFretboard::new(config.clone()).unwrap();

            // Test consistency across all strings
            for string_idx in 0..config.strings.len() {
                // Test that adjacent frets are exactly one semitone apart
                for fret in 0..std::cmp::min(config.fret_count, 12) {
                    let pos1 = StringedPosition::new(string_idx, fret);
                    let pos2 = StringedPosition::new(string_idx, fret + 1);

                    if let (Some(tuning1), Some(tuning2)) = (
                        fretboard.tuning_at_position(&pos1),
                        fretboard.tuning_at_position(&pos2)
                    ) {
                        // Calculate the interval between adjacent frets
                        let semitone_diff = tuning2.number() - tuning1.number();

                        prop_assert_eq!(
                            semitone_diff, 1,
                            "Adjacent frets on string {} (fret {} -> {}) should differ by exactly 1 semitone. \
                             Found: {} -> {} (difference: {})",
                            string_idx, fret, fret + 1, tuning1, tuning2, semitone_diff
                        );
                    }
                }

                // Test that the tuning system is consistent with chromatic intervals
                // Each fret should raise the pitch by exactly the expected number of semitones
                if let Some(open_tuning) = fretboard.string_tuning(string_idx) {
                    for fret in 1..=std::cmp::min(config.fret_count, 12) {
                        let position = StringedPosition::new(string_idx, fret);

                        if let Some(fretted_tuning) = fretboard.tuning_at_position(&position) {
                            let expected_pitch_number = open_tuning.number() + fret as i8;

                            prop_assert_eq!(
                                fretted_tuning.number(), expected_pitch_number,
                                "Fret {} on string {} should produce pitch number {} (open: {} + {} frets), \
                                 but found pitch number {} ({})",
                                fret, string_idx, expected_pitch_number,
                                open_tuning.number(), fret, fretted_tuning.number(), fretted_tuning
                            );
                        }
                    }
                }
            }
        }

        /// **Property 4 Extended: Tuning System Consistency with Alternative Tunings**
        /// **Validates: Requirements 3.3, 3.6**
        ///
        /// Test tuning system consistency when using alternative tunings (scordatura).
        /// The chromatic interval relationships should remain consistent regardless
        /// of the base tuning of each string.
        #[test]
        fn prop_tuning_system_consistency_scordatura(
            config in arb_stringed_fretboard_config(),
        ) {
            let fretboard = StringedFretboard::new(config.clone()).unwrap();

            // Apply a simple scordatura: lower the first string by a whole tone
            if !config.strings.is_empty() {
                let original_tuning = config.strings[0];

                // Calculate a tuning 2 semitones lower (whole tone down)
                if let Ok(lower_interval) = Interval::from_semitones(-2) {
                    if let Ok(new_tuning) = original_tuning.add_interval(&lower_interval) {
                        let scordatura = vec![(0, new_tuning)];

                        if let Ok(alt_fretboard) = fretboard.with_scordatura(scordatura) {
                            // Test that the chromatic consistency is maintained on the retuned string
                            for fret in 0..std::cmp::min(config.fret_count, 8) {
                                let pos1 = StringedPosition::new(0, fret);
                                let pos2 = StringedPosition::new(0, fret + 1);

                                if let (Some(tuning1), Some(tuning2)) = (
                                    alt_fretboard.tuning_at_position(&pos1),
                                    alt_fretboard.tuning_at_position(&pos2)
                                ) {
                                    let semitone_diff = tuning2.number() - tuning1.number();

                                    prop_assert_eq!(
                                        semitone_diff, 1,
                                        "Scordatura: Adjacent frets on retuned string 0 (fret {} -> {}) \
                                         should still differ by exactly 1 semitone. \
                                         Found: {} -> {} (difference: {})",
                                        fret, fret + 1, tuning1, tuning2, semitone_diff
                                    );
                                }
                            }

                            // Test that other strings remain unaffected
                            if config.strings.len() > 1 {
                                for string_idx in 1..std::cmp::min(config.strings.len(), 3) {
                                    let original_pos = StringedPosition::new(string_idx, 0);
                                    let alt_pos = StringedPosition::new(string_idx, 0);

                                    if let (Some(original_tuning), Some(alt_tuning)) = (
                                        fretboard.tuning_at_position(&original_pos),
                                        alt_fretboard.tuning_at_position(&alt_pos)
                                    ) {
                                        prop_assert_eq!(
                                            original_tuning.number(), alt_tuning.number(),
                                            "Scordatura should not affect other strings. \
                                             String {} changed from {} to {}",
                                            string_idx, original_tuning, alt_tuning
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        /// **Property 4 Capo: Tuning System Consistency with Capo**
        /// **Validates: Requirements 3.3, 3.6**
        ///
        /// Test that tuning system consistency is maintained when using a capo.
        /// All strings should be raised by the same interval, and chromatic
        /// relationships should be preserved.
        #[test]
        fn prop_tuning_system_consistency_capo(
            config in arb_stringed_fretboard_config(),
        ) {
            let fretboard = StringedFretboard::new(config.clone()).unwrap();

            // Test with capo at various positions
            for capo_fret in 1..=std::cmp::min(config.fret_count / 2, 5) {
                if let Ok(capo_fretboard) = fretboard.with_capo(capo_fret) {
                    // Test that all open strings are raised by exactly capo_fret semitones
                    for string_idx in 0..config.strings.len() {
                        let original_open = StringedPosition::new(string_idx, 0);
                        let capo_open = StringedPosition::new(string_idx, 0);

                        if let (Some(original_tuning), Some(capo_tuning)) = (
                            fretboard.tuning_at_position(&original_open),
                            capo_fretboard.tuning_at_position(&capo_open)
                        ) {
                            let pitch_diff = capo_tuning.number() - original_tuning.number();

                            prop_assert_eq!(
                                pitch_diff, capo_fret as i8,
                                "Capo at fret {} should raise string {} by {} semitones. \
                                 Original: {} ({}), Capo: {} ({}), Difference: {}",
                                capo_fret, string_idx, capo_fret,
                                original_tuning, original_tuning.number(),
                                capo_tuning, capo_tuning.number(), pitch_diff
                            );
                        }
                    }

                    // Test that chromatic consistency is maintained with capo
                    for string_idx in 0..std::cmp::min(config.strings.len(), 2) {
                        for fret in 0..std::cmp::min(capo_fretboard.fret_count(), 6) {
                            let pos1 = StringedPosition::new(string_idx, fret);
                            let pos2 = StringedPosition::new(string_idx, fret + 1);

                            if let (Some(tuning1), Some(tuning2)) = (
                                capo_fretboard.tuning_at_position(&pos1),
                                capo_fretboard.tuning_at_position(&pos2)
                            ) {
                                let semitone_diff = tuning2.number() - tuning1.number();

                                prop_assert_eq!(
                                    semitone_diff, 1,
                                    "Capo: Adjacent frets on string {} (fret {} -> {}) \
                                     should differ by exactly 1 semitone. \
                                     Found: {} -> {} (difference: {})",
                                    string_idx, fret, fret + 1, tuning1, tuning2, semitone_diff
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

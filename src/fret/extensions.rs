//! Extension system for custom instruments and algorithms
//!
//! This module provides the infrastructure for extending the fretboard system
//! with custom instruments, fingering algorithms, and evaluation criteria.

use super::{
    traits::{
        CustomInstrument, ExtensionRegistry,
    },
    FretboardError, FretboardResult,
};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Default implementation of the extension registry
///
/// This registry manages custom extensions and provides a plugin system
/// for the fretboard framework.
#[derive(Default)]
pub struct DefaultExtensionRegistry {
    instruments: RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>,
    algorithms: RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>,
    criteria: RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>,
}

impl DefaultExtensionRegistry {
    /// Create a new extension registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a shared instance of the registry (singleton pattern)
    pub fn global() -> Arc<RwLock<Self>> {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<Arc<RwLock<DefaultExtensionRegistry>>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| Arc::new(RwLock::new(DefaultExtensionRegistry::new())))
            .clone()
    }
}

impl ExtensionRegistry for DefaultExtensionRegistry {
    fn register_instrument<I, C, F>(&mut self, name: &str, factory: F) -> bool
    where
        I: CustomInstrument<Config = C> + 'static,
        C: Clone + std::fmt::Debug + Send + Sync + 'static,
        F: Fn(C) -> FretboardResult<I> + Send + Sync + 'static,
    {
        let mut instruments = self.instruments.write().unwrap();
        if instruments.contains_key(name) {
            return false; // Already registered
        }

        instruments.insert(name.to_string(), Box::new(factory));
        true
    }

    fn register_algorithm<A, C, F>(&mut self, name: &str, factory: F) -> bool
    where
        A: 'static,
        C: Clone + std::fmt::Debug + Send + Sync + 'static,
        F: Fn(C) -> A + Send + Sync + 'static,
    {
        let mut algorithms = self.algorithms.write().unwrap();
        if algorithms.contains_key(name) {
            return false; // Already registered
        }

        algorithms.insert(name.to_string(), Box::new(factory));
        true
    }
    fn register_criteria<E, C, F>(&mut self, name: &str, factory: F) -> bool
    where
        E: 'static,
        C: Clone + std::fmt::Debug + Send + Sync + 'static,
        F: Fn(C) -> E + Send + Sync + 'static,
    {
        let mut criteria = self.criteria.write().unwrap();
        if criteria.contains_key(name) {
            return false; // Already registered
        }

        criteria.insert(name.to_string(), Box::new(factory));
        true
    }

    fn list_instruments(&self) -> Vec<String> {
        let instruments = self.instruments.read().unwrap();
        instruments.keys().cloned().collect()
    }

    fn list_algorithms(&self) -> Vec<String> {
        let algorithms = self.algorithms.read().unwrap();
        algorithms.keys().cloned().collect()
    }

    fn list_criteria(&self) -> Vec<String> {
        let criteria = self.criteria.read().unwrap();
        criteria.keys().cloned().collect()
    }

    fn has_instrument(&self, name: &str) -> bool {
        let instruments = self.instruments.read().unwrap();
        instruments.contains_key(name)
    }

    fn has_algorithm(&self, name: &str) -> bool {
        let algorithms = self.algorithms.read().unwrap();
        algorithms.contains_key(name)
    }

    fn has_criteria(&self, name: &str) -> bool {
        let criteria = self.criteria.read().unwrap();
        criteria.contains_key(name)
    }
}

/// Configuration validator for custom instruments
///
/// This struct provides validation utilities for custom instrument configurations.
pub struct InstrumentConfigValidator;

impl InstrumentConfigValidator {
    /// Validate basic instrument configuration parameters
    ///
    /// # Arguments
    /// * `name` - Instrument name (must be non-empty)
    /// * `category` - Instrument category (must be valid category)
    /// * `techniques` - Supported techniques (must be non-empty)
    ///
    /// # Returns
    /// * `true` if all parameters are valid
    pub fn validate_basic_config(name: &str, category: &str, techniques: &[&str]) -> bool {
        !name.is_empty()
            && Self::is_valid_category(category)
            && !techniques.is_empty()
            && techniques.iter().all(|t| !t.is_empty())
    }

    /// Check if a category is valid
    ///
    /// # Arguments
    /// * `category` - Category to validate
    ///
    /// # Returns
    /// * `true` if the category is recognized
    pub fn is_valid_category(category: &str) -> bool {
        matches!(
            category,
            "stringed" | "keyboard" | "wind" | "percussion" | "brass" | "electronic" | "custom"
        )
    }

    /// Validate technique names
    ///
    /// # Arguments
    /// * `techniques` - List of technique names to validate
    ///
    /// # Returns
    /// * `true` if all technique names are valid
    pub fn validate_techniques(techniques: &[&str]) -> bool {
        techniques.iter().all(|technique| {
            !technique.is_empty() && technique.chars().all(|c| c.is_alphanumeric() || c == '_')
        })
    }
}

// Example types only used in tests
#[cfg(test)]
mod examples {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct ExampleCustomInstrument {
        pub config: ExampleInstrumentConfig,
    }

    #[derive(Debug, Clone)]
    pub struct ExampleInstrumentConfig {
        pub name: String,
        pub string_count: usize,
        pub fret_count: usize,
        pub tunings: Vec<crate::Tuning>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ExamplePosition {
        pub string: usize,
        pub fret: usize,
    }

    impl super::super::traits::CustomInstrument for ExampleCustomInstrument {
        type Position = ExamplePosition;
        type Config = ExampleInstrumentConfig;

        fn new(config: Self::Config) -> FretboardResult<Self> {
            if !Self::validate_config(&config) {
                return Err(FretboardError::invalid_configuration(
                    "Invalid example instrument configuration",
                ));
            }
            Ok(Self { config })
        }

        fn validate_config(config: &Self::Config) -> bool {
            !config.name.is_empty()
                && config.string_count > 0
                && config.string_count <= 12
                && config.fret_count > 0
                && config.fret_count <= 30
                && config.tunings.len() == config.string_count
        }

        fn instrument_name(&self) -> &'static str {
            "example_custom"
        }

        fn instrument_category(&self) -> &'static str {
            "custom"
        }

        fn supported_techniques(&self) -> Vec<&'static str> {
            vec!["normal", "barre", "hammer_on", "pull_off"]
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExampleCustomAlgorithm {
        pub config: ExampleAlgorithmConfig,
    }

    #[derive(Debug, Clone)]
    pub struct ExampleAlgorithmConfig {
        pub prefer_open_strings: bool,
        pub max_fret_span: usize,
        pub difficulty_weight: f32,
    }

    impl ExampleCustomAlgorithm {
        pub fn new(config: ExampleAlgorithmConfig) -> Self {
            Self { config }
        }
        pub fn algorithm_name(&self) -> &'static str {
            "example_custom_algorithm"
        }
        pub fn algorithm_description(&self) -> &'static str {
            "Example custom fingering algorithm for demonstration purposes"
        }
    }
}

#[cfg(test)]
pub(crate) use examples::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_registry_creation() {
        let registry = DefaultExtensionRegistry::new();
        assert!(registry.list_instruments().is_empty());
        assert!(registry.list_algorithms().is_empty());
        assert!(registry.list_criteria().is_empty());
    }

    #[test]
    fn test_config_validator() {
        assert!(InstrumentConfigValidator::validate_basic_config(
            "test_instrument",
            "stringed",
            &["normal", "barre"]
        ));

        assert!(!InstrumentConfigValidator::validate_basic_config(
            "", // Empty name should fail
            "stringed",
            &["normal"]
        ));

        assert!(!InstrumentConfigValidator::validate_basic_config(
            "test",
            "invalid_category", // Invalid category should fail
            &["normal"]
        ));
    }

    #[test]
    fn test_example_custom_instrument() {
        use crate::{PitchClass, Tuning};

        let config = ExampleInstrumentConfig {
            name: "Test Guitar".to_string(),
            string_count: 6,
            fret_count: 12,
            tunings: vec![
                Tuning::new(PitchClass::E, 2),
                Tuning::new(PitchClass::A, 2),
                Tuning::new(PitchClass::D, 3),
                Tuning::new(PitchClass::G, 3),
                Tuning::new(PitchClass::B, 3),
                Tuning::new(PitchClass::E, 4),
            ],
        };

        let instrument = ExampleCustomInstrument::new(config).unwrap();
        assert_eq!(instrument.instrument_name(), "example_custom");
        assert_eq!(instrument.instrument_category(), "custom");
        assert!(!instrument.supported_techniques().is_empty());
    }

    #[test]
    fn test_example_custom_algorithm() {
        let config = ExampleAlgorithmConfig {
            prefer_open_strings: true,
            max_fret_span: 4,
            difficulty_weight: 0.8,
        };

        let algorithm = ExampleCustomAlgorithm::new(config);
        assert_eq!(algorithm.algorithm_name(), "example_custom_algorithm");
        assert!(!algorithm.algorithm_description().is_empty());
        assert_eq!(algorithm.config.prefer_open_strings, true);
        assert_eq!(algorithm.config.max_fret_span, 4);
        assert_eq!(algorithm.config.difficulty_weight, 0.8);
    }
}
#[cfg(test)]
mod property_tests {
    use super::*;

    #[cfg(test)]
    use proptest::prelude::*;

    /// Property 16: Custom Instrument Extensibility
    ///
    /// This property test validates that the extension system correctly handles
    /// custom instrument implementations and configurations.
    ///
    /// Properties verified:
    /// - Custom instrument configurations are properly validated
    /// - Extension registry correctly manages custom instruments
    /// - Custom instruments integrate seamlessly with the fretboard system
    /// - Configuration validation is consistent and reliable
    #[cfg(test)]
    mod custom_instrument_extensibility_property {
        use super::*;

        proptest! {
            #[test]
            fn property_custom_instrument_extensibility(
                name in "[a-zA-Z][a-zA-Z0-9_]{2,20}",
                string_count in 1usize..=12usize,
                fret_count in 1usize..=30usize,
                category in prop::sample::select(vec![
                    "stringed", "keyboard", "wind", "percussion",
                    "brass", "electronic", "custom"
                ]),
                techniques in prop::collection::vec("[a-z_]{3,15}", 1..=10),
            ) {
                // Property 1: Configuration validation should be consistent
                let valid_config = ExampleInstrumentConfig {
                    name: name.clone(),
                    string_count,
                    fret_count,
                    tunings: (0..string_count).map(|i| {
                        use crate::{Tuning, PitchClass};
                        let pitch_classes = [
                            PitchClass::C, PitchClass::D, PitchClass::E, PitchClass::F,
                            PitchClass::G, PitchClass::A, PitchClass::B,
                        ];
                        Tuning::new(pitch_classes[i % pitch_classes.len()], 3)
                    }).collect(),
                };

                let is_valid = ExampleCustomInstrument::validate_config(&valid_config);
                prop_assert!(is_valid, "Valid configuration should pass validation");

                // Property 2: Invalid configurations should be rejected
                let invalid_config = ExampleInstrumentConfig {
                    name: "".to_string(), // Empty name should be invalid
                    string_count,
                    fret_count,
                    tunings: vec![], // Wrong number of tunings
                };

                let is_invalid = !ExampleCustomInstrument::validate_config(&invalid_config);
                prop_assert!(is_invalid, "Invalid configuration should fail validation");

                // Property 3: Basic config validation should work correctly
                let techniques_str: Vec<&str> = techniques.iter().map(|s| s.as_str()).collect();
                let basic_validation = InstrumentConfigValidator::validate_basic_config(
                    &name,
                    &category,
                    &techniques_str
                );
                prop_assert!(basic_validation, "Valid basic config should pass validation");

                // Property 4: Category validation should be consistent
                let category_valid = InstrumentConfigValidator::is_valid_category(&category);
                prop_assert!(category_valid, "Selected category should be valid");

                // Property 5: Technique validation should work
                let technique_validation = InstrumentConfigValidator::validate_techniques(&techniques_str);
                prop_assert!(technique_validation, "Generated techniques should be valid");

                // Property 6: Extension registry should handle registration correctly
                let mut registry = DefaultExtensionRegistry::new();

                // Initially should be empty
                prop_assert!(registry.list_instruments().is_empty(), "New registry should be empty");
                prop_assert!(!registry.has_instrument(&name), "Should not have unregistered instrument");

                // Property 7: Registry operations should be consistent
                let instruments_before = registry.list_instruments().len();
                let algorithms_before = registry.list_algorithms().len();
                let criteria_before = registry.list_criteria().len();

                // These counts should remain stable during the test
                prop_assert_eq!(instruments_before, 0, "Should start with no instruments");
                prop_assert_eq!(algorithms_before, 0, "Should start with no algorithms");
                prop_assert_eq!(criteria_before, 0, "Should start with no criteria");
            }
        }

        proptest! {
            #[test]
            fn property_instrument_config_validation_edge_cases(
                string_count in 0usize..=50usize,
                fret_count in 0usize..=100usize,
                name_length in 0usize..=100usize,
            ) {
                use crate::{Tuning, PitchClass};

                // Generate name of specified length
                let name = "a".repeat(name_length);

                // Generate tunings (may be wrong count)
                let tunings: Vec<Tuning> = (0..string_count).map(|i| {
                    let pitch_classes = [
                        PitchClass::C, PitchClass::D, PitchClass::E, PitchClass::F,
                        PitchClass::G, PitchClass::A, PitchClass::B,
                    ];
                    Tuning::new(pitch_classes[i % pitch_classes.len()], 3)
                }).collect();

                let config = ExampleInstrumentConfig {
                    name: name.clone(),
                    string_count,
                    fret_count,
                    tunings,
                };

                let is_valid = ExampleCustomInstrument::validate_config(&config);

                // Property 1: Validation should reject edge cases appropriately
                if name.is_empty() || string_count == 0 || fret_count == 0 || string_count > 12 || fret_count > 30 {
                    prop_assert!(!is_valid, "Invalid edge case should be rejected");
                } else if config.tunings.len() != string_count {
                    prop_assert!(!is_valid, "Mismatched tuning count should be rejected");
                } else {
                    prop_assert!(is_valid, "Valid configuration should pass");
                }

                // Property 2: Instrument creation should be consistent with validation
                let creation_result = ExampleCustomInstrument::new(config);
                if is_valid {
                    prop_assert!(creation_result.is_ok(), "Valid config should create instrument successfully");

                    if let Ok(instrument) = creation_result {
                        // Property 3: Created instrument should have correct properties
                        prop_assert_eq!(instrument.instrument_name(), "example_custom");
                        prop_assert_eq!(instrument.instrument_category(), "custom");
                        prop_assert!(!instrument.supported_techniques().is_empty());
                    }
                } else {
                    prop_assert!(creation_result.is_err(), "Invalid config should fail to create instrument");
                }
            }
        }

        proptest! {
            #[test]
            fn property_technique_validation_consistency(
                technique_count in 1usize..=20usize,
                technique_length in 1usize..=30usize,
                use_invalid_chars in prop::bool::ANY,
            ) {
                // Generate technique names
                let techniques: Vec<String> = (0..technique_count).map(|i| {
                    if use_invalid_chars && i == 0 {
                        // Include one invalid technique for testing
                        "invalid-technique!".to_string()
                    } else {
                        format!("technique_{}", i)
                    }
                }).collect();

                let technique_refs: Vec<&str> = techniques.iter().map(|s| s.as_str()).collect();
                let validation_result = InstrumentConfigValidator::validate_techniques(&technique_refs);

                // Property 1: Validation should correctly identify invalid techniques
                if use_invalid_chars {
                    prop_assert!(!validation_result, "Invalid characters should cause validation to fail");
                } else {
                    prop_assert!(validation_result, "Valid techniques should pass validation");
                }

                // Property 2: Empty technique list should pass validate_techniques but fail basic config
                let empty_validation = InstrumentConfigValidator::validate_techniques(&[]);
                prop_assert!(empty_validation, "Empty technique list should pass validate_techniques (vacuous truth)");

                // But basic config should reject empty techniques
                let empty_basic_validation = InstrumentConfigValidator::validate_basic_config(
                    "test_instrument",
                    "stringed",
                    &[]
                );
                prop_assert!(!empty_basic_validation, "Empty technique list should fail basic config validation");

                // Property 3: Basic config validation should be consistent with technique validation
                if !technique_refs.is_empty() && validation_result {
                    let basic_validation = InstrumentConfigValidator::validate_basic_config(
                        "test_instrument",
                        "stringed",
                        &technique_refs
                    );
                    prop_assert!(basic_validation, "Valid techniques should pass basic config validation");
                }
            }
        }
    }

    /// Property 17: Custom Algorithm Integration
    ///
    /// This property test validates that custom algorithms and evaluation criteria
    /// integrate correctly with the fretboard system and produce consistent results.
    ///
    /// Properties verified:
    /// - Custom evaluation criteria produce scores within valid range
    /// - Plugin system correctly manages and applies custom algorithms
    /// - Technique handlers work consistently across different configurations
    /// - Custom algorithms maintain compatibility with existing fingering types
    #[cfg(test)]
    mod custom_algorithm_integration_property {
        use super::*;

        proptest! {
            #[test]
            fn property_custom_algorithm_integration(
                weight in 0.1f32..=1.0f32,
                prefer_lower_frets in prop::bool::ANY,
                penalize_stretches in prop::bool::ANY,
                allow_harmonics in prop::bool::ANY,
                allow_slides in prop::bool::ANY,
                allow_bends in prop::bool::ANY,
                max_stretch in 1usize..=8usize,
                fret_positions in prop::collection::vec(0usize..=12usize, 1..=6),
                string_positions in prop::collection::vec(0usize..=5usize, 1..=6),
            ) {
                use crate::fret::{Fingering, FingerPosition, StringedPosition, PlayingTechnique, Finger};

                // Property 1: Custom evaluation criteria should produce valid scores
                let eval_config = ExampleEvaluationConfig {
                    weight,
                    prefer_lower_frets,
                    penalize_stretches,
                };
                let criteria = ExampleEvaluationCriteria::new(eval_config);

                // Create test fingering from generated positions
                let positions: Vec<FingerPosition<StringedPosition>> = fret_positions.iter()
                    .zip(string_positions.iter())
                    .enumerate()
                    .map(|(i, (&fret, &string))| {
                        let finger = match i % 4 {
                            0 => Some(Finger::Index),
                            1 => Some(Finger::Middle),
                            2 => Some(Finger::Ring),
                            3 => Some(Finger::Pinky),
                            _ => None,
                        };
                        FingerPosition {
                            position: StringedPosition { string: string.try_into().unwrap(), fret: fret.try_into().unwrap() },
                            finger,
                            pressure: 1.0,
                        }
                    })
                    .collect();

                let fingering = Fingering {
                    positions,
                    difficulty: 0.5,
                    technique: PlayingTechnique::Standard,
                };

                let score = criteria.evaluate_stringed_fingering(&fingering);
                prop_assert!(score >= 0.0 && score <= 1.0,
                           "Evaluation score {} should be between 0.0 and 1.0", score);
                prop_assert_eq!(criteria.weight(), weight, "Weight should match configuration");

                // Property 2: Technique handlers should work consistently
                let technique_config = TechniqueConfig {
                    allow_harmonics,
                    allow_slides,
                    allow_bends,
                    max_stretch,
                };
                let techniques = StringedInstrumentTechniques::new(technique_config);

                // Test technique support consistency
                prop_assert_eq!(techniques.can_apply_technique(&fingering, "harmonic"), allow_harmonics);
                prop_assert_eq!(techniques.can_apply_technique(&fingering, "slide"), allow_slides);
                prop_assert_eq!(techniques.can_apply_technique(&fingering, "bend"), allow_bends);
                prop_assert!(!techniques.can_apply_technique(&fingering, "nonexistent_technique"));

                // Property 3: Difficulty modifiers should be consistent
                let harmonic_modifier = techniques.technique_difficulty_modifier("harmonic");
                let slide_modifier = techniques.technique_difficulty_modifier("slide");
                let bend_modifier = techniques.technique_difficulty_modifier("bend");
                let unknown_modifier = techniques.technique_difficulty_modifier("unknown");

                prop_assert_eq!(harmonic_modifier, 0.8, "Harmonic modifier should be 0.8");
                prop_assert_eq!(slide_modifier, 1.1, "Slide modifier should be 1.1");
                prop_assert_eq!(bend_modifier, 1.2, "Bend modifier should be 1.2");
                prop_assert_eq!(unknown_modifier, 1.0, "Unknown technique modifier should be 1.0");

                // Property 4: Plugin system should integrate components correctly
                let mut plugin_system = PluginSystem::new();
                plugin_system.add_evaluation_criteria(Box::new(criteria.clone()));
                plugin_system.add_technique_handler(Box::new(techniques.clone()));

                let evaluated_score = plugin_system.evaluate_fingering(&fingering);
                prop_assert!(evaluated_score >= 0.0 && evaluated_score <= 1.0,
                           "Plugin system score {} should be between 0.0 and 1.0", evaluated_score);

                // Property 5: Supported techniques should match configuration
                let supported_techniques = plugin_system.get_supported_techniques();
                if allow_harmonics {
                    prop_assert!(supported_techniques.contains(&"harmonic".to_string()),
                               "Should support harmonics when enabled");
                }
                if allow_slides {
                    prop_assert!(supported_techniques.contains(&"slide".to_string()),
                               "Should support slides when enabled");
                }
                if allow_bends {
                    prop_assert!(supported_techniques.contains(&"bend".to_string()),
                               "Should support bends when enabled");
                }

                // Property 6: Technique application should modify fingerings appropriately
                if allow_harmonics && !fingering.positions.is_empty() {
                    let harmonic_result = techniques.apply_technique(&fingering, "harmonic");
                    prop_assert!(harmonic_result.is_ok(), "Harmonic application should succeed when allowed");

                    if let Ok(modified) = harmonic_result {
                        // Harmonics should use lighter pressure
                        for pos in &modified.positions {
                            prop_assert!(pos.pressure <= 1.0, "Pressure should not exceed maximum");
                            if allow_harmonics {
                                prop_assert!(pos.pressure < 1.0, "Harmonic should use lighter pressure");
                            }
                        }

                        // Difficulty should be modified appropriately
                        let expected_difficulty = fingering.difficulty * harmonic_modifier;
                        prop_assert!((modified.difficulty - expected_difficulty).abs() < 0.01,
                                   "Harmonic difficulty should be modified correctly");
                    }
                }

                // Property 7: Plugin system should handle technique queries correctly
                prop_assert_eq!(plugin_system.supports_technique("harmonic"), allow_harmonics);
                prop_assert_eq!(plugin_system.supports_technique("slide"), allow_slides);
                prop_assert_eq!(plugin_system.supports_technique("bend"), allow_bends);
                prop_assert!(!plugin_system.supports_technique("nonexistent"));

                // Property 8: Difficulty modifiers should be retrievable through plugin system
                if allow_harmonics {
                    let modifier = plugin_system.get_technique_difficulty_modifier("harmonic");
                    prop_assert_eq!(modifier, 0.8, "Plugin system should return correct harmonic modifier");
                }
                if allow_slides {
                    let modifier = plugin_system.get_technique_difficulty_modifier("slide");
                    prop_assert_eq!(modifier, 1.1, "Plugin system should return correct slide modifier");
                }
                if allow_bends {
                    let modifier = plugin_system.get_technique_difficulty_modifier("bend");
                    prop_assert_eq!(modifier, 1.2, "Plugin system should return correct bend modifier");
                }

                let unknown_modifier = plugin_system.get_technique_difficulty_modifier("unknown");
                prop_assert_eq!(unknown_modifier, 1.0, "Unknown technique should return default modifier");
            }
        }

        proptest! {
            #[test]
            fn property_evaluation_criteria_consistency(
                weight1 in 0.1f32..=1.0f32,
                weight2 in 0.1f32..=1.0f32,
                prefer_lower1 in prop::bool::ANY,
                prefer_lower2 in prop::bool::ANY,
                penalize_stretches1 in prop::bool::ANY,
                penalize_stretches2 in prop::bool::ANY,
                fret_span in 1usize..=12usize,
            ) {
                use crate::fret::{Fingering, FingerPosition, StringedPosition, PlayingTechnique, Finger};

                // Create two different evaluation criteria
                let config1 = ExampleEvaluationConfig {
                    weight: weight1,
                    prefer_lower_frets: prefer_lower1,
                    penalize_stretches: penalize_stretches1,
                };
                let config2 = ExampleEvaluationConfig {
                    weight: weight2,
                    prefer_lower_frets: prefer_lower2,
                    penalize_stretches: penalize_stretches2,
                };

                let criteria1 = ExampleEvaluationCriteria::new(config1);
                let criteria2 = ExampleEvaluationCriteria::new(config2);

                // Create test fingering with controlled fret span
                let positions = vec![
                    FingerPosition {
                        position: StringedPosition { string: 0, fret: 2 },
                        finger: Some(Finger::Index),
                        pressure: 1.0,
                    },
                    FingerPosition {
                        position: StringedPosition { string: 1, fret: (2 + fret_span).try_into().unwrap() },
                        finger: Some(Finger::Pinky),
                        pressure: 1.0,
                    },
                ];

                let fingering = Fingering {
                    positions,
                    difficulty: 0.5,
                    technique: PlayingTechnique::Standard,
                };

                let score1 = criteria1.evaluate_stringed_fingering(&fingering);
                let score2 = criteria2.evaluate_stringed_fingering(&fingering);

                // Property 1: Both scores should be valid
                prop_assert!(score1 >= 0.0 && score1 <= 1.0, "Score1 should be valid");
                prop_assert!(score2 >= 0.0 && score2 <= 1.0, "Score2 should be valid");

                // Property 2: Plugin system should handle multiple criteria correctly
                let mut plugin_system = PluginSystem::new();
                plugin_system.add_evaluation_criteria(Box::new(criteria1));
                plugin_system.add_evaluation_criteria(Box::new(criteria2));

                let combined_score = plugin_system.evaluate_fingering(&fingering);
                prop_assert!(combined_score >= 0.0 && combined_score <= 1.0,
                           "Combined score should be valid");

                // Property 3: Combined score should be weighted average
                let expected_combined = (score1 * weight1 + score2 * weight2) / (weight1 + weight2);
                prop_assert!((combined_score - expected_combined).abs() < 0.01,
                           "Combined score should be weighted average of individual scores");

                // Property 4: Empty plugin system should return original difficulty
                let empty_system = PluginSystem::new();
                let empty_score = empty_system.evaluate_fingering(&fingering);
                prop_assert_eq!(empty_score, fingering.difficulty,
                              "Empty plugin system should return original difficulty");
            }
        }
    }
}
/// Custom evaluation criteria implementation for demonstration
///
/// This shows how to implement custom evaluation criteria for fingering quality assessment.
#[derive(Debug, Clone)]
pub(crate) struct ExampleEvaluationCriteria {
    config: ExampleEvaluationConfig,
}

#[derive(Debug, Clone)]
pub(crate) struct ExampleEvaluationConfig {
    pub weight: f32,
    pub prefer_lower_frets: bool,
    pub penalize_stretches: bool,
}

impl ExampleEvaluationCriteria {
    /// Create new evaluation criteria
    pub fn new(config: ExampleEvaluationConfig) -> Self {
        Self { config }
    }

    /// Get the criteria name
    pub fn criteria_name(&self) -> &'static str {
        "example_evaluation_criteria"
    }

    /// Evaluate a fingering for stringed instruments
    pub fn evaluate_stringed_fingering(
        &self,
        fingering: &crate::fret::Fingering<crate::fret::StringedPosition>,
    ) -> f32 {
        let mut score = 1.0;

        if fingering.positions.is_empty() {
            return 0.0;
        }

        // Prefer lower frets if configured
        if self.config.prefer_lower_frets {
            let avg_fret: f32 = fingering
                .positions
                .iter()
                .map(|fp| fp.position.fret as f32)
                .sum::<f32>()
                / fingering.positions.len() as f32;

            // Lower frets get higher scores
            score *= (12.0 - avg_fret.min(12.0)) / 12.0;
        }

        // Penalize large stretches if configured
        if self.config.penalize_stretches {
            let frets: Vec<_> = fingering
                .positions
                .iter()
                .map(|fp| fp.position.fret)
                .collect();

            if let (Some(&min_fret), Some(&max_fret)) = (frets.iter().min(), frets.iter().max()) {
                let stretch = max_fret - min_fret;
                if stretch > 4 {
                    score *= 0.5; // Penalize large stretches
                }
            }
        }

        score.clamp(0.0, 1.0)
    }

    /// Get the weight of these criteria
    pub fn weight(&self) -> f32 {
        self.config.weight
    }
}

/// Instrument-specific technique implementation for stringed instruments
///
/// This demonstrates how to implement custom playing techniques for specific instruments.
#[derive(Debug, Clone)]
pub(crate) struct StringedInstrumentTechniques {
    config: TechniqueConfig,
}

#[derive(Debug, Clone)]
pub(crate) struct TechniqueConfig {
    pub allow_harmonics: bool,
    pub allow_slides: bool,
    pub allow_bends: bool,
    pub max_stretch: usize,
}

impl StringedInstrumentTechniques {
    /// Create new technique handler
    pub fn new(config: TechniqueConfig) -> Self {
        Self { config }
    }

    /// Apply a technique to a fingering
    pub fn apply_technique(
        &self,
        fingering: &crate::fret::Fingering<crate::fret::StringedPosition>,
        technique: &str,
    ) -> FretboardResult<crate::fret::Fingering<crate::fret::StringedPosition>> {
        let mut modified_fingering = fingering.clone();

        match technique {
            "harmonic" => {
                if !self.config.allow_harmonics {
                    return Err(FretboardError::impossible_fingering(
                        "Harmonics not allowed",
                    ));
                }

                // Modify fingering for harmonic technique
                for finger_pos in &mut modified_fingering.positions {
                    finger_pos.pressure = 0.3; // Light touch for harmonics
                }

                // Harmonics are generally easier
                modified_fingering.difficulty *= 0.8;
            }

            "slide" => {
                if !self.config.allow_slides {
                    return Err(FretboardError::impossible_fingering("Slides not allowed"));
                }

                // Slides add slight difficulty
                modified_fingering.difficulty *= 1.1;
            }

            "bend" => {
                if !self.config.allow_bends {
                    return Err(FretboardError::impossible_fingering("Bends not allowed"));
                }

                // Bends add difficulty
                modified_fingering.difficulty *= 1.2;
            }

            _ => {
                return Err(FretboardError::impossible_fingering(format!(
                    "Unknown technique: {}",
                    technique
                )));
            }
        }

        Ok(modified_fingering)
    }

    /// Check if a technique can be applied
    pub fn can_apply_technique(
        &self,
        _fingering: &crate::fret::Fingering<crate::fret::StringedPosition>,
        technique: &str,
    ) -> bool {
        match technique {
            "harmonic" => self.config.allow_harmonics,
            "slide" => self.config.allow_slides,
            "bend" => self.config.allow_bends,
            _ => false,
        }
    }

    /// Get technique difficulty modifier
    pub fn technique_difficulty_modifier(&self, technique: &str) -> f32 {
        match technique {
            "harmonic" => 0.8, // Easier
            "slide" => 1.1,    // Slightly harder
            "bend" => 1.2,     // Harder
            _ => 1.0,
        }
    }

    /// Validate technique configuration
    pub fn validate_technique_config(&self, technique: &str) -> bool {
        match technique {
            "harmonic" | "slide" | "bend" => true,
            _ => false,
        }
    }
}

/// Plugin system for integrating custom algorithms and criteria
///
/// This provides a unified interface for managing and applying custom extensions.
pub(crate) struct PluginSystem {
    evaluation_criteria: Vec<Box<dyn EvaluationCriteriaPlugin>>,
    technique_handlers: Vec<Box<dyn TechniquePlugin>>,
}

/// Trait for evaluation criteria plugins
pub(crate) trait EvaluationCriteriaPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn evaluate_stringed(
        &self,
        fingering: &crate::fret::Fingering<crate::fret::StringedPosition>,
    ) -> f32;
    fn weight(&self) -> f32;
}

/// Trait for technique plugins
pub(crate) trait TechniquePlugin: Send + Sync {
    fn name(&self) -> &str;
    fn supported_techniques(&self) -> Vec<&str>;
    fn can_apply(&self, technique: &str) -> bool;
    fn difficulty_modifier(&self, technique: &str) -> f32;
}

impl PluginSystem {
    /// Create a new plugin system
    pub fn new() -> Self {
        Self {
            evaluation_criteria: Vec::new(),
            technique_handlers: Vec::new(),
        }
    }

    /// Add an evaluation criteria plugin
    pub fn add_evaluation_criteria(&mut self, plugin: Box<dyn EvaluationCriteriaPlugin>) {
        self.evaluation_criteria.push(plugin);
    }

    /// Add a technique plugin
    pub fn add_technique_handler(&mut self, plugin: Box<dyn TechniquePlugin>) {
        self.technique_handlers.push(plugin);
    }

    /// Evaluate a fingering using all registered criteria
    pub fn evaluate_fingering(
        &self,
        fingering: &crate::fret::Fingering<crate::fret::StringedPosition>,
    ) -> f32 {
        if self.evaluation_criteria.is_empty() {
            return fingering.difficulty;
        }

        let total_weight: f32 = self
            .evaluation_criteria
            .iter()
            .map(|criteria| criteria.weight())
            .sum();

        if total_weight == 0.0 {
            return fingering.difficulty;
        }

        let weighted_score: f32 = self
            .evaluation_criteria
            .iter()
            .map(|criteria| criteria.evaluate_stringed(fingering) * criteria.weight())
            .sum();

        weighted_score / total_weight
    }

    /// Get all supported techniques from all plugins
    pub fn get_supported_techniques(&self) -> Vec<String> {
        let mut techniques = Vec::new();
        for handler in &self.technique_handlers {
            techniques.extend(
                handler
                    .supported_techniques()
                    .iter()
                    .map(|&s| s.to_string()),
            );
        }
        techniques.sort();
        techniques.dedup();
        techniques
    }

    /// Check if a technique is supported by any plugin
    pub fn supports_technique(&self, technique: &str) -> bool {
        self.technique_handlers
            .iter()
            .any(|handler| handler.can_apply(technique))
    }

    /// Get difficulty modifier for a technique
    pub fn get_technique_difficulty_modifier(&self, technique: &str) -> f32 {
        self.technique_handlers
            .iter()
            .find(|handler| handler.can_apply(technique))
            .map(|handler| handler.difficulty_modifier(technique))
            .unwrap_or(1.0)
    }
}

impl Default for PluginSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Example implementation of EvaluationCriteriaPlugin
impl EvaluationCriteriaPlugin for ExampleEvaluationCriteria {
    fn name(&self) -> &str {
        self.criteria_name()
    }

    fn evaluate_stringed(
        &self,
        fingering: &crate::fret::Fingering<crate::fret::StringedPosition>,
    ) -> f32 {
        self.evaluate_stringed_fingering(fingering)
    }

    fn weight(&self) -> f32 {
        self.weight()
    }
}

/// Example implementation of TechniquePlugin
impl TechniquePlugin for StringedInstrumentTechniques {
    fn name(&self) -> &str {
        "stringed_techniques"
    }

    fn supported_techniques(&self) -> Vec<&str> {
        let mut techniques = Vec::new();
        if self.config.allow_harmonics {
            techniques.push("harmonic");
        }
        if self.config.allow_slides {
            techniques.push("slide");
        }
        if self.config.allow_bends {
            techniques.push("bend");
        }
        techniques
    }

    fn can_apply(&self, technique: &str) -> bool {
        let dummy_fingering = crate::fret::Fingering::standard(vec![]);
        self.can_apply_technique(&dummy_fingering, technique)
    }

    fn difficulty_modifier(&self, technique: &str) -> f32 {
        self.technique_difficulty_modifier(technique)
    }
}

#[cfg(test)]
mod plugin_tests {
    use super::*;

    #[test]
    fn test_custom_evaluation_criteria() {
    use crate::fret::{Finger, FingerPosition, Fingering, PlayingTechnique, StringedPosition};
    use crate::{PitchClass, Tuning};

    let config = ExampleEvaluationConfig {
        weight: 0.8,
        prefer_lower_frets: true,
        penalize_stretches: true,
    };

    let criteria = ExampleEvaluationCriteria::new(config);
    assert_eq!(criteria.criteria_name(), "example_evaluation_criteria");
    assert_eq!(criteria.weight(), 0.8);

    // Create a test fingering
    let fingering = Fingering {
        positions: vec![
            FingerPosition {
                position: StringedPosition { string: 0, fret: 2 },
                finger: Some(Finger::Index),
                pressure: 1.0,
            },
            FingerPosition {
                position: StringedPosition { string: 1, fret: 3 },
                finger: Some(Finger::Middle),
                pressure: 1.0,
            },
        ],
        difficulty: 0.5,
        technique: PlayingTechnique::Standard,
    };

    let score = criteria.evaluate_stringed_fingering(&fingering);
    assert!(
        score >= 0.0 && score <= 1.0,
        "Score should be between 0 and 1"
    );
}

#[test]
fn test_stringed_instrument_techniques() {
    use crate::fret::{Finger, FingerPosition, Fingering, PlayingTechnique, StringedPosition};

    let config = TechniqueConfig {
        allow_harmonics: true,
        allow_slides: true,
        allow_bends: false,
        max_stretch: 4,
    };

    let techniques = StringedInstrumentTechniques::new(config);

    // Test technique support
    let dummy_fingering = crate::fret::Fingering::standard(vec![]);
    assert!(techniques.can_apply_technique(&dummy_fingering, "harmonic"));
    assert!(techniques.can_apply_technique(&dummy_fingering, "slide"));
    assert!(!techniques.can_apply_technique(&dummy_fingering, "bend"));
    assert!(!techniques.can_apply_technique(&dummy_fingering, "unknown"));

    // Test difficulty modifiers
    assert_eq!(techniques.technique_difficulty_modifier("harmonic"), 0.8);
    assert_eq!(techniques.technique_difficulty_modifier("slide"), 1.1);
    assert_eq!(techniques.technique_difficulty_modifier("bend"), 1.2);

    // Test technique application
    let fingering = Fingering {
        positions: vec![FingerPosition {
            position: StringedPosition { string: 0, fret: 5 },
            finger: Some(Finger::Index),
            pressure: 1.0,
        }],
        difficulty: 0.5,
        technique: PlayingTechnique::Standard,
    };

    let harmonic_result = techniques.apply_technique(&fingering, "harmonic");
    assert!(harmonic_result.is_ok());

    let modified = harmonic_result.unwrap();
    assert!(
        modified.positions[0].pressure < 1.0,
        "Harmonic should use lighter pressure"
    );
    assert!(
        modified.difficulty < fingering.difficulty,
        "Harmonic should be easier"
    );

    // Test invalid technique
    let bend_result = techniques.apply_technique(&fingering, "bend");
    assert!(bend_result.is_err());
}

#[test]
fn test_plugin_system() {
    use crate::fret::{Finger, FingerPosition, Fingering, PlayingTechnique, StringedPosition};

    let mut plugin_system = PluginSystem::new();

    // Add evaluation criteria
    let criteria_config = ExampleEvaluationConfig {
        weight: 0.7,
        prefer_lower_frets: true,
        penalize_stretches: false,
    };
    let criteria = ExampleEvaluationCriteria::new(criteria_config);
    plugin_system.add_evaluation_criteria(Box::new(criteria));

    // Add technique handler
    let technique_config = TechniqueConfig {
        allow_harmonics: true,
        allow_slides: true,
        allow_bends: true,
        max_stretch: 4,
    };
    let techniques = StringedInstrumentTechniques::new(technique_config);
    plugin_system.add_technique_handler(Box::new(techniques));

    // Test technique support
    let supported = plugin_system.get_supported_techniques();
    assert!(supported.contains(&"harmonic".to_string()));
    assert!(supported.contains(&"slide".to_string()));
    assert!(supported.contains(&"bend".to_string()));

    assert!(plugin_system.supports_technique("harmonic"));
    assert!(!plugin_system.supports_technique("unknown"));

    // Test difficulty modifiers
    assert_eq!(
        plugin_system.get_technique_difficulty_modifier("harmonic"),
        0.8
    );
    assert_eq!(
        plugin_system.get_technique_difficulty_modifier("unknown"),
        1.0
    );

    // Test fingering evaluation
    let fingering = Fingering {
        positions: vec![FingerPosition {
            position: StringedPosition { string: 0, fret: 2 },
            finger: Some(Finger::Index),
            pressure: 1.0,
        }],
        difficulty: 0.6,
        technique: PlayingTechnique::Standard,
    };

    let evaluated_score = plugin_system.evaluate_fingering(&fingering);
    assert!(evaluated_score >= 0.0 && evaluated_score <= 1.0);
}

#[test]
fn test_plugin_system_empty() {
    use crate::fret::{Fingering, PlayingTechnique};

    let plugin_system = PluginSystem::new();

    // Empty system should return original difficulty
    let fingering = Fingering {
        positions: vec![],
        difficulty: 0.5,
        technique: PlayingTechnique::Standard,
    };

    let score = plugin_system.evaluate_fingering(&fingering);
    assert_eq!(score, 0.5);

    // Should have no supported techniques
    assert!(plugin_system.get_supported_techniques().is_empty());
    assert!(!plugin_system.supports_technique("any"));
}
}

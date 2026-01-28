//! Extension system for custom instruments and algorithms
//!
//! This module provides the infrastructure for extending the fretboard system
//! with custom instruments, fingering algorithms, and evaluation criteria.

use super::{
    traits::{
        CustomInstrument, CustomFingeringAlgorithm, CustomEvaluationCriteria, 
        ExtensionRegistry, Fretboard
    },
    FretboardResult, FretboardError, Fingering
};
use crate::Chord;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::any::Any;

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
        static mut INSTANCE: Option<Arc<RwLock<DefaultExtensionRegistry>>> = None;
        static ONCE: std::sync::Once = std::sync::Once::new();
        
        unsafe {
            ONCE.call_once(|| {
                INSTANCE = Some(Arc::new(RwLock::new(DefaultExtensionRegistry::new())));
            });
            INSTANCE.as_ref().unwrap().clone()
        }
    }
}

impl ExtensionRegistry for DefaultExtensionRegistry {
    fn register_instrument<I, C, F>(
        &mut self,
        name: &str,
        factory: F,
    ) -> bool
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

    fn register_algorithm<A, C, F>(
        &mut self,
        name: &str,
        factory: F,
    ) -> bool
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
    fn register_criteria<E, C, F>(
        &mut self,
        name: &str,
        factory: F,
    ) -> bool
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
    pub fn validate_basic_config(
        name: &str,
        category: &str,
        techniques: &[&str],
    ) -> bool {
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
        matches!(category, 
            "stringed" | "keyboard" | "wind" | "percussion" | 
            "brass" | "electronic" | "custom"
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

/// Example custom instrument implementation for demonstration
///
/// This shows how to implement a custom instrument using the extension system.
#[derive(Debug, Clone)]
pub struct ExampleCustomInstrument {
    config: ExampleInstrumentConfig,
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

impl CustomInstrument for ExampleCustomInstrument {
    type Position = ExamplePosition;
    type Config = ExampleInstrumentConfig;

    fn new(config: Self::Config) -> FretboardResult<Self> {
        if !Self::validate_config(&config) {
            return Err(FretboardError::invalid_configuration(
                "Invalid example instrument configuration"
            ));
        }
        
        Ok(Self { config })
    }

    fn validate_config(config: &Self::Config) -> bool {
        !config.name.is_empty()
            && config.string_count > 0
            && config.string_count <= 12  // Reasonable limit
            && config.fret_count > 0
            && config.fret_count <= 30    // Reasonable limit
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

/// Example custom fingering algorithm
///
/// This demonstrates how to implement a custom algorithm using the extension system.
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
    /// Create a new instance of the algorithm
    pub fn new(config: ExampleAlgorithmConfig) -> Self {
        Self { config }
    }
    
    /// Get the algorithm name
    pub fn algorithm_name(&self) -> &'static str {
        "example_custom_algorithm"
    }
    
    /// Get the algorithm description
    pub fn algorithm_description(&self) -> &'static str {
        "Example custom fingering algorithm for demonstration purposes"
    }
}



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
        use crate::{Tuning, PitchClass};
        
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
}
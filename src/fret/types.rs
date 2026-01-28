//! Core type definitions for the fretboard system

use crate::Tuning;
use std::fmt::{Debug, Display};

#[cfg(feature = "bindgen")]
use uniffi;

use serde::{Deserialize, Serialize};

/// Position on a stringed instrument (guitar, bass, etc.)
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct StringedPosition {
    /// String index (0-based, where 0 is typically the lowest/thickest string)
    pub string: usize,
    /// Fret number (0 = open string, 1 = first fret, etc.)
    pub fret: usize,
}

impl StringedPosition {
    /// Create a new stringed position
    pub fn new(string: usize, fret: usize) -> Self {
        Self { string, fret }
    }

    /// Check if this is an open string position
    pub fn is_open(&self) -> bool {
        self.fret == 0
    }
}

impl Display for StringedPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_open() {
            write!(f, "String {} Open", self.string)
        } else {
            write!(f, "String {} Fret {}", self.string, self.fret)
        }
    }
}

/// Position on a keyboard instrument (piano, organ, etc.)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct KeyboardPosition {
    /// Key index (0-based, typically starting from the lowest key)
    pub key: usize,
}

impl KeyboardPosition {
    /// Create a new keyboard position
    pub fn new(key: usize) -> Self {
        Self { key }
    }
}

impl Display for KeyboardPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key {}", self.key)
    }
}

/// Position on a continuous fretboard (violin family, etc.)
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct ContinuousPosition {
    /// String index (0-based)
    pub string: usize,
    /// Position along the string (0.0 = nut, 1.0 = bridge)
    pub position: f32,
}

impl ContinuousPosition {
    /// Create a new continuous position
    pub fn new(string: usize, position: f32) -> Self {
        Self { string, position }
    }

    /// Check if this is an open string position
    pub fn is_open(&self) -> bool {
        self.position == 0.0
    }
}

impl Display for ContinuousPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_open() {
            write!(f, "String {} Open", self.string)
        } else {
            write!(f, "String {} Position {:.3}", self.string, self.position)
        }
    }
}

/// Finger designation for fingering patterns
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
pub enum Finger {
    /// Thumb (T)
    Thumb,
    /// Index finger (1)
    Index,
    /// Middle finger (2)
    Middle,
    /// Ring finger (3)
    Ring,
    /// Pinky finger (4)
    Pinky,
}

impl Display for Finger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Finger::Thumb => write!(f, "T"),
            Finger::Index => write!(f, "1"),
            Finger::Middle => write!(f, "2"),
            Finger::Ring => write!(f, "3"),
            Finger::Pinky => write!(f, "4"),
        }
    }
}

/// Playing technique for a fingering
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
pub enum PlayingTechnique {
    /// Standard fingering
    Standard,
    /// Barre chord technique
    Barre {
        /// Starting string index
        start_string: usize,
        /// Ending string index
        end_string: usize,
        /// Fret number for the barre
        fret: usize,
    },
    /// Hammer-on technique
    Hammer,
    /// Pull-off technique
    Pull,
    /// Slide technique
    Slide,
    /// Harmonic technique
    Harmonic,
}

impl Display for PlayingTechnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayingTechnique::Standard => write!(f, "Standard"),
            PlayingTechnique::Barre {
                start_string,
                end_string,
                fret,
            } => {
                write!(
                    f,
                    "Barre (strings {}-{}, fret {})",
                    start_string, end_string, fret
                )
            }
            PlayingTechnique::Hammer => write!(f, "Hammer-on"),
            PlayingTechnique::Pull => write!(f, "Pull-off"),
            PlayingTechnique::Slide => write!(f, "Slide"),
            PlayingTechnique::Harmonic => write!(f, "Harmonic"),
        }
    }
}

/// A finger position within a fingering pattern
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct FingerPosition<P> {
    /// The position on the fretboard
    pub position: P,
    /// Which finger is used (None for open strings or unpressed positions)
    pub finger: Option<Finger>,
    /// Pressure applied (0.0 = no pressure, 1.0 = full pressure)
    pub pressure: f32,
}

impl<P> FingerPosition<P> {
    /// Create a new finger position
    pub fn new(position: P, finger: Option<Finger>, pressure: f32) -> Self {
        Self {
            position,
            finger,
            pressure,
        }
    }

    /// Create an open string position (no finger, no pressure)
    pub fn open(position: P) -> Self {
        Self {
            position,
            finger: None,
            pressure: 0.0,
        }
    }

    /// Create a pressed position with full pressure
    pub fn pressed(position: P, finger: Finger) -> Self {
        Self {
            position,
            finger: Some(finger),
            pressure: 1.0,
        }
    }
}

impl<P: Display> Display for FingerPosition<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.finger {
            Some(finger) => write!(f, "{} ({})", self.position, finger),
            None => write!(f, "{} (open)", self.position),
        }
    }
}

/// Complete fingering pattern for a chord or musical phrase
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct Fingering<P> {
    /// All finger positions in this fingering
    pub positions: Vec<FingerPosition<P>>,
    /// Playing technique used
    pub technique: PlayingTechnique,
    /// Difficulty rating (0.0 = easiest, 1.0 = hardest)
    pub difficulty: f32,
}

impl<P> Fingering<P> {
    /// Create a new fingering
    pub fn new(
        positions: Vec<FingerPosition<P>>,
        technique: PlayingTechnique,
        difficulty: f32,
    ) -> Self {
        Self {
            positions,
            technique,
            difficulty,
        }
    }

    /// Create a standard fingering with calculated difficulty
    pub fn standard(positions: Vec<FingerPosition<P>>) -> Self {
        // Basic difficulty calculation based on number of positions
        let difficulty = (positions.len() as f32 / 6.0).min(1.0);

        Self {
            positions,
            technique: PlayingTechnique::Standard,
            difficulty,
        }
    }

    /// Get the number of fingers used in this fingering
    pub fn finger_count(&self) -> usize {
        self.positions
            .iter()
            .filter(|pos| pos.finger.is_some())
            .count()
    }

    /// Check if this fingering uses a specific finger
    pub fn uses_finger(&self, finger: Finger) -> bool {
        self.positions.iter().any(|pos| pos.finger == Some(finger))
    }

    /// Get all fingers used in this fingering
    pub fn get_fingers_used(&self) -> Vec<Finger> {
        let mut fingers: Vec<Finger> = self.positions.iter().filter_map(|pos| pos.finger).collect();
        fingers.sort_by_key(|f| match f {
            Finger::Thumb => 0,
            Finger::Index => 1,
            Finger::Middle => 2,
            Finger::Ring => 3,
            Finger::Pinky => 4,
        });
        fingers.dedup();
        fingers
    }
}

impl<P: Display> Display for Fingering<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Fingering ({}, difficulty: {:.2}):",
            self.technique, self.difficulty
        )?;
        for (i, pos) in self.positions.iter().enumerate() {
            writeln!(f, "  {}: {}", i, pos)?;
        }
        Ok(())
    }
}

/// Skill level for fingering generation and optimization
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
pub enum SkillLevel {
    /// Beginner level (prefer simple, open positions)
    Beginner,
    /// Intermediate level (moderate complexity allowed)
    Intermediate,
    /// Advanced level (complex fingerings and techniques allowed)
    Advanced,
    /// Expert level (all techniques and positions available)
    Expert,
}

impl Display for SkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillLevel::Beginner => write!(f, "Beginner"),
            SkillLevel::Intermediate => write!(f, "Intermediate"),
            SkillLevel::Advanced => write!(f, "Advanced"),
            SkillLevel::Expert => write!(f, "Expert"),
        }
    }
}

/// Configuration for stringed instruments
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct StringedInstrumentConfig {
    /// Tuning for each string (from lowest to highest)
    pub strings: Vec<Tuning>,
    /// Number of frets available
    pub fret_count: usize,
    /// Scale length in millimeters
    pub scale_length: f32,
    /// Nut width in millimeters
    pub nut_width: f32,
    /// String spacing in millimeters
    pub string_spacing: f32,
}

impl StringedInstrumentConfig {
    /// Create a new stringed instrument configuration
    pub fn new(
        strings: Vec<Tuning>,
        fret_count: usize,
        scale_length: f32,
        nut_width: f32,
        string_spacing: f32,
    ) -> Self {
        Self {
            strings,
            fret_count,
            scale_length,
            nut_width,
            string_spacing,
        }
    }

    /// Get the number of strings
    pub fn string_count(&self) -> usize {
        self.strings.len()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.strings.is_empty() {
            return Err("Must have at least one string".to_string());
        }

        // Note: fret_count can be 0 for continuous instruments (violin family)
        // No validation needed for fret_count

        if self.scale_length <= 0.0 {
            return Err("Scale length must be positive".to_string());
        }

        if self.nut_width <= 0.0 {
            return Err("Nut width must be positive".to_string());
        }

        if self.string_spacing <= 0.0 {
            return Err("String spacing must be positive".to_string());
        }

        Ok(())
    }
}

/// Keyboard layout type
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
pub enum KeyLayout {
    /// Standard piano layout (white and black keys)
    Piano,
    /// Chromatic layout (all keys same size)
    Chromatic,
    /// Custom layout
    Custom,
}

/// Configuration for keyboard instruments
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct KeyboardConfig {
    /// Lowest key tuning
    pub lowest_key: Tuning,
    /// Total number of keys
    pub key_count: usize,
    /// Keyboard layout type
    pub key_layout: KeyLayout,
}

impl KeyboardConfig {
    /// Create a new keyboard configuration
    pub fn new(lowest_key: Tuning, key_count: usize, key_layout: KeyLayout) -> Self {
        Self {
            lowest_key,
            key_count,
            key_layout,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.key_count == 0 {
            return Err("Must have at least one key".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn test_stringed_position_creation() {
        let pos = StringedPosition::new(0, 3);
        assert_eq!(pos.string, 0);
        assert_eq!(pos.fret, 3);
        assert!(!pos.is_open());

        let open_pos = StringedPosition::new(1, 0);
        assert!(open_pos.is_open());
    }

    #[test]
    fn test_keyboard_position_creation() {
        let pos = KeyboardPosition::new(42);
        assert_eq!(pos.key, 42);
    }

    #[test]
    fn test_continuous_position_creation() {
        let pos = ContinuousPosition::new(2, 0.5);
        assert_eq!(pos.string, 2);
        assert_eq!(pos.position, 0.5);
        assert!(!pos.is_open());

        let open_pos = ContinuousPosition::new(0, 0.0);
        assert!(open_pos.is_open());
    }

    #[test]
    fn test_finger_display() {
        assert_eq!(Finger::Thumb.to_string(), "T");
        assert_eq!(Finger::Index.to_string(), "1");
        assert_eq!(Finger::Middle.to_string(), "2");
        assert_eq!(Finger::Ring.to_string(), "3");
        assert_eq!(Finger::Pinky.to_string(), "4");
    }

    #[test]
    fn test_finger_position_creation() {
        let pos = StringedPosition::new(0, 3);
        let finger_pos = FingerPosition::pressed(pos.clone(), Finger::Index);

        assert_eq!(finger_pos.position, pos);
        assert_eq!(finger_pos.finger, Some(Finger::Index));
        assert_eq!(finger_pos.pressure, 1.0);

        let open_pos = FingerPosition::open(pos);
        assert_eq!(open_pos.finger, None);
        assert_eq!(open_pos.pressure, 0.0);
    }

    #[test]
    fn test_fingering_creation() {
        let positions = vec![
            FingerPosition::pressed(StringedPosition::new(0, 3), Finger::Index),
            FingerPosition::pressed(StringedPosition::new(1, 2), Finger::Middle),
            FingerPosition::open(StringedPosition::new(2, 0)),
        ];

        let fingering = Fingering::standard(positions.clone());

        assert_eq!(fingering.positions.len(), 3);
        assert_eq!(fingering.technique, PlayingTechnique::Standard);
        assert_eq!(fingering.finger_count(), 2);
        assert!(fingering.uses_finger(Finger::Index));
        assert!(fingering.uses_finger(Finger::Middle));
        assert!(!fingering.uses_finger(Finger::Ring));

        let fingers_used = fingering.get_fingers_used();
        assert_eq!(fingers_used, vec![Finger::Index, Finger::Middle]);
    }

    #[test]
    fn test_stringed_instrument_config() {
        let tunings = vec![
            Tuning::from_str("E2").unwrap(),
            Tuning::from_str("A2").unwrap(),
            Tuning::from_str("D3").unwrap(),
            Tuning::from_str("G3").unwrap(),
            Tuning::from_str("B3").unwrap(),
            Tuning::from_str("E4").unwrap(),
        ];

        let config = StringedInstrumentConfig::new(tunings, 24, 648.0, 43.0, 10.5);

        assert_eq!(config.string_count(), 6);
        assert_eq!(config.fret_count, 24);
        assert!(config.validate().is_ok());

        // Test invalid config
        let invalid_config = StringedInstrumentConfig::new(vec![], 0, -1.0, 0.0, 0.0);
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_keyboard_config() {
        let config = KeyboardConfig::new(Tuning::from_str("A0").unwrap(), 88, KeyLayout::Piano);

        assert_eq!(config.key_count, 88);
        assert_eq!(config.key_layout, KeyLayout::Piano);
        assert!(config.validate().is_ok());

        // Test invalid config
        let invalid_config =
            KeyboardConfig::new(Tuning::from_str("C4").unwrap(), 0, KeyLayout::Piano);
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_barre_technique_support() {
        // Test barre technique creation and display (Requirements 4.5, 6.4)
        let barre_technique = PlayingTechnique::Barre {
            start_string: 0,
            end_string: 5,
            fret: 3,
        };

        // Verify barre technique displays correctly
        let display_str = barre_technique.to_string();
        assert!(display_str.contains("Barre"));
        assert!(display_str.contains("strings 0-5"));
        assert!(display_str.contains("fret 3"));

        // Test barre fingering creation
        let positions = vec![
            FingerPosition::pressed(StringedPosition::new(0, 3), Finger::Index),
            FingerPosition::pressed(StringedPosition::new(1, 3), Finger::Index),
            FingerPosition::pressed(StringedPosition::new(2, 3), Finger::Index),
            FingerPosition::pressed(StringedPosition::new(3, 5), Finger::Ring),
            FingerPosition::pressed(StringedPosition::new(4, 5), Finger::Pinky),
            FingerPosition::pressed(StringedPosition::new(5, 3), Finger::Index),
        ];

        let barre_fingering = Fingering::new(positions, barre_technique, 0.7);

        assert_eq!(
            barre_fingering.technique,
            PlayingTechnique::Barre {
                start_string: 0,
                end_string: 5,
                fret: 3,
            }
        );
        assert_eq!(barre_fingering.finger_count(), 6);
        assert!(barre_fingering.uses_finger(Finger::Index));
        assert!(barre_fingering.uses_finger(Finger::Ring));
        assert!(barre_fingering.uses_finger(Finger::Pinky));
    }

    #[test]
    fn test_special_techniques_support() {
        // Test all special techniques (Requirements 6.4)
        let techniques = vec![
            PlayingTechnique::Standard,
            PlayingTechnique::Barre {
                start_string: 0,
                end_string: 2,
                fret: 5,
            },
            PlayingTechnique::Hammer,
            PlayingTechnique::Pull,
            PlayingTechnique::Slide,
            PlayingTechnique::Harmonic,
        ];

        for technique in techniques {
            // Verify each technique has a meaningful display representation
            let display_str = technique.to_string();
            assert!(!display_str.is_empty());

            // Create a fingering with this technique
            let positions = vec![FingerPosition::pressed(
                StringedPosition::new(0, 2),
                Finger::Index,
            )];
            let fingering = Fingering::new(positions, technique.clone(), 0.5);
            assert_eq!(fingering.technique, technique);
        }
    }

    #[test]
    fn test_finger_pressure_modeling() {
        // Test pressure modeling in finger positions (Requirements 4.5)
        let position = StringedPosition::new(1, 3);

        // Test different pressure levels
        let light_pressure = FingerPosition::new(position.clone(), Some(Finger::Index), 0.3);
        let medium_pressure = FingerPosition::new(position.clone(), Some(Finger::Index), 0.6);
        let full_pressure = FingerPosition::pressed(position.clone(), Finger::Index);
        let open_string = FingerPosition::open(position);

        assert_eq!(light_pressure.pressure, 0.3);
        assert_eq!(medium_pressure.pressure, 0.6);
        assert_eq!(full_pressure.pressure, 1.0);
        assert_eq!(open_string.pressure, 0.0);
        assert_eq!(open_string.finger, None);
    }
}

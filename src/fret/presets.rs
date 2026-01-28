//! Instrument preset configurations for common stringed instruments
//!
//! This module provides pre-configured instrument setups for guitars, basses,
//! and other common stringed instruments. Each preset includes standard tunings,
//! fret counts, and physical dimensions based on typical instrument specifications.

use super::types::{KeyLayout, KeyboardConfig, StringedInstrumentConfig};
use crate::{PitchClass, Tuning};

#[cfg(feature = "bindgen")]
use uniffi;

/// Collection of preset configurations for common stringed instruments
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bindgen", derive(uniffi::Record))]
pub struct InstrumentPresets;

impl InstrumentPresets {
    // Guitar Presets

    /// Standard 6-string guitar in standard tuning (E-A-D-G-B-E)
    ///
    /// # Configuration
    /// - Tuning: E2-A2-D3-G3-B3-E4 (standard guitar tuning)
    /// - Frets: 24 (typical electric guitar)
    /// - Scale Length: 648mm (25.5" - Fender scale)
    /// - Nut Width: 43mm (standard electric guitar)
    /// - String Spacing: 10.5mm (standard electric guitar)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::guitar_standard();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 6);
    /// ```
    pub fn guitar_standard() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::E, 2), // Low E (6th string)
            Tuning::new(PitchClass::A, 2), // A (5th string)
            Tuning::new(PitchClass::D, 3), // D (4th string)
            Tuning::new(PitchClass::G, 3), // G (3rd string)
            Tuning::new(PitchClass::B, 3), // B (2nd string)
            Tuning::new(PitchClass::E, 4), // High E (1st string)
        ];

        StringedInstrumentConfig::new(
            tunings, 24,    // frets
            648.0, // scale length (mm) - Fender 25.5"
            43.0,  // nut width (mm)
            10.5,  // string spacing (mm)
        )
    }

    /// 7-string guitar in standard tuning (B-E-A-D-G-B-E)
    ///
    /// # Configuration
    /// - Tuning: B1-E2-A2-D3-G3-B3-E4 (standard 7-string tuning)
    /// - Frets: 24 (typical extended range guitar)
    /// - Scale Length: 673mm (26.5" - extended scale for lower tuning)
    /// - Nut Width: 48mm (wider to accommodate extra string)
    /// - String Spacing: 10.0mm (slightly tighter spacing)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::guitar_7_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 7);
    /// ```
    pub fn guitar_7_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::B, 1), // Low B (7th string)
            Tuning::new(PitchClass::E, 2), // Low E (6th string)
            Tuning::new(PitchClass::A, 2), // A (5th string)
            Tuning::new(PitchClass::D, 3), // D (4th string)
            Tuning::new(PitchClass::G, 3), // G (3rd string)
            Tuning::new(PitchClass::B, 3), // B (2nd string)
            Tuning::new(PitchClass::E, 4), // High E (1st string)
        ];

        StringedInstrumentConfig::new(
            tunings, 24,    // frets
            673.0, // scale length (mm) - 26.5" extended scale
            48.0,  // nut width (mm) - wider for 7 strings
            10.0,  // string spacing (mm)
        )
    }

    /// 12-string guitar in standard tuning (doubled strings)
    ///
    /// # Configuration
    /// - Tuning: E2-E3-A2-A3-D3-D4-G3-G4-B3-B3-E4-E4 (octave pairs for lower 4 strings, unison pairs for upper 2)
    /// - Frets: 22 (typical acoustic 12-string)
    /// - Scale Length: 648mm (25.5" standard scale)
    /// - Nut Width: 54mm (much wider to accommodate 12 strings)
    /// - String Spacing: 4.5mm (tight spacing for doubled strings)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::guitar_12_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 12);
    /// ```
    pub fn guitar_12_string() -> StringedInstrumentConfig {
        let tunings = vec![
            // Course 6 (lowest) - E octave pair
            Tuning::new(PitchClass::E, 2), // Low E fundamental
            Tuning::new(PitchClass::E, 3), // Low E octave
            // Course 5 - A octave pair
            Tuning::new(PitchClass::A, 2), // A fundamental
            Tuning::new(PitchClass::A, 3), // A octave
            // Course 4 - D octave pair
            Tuning::new(PitchClass::D, 3), // D fundamental
            Tuning::new(PitchClass::D, 4), // D octave
            // Course 3 - G octave pair
            Tuning::new(PitchClass::G, 3), // G fundamental
            Tuning::new(PitchClass::G, 4), // G octave
            // Course 2 - B unison pair
            Tuning::new(PitchClass::B, 3), // B fundamental
            Tuning::new(PitchClass::B, 3), // B unison
            // Course 1 (highest) - E unison pair
            Tuning::new(PitchClass::E, 4), // High E fundamental
            Tuning::new(PitchClass::E, 4), // High E unison
        ];

        StringedInstrumentConfig::new(
            tunings, 22,    // frets (typical for acoustic 12-string)
            648.0, // scale length (mm) - standard 25.5"
            54.0,  // nut width (mm) - wide for 12 strings
            4.5,   // string spacing (mm) - tight for doubled strings
        )
    }

    // Bass Presets

    /// Standard 4-string bass in standard tuning (E-A-D-G)
    ///
    /// # Configuration
    /// - Tuning: E1-A1-D2-G2 (standard bass tuning, one octave below guitar)
    /// - Frets: 24 (typical modern bass)
    /// - Scale Length: 864mm (34" - standard bass scale)
    /// - Nut Width: 38mm (standard 4-string bass)
    /// - String Spacing: 12mm (standard bass string spacing)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::bass_4_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn bass_4_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::E, 1), // Low E (4th string)
            Tuning::new(PitchClass::A, 1), // A (3rd string)
            Tuning::new(PitchClass::D, 2), // D (2nd string)
            Tuning::new(PitchClass::G, 2), // G (1st string)
        ];

        StringedInstrumentConfig::new(
            tunings, 24,    // frets
            864.0, // scale length (mm) - 34" standard bass scale
            38.0,  // nut width (mm)
            12.0,  // string spacing (mm) - adjusted for realistic bass dimensions
        )
    }

    /// 5-string bass with low B string (B-E-A-D-G)
    ///
    /// # Configuration
    /// - Tuning: B0-E1-A1-D2-G2 (standard 5-string bass tuning)
    /// - Frets: 24 (typical modern bass)
    /// - Scale Length: 889mm (35" - extended scale for low B)
    /// - Nut Width: 45mm (wider for 5 strings)
    /// - String Spacing: 18mm (slightly tighter spacing)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::bass_5_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 5);
    /// ```
    pub fn bass_5_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::B, 0), // Low B (5th string)
            Tuning::new(PitchClass::E, 1), // Low E (4th string)
            Tuning::new(PitchClass::A, 1), // A (3rd string)
            Tuning::new(PitchClass::D, 2), // D (2nd string)
            Tuning::new(PitchClass::G, 2), // G (1st string)
        ];

        StringedInstrumentConfig::new(
            tunings, 24,    // frets
            889.0, // scale length (mm) - 35" extended scale
            45.0,  // nut width (mm)
            11.0,  // string spacing (mm) - adjusted for 5-string bass
        )
    }

    /// 6-string bass with high C string (B-E-A-D-G-C)
    ///
    /// # Configuration
    /// - Tuning: B0-E1-A1-D2-G2-C3 (standard 6-string bass tuning)
    /// - Frets: 24 (typical modern bass)
    /// - Scale Length: 889mm (35" - extended scale)
    /// - Nut Width: 54mm (wide for 6 strings)
    /// - String Spacing: 16.5mm (tighter spacing for 6 strings)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::bass_6_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 6);
    /// ```
    pub fn bass_6_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::B, 0), // Low B (6th string)
            Tuning::new(PitchClass::E, 1), // Low E (5th string)
            Tuning::new(PitchClass::A, 1), // A (4th string)
            Tuning::new(PitchClass::D, 2), // D (3rd string)
            Tuning::new(PitchClass::G, 2), // G (2nd string)
            Tuning::new(PitchClass::C, 3), // High C (1st string)
        ];

        StringedInstrumentConfig::new(
            tunings, 24,    // frets
            889.0, // scale length (mm) - 35" extended scale
            54.0,  // nut width (mm)
            10.0,  // string spacing (mm) - adjusted for 6-string bass
        )
    }

    // Additional Stringed Instruments

    /// Standard ukulele in C tuning (G-C-E-A)
    ///
    /// # Configuration
    /// - Tuning: G4-C4-E4-A4 (standard ukulele tuning - re-entrant)
    /// - Frets: 15 (typical soprano ukulele)
    /// - Scale Length: 346mm (13.6" - soprano scale)
    /// - Nut Width: 35mm (standard ukulele)
    /// - String Spacing: 11mm (standard ukulele spacing)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::ukulele_soprano();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn ukulele_soprano() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 4), // G (4th string - re-entrant, higher than C)
            Tuning::new(PitchClass::C, 4), // C (3rd string)
            Tuning::new(PitchClass::E, 4), // E (2nd string)
            Tuning::new(PitchClass::A, 4), // A (1st string)
        ];

        StringedInstrumentConfig::new(
            tunings, 15,    // frets
            346.0, // scale length (mm) - 13.6" soprano scale
            35.0,  // nut width (mm)
            11.0,  // string spacing (mm)
        )
    }

    /// Standard mandolin in violin tuning (G-D-A-E)
    ///
    /// # Configuration
    /// - Tuning: G3-D4-A4-E5 (standard mandolin tuning, same as violin but octave higher)
    /// - Frets: 20 (typical mandolin)
    /// - Scale Length: 330mm (13" - standard mandolin scale)
    /// - Nut Width: 30mm (narrow mandolin neck)
    /// - String Spacing: 3.5mm (very tight spacing for doubled strings)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::mandolin_standard();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 8);
    /// ```
    pub fn mandolin_standard() -> StringedInstrumentConfig {
        let tunings = vec![
            // Course 4 (lowest) - G unison pair
            Tuning::new(PitchClass::G, 3), // G fundamental
            Tuning::new(PitchClass::G, 3), // G unison
            // Course 3 - D unison pair
            Tuning::new(PitchClass::D, 4), // D fundamental
            Tuning::new(PitchClass::D, 4), // D unison
            // Course 2 - A unison pair
            Tuning::new(PitchClass::A, 4), // A fundamental
            Tuning::new(PitchClass::A, 4), // A unison
            // Course 1 (highest) - E unison pair
            Tuning::new(PitchClass::E, 5), // E fundamental
            Tuning::new(PitchClass::E, 5), // E unison
        ];

        StringedInstrumentConfig::new(
            tunings, 20,    // frets
            330.0, // scale length (mm) - 13" mandolin scale
            30.0,  // nut width (mm)
            3.5,   // string spacing (mm) - very tight for doubled strings
        )
    }

    /// 5-string banjo in open G tuning (D-G-D-G-B)
    ///
    /// # Configuration
    /// - Tuning: D4-G2-D3-G3-B3 (open G banjo tuning with short 5th string)
    /// - Frets: 22 (typical banjo)
    /// - Scale Length: 673mm (26.5" - long scale for tension)
    /// - Nut Width: 32mm (narrow banjo neck)
    /// - String Spacing: 8mm (tight spacing)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::banjo_5_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 5);
    /// ```
    pub fn banjo_5_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::D, 4), // 5th string (short drone string)
            Tuning::new(PitchClass::G, 2), // 4th string (lowest full-length)
            Tuning::new(PitchClass::D, 3), // 3rd string
            Tuning::new(PitchClass::G, 3), // 2nd string
            Tuning::new(PitchClass::B, 3), // 1st string (highest)
        ];

        StringedInstrumentConfig::new(
            tunings, 22,    // frets
            673.0, // scale length (mm) - 26.5"
            32.0,  // nut width (mm)
            8.0,   // string spacing (mm)
        )
    }

    // Keyboard Presets

    /// Standard 88-key piano (A0 to C8)
    ///
    /// # Configuration
    /// - Range: A0 to C8 (88 keys total)
    /// - Layout: Piano (white and black keys)
    /// - Lowest Key: A0 (MIDI note 21)
    /// - Highest Key: C8 (MIDI note 108)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, KeyboardFretboard};
    ///
    /// let config = InstrumentPresets::piano_88_key();
    /// let fretboard = KeyboardFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.key_count(), 88);
    /// ```
    pub fn piano_88_key() -> KeyboardConfig {
        KeyboardConfig::new(
            Tuning::new(PitchClass::A, 0), // A0 - standard piano lowest key
            88,                            // 88 keys total
            KeyLayout::Piano,
        )
    }

    /// 76-key keyboard (E1 to G7)
    ///
    /// # Configuration
    /// - Range: E1 to G7 (76 keys total)
    /// - Layout: Piano (white and black keys)
    /// - Common in stage pianos and workstations
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, KeyboardFretboard};
    ///
    /// let config = InstrumentPresets::keyboard_76_key();
    /// let fretboard = KeyboardFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.key_count(), 76);
    /// ```
    pub fn keyboard_76_key() -> KeyboardConfig {
        KeyboardConfig::new(
            Tuning::new(PitchClass::E, 1), // E1 - common 76-key starting point
            76,                            // 76 keys total
            KeyLayout::Piano,
        )
    }

    /// 61-key keyboard (C2 to C7)
    ///
    /// # Configuration
    /// - Range: C2 to C7 (61 keys total)
    /// - Layout: Piano (white and black keys)
    /// - Common in home keyboards and synthesizers
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, KeyboardFretboard};
    ///
    /// let config = InstrumentPresets::keyboard_61_key();
    /// let fretboard = KeyboardFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.key_count(), 61);
    /// ```
    pub fn keyboard_61_key() -> KeyboardConfig {
        KeyboardConfig::new(
            Tuning::new(PitchClass::C, 2), // C2 - common 61-key starting point
            61,                            // 61 keys total
            KeyLayout::Piano,
        )
    }

    /// 49-key keyboard (C3 to C7)
    ///
    /// # Configuration
    /// - Range: C3 to C7 (49 keys total)
    /// - Layout: Piano (white and black keys)
    /// - Common in compact MIDI controllers
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, KeyboardFretboard};
    ///
    /// let config = InstrumentPresets::keyboard_49_key();
    /// let fretboard = KeyboardFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.key_count(), 49);
    /// ```
    pub fn keyboard_49_key() -> KeyboardConfig {
        KeyboardConfig::new(
            Tuning::new(PitchClass::C, 3), // C3 - common 49-key starting point
            49,                            // 49 keys total
            KeyLayout::Piano,
        )
    }

    /// 25-key mini keyboard (C3 to C5)
    ///
    /// # Configuration
    /// - Range: C3 to C5 (25 keys total)
    /// - Layout: Piano (white and black keys)
    /// - Common in portable MIDI controllers
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, KeyboardFretboard};
    ///
    /// let config = InstrumentPresets::keyboard_25_key();
    /// let fretboard = KeyboardFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.key_count(), 25);
    /// ```
    pub fn keyboard_25_key() -> KeyboardConfig {
        KeyboardConfig::new(
            Tuning::new(PitchClass::C, 3), // C3 - common mini keyboard starting point
            25,                            // 25 keys total
            KeyLayout::Piano,
        )
    }

    // Violin Family Presets (Continuous Fretboard Instruments)

    /// Standard violin in standard tuning (G-D-A-E)
    ///
    /// # Configuration
    /// - Tuning: G3-D4-A4-E5 (standard violin tuning)
    /// - Frets: 0 (continuous fretboard - no frets)
    /// - Scale Length: 330mm (13" - standard violin scale)
    /// - Nut Width: 24mm (standard violin nut)
    /// - String Spacing: 7mm (standard violin string spacing)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, ContinuousFretboard};
    ///
    /// let config = InstrumentPresets::violin_standard();
    /// let fretboard = ContinuousFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn violin_standard() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 3), // G string (4th string - lowest)
            Tuning::new(PitchClass::D, 4), // D string (3rd string)
            Tuning::new(PitchClass::A, 4), // A string (2nd string)
            Tuning::new(PitchClass::E, 5), // E string (1st string - highest)
        ];

        StringedInstrumentConfig::new(
            tunings,
            0,     // No frets for continuous instruments
            330.0, // scale length (mm) - 13" violin scale
            24.0,  // nut width (mm)
            7.0,   // string spacing (mm)
        )
    }

    /// Standard viola in standard tuning (C-G-D-A)
    ///
    /// # Configuration
    /// - Tuning: C3-G3-D4-A4 (standard viola tuning, perfect fifth below violin)
    /// - Frets: 0 (continuous fretboard - no frets)
    /// - Scale Length: 370mm (14.5" - standard viola scale)
    /// - Nut Width: 26mm (slightly wider than violin)
    /// - String Spacing: 8mm (slightly wider spacing than violin)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, ContinuousFretboard};
    ///
    /// let config = InstrumentPresets::viola_standard();
    /// let fretboard = ContinuousFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn viola_standard() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::C, 3), // C string (4th string - lowest)
            Tuning::new(PitchClass::G, 3), // G string (3rd string)
            Tuning::new(PitchClass::D, 4), // D string (2nd string)
            Tuning::new(PitchClass::A, 4), // A string (1st string - highest)
        ];

        StringedInstrumentConfig::new(
            tunings,
            0,     // No frets for continuous instruments
            370.0, // scale length (mm) - 14.5" viola scale
            26.0,  // nut width (mm)
            8.0,   // string spacing (mm)
        )
    }

    /// Standard cello in standard tuning (C-G-D-A)
    ///
    /// # Configuration
    /// - Tuning: C2-G2-D3-A3 (standard cello tuning, one octave below viola)
    /// - Frets: 0 (continuous fretboard - no frets)
    /// - Scale Length: 690mm (27.2" - standard cello scale)
    /// - Nut Width: 45mm (much wider for larger instrument)
    /// - String Spacing: 12mm (wider spacing for thicker strings)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, ContinuousFretboard};
    ///
    /// let config = InstrumentPresets::cello_standard();
    /// let fretboard = ContinuousFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn cello_standard() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::C, 2), // C string (4th string - lowest)
            Tuning::new(PitchClass::G, 2), // G string (3rd string)
            Tuning::new(PitchClass::D, 3), // D string (2nd string)
            Tuning::new(PitchClass::A, 3), // A string (1st string - highest)
        ];

        StringedInstrumentConfig::new(
            tunings,
            0,     // No frets for continuous instruments
            690.0, // scale length (mm) - 27.2" cello scale
            45.0,  // nut width (mm)
            12.0,  // string spacing (mm)
        )
    }

    /// Standard double bass in standard tuning (E-A-D-G)
    ///
    /// # Configuration
    /// - Tuning: E1-A1-D2-G2 (standard double bass tuning)
    /// - Frets: 0 (continuous fretboard - no frets)
    /// - Scale Length: 1060mm (41.7" - standard 3/4 double bass scale)
    /// - Nut Width: 55mm (very wide for large instrument)
    /// - String Spacing: 15mm (wide spacing for very thick strings)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, ContinuousFretboard};
    ///
    /// let config = InstrumentPresets::double_bass_standard();
    /// let fretboard = ContinuousFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn double_bass_standard() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::E, 1), // E string (4th string - lowest)
            Tuning::new(PitchClass::A, 1), // A string (3rd string)
            Tuning::new(PitchClass::D, 2), // D string (2nd string)
            Tuning::new(PitchClass::G, 2), // G string (1st string - highest)
        ];

        StringedInstrumentConfig::new(
            tunings,
            0,      // No frets for continuous instruments
            1060.0, // scale length (mm) - 41.7" 3/4 double bass scale
            55.0,   // nut width (mm)
            15.0,   // string spacing (mm)
        )
    }

    /// 5-string double bass with high C string (E-A-D-G-C)
    ///
    /// # Configuration
    /// - Tuning: E1-A1-D2-G2-C3 (5-string double bass with high C)
    /// - Frets: 0 (continuous fretboard - no frets)
    /// - Scale Length: 1060mm (41.7" - standard 3/4 double bass scale)
    /// - Nut Width: 65mm (wider for 5 strings)
    /// - String Spacing: 13mm (slightly tighter for 5 strings)
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, ContinuousFretboard};
    ///
    /// let config = InstrumentPresets::double_bass_5_string();
    /// let fretboard = ContinuousFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 5);
    /// ```
    pub fn double_bass_5_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::E, 1), // E string (5th string - lowest)
            Tuning::new(PitchClass::A, 1), // A string (4th string)
            Tuning::new(PitchClass::D, 2), // D string (3rd string)
            Tuning::new(PitchClass::G, 2), // G string (2nd string)
            Tuning::new(PitchClass::C, 3), // C string (1st string - highest)
        ];

        StringedInstrumentConfig::new(
            tunings,
            0,      // No frets for continuous instruments
            1060.0, // scale length (mm) - 41.7" 3/4 double bass scale
            65.0,   // nut width (mm)
            13.0,   // string spacing (mm)
        )
    }

    /// Violin with scordatura tuning (alternative tuning)
    ///
    /// # Configuration
    /// - Tuning: G3-D4-A4-D5 (scordatura with high D instead of E)
    /// - Frets: 0 (continuous fretboard - no frets)
    /// - Scale Length: 330mm (13" - standard violin scale)
    /// - Nut Width: 24mm (standard violin nut)
    /// - String Spacing: 7mm (standard violin string spacing)
    ///
    /// This is a common scordatura used in some classical and folk music
    /// where the E string is tuned down to D for specific musical effects.
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, ContinuousFretboard};
    ///
    /// let config = InstrumentPresets::violin_scordatura_high_d();
    /// let fretboard = ContinuousFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn violin_scordatura_high_d() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 3), // G string (4th string - lowest)
            Tuning::new(PitchClass::D, 4), // D string (3rd string)
            Tuning::new(PitchClass::A, 4), // A string (2nd string)
            Tuning::new(PitchClass::D, 5), // D string (1st string - scordatura)
        ];

        StringedInstrumentConfig::new(
            tunings,
            0,     // No frets for continuous instruments
            330.0, // scale length (mm) - 13" violin scale
            24.0,  // nut width (mm)
            7.0,   // string spacing (mm)
        )
    }

    /// Create a custom stringed instrument configuration
    ///
    /// This function allows users to define custom stringed instruments with
    /// arbitrary string counts, tunings, fret counts, and physical dimensions.
    ///
    /// # Arguments
    /// * `tunings` - Vector of tunings for each string (from lowest to highest)
    /// * `fret_count` - Number of frets on the instrument
    /// * `scale_length` - Scale length in millimeters
    /// * `nut_width` - Nut width in millimeters
    /// * `string_spacing` - String spacing in millimeters
    ///
    /// # Returns
    /// * `Ok(StringedInstrumentConfig)` if the configuration is valid
    /// * `Err(String)` with detailed error message if invalid
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, Tuning, PitchClass};
    ///
    /// let tunings = vec![
    ///     Tuning::new(PitchClass::E, 2),
    ///     Tuning::new(PitchClass::A, 2),
    ///     Tuning::new(PitchClass::D, 3),
    ///     Tuning::new(PitchClass::G, 3),
    ///     Tuning::new(PitchClass::B, 3),
    ///     Tuning::new(PitchClass::E, 4),
    /// ];
    /// let custom_guitar = InstrumentPresets::create_custom_stringed_instrument(
    ///     tunings,
    ///     24,    // 24 frets
    ///     648.0, // 25.5" scale length
    ///     43.0,  // 43mm nut width
    ///     10.5,  // 10.5mm string spacing
    /// ).unwrap();
    /// assert_eq!(custom_guitar.string_count(), 6);
    /// ```
    pub fn create_custom_stringed_instrument(
        tunings: Vec<Tuning>,
        fret_count: u32,
        scale_length: f32,
        nut_width: f32,
        string_spacing: f32,
    ) -> Result<StringedInstrumentConfig, String> {
        let config = StringedInstrumentConfig::new(
            tunings,
            fret_count,
            scale_length,
            nut_width,
            string_spacing,
        );

        // Validate the custom configuration
        Self::validate_configuration(&config)?;

        Ok(config)
    }

    /// Create a custom keyboard instrument configuration
    ///
    /// This function allows users to define custom keyboard instruments with
    /// arbitrary key counts, starting notes, and layouts.
    ///
    /// # Arguments
    /// * `lowest_key` - The tuning of the lowest key
    /// * `key_count` - Number of keys on the keyboard
    /// * `key_layout` - The layout type (Piano, Organ, etc.)
    ///
    /// # Returns
    /// * `Ok(KeyboardConfig)` if the configuration is valid
    /// * `Err(String)` with detailed error message if invalid
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, Tuning, KeyLayout, PitchClass};
    ///
    /// let custom_keyboard = InstrumentPresets::create_custom_keyboard(
    ///     Tuning::new(PitchClass::C, 3),
    ///     73, // 73-key keyboard
    ///     KeyLayout::Piano,
    /// ).unwrap();
    /// assert_eq!(custom_keyboard.key_count, 73);
    /// ```
    pub fn create_custom_keyboard(
        lowest_key: Tuning,
        key_count: u32,
        key_layout: KeyLayout,
    ) -> Result<KeyboardConfig, String> {
        let config = KeyboardConfig::new(lowest_key, key_count, key_layout);

        // Validate the custom configuration
        Self::validate_keyboard_configuration(&config)?;

        Ok(config)
    }

    /// Get all available preset names
    ///
    /// # Returns
    /// Vector of all available preset configuration names
    ///
    /// # Example
    /// ```
    /// use mutheors::InstrumentPresets;
    ///
    /// let presets = InstrumentPresets::list_presets();
    /// assert!(presets.contains(&"guitar_standard".to_string()));
    /// ```
    pub fn mandolin_standard() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 3), // G3 (course 1, string 1)
            Tuning::new(PitchClass::G, 3), // G3 (course 1, string 2)
            Tuning::new(PitchClass::D, 4), // D4 (course 2, string 1)
            Tuning::new(PitchClass::D, 4), // D4 (course 2, string 2)
            Tuning::new(PitchClass::A, 4), // A4 (course 3, string 1)
            Tuning::new(PitchClass::A, 4), // A4 (course 3, string 2)
            Tuning::new(PitchClass::E, 5), // E5 (course 4, string 1)
            Tuning::new(PitchClass::E, 5), // E5 (course 4, string 2)
        ];

        StringedInstrumentConfig::new(
            tunings,
            24,    // Fret count
            330.0, // Scale length in mm (similar to violin)
            28.0,  // Nut width in mm (wider than violin for 8 strings)
            3.5,   // String spacing in mm
        )
    }

    /// Create a mandolin with octave courses (traditional Italian style)
    ///
    /// Octave mandolin tuning: G2-G3, D3-D4, A3-A4, E4-E5
    /// The lower courses have octave pairs instead of unison pairs.
    ///
    /// # Returns
    /// A StringedInstrumentConfig for octave mandolin
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::mandolin_octave();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 8);
    /// ```
    pub fn mandolin_octave() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 2), // G2 (course 1, low string)
            Tuning::new(PitchClass::G, 3), // G3 (course 1, high string)
            Tuning::new(PitchClass::D, 3), // D3 (course 2, low string)
            Tuning::new(PitchClass::D, 4), // D4 (course 2, high string)
            Tuning::new(PitchClass::A, 3), // A3 (course 3, low string)
            Tuning::new(PitchClass::A, 4), // A4 (course 3, high string)
            Tuning::new(PitchClass::E, 4), // E4 (course 4, low string)
            Tuning::new(PitchClass::E, 5), // E5 (course 4, high string)
        ];

        StringedInstrumentConfig::new(
            tunings,
            24,    // Fret count
            330.0, // Scale length in mm
            28.0,  // Nut width in mm
            3.5,   // String spacing in mm
        )
    }

    /// Create a 5-string banjo configuration (standard G tuning)
    ///
    /// Standard 5-string banjo tuning: G5, D3, G3, B3, D4
    /// The 5th string (G5) is shorter and starts at the 5th fret.
    ///
    /// # Returns
    /// A StringedInstrumentConfig for 5-string banjo
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::banjo_5_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 5);
    /// ```
    pub fn banjo_5_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 5), // G5 (5th string, drone)
            Tuning::new(PitchClass::D, 3), // D3 (4th string)
            Tuning::new(PitchClass::G, 3), // G3 (3rd string)
            Tuning::new(PitchClass::B, 3), // B3 (2nd string)
            Tuning::new(PitchClass::D, 4), // D4 (1st string)
        ];

        StringedInstrumentConfig::new(
            tunings,
            22,    // Fret count
            670.0, // Scale length in mm (26.25 inches)
            32.0,  // Nut width in mm
            6.5,   // String spacing in mm
        )
    }

    /// Create a 4-string banjo configuration (tenor banjo)
    ///
    /// Standard tenor banjo tuning: C3, G3, D4, A4 (like viola)
    ///
    /// # Returns
    /// A StringedInstrumentConfig for 4-string tenor banjo
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::banjo_4_string();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn banjo_4_string() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::C, 3), // C3
            Tuning::new(PitchClass::G, 3), // G3
            Tuning::new(PitchClass::D, 4), // D4
            Tuning::new(PitchClass::A, 4), // A4
        ];

        StringedInstrumentConfig::new(
            tunings,
            19,    // Fret count
            584.0, // Scale length in mm (23 inches)
            28.0,  // Nut width in mm
            7.0,   // String spacing in mm
        )
    }

    /// Create a soprano ukulele configuration
    ///
    /// Standard soprano ukulele tuning: G4, C4, E4, A4 (re-entrant tuning)
    /// The G string is tuned higher than the C string.
    ///
    /// # Returns
    /// A StringedInstrumentConfig for soprano ukulele
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::ukulele_soprano();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn ukulele_soprano() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 4), // G4 (re-entrant, higher than C)
            Tuning::new(PitchClass::C, 4), // C4
            Tuning::new(PitchClass::E, 4), // E4
            Tuning::new(PitchClass::A, 4), // A4
        ];

        StringedInstrumentConfig::new(
            tunings,
            15,    // Fret count
            346.0, // Scale length in mm (13.625 inches)
            35.0,  // Nut width in mm
            8.5,   // String spacing in mm
        )
    }

    /// Create a concert ukulele configuration
    ///
    /// Standard concert ukulele tuning: G4, C4, E4, A4 (same as soprano)
    /// Larger body and longer scale length than soprano.
    ///
    /// # Returns
    /// A StringedInstrumentConfig for concert ukulele
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::ukulele_concert();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn ukulele_concert() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 4), // G4 (re-entrant)
            Tuning::new(PitchClass::C, 4), // C4
            Tuning::new(PitchClass::E, 4), // E4
            Tuning::new(PitchClass::A, 4), // A4
        ];

        StringedInstrumentConfig::new(
            tunings,
            18,    // Fret count
            381.0, // Scale length in mm (15 inches)
            38.0,  // Nut width in mm
            9.0,   // String spacing in mm
        )
    }

    /// Create a tenor ukulele configuration
    ///
    /// Standard tenor ukulele tuning: G4, C4, E4, A4 (same as soprano/concert)
    /// Larger body and longer scale length than concert.
    ///
    /// # Returns
    /// A StringedInstrumentConfig for tenor ukulele
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::ukulele_tenor();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn ukulele_tenor() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::G, 4), // G4 (re-entrant)
            Tuning::new(PitchClass::C, 4), // C4
            Tuning::new(PitchClass::E, 4), // E4
            Tuning::new(PitchClass::A, 4), // A4
        ];

        StringedInstrumentConfig::new(
            tunings,
            19,    // Fret count
            432.0, // Scale length in mm (17 inches)
            40.0,  // Nut width in mm
            9.5,   // String spacing in mm
        )
    }

    /// Create a baritone ukulele configuration
    ///
    /// Standard baritone ukulele tuning: D3, G3, B3, E4 (like top 4 guitar strings)
    /// Linear tuning (not re-entrant like smaller ukuleles).
    ///
    /// # Returns
    /// A StringedInstrumentConfig for baritone ukulele
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedFretboard};
    ///
    /// let config = InstrumentPresets::ukulele_baritone();
    /// let fretboard = StringedFretboard::new(config).unwrap();
    /// assert_eq!(fretboard.string_count(), 4);
    /// ```
    pub fn ukulele_baritone() -> StringedInstrumentConfig {
        let tunings = vec![
            Tuning::new(PitchClass::D, 3), // D3 (linear tuning)
            Tuning::new(PitchClass::G, 3), // G3
            Tuning::new(PitchClass::B, 3), // B3
            Tuning::new(PitchClass::E, 4), // E4
        ];

        StringedInstrumentConfig::new(
            tunings,
            19,    // Fret count
            508.0, // Scale length in mm (20 inches)
            42.0,  // Nut width in mm
            10.0,  // String spacing in mm
        )
    }

    /// Get all available preset names
    ///
    /// # Returns
    /// Vector of all available preset configuration names
    ///
    /// # Example
    /// ```
    /// use mutheors::InstrumentPresets;
    ///
    /// let presets = InstrumentPresets::list_presets();
    /// assert!(presets.contains(&"guitar_standard".to_string()));
    /// assert!(presets.contains(&"piano_88_key".to_string()));
    /// assert!(presets.contains(&"violin_standard".to_string()));
    /// ```
    pub fn list_presets() -> Vec<String> {
        vec![
            "guitar_standard".to_string(),
            "guitar_7_string".to_string(),
            "guitar_12_string".to_string(),
            "bass_4_string".to_string(),
            "bass_5_string".to_string(),
            "bass_6_string".to_string(),
            "ukulele_soprano".to_string(),
            "ukulele_concert".to_string(),
            "ukulele_tenor".to_string(),
            "ukulele_baritone".to_string(),
            "mandolin_standard".to_string(),
            "mandolin_octave".to_string(),
            "banjo_4_string".to_string(),
            "banjo_5_string".to_string(),
            "violin_standard".to_string(),
            "viola_standard".to_string(),
            "cello_standard".to_string(),
            "double_bass_standard".to_string(),
            "double_bass_5_string".to_string(),
            "violin_scordatura_high_d".to_string(),
            "piano_88_key".to_string(),
            "keyboard_76_key".to_string(),
            "keyboard_61_key".to_string(),
            "keyboard_49_key".to_string(),
            "keyboard_25_key".to_string(),
        ]
    }

    /// Get a preset configuration by name
    ///
    /// # Arguments
    /// * `preset_name` - Name of the preset to retrieve
    ///
    /// # Returns
    /// * `Some(StringedInstrumentConfig)` for stringed instruments
    /// * `Some(KeyboardConfig)` for keyboard instruments (returned as enum variant)
    /// * `None` if the preset name is not recognized
    ///
    /// # Example
    /// ```
    /// use mutheors::InstrumentPresets;
    ///
    /// let config = InstrumentPresets::get_stringed_preset("guitar_standard").unwrap();
    /// assert_eq!(config.string_count(), 6);
    ///
    /// let invalid = InstrumentPresets::get_stringed_preset("nonexistent");
    /// assert!(invalid.is_none());
    /// ```
    pub fn get_stringed_preset(preset_name: &str) -> Option<StringedInstrumentConfig> {
        match preset_name {
            "guitar_standard" => Some(Self::guitar_standard()),
            "guitar_7_string" => Some(Self::guitar_7_string()),
            "guitar_12_string" => Some(Self::guitar_12_string()),
            "bass_4_string" => Some(Self::bass_4_string()),
            "bass_5_string" => Some(Self::bass_5_string()),
            "bass_6_string" => Some(Self::bass_6_string()),
            "ukulele_soprano" => Some(Self::ukulele_soprano()),
            "ukulele_concert" => Some(Self::ukulele_concert()),
            "ukulele_tenor" => Some(Self::ukulele_tenor()),
            "ukulele_baritone" => Some(Self::ukulele_baritone()),
            "mandolin_standard" => Some(Self::mandolin_standard()),
            "mandolin_octave" => Some(Self::mandolin_octave()),
            "banjo_4_string" => Some(Self::banjo_4_string()),
            "banjo_5_string" => Some(Self::banjo_5_string()),
            "violin_standard" => Some(Self::violin_standard()),
            "viola_standard" => Some(Self::viola_standard()),
            "cello_standard" => Some(Self::cello_standard()),
            "double_bass_standard" => Some(Self::double_bass_standard()),
            "double_bass_5_string" => Some(Self::double_bass_5_string()),
            "violin_scordatura_high_d" => Some(Self::violin_scordatura_high_d()),
            _ => None,
        }
    }

    /// Get a keyboard preset configuration by name
    ///
    /// # Arguments
    /// * `preset_name` - Name of the preset to retrieve
    ///
    /// # Returns
    /// * `Some(KeyboardConfig)` if the preset exists
    /// * `None` if the preset name is not recognized
    ///
    /// # Example
    /// ```
    /// use mutheors::InstrumentPresets;
    ///
    /// let config = InstrumentPresets::get_keyboard_preset("piano_88_key").unwrap();
    /// assert_eq!(config.key_count, 88);
    ///
    /// let invalid = InstrumentPresets::get_keyboard_preset("nonexistent");
    /// assert!(invalid.is_none());
    /// ```
    pub fn get_keyboard_preset(preset_name: &str) -> Option<KeyboardConfig> {
        match preset_name {
            "piano_88_key" => Some(Self::piano_88_key()),
            "keyboard_76_key" => Some(Self::keyboard_76_key()),
            "keyboard_61_key" => Some(Self::keyboard_61_key()),
            "keyboard_49_key" => Some(Self::keyboard_49_key()),
            "keyboard_25_key" => Some(Self::keyboard_25_key()),
            _ => None,
        }
    }

    /// Get a preset configuration by name (legacy method for backward compatibility)
    ///
    /// # Arguments
    /// * `preset_name` - Name of the preset to retrieve
    ///
    /// # Returns
    /// * `Some(StringedInstrumentConfig)` if the preset exists and is a stringed instrument
    /// * `None` if the preset name is not recognized or is a keyboard instrument
    ///
    /// # Example
    /// ```
    /// use mutheors::InstrumentPresets;
    ///
    /// let config = InstrumentPresets::get_preset("guitar_standard").unwrap();
    /// assert_eq!(config.string_count(), 6);
    ///
    /// let invalid = InstrumentPresets::get_preset("nonexistent");
    /// assert!(invalid.is_none());
    /// ```
    pub fn get_preset(preset_name: &str) -> Option<StringedInstrumentConfig> {
        Self::get_stringed_preset(preset_name)
    }

    /// Validate a custom keyboard configuration
    ///
    /// This function performs comprehensive validation beyond the basic
    /// KeyboardConfig::validate() method, checking for musical
    /// and physical reasonableness.
    ///
    /// # Arguments
    /// * `config` - The configuration to validate
    ///
    /// # Returns
    /// * `Ok(())` if the configuration is valid
    /// * `Err(String)` with detailed error message if invalid
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, KeyboardConfig, KeyLayout, Tuning};
    /// use std::str::FromStr;
    ///
    /// let valid_config = InstrumentPresets::piano_88_key();
    /// assert!(InstrumentPresets::validate_keyboard_configuration(&valid_config).is_ok());
    ///
    /// let invalid_config = KeyboardConfig::new(
    ///     Tuning::from_str("C4").unwrap(),
    ///     200, // Too many keys
    ///     KeyLayout::Piano,
    /// );
    /// assert!(InstrumentPresets::validate_keyboard_configuration(&invalid_config).is_err());
    /// ```
    pub fn validate_keyboard_configuration(config: &KeyboardConfig) -> Result<(), String> {
        // First run basic validation
        config.validate()?;

        // Additional validation for musical and physical reasonableness

        // Check key count limits
        if config.key_count > 128 {
            return Err(format!(
                "Too many keys: {} (maximum 128 reasonable for MIDI compatibility)",
                config.key_count
            ));
        }

        if config.key_count < 8 {
            return Err(format!(
                "Too few keys: {} (minimum 8 for practical use)",
                config.key_count
            ));
        }

        // Check tuning range reasonableness (C0 to C9)
        let lowest_pitch = config.lowest_key.number();
        if lowest_pitch < 12 || lowest_pitch > 120 {
            // C0 = 12, C9 = 120 in MIDI note numbers
            return Err(format!(
                "Lowest key {} is outside reasonable range (C0-C9)",
                config.lowest_key
            ));
        }

        // Check that the highest key doesn't exceed reasonable range
        // Use i16 to avoid overflow during calculation
        let highest_pitch = lowest_pitch as i16 + (config.key_count as i16 - 1);
        if highest_pitch > 127 {
            // MIDI note 127 is G9
            return Err(format!(
                "Highest key (calculated as MIDI note {}) exceeds MIDI range (0-127)",
                highest_pitch
            ));
        }

        // Warn about unusual keyboard sizes (not an error, just unusual)
        let common_sizes = [25, 32, 37, 49, 54, 61, 73, 76, 88];
        if !common_sizes.contains(&config.key_count) {
            // This could be logged as a warning in a real implementation
            // For now, we'll allow it but note it's unusual
        }

        Ok(())
    }

    /// Validate a custom instrument configuration
    ///
    /// This function performs comprehensive validation beyond the basic
    /// StringedInstrumentConfig::validate() method, checking for musical
    /// and physical reasonableness.
    ///
    /// # Arguments
    /// * `config` - The configuration to validate
    ///
    /// # Returns
    /// * `Ok(())` if the configuration is valid
    /// * `Err(String)` with detailed error message if invalid
    ///
    /// # Example
    /// ```
    /// use mutheors::{InstrumentPresets, StringedInstrumentConfig, Tuning};
    /// use std::str::FromStr;
    ///
    /// let valid_config = InstrumentPresets::guitar_standard();
    /// assert!(InstrumentPresets::validate_configuration(&valid_config).is_ok());
    ///
    /// let invalid_config = StringedInstrumentConfig::new(
    ///     vec![Tuning::from_str("C4").unwrap()],
    ///     100, // Too many frets
    ///     10000.0, // Unrealistic scale length
    ///     200.0, // Unrealistic nut width
    ///     50.0, // Unrealistic string spacing
    /// );
    /// assert!(InstrumentPresets::validate_configuration(&invalid_config).is_err());
    /// ```
    pub fn validate_configuration(config: &StringedInstrumentConfig) -> Result<(), String> {
        // First run basic validation
        config.validate()?;

        // Additional validation for musical and physical reasonableness

        // Check string count limits
        if config.strings.len() > 12 {
            return Err(format!(
                "Too many strings: {} (maximum 12 supported)",
                config.strings.len()
            ));
        }

        // Check fret count limits
        if config.fret_count > 36 {
            return Err(format!(
                "Too many frets: {} (maximum 36 reasonable)",
                config.fret_count
            ));
        }

        // Check scale length reasonableness (100mm to 2000mm)
        if config.scale_length < 100.0 || config.scale_length > 2000.0 {
            return Err(format!(
                "Unrealistic scale length: {:.1}mm (should be 100-2000mm)",
                config.scale_length
            ));
        }

        // Check nut width reasonableness (10mm to 100mm)
        if config.nut_width < 10.0 || config.nut_width > 100.0 {
            return Err(format!(
                "Unrealistic nut width: {:.1}mm (should be 10-100mm)",
                config.nut_width
            ));
        }

        // Check string spacing reasonableness (2mm to 30mm)
        if config.string_spacing < 2.0 || config.string_spacing > 30.0 {
            return Err(format!(
                "Unrealistic string spacing: {:.1}mm (should be 2-30mm)",
                config.string_spacing
            ));
        }

        // Check that nut width can accommodate all strings
        // For most instruments, the actual string spacing at the nut is reasonable
        // We'll use a more lenient check that allows for typical instrument dimensions
        if config.strings.len() > 1 {
            let min_spacing_required =
                (config.strings.len() - 1) as f32 * (config.string_spacing * 0.8);
            if config.nut_width < min_spacing_required {
                return Err(format!(
                    "Nut width {:.1}mm is too narrow for {} strings with {:.1}mm spacing (minimum recommended: {:.1}mm)",
                    config.nut_width,
                    config.strings.len(),
                    config.string_spacing,
                    min_spacing_required
                ));
            }
        }

        // Check tuning range reasonableness (C0 to C8)
        for (i, tuning) in config.strings.iter().enumerate() {
            let pitch_number = tuning.number();
            if pitch_number < 12 || pitch_number > 108 {
                // C0 = 12, C8 = 108 in MIDI note numbers
                return Err(format!(
                    "String {} tuning {} is outside reasonable range (C0-C8)",
                    i + 1,
                    tuning
                ));
            }
        }

        // Check for reasonable tuning progression (strings should generally go from low to high)
        // Allow some flexibility for instruments like banjo or re-entrant ukulele
        let mut ascending_count = 0;
        let mut descending_count = 0;

        for i in 1..config.strings.len() {
            let prev_pitch = config.strings[i - 1].number();
            let curr_pitch = config.strings[i].number();

            if curr_pitch > prev_pitch {
                ascending_count += 1;
            } else if curr_pitch < prev_pitch {
                descending_count += 1;
            }
        }

        // Warn if tuning is neither mostly ascending nor has a clear pattern
        let total_intervals = config.strings.len() - 1;
        if ascending_count < total_intervals / 2 && descending_count < total_intervals / 2 {
            // This is just a warning, not an error - some instruments have complex tunings
            // Could be logged in a real implementation
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StringedFretboard;
    use crate::fret::ContinuousFretboard;

    #[test]
    fn test_guitar_standard_preset() {
        let config = InstrumentPresets::guitar_standard();
        assert_eq!(config.string_count(), 6);
        assert_eq!(config.fret_count, 24);
        assert!(config.validate().is_ok());

        // Test that it creates a valid fretboard
        let fretboard = StringedFretboard::new(config).unwrap();
        assert_eq!(fretboard.string_count(), 6);
        assert_eq!(fretboard.fret_count(), 24);

        // Test standard tuning
        assert_eq!(
            fretboard.string_tuning(0),
            Some(&Tuning::new(PitchClass::E, 2))
        );
        assert_eq!(
            fretboard.string_tuning(5),
            Some(&Tuning::new(PitchClass::E, 4))
        );
    }

    #[test]
    fn test_guitar_7_string_preset() {
        let config = InstrumentPresets::guitar_7_string();
        assert_eq!(config.string_count(), 7);
        assert_eq!(config.fret_count, 24);
        assert!(config.validate().is_ok());

        let fretboard = StringedFretboard::new(config).unwrap();
        assert_eq!(fretboard.string_count(), 7);

        // Test 7-string tuning (low B added)
        assert_eq!(
            fretboard.string_tuning(0),
            Some(&Tuning::new(PitchClass::B, 1))
        );
        assert_eq!(
            fretboard.string_tuning(6),
            Some(&Tuning::new(PitchClass::E, 4))
        );
    }

    #[test]
    fn test_guitar_12_string_preset() {
        let config = InstrumentPresets::guitar_12_string();
        assert_eq!(config.string_count(), 12);
        assert_eq!(config.fret_count, 22);
        assert!(config.validate().is_ok());

        let fretboard = StringedFretboard::new(config).unwrap();
        assert_eq!(fretboard.string_count(), 12);

        // Test 12-string tuning (doubled strings)
        assert_eq!(
            fretboard.string_tuning(0),
            Some(&Tuning::new(PitchClass::E, 2))
        );
        assert_eq!(
            fretboard.string_tuning(1),
            Some(&Tuning::new(PitchClass::E, 3))
        ); // Octave pair
    }

    #[test]
    fn test_bass_presets() {
        // 4-string bass
        let bass4 = InstrumentPresets::bass_4_string();
        assert_eq!(bass4.string_count(), 4);
        assert!(bass4.validate().is_ok());

        let fretboard4 = StringedFretboard::new(bass4).unwrap();
        assert_eq!(
            fretboard4.string_tuning(0),
            Some(&Tuning::new(PitchClass::E, 1))
        );

        // 5-string bass
        let bass5 = InstrumentPresets::bass_5_string();
        assert_eq!(bass5.string_count(), 5);
        assert!(bass5.validate().is_ok());

        let fretboard5 = StringedFretboard::new(bass5).unwrap();
        assert_eq!(
            fretboard5.string_tuning(0),
            Some(&Tuning::new(PitchClass::B, 0))
        );

        // 6-string bass
        let bass6 = InstrumentPresets::bass_6_string();
        assert_eq!(bass6.string_count(), 6);
        assert!(bass6.validate().is_ok());

        let fretboard6 = StringedFretboard::new(bass6).unwrap();
        assert_eq!(
            fretboard6.string_tuning(5),
            Some(&Tuning::new(PitchClass::C, 3))
        );
    }

    #[test]
    fn test_violin_family_presets() {
        // Violin
        let violin = InstrumentPresets::violin_standard();
        assert_eq!(violin.string_count(), 4);
        assert_eq!(violin.fret_count, 0); // Continuous instrument
        assert!(violin.validate().is_ok());

        let violin_fretboard = ContinuousFretboard::new(violin).unwrap();
        assert_eq!(violin_fretboard.string_count(), 4);

        // Test violin tuning (G3-D4-A4-E5)
        assert_eq!(
            violin_fretboard.string_tuning(0),
            Some(Tuning::new(PitchClass::G, 3))
        );
        assert_eq!(
            violin_fretboard.string_tuning(3),
            Some(Tuning::new(PitchClass::E, 5))
        );

        // Viola
        let viola = InstrumentPresets::viola_standard();
        assert_eq!(viola.string_count(), 4);
        assert_eq!(viola.fret_count, 0);
        assert!(viola.validate().is_ok());

        let viola_fretboard = ContinuousFretboard::new(viola).unwrap();
        assert_eq!(
            viola_fretboard.string_tuning(0),
            Some(Tuning::new(PitchClass::C, 3))
        );
        assert_eq!(
            viola_fretboard.string_tuning(3),
            Some(Tuning::new(PitchClass::A, 4))
        );

        // Cello
        let cello = InstrumentPresets::cello_standard();
        assert_eq!(cello.string_count(), 4);
        assert_eq!(cello.fret_count, 0);
        assert!(cello.validate().is_ok());

        let cello_fretboard = ContinuousFretboard::new(cello).unwrap();
        assert_eq!(
            cello_fretboard.string_tuning(0),
            Some(Tuning::new(PitchClass::C, 2))
        );
        assert_eq!(
            cello_fretboard.string_tuning(3),
            Some(Tuning::new(PitchClass::A, 3))
        );

        // Double Bass
        let double_bass = InstrumentPresets::double_bass_standard();
        assert_eq!(double_bass.string_count(), 4);
        assert_eq!(double_bass.fret_count, 0);
        assert!(double_bass.validate().is_ok());

        let double_bass_fretboard = ContinuousFretboard::new(double_bass).unwrap();
        assert_eq!(
            double_bass_fretboard.string_tuning(0),
            Some(Tuning::new(PitchClass::E, 1))
        );
        assert_eq!(
            double_bass_fretboard.string_tuning(3),
            Some(Tuning::new(PitchClass::G, 2))
        );

        // 5-string Double Bass
        let double_bass_5 = InstrumentPresets::double_bass_5_string();
        assert_eq!(double_bass_5.string_count(), 5);
        assert_eq!(double_bass_5.fret_count, 0);
        assert!(double_bass_5.validate().is_ok());

        let double_bass_5_fretboard = ContinuousFretboard::new(double_bass_5).unwrap();
        assert_eq!(
            double_bass_5_fretboard.string_tuning(4),
            Some(Tuning::new(PitchClass::C, 3))
        );
    }

    #[test]
    fn test_scordatura_support() {
        // Test scordatura (alternative tuning) support
        let violin_scordatura = InstrumentPresets::violin_scordatura_high_d();
        assert_eq!(violin_scordatura.string_count(), 4);
        assert_eq!(violin_scordatura.fret_count, 0);
        assert!(violin_scordatura.validate().is_ok());

        let scordatura_fretboard = ContinuousFretboard::new(violin_scordatura).unwrap();

        // Test that the high string is tuned to D instead of E
        assert_eq!(
            scordatura_fretboard.string_tuning(3),
            Some(Tuning::new(PitchClass::D, 5))
        );

        // Other strings should remain standard
        assert_eq!(
            scordatura_fretboard.string_tuning(0),
            Some(Tuning::new(PitchClass::G, 3))
        );
        assert_eq!(
            scordatura_fretboard.string_tuning(1),
            Some(Tuning::new(PitchClass::D, 4))
        );
        assert_eq!(
            scordatura_fretboard.string_tuning(2),
            Some(Tuning::new(PitchClass::A, 4))
        );
    }

    #[test]
    fn test_violin_family_physical_dimensions() {
        let violin = InstrumentPresets::violin_standard();
        let viola = InstrumentPresets::viola_standard();
        let cello = InstrumentPresets::cello_standard();
        let double_bass = InstrumentPresets::double_bass_standard();

        // Scale lengths should increase with instrument size
        assert!(violin.scale_length < viola.scale_length);
        assert!(viola.scale_length < cello.scale_length);
        assert!(cello.scale_length < double_bass.scale_length);

        // Nut widths should increase with instrument size
        assert!(violin.nut_width < viola.nut_width);
        assert!(viola.nut_width < cello.nut_width);
        assert!(cello.nut_width < double_bass.nut_width);

        // String spacing should increase with instrument size
        assert!(violin.string_spacing < viola.string_spacing);
        assert!(viola.string_spacing < cello.string_spacing);
        assert!(cello.string_spacing < double_bass.string_spacing);
    }

    #[test]
    fn test_other_instruments() {
        // Ukulele
        let ukulele = InstrumentPresets::ukulele_soprano();
        assert_eq!(ukulele.string_count(), 4);
        assert!(ukulele.validate().is_ok());

        // Mandolin
        let mandolin = InstrumentPresets::mandolin_standard();
        assert_eq!(mandolin.string_count(), 8);
        assert!(mandolin.validate().is_ok());

        // Banjo
        let banjo = InstrumentPresets::banjo_5_string();
        assert_eq!(banjo.string_count(), 5);
        assert!(banjo.validate().is_ok());
    }

    #[test]
    fn test_preset_listing() {
        let presets = InstrumentPresets::list_presets();
        assert!(presets.len() >= 15); // At least the presets we defined (including violin family)

        assert!(presets.contains(&"guitar_standard".to_string()));
        assert!(presets.contains(&"bass_4_string".to_string()));
        assert!(presets.contains(&"ukulele_soprano".to_string()));
        assert!(presets.contains(&"violin_standard".to_string()));
        assert!(presets.contains(&"viola_standard".to_string()));
        assert!(presets.contains(&"cello_standard".to_string()));
        assert!(presets.contains(&"double_bass_standard".to_string()));
    }

    #[test]
    fn test_get_preset() {
        // Valid presets
        let guitar = InstrumentPresets::get_preset("guitar_standard").unwrap();
        assert_eq!(guitar.string_count(), 6);

        let bass = InstrumentPresets::get_preset("bass_4_string").unwrap();
        assert_eq!(bass.string_count(), 4);

        let violin = InstrumentPresets::get_preset("violin_standard").unwrap();
        assert_eq!(violin.string_count(), 4);
        assert_eq!(violin.fret_count, 0); // Continuous instrument

        // Invalid preset
        let invalid = InstrumentPresets::get_preset("nonexistent");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_configuration_validation() {
        // Valid configurations should pass
        let valid_config = InstrumentPresets::guitar_standard();
        assert!(InstrumentPresets::validate_configuration(&valid_config).is_ok());

        // Test various invalid configurations
        let too_many_strings = StringedInstrumentConfig::new(
            vec![Tuning::new(PitchClass::C, 4); 15], // 15 strings
            24,
            648.0,
            43.0,
            10.5,
        );
        assert!(InstrumentPresets::validate_configuration(&too_many_strings).is_err());

        let too_many_frets = StringedInstrumentConfig::new(
            vec![Tuning::new(PitchClass::E, 2)],
            50, // 50 frets
            648.0,
            43.0,
            10.5,
        );
        assert!(InstrumentPresets::validate_configuration(&too_many_frets).is_err());

        let unrealistic_scale = StringedInstrumentConfig::new(
            vec![Tuning::new(PitchClass::E, 2)],
            24,
            5000.0, // 5 meter scale length
            43.0,
            10.5,
        );
        assert!(InstrumentPresets::validate_configuration(&unrealistic_scale).is_err());

        let narrow_nut = StringedInstrumentConfig::new(
            vec![
                Tuning::new(PitchClass::E, 2),
                Tuning::new(PitchClass::A, 2),
                Tuning::new(PitchClass::D, 3),
            ],
            24,
            648.0,
            5.0, // 5mm nut width for 3 strings
            10.5,
        );
        assert!(InstrumentPresets::validate_configuration(&narrow_nut).is_err());
    }

    #[test]
    fn test_keyboard_presets() {
        // 88-key piano
        let piano88 = InstrumentPresets::piano_88_key();
        assert_eq!(piano88.key_count, 88);
        assert!(piano88.validate().is_ok());

        // 76-key keyboard
        let keyboard76 = InstrumentPresets::keyboard_76_key();
        assert_eq!(keyboard76.key_count, 76);
        assert!(keyboard76.validate().is_ok());

        // 61-key keyboard
        let keyboard61 = InstrumentPresets::keyboard_61_key();
        assert_eq!(keyboard61.key_count, 61);
        assert!(keyboard61.validate().is_ok());

        // 49-key keyboard
        let keyboard49 = InstrumentPresets::keyboard_49_key();
        assert_eq!(keyboard49.key_count, 49);
        assert!(keyboard49.validate().is_ok());

        // 25-key keyboard
        let keyboard25 = InstrumentPresets::keyboard_25_key();
        assert_eq!(keyboard25.key_count, 25);
        assert!(keyboard25.validate().is_ok());
    }

    #[test]
    fn test_get_keyboard_preset() {
        // Valid presets
        let piano = InstrumentPresets::get_keyboard_preset("piano_88_key").unwrap();
        assert_eq!(piano.key_count, 88);

        let keyboard61 = InstrumentPresets::get_keyboard_preset("keyboard_61_key").unwrap();
        assert_eq!(keyboard61.key_count, 61);

        // Invalid preset
        let invalid = InstrumentPresets::get_keyboard_preset("nonexistent");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_keyboard_configuration_validation() {
        // Valid configurations should pass
        let valid_config = InstrumentPresets::piano_88_key();
        assert!(InstrumentPresets::validate_keyboard_configuration(&valid_config).is_ok());

        // Test various invalid configurations
        let too_many_keys = KeyboardConfig::new(
            Tuning::new(PitchClass::C, 4),
            200, // 200 keys
            KeyLayout::Piano,
        );
        assert!(InstrumentPresets::validate_keyboard_configuration(&too_many_keys).is_err());

        let too_few_keys = KeyboardConfig::new(
            Tuning::new(PitchClass::C, 4),
            5, // 5 keys
            KeyLayout::Piano,
        );
        assert!(InstrumentPresets::validate_keyboard_configuration(&too_few_keys).is_err());

        let too_low_range = KeyboardConfig::new(
            Tuning::new(PitchClass::C, -2), // C-2 is too low
            88,
            KeyLayout::Piano,
        );
        assert!(InstrumentPresets::validate_keyboard_configuration(&too_low_range).is_err());

        let too_high_range = KeyboardConfig::new(
            Tuning::new(PitchClass::C, 8), // C8 + 88 keys would exceed MIDI range
            88,
            KeyLayout::Piano,
        );
        assert!(InstrumentPresets::validate_keyboard_configuration(&too_high_range).is_err());
    }

    #[test]
    fn test_all_presets_are_valid() {
        // Ensure all built-in presets pass validation
        let preset_names = InstrumentPresets::list_presets();

        for preset_name in preset_names {
            // Test stringed instrument presets
            if let Some(config) = InstrumentPresets::get_stringed_preset(&preset_name) {
                match InstrumentPresets::validate_configuration(&config) {
                    Ok(()) => {}
                    Err(e) => {
                        panic!("Stringed preset '{}' failed validation: {}", preset_name, e);
                    }
                }

                // Also ensure they can create valid fretboards
                assert!(
                    StringedFretboard::new(config).is_ok(),
                    "Stringed preset '{}' cannot create valid fretboard",
                    preset_name
                );
            }

            // Test keyboard presets
            if let Some(config) = InstrumentPresets::get_keyboard_preset(&preset_name) {
                match InstrumentPresets::validate_keyboard_configuration(&config) {
                    Ok(()) => {}
                    Err(e) => {
                        panic!("Keyboard preset '{}' failed validation: {}", preset_name, e);
                    }
                }

                // Also ensure they can create valid fretboards
                use crate::fret::KeyboardFretboard;
                assert!(
                    KeyboardFretboard::new(config).is_ok(),
                    "Keyboard preset '{}' cannot create valid fretboard",
                    preset_name
                );
            }
        }
    }

    #[test]
    fn test_preset_physical_dimensions() {
        let guitar = InstrumentPresets::guitar_standard();
        let bass = InstrumentPresets::bass_4_string();
        let ukulele = InstrumentPresets::ukulele_soprano();

        // Bass should have longer scale length than guitar
        assert!(bass.scale_length > guitar.scale_length);

        // Ukulele should have shorter scale length than guitar
        assert!(ukulele.scale_length < guitar.scale_length);

        // Bass should have wider string spacing than guitar
        assert!(bass.string_spacing > guitar.string_spacing);

        // 12-string guitar should have wider nut than 6-string
        let guitar12 = InstrumentPresets::guitar_12_string();
        assert!(guitar12.nut_width > guitar.nut_width);
    }
}

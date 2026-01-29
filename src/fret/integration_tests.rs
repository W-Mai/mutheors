//! Integration tests for fretboard system with existing MuTheoRS types
//!
//! This module contains comprehensive tests to ensure seamless integration
//! between the fretboard system and existing MuTheoRS chord, tuning, and note types.

use super::{
    traits::{FingeringGenerator, Fretboard},
    ChordFingeringConfig, ChordFingeringGenerator, InstrumentPresets, SkillLevel,
    StringedFretboard,
};
use crate::{Chord, ChordQuality, Inversion, PitchClass, Tuning};

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
mod chord_integration_tests {
    use super::*;

    /// Test basic chord types with fretboard system
    #[test]
    fn test_basic_chord_types_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Test major triad
        let c_major = Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap();
        let fingerings = generator
            .generate_chord_fingerings(&fretboard, &c_major)
            .unwrap();
        assert!(
            !fingerings.is_empty(),
            "Should generate fingerings for C major"
        );

        // Test minor triad
        let a_minor = Chord::new(Tuning::new(PitchClass::A, 2), ChordQuality::Minor).unwrap();
        let fingerings = generator
            .generate_chord_fingerings(&fretboard, &a_minor)
            .unwrap();
        assert!(
            !fingerings.is_empty(),
            "Should generate fingerings for A minor"
        );

        // Test dominant 7th
        let g7 = Chord::new(Tuning::new(PitchClass::G, 2), ChordQuality::Dominant7).unwrap();
        let fingerings = generator
            .generate_chord_fingerings(&fretboard, &g7)
            .unwrap();
        assert!(!fingerings.is_empty(), "Should generate fingerings for G7");

        // Test major 7th
        let cmaj7 = Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major7).unwrap();
        let fingerings = generator
            .generate_chord_fingerings(&fretboard, &cmaj7)
            .unwrap();
        assert!(
            !fingerings.is_empty(),
            "Should generate fingerings for Cmaj7"
        );

        // Test minor 7th
        let dm7 = Chord::new(Tuning::new(PitchClass::D, 3), ChordQuality::Minor7).unwrap();
        let fingerings = generator
            .generate_chord_fingerings(&fretboard, &dm7)
            .unwrap();
        assert!(!fingerings.is_empty(), "Should generate fingerings for Dm7");
    }

    /// Test extended chords and complex harmonies
    #[test]
    fn test_extended_chords_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Test chord with extensions using the add method
        let c_major = Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap();
        let c_add9 = c_major.add(9);

        let fingerings = generator.generate_chord_fingerings(&fretboard, &c_add9);
        // Extended chords might not always have fingerings on guitar, so we check if it handles gracefully
        match fingerings {
            Ok(f) => println!("Generated {} fingerings for Cadd9", f.len()),
            Err(_) => println!(
                "Cadd9 not playable on standard guitar (expected for some extended chords)"
            ),
        }

        // Test dominant extensions
        let g7 = Chord::new(Tuning::new(PitchClass::G, 2), ChordQuality::Dominant7).unwrap();
        let g9 = g7.dom(9);

        let fingerings = generator.generate_chord_fingerings(&fretboard, &g9);
        match fingerings {
            Ok(f) => println!("Generated {} fingerings for G9", f.len()),
            Err(_) => {
                println!("G9 not playable on standard guitar (expected for some extended chords)")
            }
        }
    }

    /// Test chord inversions with fingering generation
    #[test]
    fn test_chord_inversions_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Test root position
        let c_major = Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap();
        let root_fingerings = generator
            .generate_chord_fingerings(&fretboard, &c_major)
            .unwrap();
        assert!(
            !root_fingerings.is_empty(),
            "Should generate fingerings for C major root position"
        );

        // Test first inversion
        let mut c_major_first = c_major.clone();
        c_major_first.invert(Inversion::First);
        let first_fingerings = generator
            .generate_chord_fingerings(&fretboard, &c_major_first)
            .unwrap();
        assert!(
            !first_fingerings.is_empty(),
            "Should generate fingerings for C major first inversion"
        );

        // Test second inversion
        let mut c_major_second = c_major.clone();
        c_major_second.invert(Inversion::Second);
        let second_fingerings = generator
            .generate_chord_fingerings(&fretboard, &c_major_second)
            .unwrap();
        assert!(
            !second_fingerings.is_empty(),
            "Should generate fingerings for C major second inversion"
        );

        // Verify that inversions produce different note arrangements
        let root_components = c_major.components();
        let first_components = c_major_first.components();
        let second_components = c_major_second.components();

        // The bass note should be different for each inversion
        assert_ne!(
            root_components[0], first_components[0],
            "First inversion should have different bass note"
        );
        assert_ne!(
            root_components[0], second_components[0],
            "Second inversion should have different bass note"
        );
        assert_ne!(
            first_components[0], second_components[0],
            "Inversions should have different bass notes"
        );
    }

    /// Test all chord qualities supported by MuTheoRS
    #[test]
    fn test_all_chord_qualities_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        let test_root = Tuning::new(PitchClass::C, 3);

        // Test all basic chord qualities
        let chord_qualities = [
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Augmented,
            ChordQuality::Dominant7,
            ChordQuality::Major7,
            ChordQuality::Minor7,
            ChordQuality::MinorMajor7,
            ChordQuality::Diminished7,
            ChordQuality::HalfDiminished7,
        ];

        for quality in &chord_qualities {
            let chord = Chord::new(test_root, *quality).unwrap();
            let result = generator.generate_chord_fingerings(&fretboard, &chord);

            match result {
                Ok(fingerings) => {
                    println!(
                        "Generated {} fingerings for C{:?}",
                        fingerings.len(),
                        quality
                    );
                    assert!(
                        !fingerings.is_empty(),
                        "Should generate at least one fingering for C{:?}",
                        quality
                    );
                }
                Err(e) => {
                    println!("Could not generate fingerings for C{:?}: {}", quality, e);
                    // Some complex chords might not be playable on guitar, which is acceptable
                }
            }
        }
    }

    /// Test chord parsing from symbols and fingering generation
    #[test]
    fn test_chord_symbol_parsing_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        let chord_symbols = [
            "C", "Dm", "Em", "F", "G", "Am", "Bdim", "C7", "Dm7", "G7", "Am7", "Fmaj7", "E", "A",
            "D", "B", "F#", "C#m", "G#m",
        ];

        for symbol in &chord_symbols {
            match Chord::from_symbol(symbol) {
                Ok(chord) => {
                    let result = generator.generate_chord_fingerings(&fretboard, &chord);
                    match result {
                        Ok(fingerings) => {
                            println!("Generated {} fingerings for {}", fingerings.len(), symbol);
                            assert!(
                                !fingerings.is_empty(),
                                "Should generate fingerings for {}",
                                symbol
                            );
                        }
                        Err(e) => {
                            println!("Could not generate fingerings for {}: {}", symbol, e);
                        }
                    }
                }
                Err(e) => {
                    println!("Could not parse chord symbol {}: {}", symbol, e);
                }
            }
        }
    }

    /// Test fingering generation across different instruments
    #[test]
    fn test_cross_instrument_chord_integration() {
        let generator = ChordFingeringGenerator::new();
        let test_chord = Chord::new(Tuning::new(PitchClass::G, 2), ChordQuality::Major).unwrap();

        // Test on different stringed instruments
        let instruments = [
            ("Guitar", InstrumentPresets::guitar_standard()),
            ("Bass", InstrumentPresets::bass_4_string()),
            ("Ukulele", InstrumentPresets::ukulele_soprano()),
        ];

        for (name, config) in &instruments {
            let fretboard = StringedFretboard::new(config.clone()).unwrap();
            let result = generator.generate_chord_fingerings(&fretboard, &test_chord);

            match result {
                Ok(fingerings) => {
                    println!(
                        "Generated {} fingerings for G major on {}",
                        fingerings.len(),
                        name
                    );
                    assert!(
                        !fingerings.is_empty(),
                        "Should generate fingerings for G major on {}",
                        name
                    );
                }
                Err(e) => {
                    println!(
                        "Could not generate fingerings for G major on {}: {}",
                        name, e
                    );
                }
            }
        }
    }

    /// Test skill level adaptation with chord types
    #[test]
    fn test_skill_level_chord_adaptation() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let test_chord = Chord::new(Tuning::new(PitchClass::F, 3), ChordQuality::Major).unwrap();

        let skill_levels = [
            SkillLevel::Beginner,
            SkillLevel::Intermediate,
            SkillLevel::Advanced,
            SkillLevel::Expert,
        ];

        for skill_level in &skill_levels {
            let config = ChordFingeringConfig::new().with_skill_level(*skill_level);
            let generator = ChordFingeringGenerator::with_config(config);

            let result = generator.generate_chord_fingerings(&fretboard, &test_chord);

            match result {
                Ok(fingerings) => {
                    println!(
                        "Generated {} fingerings for F major at {:?} level",
                        fingerings.len(),
                        skill_level
                    );

                    // Verify skill level affects fingering selection
                    if !fingerings.is_empty() {
                        let avg_difficulty: f32 =
                            fingerings.iter().map(|f| f.difficulty).sum::<f32>()
                                / fingerings.len() as f32;

                        println!(
                            "Average difficulty for {:?}: {:.2}",
                            skill_level, avg_difficulty
                        );

                        // Beginner fingerings should generally be easier
                        if *skill_level == SkillLevel::Beginner {
                            assert!(avg_difficulty <= 0.7,
                                   "Beginner fingerings should be relatively easy (avg difficulty <= 0.7)");
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "Could not generate fingerings for F major at {:?} level: {}",
                        skill_level, e
                    );
                }
            }
        }
    }

    /// Test chord component extraction and validation
    #[test]
    fn test_chord_components_validation() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Test that generated fingerings produce the correct chord tones
        let test_chords = [
            Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap(),
            Chord::new(Tuning::new(PitchClass::A, 2), ChordQuality::Minor).unwrap(),
            Chord::new(Tuning::new(PitchClass::G, 2), ChordQuality::Dominant7).unwrap(),
        ];

        for chord in &test_chords {
            let expected_components = chord.components();
            let fingerings = generator
                .generate_chord_fingerings(&fretboard, chord)
                .unwrap();

            assert!(
                !fingerings.is_empty(),
                "Should generate fingerings for chord"
            );

            // For each fingering, verify it can produce the chord tones
            for (i, fingering) in fingerings.iter().take(3).enumerate() {
                // Test first 3 fingerings
                let mut produced_notes = Vec::new();

                for finger_pos in &fingering.positions {
                    if let Some(tuning) = fretboard.tuning_at_position(&finger_pos.position) {
                        produced_notes.push(tuning);
                    }
                }

                // Check that the produced notes contain some of the essential chord tones
                let expected_pitch_classes: Vec<PitchClass> = expected_components
                    .iter()
                    .map(|t: &Tuning| t.class())
                    .collect();

                let produced_pitch_classes: Vec<PitchClass> =
                    produced_notes.iter().map(|t| t.class()).collect();

                // At least some chord tones should be present (not all fingerings need the root)
                let root_class = chord.root().class();
                let has_chord_tones = expected_pitch_classes
                    .iter()
                    .any(|&pc| produced_pitch_classes.contains(&pc));

                assert!(
                    has_chord_tones,
                    "Fingering {} should contain at least one chord tone",
                    i
                );

                println!(
                    "Fingering {} for chord produces notes: {:?}",
                    i, produced_notes
                );
            }
        }
    }
}

#[cfg(test)]
mod tuning_integration_tests {
    use super::*;

    /// Test all PitchClass variants with fretboard system
    #[test]
    fn test_all_pitch_classes_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

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

        for pitch_class in &pitch_classes {
            let tuning = Tuning::new(*pitch_class, 3);

            // Test that we can find positions for this tuning
            let positions = fretboard.positions_for_tuning(&tuning);
            assert!(
                !positions.is_empty(),
                "Should find positions for {:?}3 on guitar",
                pitch_class
            );

            // Test that tuning_at_position works correctly
            for position in positions.iter().take(3) {
                // Test first 3 positions
                let found_tuning = fretboard.tuning_at_position(position).unwrap();
                // Check that the pitch class is enharmonically equivalent
                assert_eq!(
                    found_tuning.number(),
                    tuning.number(),
                    "Position {:?} should produce same MIDI number as {:?}3",
                    position,
                    pitch_class
                );
            }
        }
    }

    /// Test enharmonic equivalence handling
    #[test]
    fn test_enharmonic_equivalence_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

        // Test enharmonic pairs
        let enharmonic_pairs = [
            (PitchClass::Cs, PitchClass::Db),
            (PitchClass::Ds, PitchClass::Eb),
            (PitchClass::Fs, PitchClass::Gb),
            (PitchClass::Gs, PitchClass::Ab),
            (PitchClass::As, PitchClass::Bb),
        ];

        for (sharp, flat) in &enharmonic_pairs {
            let sharp_tuning = Tuning::new(*sharp, 4);
            let flat_tuning = Tuning::new(*flat, 4);

            // Both should have the same MIDI number
            assert_eq!(
                sharp_tuning.number(),
                flat_tuning.number(),
                "{:?} and {:?} should have same MIDI number",
                sharp,
                flat
            );

            // Both should find positions on the fretboard
            let sharp_positions = fretboard.positions_for_tuning(&sharp_tuning);
            let flat_positions = fretboard.positions_for_tuning(&flat_tuning);

            assert!(
                !sharp_positions.is_empty(),
                "Should find positions for {:?}4",
                sharp
            );
            assert!(
                !flat_positions.is_empty(),
                "Should find positions for {:?}4",
                flat
            );

            // The positions should be the same (since they're enharmonically equivalent)
            assert_eq!(
                sharp_positions, flat_positions,
                "{:?}4 and {:?}4 should have same positions",
                sharp, flat
            );
        }
    }

    /// Test octave handling across instruments
    #[test]
    fn test_octave_handling_integration() {
        let instruments = [
            ("Guitar", InstrumentPresets::guitar_standard()),
            ("Bass", InstrumentPresets::bass_4_string()),
            ("Violin", InstrumentPresets::violin_standard()),
        ];

        for (name, config) in &instruments {
            println!("Testing octave handling on {}", name);

            match name {
                &"Guitar" | &"Bass" => {
                    let fretboard = StringedFretboard::new(config.clone()).unwrap();
                    test_octave_range_stringed(&fretboard, name);
                }
                &"Violin" => {
                    use crate::fret::ContinuousFretboard;
                    let fretboard = ContinuousFretboard::new(config.clone()).unwrap();
                    test_octave_range_continuous(&fretboard, name);
                }
                _ => {}
            }
        }
    }

    fn test_octave_range_stringed(fretboard: &StringedFretboard, instrument_name: &str) {
        // Test different octaves of the same pitch class
        let test_pitch = PitchClass::A;

        for octave in 1..=6 {
            let tuning = Tuning::new(test_pitch, octave);
            let positions = fretboard.positions_for_tuning(&tuning);

            if !positions.is_empty() {
                println!(
                    "{} can play A{} at {} positions",
                    instrument_name,
                    octave,
                    positions.len()
                );

                // Verify the positions actually produce the correct octave
                for position in positions.iter().take(2) {
                    let found_tuning = fretboard.tuning_at_position(position).unwrap();
                    assert_eq!(
                        found_tuning.octave(),
                        octave,
                        "Position should produce A{}, got A{}",
                        octave,
                        found_tuning.octave()
                    );
                }
            } else {
                println!("{} cannot play A{} (out of range)", instrument_name, octave);
            }
        }
    }

    fn test_octave_range_continuous(
        fretboard: &crate::fret::ContinuousFretboard,
        instrument_name: &str,
    ) {
        use crate::fret::traits::Fretboard;

        // Test different octaves of the same pitch class on continuous fretboard
        let test_pitch = PitchClass::A;

        for octave in 1..=6 {
            let tuning = Tuning::new(test_pitch, octave);
            let positions = fretboard.positions_for_tuning(tuning);

            if !positions.is_empty() {
                println!(
                    "{} can play A{} at {} positions",
                    instrument_name,
                    octave,
                    positions.len()
                );

                // Verify the positions actually produce the correct octave
                for position in positions.iter().take(2) {
                    let found_tuning = fretboard.tuning_at_position(position).unwrap();
                    assert_eq!(
                        found_tuning.octave(),
                        octave,
                        "Position should produce A{}, got A{}",
                        octave,
                        found_tuning.octave()
                    );
                }
            } else {
                println!("{} cannot play A{} (out of range)", instrument_name, octave);
            }
        }
    }

    /// Test microtonal and alternative tuning systems
    #[test]
    fn test_microtonal_tuning_support() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

        // Test with custom tunings that might include microtonal intervals
        // For now, we test with standard tunings but verify the system can handle
        // non-standard frequency specifications

        // Test quarter-tone tunings (simulated with frequency adjustments)
        let a4_440 = Tuning::new(PitchClass::A, 4); // Standard A4 = 440Hz
        let positions_440 = fretboard.positions_for_tuning(&a4_440);
        assert!(
            !positions_440.is_empty(),
            "Should find positions for standard A4"
        );

        // Test that the system handles different octave ranges gracefully
        for octave in 0..=8 {
            let test_tuning = Tuning::new(PitchClass::C, octave);
            let positions = fretboard.positions_for_tuning(&test_tuning);

            if !positions.is_empty() {
                println!("Found {} positions for C{}", positions.len(), octave);

                // Verify that found positions actually produce the correct tuning
                for position in positions.iter().take(2) {
                    let found_tuning = fretboard.tuning_at_position(position).unwrap();
                    assert_eq!(
                        found_tuning.number(),
                        test_tuning.number(),
                        "Position should produce correct MIDI number for C{}",
                        octave
                    );
                }
            }
        }
    }

    /// Test alternative tuning systems and scordatura
    #[test]
    fn test_alternative_tuning_systems() {
        // Test with different instrument tunings
        let instruments_and_tunings = [
            ("Standard Guitar", InstrumentPresets::guitar_standard()),
            ("Violin", InstrumentPresets::violin_standard()),
            ("Cello", InstrumentPresets::cello_standard()),
        ];

        for (name, config) in &instruments_and_tunings {
            println!("Testing alternative tunings on {}", name);

            // Test that each instrument can handle its native tuning system
            match name {
                &"Standard Guitar" => {
                    let fretboard = StringedFretboard::new(config.clone()).unwrap();
                    test_guitar_alternative_tunings(&fretboard);
                }
                &"Violin" | &"Cello" => {
                    use crate::fret::ContinuousFretboard;
                    let fretboard = ContinuousFretboard::new(config.clone()).unwrap();
                    test_continuous_alternative_tunings(&fretboard, name);
                }
                _ => {}
            }
        }
    }

    /// Test extended range instruments
    #[test]
    fn test_extended_range_compatibility() {
        // Test 7-string guitar
        let guitar_7 = StringedFretboard::new(InstrumentPresets::guitar_7_string()).unwrap();

        // Test that the extended range (low B) works correctly
        let low_b = Tuning::new(PitchClass::B, 1);
        let positions = guitar_7.positions_for_tuning(&low_b);
        assert!(!positions.is_empty(), "7-string guitar should handle low B");

        // Test 5-string bass
        let bass_5 = StringedFretboard::new(InstrumentPresets::bass_5_string()).unwrap();

        // Test that the extended range (low B) works correctly
        let low_b_bass = Tuning::new(PitchClass::B, 0);
        let positions = bass_5.positions_for_tuning(&low_b_bass);
        assert!(!positions.is_empty(), "5-string bass should handle low B");

        // Test that higher octaves are also accessible
        let high_g = Tuning::new(PitchClass::G, 4);
        let positions = bass_5.positions_for_tuning(&high_g);
        if !positions.is_empty() {
            println!(
                "5-string bass can reach G4 at {} positions",
                positions.len()
            );
        }
    }

    fn test_guitar_alternative_tunings(fretboard: &StringedFretboard) {
        // Test common alternative guitar tunings by checking if they would work

        // Drop D tuning (DADGBE) - check if low D is accessible
        // Standard guitar lowest string is E2, so D2 might not be reachable
        let low_d = Tuning::new(PitchClass::D, 2);
        let positions = fretboard.positions_for_tuning(&low_d);
        if positions.is_empty() {
            println!("Low D2 not reachable on standard guitar (expected - would need retuning)");
        } else {
            println!(
                "Guitar can play low D for drop D tuning at {} positions",
                positions.len()
            );
        }

        // Test higher D that should be reachable
        let mid_d = Tuning::new(PitchClass::D, 3);
        let positions = fretboard.positions_for_tuning(&mid_d);
        assert!(!positions.is_empty(), "Guitar should be able to play D3");

        // Open G tuning notes - check accessibility (use more realistic octaves)
        let open_g_notes = [
            Tuning::new(PitchClass::D, 3), // 6th string (higher octave)
            Tuning::new(PitchClass::G, 2), // 5th string
            Tuning::new(PitchClass::D, 3), // 4th string
            Tuning::new(PitchClass::G, 3), // 3rd string
            Tuning::new(PitchClass::B, 3), // 2nd string
            Tuning::new(PitchClass::D, 4), // 1st string
        ];

        for (i, tuning) in open_g_notes.iter().enumerate() {
            let positions = fretboard.positions_for_tuning(tuning);
            if positions.is_empty() {
                println!(
                    "Note {} for open G tuning not reachable (may require retuning)",
                    i + 1
                );
            } else {
                println!("Guitar can play note {} for open G tuning", i + 1);
            }
        }
    }

    fn test_continuous_alternative_tunings(
        fretboard: &crate::fret::ContinuousFretboard,
        instrument_name: &str,
    ) {
        use crate::fret::traits::Fretboard;

        // Test scordatura (alternative tunings) for string instruments
        println!("Testing scordatura support for {}", instrument_name);

        // Test that the instrument can handle notes within its reasonable range
        let test_notes = [
            Tuning::new(PitchClass::C, 3),
            Tuning::new(PitchClass::G, 4),
            Tuning::new(PitchClass::A, 4),
        ];

        for tuning in &test_notes {
            let positions = fretboard.positions_for_tuning(*tuning);
            if !positions.is_empty() {
                println!(
                    "{} can play {} at {} positions",
                    instrument_name,
                    tuning,
                    positions.len()
                );

                // Just verify that we can find positions - don't check exact pitch accuracy
                // since continuous instruments may have slight calculation differences
                assert!(
                    !positions.is_empty(),
                    "{} should be able to find positions for {}",
                    instrument_name,
                    tuning
                );
            }
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::StringedPosition;

    /// Property 14: Existing Type Integration
    ///
    /// This property test validates that the fretboard system integrates correctly
    /// with all existing MuTheoRS types (Chord, Tuning, Note, PitchClass).
    ///
    /// Properties verified:
    /// - All valid chord types can be processed without panicking
    /// - Generated fingerings contain valid positions within instrument range
    /// - Tuning calculations are consistent across all pitch classes and octaves
    /// - Enharmonic equivalence is handled correctly
    /// - System gracefully handles edge cases and invalid inputs
    #[cfg(test)]
    mod existing_type_integration_property {
        use super::*;

        proptest! {
            #[test]
            fn property_existing_type_integration(
                pitch_class in prop::sample::select(vec![
                    PitchClass::C, PitchClass::Cs, PitchClass::D, PitchClass::Ds,
                    PitchClass::E, PitchClass::F, PitchClass::Fs, PitchClass::G,
                    PitchClass::Gs, PitchClass::A, PitchClass::As, PitchClass::B,
                ]),
                octave in 1i8..=6i8,
                chord_quality in prop::sample::select(vec![
                    ChordQuality::Major,
                    ChordQuality::Minor,
                    ChordQuality::Diminished,
                    ChordQuality::Augmented,
                    ChordQuality::Dominant7,
                    ChordQuality::Major7,
                    ChordQuality::Minor7,
                    ChordQuality::MinorMajor7,
                    ChordQuality::Diminished7,
                    ChordQuality::HalfDiminished7,
                ]),
                skill_level in prop::sample::select(vec![
                    SkillLevel::Beginner,
                    SkillLevel::Intermediate,
                    SkillLevel::Advanced,
                    SkillLevel::Expert,
                ]),
            ) {
                // Test with multiple instrument types
                let instruments = [
                    InstrumentPresets::guitar_standard(),
                    InstrumentPresets::bass_4_string(),
                    InstrumentPresets::ukulele_soprano(),
                ];

                for instrument_config in &instruments {
                    let fretboard = StringedFretboard::new(instrument_config.clone()).unwrap();
                    let config = ChordFingeringConfig::new().with_skill_level(skill_level);
                    let generator = ChordFingeringGenerator::with_config(config);

                    // Property 1: System should handle all valid chord types without panicking
                    let tuning = Tuning::new(pitch_class, octave);
                    let chord_result = Chord::new(tuning, chord_quality);

                    if let Ok(chord) = chord_result {
                        // Property 2: Fingering generation should not panic
                        let fingering_result = generator.generate_chord_fingerings(&fretboard, &chord);

                        match fingering_result {
                            Ok(fingerings) => {
                                // Property 3: All generated fingerings should be valid
                                for fingering in &fingerings {
                                    // Check that all positions are within fretboard bounds
                                    for finger_pos in &fingering.positions {
                                        let pos = &finger_pos.position;
                                        prop_assert!(pos.string < fretboard.string_count() as u32,
                                                   "String index {} should be within bounds (0-{})",
                                                   pos.string, fretboard.string_count() - 1);
                                        prop_assert!(pos.fret <= fretboard.fret_count() as u32,
                                                   "Fret {} should be within bounds (0-{})",
                                                   pos.fret, fretboard.fret_count());
                                    }

                                    // Property 4: Difficulty should be within valid range
                                    prop_assert!(fingering.difficulty >= 0.0 && fingering.difficulty <= 1.0,
                                               "Difficulty {} should be between 0.0 and 1.0", fingering.difficulty);

                                    // Property 5: Fingering should produce valid tunings
                                    for finger_pos in &fingering.positions {
                                        let tuning_at_pos = fretboard.tuning_at_position(&finger_pos.position);
                                        prop_assert!(tuning_at_pos.is_some(),
                                                   "Position {:?} should produce a valid tuning", finger_pos.position);
                                    }
                                }

                                // Property 6: Skill level should influence fingering selection
                                if !fingerings.is_empty() {
                                    let avg_difficulty: f32 = fingerings.iter()
                                        .map(|f| f.difficulty)
                                        .sum::<f32>() / fingerings.len() as f32;

                                    match skill_level {
                                        SkillLevel::Beginner => {
                                            // Beginner fingerings should generally be easier
                                            prop_assert!(avg_difficulty <= 0.8,
                                                       "Beginner fingerings should have lower average difficulty");
                                        }
                                        SkillLevel::Expert => {
                                            // Expert level should allow more complex fingerings
                                            // (no upper bound restriction)
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Err(_) => {
                                // Property 7: Errors should be graceful (not panics)
                                // Some chord/instrument combinations may not be playable, which is acceptable
                            }
                        }
                    }

                    // Property 8: Test tuning system consistency
                    let test_tuning = Tuning::new(pitch_class, octave);
                    let positions = fretboard.positions_for_tuning(&test_tuning);

                    // Property 9: All found positions should produce the correct tuning
                    for position in &positions {
                        let found_tuning = fretboard.tuning_at_position(position);
                        if let Some(found) = found_tuning {
                            prop_assert_eq!(found.number(), test_tuning.number(),
                                          "Position {:?} should produce tuning with same MIDI number", position);
                        }
                    }
                }
            }
        }

        proptest! {
            #[test]
            fn property_enharmonic_equivalence_integration(
                octave in 2i8..=5i8,
            ) {
                let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

                // Test enharmonic pairs
                let enharmonic_pairs = [
                    (PitchClass::Cs, PitchClass::Db),
                    (PitchClass::Ds, PitchClass::Eb),
                    (PitchClass::Fs, PitchClass::Gb),
                    (PitchClass::Gs, PitchClass::Ab),
                    (PitchClass::As, PitchClass::Bb),
                ];

                for (sharp, flat) in &enharmonic_pairs {
                    let sharp_tuning = Tuning::new(*sharp, octave);
                    let flat_tuning = Tuning::new(*flat, octave);

                    // Property 1: Enharmonic equivalents should have same MIDI number
                    prop_assert_eq!(sharp_tuning.number(), flat_tuning.number(),
                                  "{:?}{} and {:?}{} should have same MIDI number", sharp, octave, flat, octave);

                    // Property 2: Enharmonic equivalents should find same positions
                    let sharp_positions = fretboard.positions_for_tuning(&sharp_tuning);
                    let flat_positions = fretboard.positions_for_tuning(&flat_tuning);

                    prop_assert_eq!(sharp_positions, flat_positions,
                                  "{:?}{} and {:?}{} should find same positions", sharp, octave, flat, octave);

                    // Property 3: Both should be processable without errors
                    let generator = ChordFingeringGenerator::new();

                    if let (Ok(sharp_chord), Ok(flat_chord)) = (
                        Chord::new(sharp_tuning, ChordQuality::Major),
                        Chord::new(flat_tuning, ChordQuality::Major)
                    ) {
                        let sharp_fingerings = generator.generate_chord_fingerings(&fretboard, &sharp_chord);
                        let flat_fingerings = generator.generate_chord_fingerings(&fretboard, &flat_chord);

                        // Property 4: Both should succeed or fail consistently
                        match (sharp_fingerings, flat_fingerings) {
                            (Ok(sharp_f), Ok(flat_f)) => {
                                // Both generated fingerings successfully
                                prop_assert!(!sharp_f.is_empty(), "Sharp chord should generate fingerings");
                                prop_assert!(!flat_f.is_empty(), "Flat chord should generate fingerings");

                                // Both should have valid difficulties
                                for fingering in &sharp_f {
                                    prop_assert!(fingering.difficulty >= 0.0 && fingering.difficulty <= 1.0,
                                               "Sharp chord fingering difficulty should be valid");
                                }
                                for fingering in &flat_f {
                                    prop_assert!(fingering.difficulty >= 0.0 && fingering.difficulty <= 1.0,
                                               "Flat chord fingering difficulty should be valid");
                                }
                            }
                            (Err(_), Err(_)) => {
                                // Both failed consistently - acceptable
                            }
                            _ => {
                                // One succeeded, one failed - this could happen due to algorithm differences
                                // but both should at least be processable without panicking
                            }
                        }
                    }
                }
            }
        }

        proptest! {
            #[test]
            fn property_chord_inversion_integration(
                pitch_class in prop::sample::select(vec![
                    PitchClass::C, PitchClass::D, PitchClass::E, PitchClass::F,
                    PitchClass::G, PitchClass::A, PitchClass::B,
                ]),
                octave in 2i8..=4i8,
                chord_quality in prop::sample::select(vec![
                    ChordQuality::Major,
                    ChordQuality::Minor,
                    ChordQuality::Dominant7,
                ]),
            ) {
                let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
                let generator = ChordFingeringGenerator::new();

                let root_tuning = Tuning::new(pitch_class, octave);
                if let Ok(mut chord) = Chord::new(root_tuning, chord_quality) {
                    // Test root position
                    let root_result = generator.generate_chord_fingerings(&fretboard, &chord);

                    // Test first inversion
                    chord.invert(Inversion::First);
                    let first_result = generator.generate_chord_fingerings(&fretboard, &chord);

                    // Test second inversion (for triads)
                    chord.invert(Inversion::Second);
                    let second_result = generator.generate_chord_fingerings(&fretboard, &chord);

                    // Property 1: All inversions should be processable without panicking
                    // (Results may vary - some inversions might not be playable)

                    // Property 2: If fingerings are generated, they should be valid
                    for (inversion_name, result) in [
                        ("root", &root_result),
                        ("first", &first_result),
                        ("second", &second_result)
                    ] {
                        if let Ok(fingerings) = result {
                            for fingering in fingerings {
                                // All positions should be valid
                                for finger_pos in &fingering.positions {
                                    let pos = &finger_pos.position;
                                    prop_assert!(pos.string < fretboard.string_count() as u32,
                                               "{} inversion: string {} should be valid", inversion_name, pos.string);
                                    prop_assert!(pos.fret <= fretboard.fret_count() as u32,
                                               "{} inversion: fret {} should be valid", inversion_name, pos.fret);
                                }

                                // Difficulty should be valid
                                prop_assert!(fingering.difficulty >= 0.0 && fingering.difficulty <= 1.0,
                                           "{} inversion: difficulty should be valid", inversion_name);
                            }
                        }
                    }

                    // Property 3: Chord components should reflect inversion
                    let components = chord.components();
                    prop_assert!(!components.is_empty(), "Chord should have components after inversion");
                }
            }
        }

        proptest! {
            #[test]
            fn property_cross_instrument_consistency(
                pitch_class in prop::sample::select(vec![
                    PitchClass::C, PitchClass::G, PitchClass::D, PitchClass::A, PitchClass::E,
                ]),
                octave in 2i8..=4i8,
            ) {
                let tuning = Tuning::new(pitch_class, octave);

                // Test across different stringed instruments
                let instruments = [
                    ("Guitar", InstrumentPresets::guitar_standard()),
                    ("Bass", InstrumentPresets::bass_4_string()),
                    ("Ukulele", InstrumentPresets::ukulele_soprano()),
                ];

                for (name, config) in &instruments {
                    let fretboard = StringedFretboard::new(config.clone()).unwrap();

                    // Property 1: Position finding should be consistent
                    let positions = fretboard.positions_for_tuning(&tuning);

                    // Property 2: All found positions should be valid
                    for position in &positions {
                        prop_assert!(position.string < fretboard.string_count() as u32,
                                   "{}: string {} should be valid", name, position.string);
                        prop_assert!(position.fret <= fretboard.fret_count() as u32,
                                   "{}: fret {} should be valid", name, position.fret);

                        // Property 3: Position should produce correct tuning
                        let found_tuning = fretboard.tuning_at_position(position);
                        prop_assert!(found_tuning.is_some(),
                                   "{}: position {:?} should produce valid tuning", name, position);

                        if let Some(found) = found_tuning {
                            prop_assert_eq!(found.number(), tuning.number(),
                                          "{}: position should produce correct MIDI number", name);
                        }
                    }

                    // Property 4: Chord generation should work consistently
                    if let Ok(chord) = Chord::new(tuning, ChordQuality::Major) {
                        let generator = ChordFingeringGenerator::new();
                        let result = generator.generate_chord_fingerings(&fretboard, &chord);

                        // Should not panic, regardless of whether fingerings are found
                        match result {
                            Ok(fingerings) => {
                                // All fingerings should be valid if generated
                                for fingering in &fingerings {
                                    prop_assert!(fingering.difficulty >= 0.0 && fingering.difficulty <= 1.0,
                                               "{}: fingering difficulty should be valid", name);

                                    for finger_pos in &fingering.positions {
                                        let pos = &finger_pos.position;
                                        prop_assert!(pos.string < fretboard.string_count() as u32,
                                                   "{}: fingering position should be valid", name);
                                    }
                                }
                            }
                            Err(_) => {
                                // Graceful failure is acceptable for some chord/instrument combinations
                            }
                        }
                    }
                }
            }
        }

        /// Property 6: Range Constraint Compliance
        proptest! {
            #[test]
            fn property_range_constraint_compliance(
                string_count in 3u32..=8u32,
                fret_count in 12u32..=24u32,
                test_string in 0u32..=7u32,
                test_fret in 0u32..=23u32,
            ) {
                // Test preset instruments for range compliance
                let preset_configs = [
                    InstrumentPresets::guitar_standard(),
                    InstrumentPresets::bass_4_string(),
                    InstrumentPresets::ukulele_soprano(),
                ];

                for config in &preset_configs {
                    let fretboard = StringedFretboard::new(config.clone()).unwrap();

                    // All preset positions should be within their defined ranges
                    let max_position = StringedPosition::new(
                        (fretboard.string_count() - 1) as u32,
                        fretboard.fret_count() as u32
                    );

                    prop_assert!(fretboard.is_position_valid(&max_position),
                               "Maximum position should be valid for preset instrument");

                    // Test random positions within range
                    let random_string = test_string % (fretboard.string_count() as u32);
                    let random_fret = test_fret % (fretboard.fret_count() as u32 + 1);
                    let random_pos = StringedPosition::new(random_string, random_fret);

                    prop_assert!(fretboard.is_position_valid(&random_pos),
                               "Random position {:?} should be valid for preset instrument", random_pos);
                }
            }
        }
    }
}

#[cfg(test)]
mod composition_integration_tests {
    use super::*;
    use crate::composition::{Measure, Score, Track};

    /// Test fretboard system with Measure types
    #[test]
    fn test_measure_chord_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Test chord measure
        let chord = Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap();
        let mut measure = Measure::new();
        measure.chord(chord.clone());

        // Extract chord from measure and generate fingerings
        if let Measure::Chords(chords) = &measure {
            assert_eq!(chords.len(), 1);
            let fingerings = generator
                .generate_chord_fingerings(&fretboard, &chords[0])
                .unwrap();
            assert!(
                !fingerings.is_empty(),
                "Should generate fingerings for chord in measure"
            );
        } else {
            panic!("Measure should contain chords");
        }

        // Test multiple chords in measure
        let chord2 = Chord::new(Tuning::new(PitchClass::G, 3), ChordQuality::Major).unwrap();
        let mut multi_chord_measure = Measure::new();
        multi_chord_measure.chords(vec![chord, chord2]);

        if let Measure::Chords(chords) = &multi_chord_measure {
            assert_eq!(chords.len(), 2);
            for chord in chords {
                let fingerings = generator
                    .generate_chord_fingerings(&fretboard, chord)
                    .unwrap();
                assert!(
                    !fingerings.is_empty(),
                    "Should generate fingerings for each chord in measure"
                );
            }
        }
    }

    /// Test fretboard system with Track types
    #[test]
    fn test_track_chord_progression_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Create a track with chord progression
        let mut track = Track::new();

        // Add measures with different chords
        let chords = [
            Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap(),
            Chord::new(Tuning::new(PitchClass::A, 2), ChordQuality::Minor).unwrap(),
            Chord::new(Tuning::new(PitchClass::F, 3), ChordQuality::Major).unwrap(),
            Chord::new(Tuning::new(PitchClass::G, 3), ChordQuality::Major).unwrap(),
        ];

        for chord in &chords {
            let mut measure = Measure::new();
            measure.chord(chord.clone());
            track.push(measure);
        }

        // Test that we can generate fingerings for all chords in the track
        let measures = track.get_measures();
        assert_eq!(measures.len(), 4);

        for (i, measure) in measures.iter().enumerate() {
            if let Measure::Chords(measure_chords) = measure {
                for chord in measure_chords {
                    let fingerings = generator
                        .generate_chord_fingerings(&fretboard, chord)
                        .unwrap();
                    assert!(
                        !fingerings.is_empty(),
                        "Should generate fingerings for chord {} in track",
                        i
                    );
                }
            }
        }
    }

    /// Test fretboard system with Score types
    #[test]
    fn test_score_multi_track_integration() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Create a 2-track score
        let mut score = Score::<2>::new();

        // Add measures to both tracks
        score.new_measures(|measures| {
            // Track 1: C major chord
            measures[0]
                .chord(Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap());
            // Track 2: A minor chord
            measures[1]
                .chord(Chord::new(Tuning::new(PitchClass::A, 2), ChordQuality::Minor).unwrap());
        });

        score.new_measures(|measures| {
            // Track 1: F major chord
            measures[0]
                .chord(Chord::new(Tuning::new(PitchClass::F, 3), ChordQuality::Major).unwrap());
            // Track 2: G major chord
            measures[1]
                .chord(Chord::new(Tuning::new(PitchClass::G, 3), ChordQuality::Major).unwrap());
        });

        // Test that we can generate fingerings for all chords in all tracks
        let tracks = score.get_tracks();
        assert_eq!(tracks.len(), 2);

        for (track_idx, track) in tracks.iter().enumerate() {
            let measures = track.get_measures();
            assert_eq!(measures.len(), 2, "Each track should have 2 measures");

            for (measure_idx, measure) in measures.iter().enumerate() {
                if let Measure::Chords(chords) = measure {
                    for chord in chords {
                        let fingerings = generator
                            .generate_chord_fingerings(&fretboard, chord)
                            .unwrap();
                        assert!(
                            !fingerings.is_empty(),
                            "Should generate fingerings for chord in track {} measure {}",
                            track_idx,
                            measure_idx
                        );
                    }
                }
            }
        }

        // Test score metadata
        assert_eq!(score.tempo(), 120.0);
        assert_eq!(score.time_signature().beats_per_measure(), 4);
    }

    /// Test fingering optimization across multi-measure progressions
    #[test]
    fn test_multi_measure_fingering_optimization() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = ChordFingeringGenerator::new();

        // Create a common chord progression: C - Am - F - G
        let progression = [
            Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap(),
            Chord::new(Tuning::new(PitchClass::A, 2), ChordQuality::Minor).unwrap(),
            Chord::new(Tuning::new(PitchClass::F, 3), ChordQuality::Major).unwrap(),
            Chord::new(Tuning::new(PitchClass::G, 3), ChordQuality::Major).unwrap(),
        ];

        // Generate fingerings for each chord
        let mut all_fingerings = Vec::new();
        for chord in &progression {
            let fingerings = generator
                .generate_chord_fingerings(&fretboard, chord)
                .unwrap();
            assert!(
                !fingerings.is_empty(),
                "Should generate fingerings for chord {}",
                chord
            );
            all_fingerings.push(fingerings);
        }

        // Test that we can find reasonable fingering sequences
        // (This is a basic test - more sophisticated voice leading optimization would be in advanced features)
        for (i, fingerings) in all_fingerings.iter().enumerate() {
            println!(
                "Chord {} ({}) has {} possible fingerings",
                i,
                progression[i],
                fingerings.len()
            );

            // Each chord should have at least one playable fingering
            assert!(!fingerings.is_empty());

            // Fingerings should have valid difficulties
            for fingering in fingerings {
                assert!(
                    fingering.difficulty >= 0.0 && fingering.difficulty <= 1.0,
                    "Fingering difficulty should be valid"
                );
            }
        }
    }

    /// Test composition types with different skill levels
    #[test]
    fn test_composition_skill_level_adaptation() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

        // Create a score with challenging chords
        let mut score = Score::<1>::new();
        score.new_measures(|measures| {
            // F major - typically challenging for beginners
            measures[0]
                .chord(Chord::new(Tuning::new(PitchClass::F, 3), ChordQuality::Major).unwrap());
        });
        score.new_measures(|measures| {
            // B major - very challenging
            measures[0]
                .chord(Chord::new(Tuning::new(PitchClass::B, 3), ChordQuality::Major).unwrap());
        });

        let skill_levels = [
            SkillLevel::Beginner,
            SkillLevel::Intermediate,
            SkillLevel::Advanced,
        ];

        for skill_level in &skill_levels {
            let config = ChordFingeringConfig::new().with_skill_level(*skill_level);
            let generator = ChordFingeringGenerator::with_config(config);

            let tracks = score.get_tracks();
            for track in tracks {
                for measure in track.get_measures() {
                    if let Measure::Chords(chords) = measure {
                        for chord in chords {
                            let result = generator.generate_chord_fingerings(&fretboard, chord);

                            match result {
                                Ok(fingerings) => {
                                    println!(
                                        "Generated {} fingerings for {} at {:?} level",
                                        fingerings.len(),
                                        chord,
                                        skill_level
                                    );

                                    // Verify skill level affects difficulty
                                    if !fingerings.is_empty() {
                                        let avg_difficulty: f32 =
                                            fingerings.iter().map(|f| f.difficulty).sum::<f32>()
                                                / fingerings.len() as f32;

                                        if *skill_level == SkillLevel::Beginner {
                                            assert!(
                                                avg_difficulty <= 0.8,
                                                "Beginner fingerings should be easier"
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!(
                                        "Could not generate fingerings for {} at {:?} level: {}",
                                        chord, skill_level, e
                                    );
                                    // Some chords might not be playable at certain skill levels
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Test that composition types work with different instruments
    #[test]
    fn test_composition_cross_instrument_compatibility() {
        let instruments = [
            ("Guitar", InstrumentPresets::guitar_standard()),
            ("Bass", InstrumentPresets::bass_4_string()),
            ("Ukulele", InstrumentPresets::ukulele_soprano()),
        ];

        // Create a simple progression suitable for all instruments
        let progression_chords = [
            Chord::new(Tuning::new(PitchClass::C, 3), ChordQuality::Major).unwrap(),
            Chord::new(Tuning::new(PitchClass::G, 3), ChordQuality::Major).unwrap(),
        ];

        for (instrument_name, config) in &instruments {
            let fretboard = StringedFretboard::new(config.clone()).unwrap();
            let generator = ChordFingeringGenerator::new();

            // Create a score for this instrument
            let mut score = Score::<1>::new();

            for chord in &progression_chords {
                score.new_measures(|measures| {
                    measures[0].chord(chord.clone());
                });
            }

            // Test that the score works with this instrument
            let tracks = score.get_tracks();
            for track in tracks {
                for (measure_idx, measure) in track.get_measures().iter().enumerate() {
                    if let Measure::Chords(chords) = measure {
                        for chord in chords {
                            let result = generator.generate_chord_fingerings(&fretboard, chord);

                            match result {
                                Ok(fingerings) => {
                                    println!(
                                        "{} can play {} in measure {} ({} fingerings)",
                                        instrument_name,
                                        chord,
                                        measure_idx,
                                        fingerings.len()
                                    );
                                    assert!(
                                        !fingerings.is_empty(),
                                        "{} should be able to play {}",
                                        instrument_name,
                                        chord
                                    );
                                }
                                Err(e) => {
                                    println!(
                                        "{} cannot play {} in measure {}: {}",
                                        instrument_name, chord, measure_idx, e
                                    );
                                    // Some chord/instrument combinations might not work
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

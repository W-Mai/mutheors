//! End-to-end tests for audio module
//!
//! Tests the full pipeline: audio signal → pitch detection → Tuning integration

use mutheors::audio::{detect_pitch, detect_pitch_with_config, YinConfig};
use mutheors::*;
use std::f32::consts::PI;

fn generate_sine(freq: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
    let n = (sample_rate * duration) as usize;
    (0..n)
        .map(|i| (2.0 * PI * freq * i as f32 / sample_rate).sin())
        .collect()
}

/// Generate a signal with harmonics (more realistic than pure sine)
fn generate_with_harmonics(fundamental: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
    let n = (sample_rate * duration) as usize;
    (0..n)
        .map(|i| {
            let t = i as f32 / sample_rate;
            let f = fundamental;
            0.6 * (2.0 * PI * f * t).sin()       // fundamental
                + 0.25 * (2.0 * PI * 2.0 * f * t).sin() // 2nd harmonic
                + 0.1 * (2.0 * PI * 3.0 * f * t).sin()  // 3rd harmonic
                + 0.05 * (2.0 * PI * 4.0 * f * t).sin()  // 4th harmonic
        })
        .collect()
}

#[test]
fn e2e_guitar_standard_tuning() {
    // All 6 strings of standard guitar tuning
    let strings = [
        (82.41, PitchClass::E, 2i8),   // 6th string E2
        (110.00, PitchClass::A, 2),     // 5th string A2
        (146.83, PitchClass::D, 3),     // 4th string D3
        (196.00, PitchClass::G, 3),     // 3rd string G3
        (246.94, PitchClass::B, 3),     // 2nd string B3
        (329.63, PitchClass::E, 4),     // 1st string E4
    ];

    let config = YinConfig::guitar_tuner();

    for (freq, expected_class, expected_octave) in strings {
        let samples = generate_with_harmonics(freq, 44100.0, 0.1);
        let result = detect_pitch_with_config(&samples, 44100.0, &config)
            .unwrap_or_else(|| panic!("Failed to detect pitch for {} Hz", freq));

        assert_eq!(
            result.tuning.class(),
            expected_class,
            "String {} Hz: expected {:?}, got {:?}",
            freq,
            expected_class,
            result.tuning.class()
        );
        assert_eq!(
            result.tuning.octave(),
            expected_octave,
            "String {} Hz: expected octave {}, got {}",
            freq,
            expected_octave,
            result.tuning.octave()
        );
        assert!(
            result.cents.abs() < 10.0,
            "String {} Hz: cent offset too large: {:.1}",
            freq,
            result.cents
        );
        assert!(result.confidence > 0.7, "Low confidence for {} Hz", freq);
    }
}

#[test]
fn e2e_pitch_to_scale_membership() {
    // Detect A4, then check it belongs to A major scale
    let samples = generate_sine(440.0, 44100.0, 0.1);
    let result = detect_pitch(&samples, 44100.0).unwrap();

    let a_major = Scale::new(Tuning::new(PitchClass::A, 4), ScaleType::Major).unwrap();
    assert!(
        a_major.contains(&result.tuning),
        "Detected {:?} should be in A major scale",
        result.tuning
    );
}

#[test]
fn e2e_pitch_to_chord_component() {
    // Detect E4 (329.63 Hz), verify it's a component of C major chord
    let samples = generate_sine(329.63, 44100.0, 0.1);
    let result = detect_pitch(&samples, 44100.0).unwrap();

    let c_major = Chord::new(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap();
    let components: Vec<_> = c_major.components().iter().map(|t| t.class()).collect();

    assert!(
        components.contains(&result.tuning.class()),
        "Detected {:?} should be a component of C major (components: {:?})",
        result.tuning.class(),
        components
    );
}

#[test]
fn e2e_from_frequency_roundtrip() {
    // Tuning → frequency → from_frequency → same Tuning
    let original = Tuning::new(PitchClass::Fs, 4);
    let freq = original.frequency() as f64;
    let (recovered, cents) = Tuning::from_frequency(freq);

    assert_eq!(recovered.class(), original.class());
    assert_eq!(recovered.octave(), original.octave());
    assert!(cents.abs() < 0.01, "Roundtrip cent error: {}", cents);
}

#[test]
fn e2e_detect_then_fretboard_lookup() {
    // Detect A2 → find positions on guitar fretboard
    let samples = generate_sine(110.0, 44100.0, 0.1);
    let result = detect_pitch(&samples, 44100.0).unwrap();

    let guitar = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
    let positions = guitar.positions_for_tuning(&result.tuning);

    assert!(
        !positions.is_empty(),
        "A2 should be playable on guitar, detected: {:?}",
        result.tuning
    );
    // A2 is the open 5th string
    assert!(
        positions.iter().any(|p| p.fret == 0),
        "A2 should have an open string position"
    );
}

#[test]
fn e2e_chromatic_scale_detection() {
    // Detect all 12 chromatic notes, verify no duplicates or misidentifications
    let chromatic_freqs = [
        261.63, 277.18, 293.66, 311.13, 329.63, 349.23,
        369.99, 392.00, 415.30, 440.00, 466.16, 493.88,
    ];
    let expected_classes = [
        PitchClass::C, PitchClass::Cs, PitchClass::D, PitchClass::Ds,
        PitchClass::E, PitchClass::F, PitchClass::Fs, PitchClass::G,
        PitchClass::Gs, PitchClass::A, PitchClass::As, PitchClass::B,
    ];

    let mut detected_classes = Vec::new();
    for (freq, expected) in chromatic_freqs.iter().zip(expected_classes.iter()) {
        let samples = generate_sine(*freq, 44100.0, 0.1);
        let result = detect_pitch(&samples, 44100.0).unwrap();
        assert_eq!(
            result.tuning.class(),
            *expected,
            "Freq {}: expected {:?}, got {:?}",
            freq,
            expected,
            result.tuning.class()
        );
        detected_classes.push(result.tuning.class());
    }

    // All 12 should be distinct
    let unique: std::collections::HashSet<_> = detected_classes.iter().map(|c| c.semitones()).collect();
    assert_eq!(unique.len(), 12, "Should detect 12 distinct pitch classes");
}

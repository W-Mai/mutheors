//! Demonstration of instrument preset configurations
//!
//! This example shows how to use the built-in instrument presets
//! to create fretboards for different instruments.

use mutheors::{Fretboard, InstrumentPresets, PitchClass, StringedFretboard, Tuning};

fn main() {
    println!("=== MuTheoRS Instrument Presets Demo ===\n");

    // List all available presets
    println!("Available presets:");
    for preset_name in InstrumentPresets::list_presets() {
        println!("  - {}", preset_name);
    }
    println!();

    // Create a standard guitar
    println!("=== Standard Guitar ===");
    let guitar_config = InstrumentPresets::guitar_standard();
    let guitar = StringedFretboard::new(guitar_config).unwrap();

    println!("Strings: {}", guitar.string_count());
    println!("Frets: {}", guitar.fret_count());

    // Show tuning for each string
    for i in 0..guitar.string_count() {
        if let Some(tuning) = guitar.string_tuning(i) {
            println!("String {}: {}", i + 1, tuning);
        }
    }
    println!();

    // Create a 4-string bass
    println!("=== 4-String Bass ===");
    let bass_config = InstrumentPresets::bass_4_string();
    let bass = StringedFretboard::new(bass_config).unwrap();

    println!("Strings: {}", bass.string_count());
    println!("Frets: {}", bass.fret_count());

    for i in 0..bass.string_count() {
        if let Some(tuning) = bass.string_tuning(i) {
            println!("String {}: {}", i + 1, tuning);
        }
    }
    println!();

    // Demonstrate finding positions for a note
    println!("=== Finding Positions for A2 on Guitar ===");
    let a2 = Tuning::new(PitchClass::A, 2);
    let positions = guitar.positions_for_tuning(&a2);

    for position in positions {
        println!("String {} Fret {}", position.string + 1, position.fret);
    }
    println!();

    // Show preset validation
    println!("=== Preset Validation ===");
    let mandolin_config = InstrumentPresets::mandolin_standard();
    match InstrumentPresets::validate_configuration(&mandolin_config) {
        Ok(()) => println!("Mandolin preset: Valid ✓"),
        Err(e) => println!("Mandolin preset: Invalid - {}", e),
    }

    let ukulele_config = InstrumentPresets::ukulele_soprano();
    match InstrumentPresets::validate_configuration(&ukulele_config) {
        Ok(()) => println!("Ukulele preset: Valid ✓"),
        Err(e) => println!("Ukulele preset: Invalid - {}", e),
    }
}

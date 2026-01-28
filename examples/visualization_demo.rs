// Demo of fretboard visualization functionality
use mutheors::*;
use std::str::FromStr;

fn main() {
    println!("Testing fretboard visualization system...");

    // Create a standard guitar fretboard
    let tunings = vec![
        Tuning::from_str("E2").unwrap(), // Low E
        Tuning::from_str("A2").unwrap(), // A
        Tuning::from_str("D3").unwrap(), // D
        Tuning::from_str("G3").unwrap(), // G
        Tuning::from_str("B3").unwrap(), // B
        Tuning::from_str("E4").unwrap(), // High E
    ];

    let config = StringedInstrumentConfig::new(tunings, 24, 648.0, 43.0, 10.5);
    let fretboard = StringedFretboard::new(config).unwrap();

    // Create a simple C major chord fingering
    let fingering = Fingering::new(
        vec![
            FingerPosition::open(StringedPosition::new(0, 0)), // High E open
            FingerPosition::pressed(StringedPosition::new(1, 1), Finger::Index), // B fret 1
            FingerPosition::open(StringedPosition::new(2, 0)), // G open
            FingerPosition::pressed(StringedPosition::new(3, 2), Finger::Middle), // D fret 2
            FingerPosition::pressed(StringedPosition::new(4, 3), Finger::Ring), // A fret 3
                                                               // Low E muted
        ],
        PlayingTechnique::Standard,
        0.3,
    );

    // Create diagram generator
    let generator = FretboardDiagramGenerator::new();

    // Generate ASCII diagram
    match generator.generate_diagram(&fretboard, &fingering) {
        Ok(diagram) => {
            println!("ASCII Diagram:");
            println!("{}", diagram);
        }
        Err(e) => {
            println!("Error generating diagram: {:?}", e);
        }
    }

    // Generate compact diagram
    match generator.generate_compact_diagram(&fretboard, &fingering) {
        Ok(compact) => {
            println!("Compact Diagram: {}", compact);
        }
        Err(e) => {
            println!("Error generating compact diagram: {:?}", e);
        }
    }

    println!("Visualization test completed successfully!");
}

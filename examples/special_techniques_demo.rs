// Demo of special technique visualization
use mutheors::*;
use std::str::FromStr;

fn main() {
    println!("Testing special technique visualization...");

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

    // Create diagram generator with custom technique characters
    let diagram_config = DiagramConfig::new().with_technique_chars('═', 'H', 'P', 'S', '◊');
    let generator = FretboardDiagramGenerator::with_config(diagram_config);

    println!("\n=== BARRE CHORD (F Major) ===");
    let barre_fingering = Fingering::new(
        vec![
            FingerPosition::pressed(StringedPosition::new(0, 1), Finger::Index), // High E fret 1
            FingerPosition::pressed(StringedPosition::new(1, 1), Finger::Index), // B fret 1
            FingerPosition::pressed(StringedPosition::new(2, 2), Finger::Middle), // G fret 2
            FingerPosition::pressed(StringedPosition::new(3, 3), Finger::Ring),  // D fret 3
            FingerPosition::pressed(StringedPosition::new(4, 3), Finger::Pinky), // A fret 3
            FingerPosition::pressed(StringedPosition::new(5, 1), Finger::Index), // Low E fret 1
        ],
        PlayingTechnique::Barre {
            start_string: 0,
            end_string: 5,
            fret: 1,
        },
        0.8,
    );

    match generator.generate_diagram(&fretboard, &barre_fingering) {
        Ok(diagram) => println!("{}", diagram),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\n=== HARMONIC TECHNIQUE ===");
    let harmonic_fingering = Fingering::new(
        vec![
            FingerPosition::pressed(StringedPosition::new(0, 12), Finger::Index), // 12th fret harmonic
            FingerPosition::pressed(StringedPosition::new(1, 12), Finger::Index),
        ],
        PlayingTechnique::Harmonic,
        0.2,
    );

    match generator.generate_diagram(&fretboard, &harmonic_fingering) {
        Ok(diagram) => println!("{}", diagram),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\n=== HAMMER-ON TECHNIQUE ===");
    let hammer_fingering = Fingering::new(
        vec![
            FingerPosition::pressed(StringedPosition::new(0, 2), Finger::Index),
            FingerPosition::pressed(StringedPosition::new(0, 4), Finger::Ring),
        ],
        PlayingTechnique::Hammer,
        0.4,
    );

    match generator.generate_diagram(&fretboard, &hammer_fingering) {
        Ok(diagram) => println!("{}", diagram),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\n=== SLIDE TECHNIQUE ===");
    let slide_fingering = Fingering::new(
        vec![
            FingerPosition::pressed(StringedPosition::new(0, 3), Finger::Index),
            FingerPosition::pressed(StringedPosition::new(0, 5), Finger::Index),
        ],
        PlayingTechnique::Slide,
        0.3,
    );

    match generator.generate_diagram(&fretboard, &slide_fingering) {
        Ok(diagram) => println!("{}", diagram),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\n=== PULL-OFF TECHNIQUE ===");
    let pull_fingering = Fingering::new(
        vec![
            FingerPosition::pressed(StringedPosition::new(0, 5), Finger::Ring),
            FingerPosition::pressed(StringedPosition::new(0, 3), Finger::Index),
        ],
        PlayingTechnique::Pull,
        0.3,
    );

    match generator.generate_diagram(&fretboard, &pull_fingering) {
        Ok(diagram) => println!("{}", diagram),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("Special technique visualization demo completed!");
}

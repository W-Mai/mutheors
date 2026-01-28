//! Fretboard visualization system for generating ASCII diagrams
//!
//! This module provides functionality to generate text-based diagrams of fretboard
//! fingerings, including finger positions, fret numbers, and string labels.

use super::{
    errors::FretboardResult,
    types::{Finger, FingerPosition, Fingering, PlayingTechnique, StringedPosition},
    StringedFretboard,
};

#[cfg(feature = "bindgen")]
use uniffi;

/// Configuration for fretboard diagram generation
#[derive(Clone, Debug)]
pub struct DiagramConfig {
    /// Show fret numbers on the diagram
    pub show_fret_numbers: bool,
    /// Show string labels (tuning names)
    pub show_string_labels: bool,
    /// Show finger numbers on positions
    pub show_finger_numbers: bool,
    /// Maximum number of frets to display
    pub max_frets: usize,
    /// Minimum fret to start display from (for higher position chords)
    pub min_fret: usize,
    /// Character to use for fret lines
    pub fret_char: char,
    /// Character to use for string lines
    pub string_char: char,
    /// Character to use for finger positions
    pub finger_char: char,
    /// Character to use for open strings
    pub open_char: char,
    /// Character to use for muted strings
    pub mute_char: char,
}

impl Default for DiagramConfig {
    fn default() -> Self {
        Self {
            show_fret_numbers: true,
            show_string_labels: true,
            show_finger_numbers: true,
            max_frets: 5,
            min_fret: 0,
            fret_char: '-',
            string_char: '|',
            finger_char: '●',
            open_char: 'O',
            mute_char: 'X',
        }
    }
}

impl DiagramConfig {
    /// Create a new diagram configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to show fret numbers
    pub fn with_fret_numbers(mut self, show: bool) -> Self {
        self.show_fret_numbers = show;
        self
    }

    /// Set whether to show string labels
    pub fn with_string_labels(mut self, show: bool) -> Self {
        self.show_string_labels = show;
        self
    }

    /// Set whether to show finger numbers
    pub fn with_finger_numbers(mut self, show: bool) -> Self {
        self.show_finger_numbers = show;
        self
    }

    /// Set the fret range to display
    pub fn with_fret_range(mut self, min_fret: usize, max_frets: usize) -> Self {
        self.min_fret = min_fret;
        self.max_frets = max_frets;
        self
    }

    /// Set custom characters for diagram elements
    pub fn with_chars(
        mut self,
        fret_char: char,
        string_char: char,
        finger_char: char,
        open_char: char,
        mute_char: char,
    ) -> Self {
        self.fret_char = fret_char;
        self.string_char = string_char;
        self.finger_char = finger_char;
        self.open_char = open_char;
        self.mute_char = mute_char;
        self
    }
}

/// ASCII fretboard diagram generator
#[derive(Clone, Debug)]
pub struct FretboardDiagramGenerator {
    /// Configuration for diagram generation
    config: DiagramConfig,
}

impl FretboardDiagramGenerator {
    /// Create a new diagram generator with default configuration
    pub fn new() -> Self {
        Self {
            config: DiagramConfig::default(),
        }
    }

    /// Create a new diagram generator with custom configuration
    pub fn with_config(config: DiagramConfig) -> Self {
        Self { config }
    }

    /// Generate an ASCII diagram for a fingering on a stringed fretboard
    pub fn generate_diagram(
        &self,
        fretboard: &StringedFretboard,
        fingering: &Fingering<StringedPosition>,
    ) -> FretboardResult<String> {
        let string_count = fretboard.string_count();

        // Determine the fret range to display
        let (min_fret, max_fret) = self.calculate_fret_range(fingering)?;
        let _display_frets = max_fret - min_fret + 1;

        // Build the diagram
        let mut diagram = String::new();

        // Add title if there are special techniques
        if !matches!(fingering.technique, PlayingTechnique::Standard) {
            diagram.push_str(&format!(
                "Technique: {}\n",
                self.format_technique(&fingering.technique)
            ));
        }

        // Add string labels header if enabled
        if self.config.show_string_labels {
            diagram.push_str(&self.generate_string_labels_header(fretboard)?);
        }

        // Add the nut (if starting from fret 0) or position marker
        if min_fret == 0 {
            diagram.push_str(&self.generate_nut_line(string_count));
        } else {
            diagram.push_str(&self.generate_position_marker(string_count, min_fret));
        }

        // Generate fret lines with finger positions
        for fret in min_fret..=max_fret {
            diagram.push_str(&self.generate_fret_line(fretboard, fingering, fret, string_count)?);
        }

        // Add fret numbers if enabled
        if self.config.show_fret_numbers {
            diagram.push_str(&self.generate_fret_numbers(min_fret, max_fret));
        }

        Ok(diagram)
    }

    /// Calculate the appropriate fret range to display for a fingering
    fn calculate_fret_range(
        &self,
        fingering: &Fingering<StringedPosition>,
    ) -> FretboardResult<(usize, usize)> {
        if fingering.positions.is_empty() {
            return Ok((
                self.config.min_fret,
                self.config.min_fret + self.config.max_frets,
            ));
        }

        // Find the range of frets used in the fingering
        let fretted_positions: Vec<_> = fingering
            .positions
            .iter()
            .filter(|fp| fp.position.fret > 0)
            .collect();

        if fretted_positions.is_empty() {
            // All open strings
            return Ok((0, self.config.max_frets));
        }

        let min_used_fret = fretted_positions
            .iter()
            .map(|fp| fp.position.fret)
            .min()
            .unwrap();
        let max_used_fret = fretted_positions
            .iter()
            .map(|fp| fp.position.fret)
            .max()
            .unwrap();

        // Determine display range
        let min_fret = if min_used_fret <= 1 {
            0 // Show nut if we're in the first few frets
        } else {
            (min_used_fret.saturating_sub(1)).max(self.config.min_fret)
        };

        let max_fret = (max_used_fret + 1).min(min_fret + self.config.max_frets);

        Ok((min_fret, max_fret))
    }

    /// Generate string labels header
    fn generate_string_labels_header(
        &self,
        fretboard: &StringedFretboard,
    ) -> FretboardResult<String> {
        let mut header = String::new();

        // Add spacing for fret number column if enabled
        if self.config.show_fret_numbers {
            header.push_str("   ");
        }

        for string_num in 0..fretboard.string_count() {
            let tuning = fretboard.string_tuning(string_num).ok_or_else(|| {
                super::errors::FretboardError::InvalidPosition {
                    position: format!("String index {} out of range", string_num),
                }
            })?;
            header.push_str(&format!(" {} ", tuning.class()));
        }
        header.push('\n');

        Ok(header)
    }

    /// Generate the nut line (for fret 0)
    fn generate_nut_line(&self, string_count: usize) -> String {
        let mut line = String::new();

        // Add fret number if enabled
        if self.config.show_fret_numbers {
            line.push_str("   ");
        }

        // Generate nut representation
        for _ in 0..string_count {
            line.push_str("===");
        }
        line.push('\n');

        line
    }

    /// Generate position marker for higher frets
    fn generate_position_marker(&self, string_count: usize, fret: usize) -> String {
        let mut line = String::new();

        // Add position indicator
        if self.config.show_fret_numbers {
            line.push_str(&format!("{:2} ", fret));
        }

        for _ in 0..string_count {
            line.push_str(&format!(
                "{}{}{}",
                self.config.fret_char, self.config.fret_char, self.config.fret_char
            ));
        }
        line.push('\n');

        line
    }

    /// Generate a single fret line with finger positions
    fn generate_fret_line(
        &self,
        _fretboard: &StringedFretboard,
        fingering: &Fingering<StringedPosition>,
        fret: usize,
        string_count: usize,
    ) -> FretboardResult<String> {
        let mut line = String::new();

        // Add fret number if enabled (only for fretted positions)
        if self.config.show_fret_numbers && fret > 0 {
            line.push_str(&format!("{:2} ", fret));
        } else if self.config.show_fret_numbers {
            line.push_str("   ");
        }

        // Generate string positions
        for string_num in 0..string_count {
            let position = StringedPosition::new(string_num, fret);
            let symbol = self.get_position_symbol(fingering, &position);

            line.push(self.config.string_char);
            line.push(symbol);
            line.push(self.config.string_char);
        }
        line.push('\n');

        Ok(line)
    }

    /// Get the symbol to display at a specific position
    fn get_position_symbol(
        &self,
        fingering: &Fingering<StringedPosition>,
        position: &StringedPosition,
    ) -> char {
        // Check if this position is used in the fingering
        for finger_pos in &fingering.positions {
            if finger_pos.position == *position {
                if position.fret == 0 {
                    return self.config.open_char;
                } else if self.config.show_finger_numbers {
                    if let Some(finger) = &finger_pos.finger {
                        return self.finger_to_char(finger);
                    }
                }
                return self.config.finger_char;
            }
        }

        // Check if this string is muted (not used at all in the fingering)
        let string_used = fingering
            .positions
            .iter()
            .any(|fp| fp.position.string == position.string);

        if !string_used && position.fret == 0 {
            self.config.mute_char
        } else {
            ' '
        }
    }

    /// Convert finger enum to display character
    fn finger_to_char(&self, finger: &Finger) -> char {
        match finger {
            Finger::Thumb => 'T',
            Finger::Index => '1',
            Finger::Middle => '2',
            Finger::Ring => '3',
            Finger::Pinky => '4',
        }
    }

    /// Generate fret numbers footer
    fn generate_fret_numbers(&self, min_fret: usize, max_fret: usize) -> String {
        let mut line = String::new();

        // Add spacing for fret number column
        line.push_str("   ");

        // Add fret numbers
        for fret in min_fret..=max_fret {
            if fret == 0 {
                line.push_str("   ");
            } else {
                line.push_str(&format!(" {} ", fret));
            }
        }
        line.push('\n');

        line
    }

    /// Format playing technique for display
    fn format_technique(&self, technique: &PlayingTechnique) -> String {
        match technique {
            PlayingTechnique::Standard => "Standard".to_string(),
            PlayingTechnique::Barre {
                start_string,
                end_string,
                fret,
            } => {
                format!(
                    "Barre (strings {}-{}, fret {})",
                    start_string, end_string, fret
                )
            }
            PlayingTechnique::Hammer => "Hammer-on".to_string(),
            PlayingTechnique::Pull => "Pull-off".to_string(),
            PlayingTechnique::Slide => "Slide".to_string(),
            PlayingTechnique::Harmonic => "Harmonic".to_string(),
        }
    }

    /// Generate a compact diagram (single line representation)
    pub fn generate_compact_diagram(
        &self,
        fretboard: &StringedFretboard,
        fingering: &Fingering<StringedPosition>,
    ) -> FretboardResult<String> {
        let string_count = fretboard.string_count();
        let mut diagram = String::new();

        // Generate compact representation: X-3-2-0-1-0 (mute-fret3-fret2-open-fret1-open)
        for string_num in 0..string_count {
            if string_num > 0 {
                diagram.push('-');
            }

            // Find if this string is used in the fingering
            let finger_pos = fingering
                .positions
                .iter()
                .find(|fp| fp.position.string == string_num);

            match finger_pos {
                Some(fp) => {
                    if fp.position.fret == 0 {
                        diagram.push('0');
                    } else {
                        diagram.push_str(&fp.position.fret.to_string());
                    }
                }
                None => {
                    diagram.push('X'); // Muted string
                }
            }
        }

        Ok(diagram)
    }
}

impl Default for FretboardDiagramGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fret::presets::InstrumentPresets;
    use crate::fret::types::{FingerPosition, PlayingTechnique};
    use crate::{PitchClass, Tuning};

    #[test]
    fn test_diagram_generator_creation() {
        let generator = FretboardDiagramGenerator::new();
        assert!(generator.config.show_fret_numbers);
        assert!(generator.config.show_string_labels);
        assert!(generator.config.show_finger_numbers);

        let custom_config = DiagramConfig::new()
            .with_fret_numbers(false)
            .with_string_labels(false);
        let custom_generator = FretboardDiagramGenerator::with_config(custom_config);
        assert!(!custom_generator.config.show_fret_numbers);
        assert!(!custom_generator.config.show_string_labels);
    }

    #[test]
    fn test_simple_chord_diagram() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        // Create a simple C major chord fingering (simplified)
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

        let diagram = generator.generate_diagram(&fretboard, &fingering);
        assert!(diagram.is_ok());

        let diagram_text = diagram.unwrap();
        assert!(diagram_text.contains("E")); // String labels
        assert!(diagram_text.contains("1")); // Finger numbers
        assert!(diagram_text.contains("O")); // Open strings
        assert!(diagram_text.contains("X")); // Muted strings
    }

    #[test]
    fn test_compact_diagram() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        // Create a simple fingering
        let fingering = Fingering::new(
            vec![
                FingerPosition::open(StringedPosition::new(0, 0)), // High E open
                FingerPosition::pressed(StringedPosition::new(1, 1), Finger::Index), // B fret 1
                FingerPosition::open(StringedPosition::new(2, 0)), // G open
                FingerPosition::pressed(StringedPosition::new(3, 2), Finger::Middle), // D fret 2
                FingerPosition::pressed(StringedPosition::new(4, 3), Finger::Ring), // A fret 3
                                                                   // Low E muted (string 5)
            ],
            PlayingTechnique::Standard,
            0.3,
        );

        let compact = generator.generate_compact_diagram(&fretboard, &fingering);
        assert!(compact.is_ok());

        let compact_text = compact.unwrap();
        // Should be something like: 0-1-0-2-3-X (from high E to low E)
        assert!(compact_text.contains("0"));
        assert!(compact_text.contains("1"));
        assert!(compact_text.contains("2"));
        assert!(compact_text.contains("3"));
        assert!(compact_text.contains("X"));
        assert!(compact_text.contains("-"));
    }

    #[test]
    fn test_barre_chord_diagram() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        // Create a barre chord fingering
        let fingering = Fingering::new(
            vec![
                FingerPosition::pressed(StringedPosition::new(0, 1), Finger::Index), // High E fret 1
                FingerPosition::pressed(StringedPosition::new(1, 1), Finger::Index), // B fret 1
                FingerPosition::pressed(StringedPosition::new(2, 1), Finger::Index), // G fret 1
                FingerPosition::pressed(StringedPosition::new(3, 3), Finger::Ring),  // D fret 3
                FingerPosition::pressed(StringedPosition::new(4, 3), Finger::Ring),  // A fret 3
                FingerPosition::pressed(StringedPosition::new(5, 1), Finger::Index), // Low E fret 1
            ],
            PlayingTechnique::Barre {
                start_string: 0,
                end_string: 5,
                fret: 1,
            },
            0.7,
        );

        let diagram = generator.generate_diagram(&fretboard, &fingering);
        assert!(diagram.is_ok());

        let diagram_text = diagram.unwrap();
        assert!(diagram_text.contains("Barre")); // Technique description
        assert!(diagram_text.contains("1")); // Finger numbers
        assert!(diagram_text.contains("3")); // Finger numbers
    }

    #[test]
    fn test_fret_range_calculation() {
        let generator = FretboardDiagramGenerator::new();

        // Test with open strings only
        let open_fingering = Fingering::new(
            vec![FingerPosition::open(StringedPosition::new(0, 0))],
            PlayingTechnique::Standard,
            0.1,
        );
        let (min, max) = generator.calculate_fret_range(&open_fingering).unwrap();
        assert_eq!(min, 0);
        assert!(max >= 0);

        // Test with higher frets
        let high_fingering = Fingering::new(
            vec![
                FingerPosition::pressed(StringedPosition::new(0, 7), Finger::Index),
                FingerPosition::pressed(StringedPosition::new(1, 9), Finger::Ring),
            ],
            PlayingTechnique::Standard,
            0.5,
        );
        let (min, max) = generator.calculate_fret_range(&high_fingering).unwrap();
        assert!(min >= 6); // Should start before the lowest fret
        assert!(max >= 9); // Should include the highest fret
    }

    #[test]
    fn test_custom_config() {
        let config = DiagramConfig::new()
            .with_fret_numbers(false)
            .with_string_labels(false)
            .with_finger_numbers(false)
            .with_chars('=', ':', '@', 'o', 'x');

        assert!(!config.show_fret_numbers);
        assert!(!config.show_string_labels);
        assert!(!config.show_finger_numbers);
        assert_eq!(config.fret_char, '=');
        assert_eq!(config.string_char, ':');
        assert_eq!(config.finger_char, '@');
        assert_eq!(config.open_char, 'o');
        assert_eq!(config.mute_char, 'x');
    }
}

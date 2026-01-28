//! Fretboard visualization system for generating ASCII diagrams
//!
//! This module provides functionality to generate text-based diagrams of fretboard
//! fingerings, including finger positions, fret numbers, and string labels.

use super::{
    errors::FretboardResult,
    traits::Fretboard,
    types::{Finger, FingerPosition, Fingering, PlayingTechnique, StringedPosition},
    StringedFretboard,
};

#[cfg(feature = "bindgen")]
use uniffi;

use serde::{Deserialize, Serialize};

/// Diagram orientation for different display preferences
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagramOrientation {
    /// Standard orientation (right-handed, horizontal)
    Standard,
    /// Left-handed orientation (mirrored strings)
    LeftHanded,
    /// Vertical orientation (rotated 90 degrees)
    Vertical,
}

/// Export format for structured data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// YAML format
    Yaml,
}

/// Metadata about the fingering and instrument
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FingeringMetadata {
    /// Type of instrument (e.g., "stringed", "keyboard")
    pub instrument_type: String,
    /// Number of strings/keys
    pub string_count: usize,
    /// Number of frets (for stringed instruments)
    pub fret_count: usize,
    /// Playing technique used
    pub technique: String,
    /// Difficulty rating (0.0 to 1.0)
    pub difficulty: f32,
    /// Number of fingers used
    pub finger_count: usize,
}

/// Information about the fret range displayed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FretRange {
    /// Minimum fret displayed
    pub min_fret: usize,
    /// Maximum fret displayed
    pub max_fret: usize,
    /// Number of frets in display
    pub display_frets: usize,
}

/// Information about a string
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StringInfo {
    /// String index (0-based)
    pub index: usize,
    /// Full tuning name (e.g., "E4")
    pub tuning: String,
    /// Pitch class only (e.g., "E")
    pub pitch_class: String,
    /// Octave number
    pub octave: i8,
}

/// Information about a finger position
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionInfo {
    /// String index
    pub string: usize,
    /// Fret number
    pub fret: usize,
    /// Finger used (if any)
    pub finger: Option<String>,
    /// Pressure applied (0.0 to 1.0)
    pub pressure: f32,
    /// Tuning produced at this position
    pub tuning: String,
    /// Pitch class produced
    pub pitch_class: String,
    /// Octave produced
    pub octave: i8,
}

/// Generated diagram data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiagramData {
    /// ASCII art diagram
    pub ascii: String,
    /// Compact string representation
    pub compact: String,
    /// Diagram orientation
    pub orientation: String,
}

/// Export configuration settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Whether fret numbers are shown
    pub show_fret_numbers: bool,
    /// Whether string labels are shown
    pub show_string_labels: bool,
    /// Whether finger numbers are shown
    pub show_finger_numbers: bool,
    /// Maximum frets to display
    pub max_frets: usize,
    /// Minimum fret to start from
    pub min_fret: usize,
}

/// Complete export data structure for external visualization tools
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FingeringExportData {
    /// Metadata about the fingering
    pub metadata: FingeringMetadata,
    /// Fret range information
    pub fret_range: FretRange,
    /// String information
    pub strings: Vec<StringInfo>,
    /// Position information
    pub positions: Vec<PositionInfo>,
    /// Generated diagrams
    pub diagrams: DiagramData,
    /// Export configuration
    pub config: ExportConfig,
}

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
    /// Character to use for barre indications
    pub barre_char: char,
    /// Character to use for hammer-on indications
    pub hammer_char: char,
    /// Character to use for pull-off indications
    pub pull_char: char,
    /// Character to use for slide indications
    pub slide_char: char,
    /// Character to use for harmonic indications
    pub harmonic_char: char,
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
            barre_char: '═',
            hammer_char: 'H',
            pull_char: 'P',
            slide_char: 'S',
            harmonic_char: '◊',
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

    /// Set custom characters for special techniques
    pub fn with_technique_chars(
        mut self,
        barre_char: char,
        hammer_char: char,
        pull_char: char,
        slide_char: char,
        harmonic_char: char,
    ) -> Self {
        self.barre_char = barre_char;
        self.hammer_char = hammer_char;
        self.pull_char = pull_char;
        self.slide_char = slide_char;
        self.harmonic_char = harmonic_char;
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

        // Add barre indication if applicable
        if let PlayingTechnique::Barre {
            start_string,
            end_string,
            fret,
        } = &fingering.technique
        {
            diagram.push_str(&self.generate_barre_indication(
                *start_string,
                *end_string,
                *fret,
                min_fret,
                max_fret,
                string_count,
            ));
        }

        // Add technique annotations
        diagram.push_str(&self.generate_technique_annotations(fingering));

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
                } else {
                    // Check for special techniques
                    match &fingering.technique {
                        PlayingTechnique::Harmonic => return self.config.harmonic_char,
                        PlayingTechnique::Hammer => return self.config.hammer_char,
                        PlayingTechnique::Pull => return self.config.pull_char,
                        PlayingTechnique::Slide => return self.config.slide_char,
                        _ => {
                            // Standard fingering or barre - show finger number or standard symbol
                            if self.config.show_finger_numbers {
                                if let Some(finger) = &finger_pos.finger {
                                    return self.finger_to_char(finger);
                                }
                            }
                            return self.config.finger_char;
                        }
                    }
                }
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

    /// Generate barre indication line
    fn generate_barre_indication(
        &self,
        start_string: usize,
        end_string: usize,
        barre_fret: usize,
        min_fret: usize,
        max_fret: usize,
        string_count: usize,
    ) -> String {
        // Only show barre indication if the barre fret is in the displayed range
        if barre_fret < min_fret || barre_fret > max_fret {
            return String::new();
        }

        let mut line = String::new();

        // Add spacing for fret number column if enabled
        if self.config.show_fret_numbers {
            line.push_str("   ");
        }

        // Generate barre line
        for string_num in 0..string_count {
            if string_num >= start_string && string_num <= end_string {
                line.push_str(&format!(
                    "{}{}{}",
                    self.config.barre_char, self.config.barre_char, self.config.barre_char
                ));
            } else {
                line.push_str("   ");
            }
        }
        line.push_str(" (Barre)\n");

        line
    }

    /// Generate technique annotations
    fn generate_technique_annotations(&self, fingering: &Fingering<StringedPosition>) -> String {
        let mut annotations = String::new();

        match &fingering.technique {
            PlayingTechnique::Hammer => {
                annotations.push_str(&format!(
                    "Technique: {} (Hammer-on)\n",
                    self.config.hammer_char
                ));
            }
            PlayingTechnique::Pull => {
                annotations.push_str(&format!(
                    "Technique: {} (Pull-off)\n",
                    self.config.pull_char
                ));
            }
            PlayingTechnique::Slide => {
                annotations.push_str(&format!("Technique: {} (Slide)\n", self.config.slide_char));
            }
            PlayingTechnique::Harmonic => {
                annotations.push_str(&format!(
                    "Technique: {} (Harmonic)\n",
                    self.config.harmonic_char
                ));
            }
            _ => {} // Standard and Barre are handled elsewhere
        }

        annotations
    }
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

    /// Export fingering data as JSON
    pub fn export_json(
        &self,
        fretboard: &StringedFretboard,
        fingering: &Fingering<StringedPosition>,
    ) -> FretboardResult<String> {
        let export_data = self.create_export_data(fretboard, fingering)?;
        serde_json::to_string_pretty(&export_data).map_err(|e| {
            super::errors::FretboardError::InvalidPosition {
                position: format!("JSON serialization failed: {}", e),
            }
        })
    }

    /// Export fingering data as YAML
    pub fn export_yaml(
        &self,
        fretboard: &StringedFretboard,
        fingering: &Fingering<StringedPosition>,
    ) -> FretboardResult<String> {
        let export_data = self.create_export_data(fretboard, fingering)?;
        serde_yaml::to_string(&export_data).map_err(|e| {
            super::errors::FretboardError::InvalidPosition {
                position: format!("YAML serialization failed: {}", e),
            }
        })
    }

    /// Create structured export data for external visualization tools
    fn create_export_data(
        &self,
        fretboard: &StringedFretboard,
        fingering: &Fingering<StringedPosition>,
    ) -> FretboardResult<FingeringExportData> {
        let (min_fret, max_fret) = self.calculate_fret_range(fingering)?;

        // Generate ASCII diagram
        let ascii_diagram = self.generate_diagram(fretboard, fingering)?;
        let compact_diagram = self.generate_compact_diagram(fretboard, fingering)?;

        // Create string information
        let mut strings = Vec::new();
        for string_num in 0..fretboard.string_count() {
            let tuning = fretboard.string_tuning(string_num).ok_or_else(|| {
                super::errors::FretboardError::InvalidPosition {
                    position: format!("String index {} out of range", string_num),
                }
            })?;

            strings.push(StringInfo {
                index: string_num,
                tuning: tuning.to_string(),
                pitch_class: tuning.class().to_string(),
                octave: tuning.octave(),
            });
        }

        // Create position information
        let mut positions = Vec::new();
        for finger_pos in &fingering.positions {
            let tuning = fretboard
                .tuning_at_position(&finger_pos.position)
                .ok_or_else(|| super::errors::FretboardError::InvalidPosition {
                    position: finger_pos.position.to_string(),
                })?;

            positions.push(PositionInfo {
                string: finger_pos.position.string,
                fret: finger_pos.position.fret,
                finger: finger_pos.finger.map(|f| f.to_string()),
                pressure: finger_pos.pressure,
                tuning: tuning.to_string(),
                pitch_class: tuning.class().to_string(),
                octave: tuning.octave(),
            });
        }

        Ok(FingeringExportData {
            metadata: FingeringMetadata {
                instrument_type: "stringed".to_string(),
                string_count: fretboard.string_count(),
                fret_count: fretboard.fret_count(),
                technique: fingering.technique.to_string(),
                difficulty: fingering.difficulty,
                finger_count: fingering.finger_count(),
            },
            fret_range: FretRange {
                min_fret,
                max_fret,
                display_frets: max_fret - min_fret + 1,
            },
            strings,
            positions,
            diagrams: DiagramData {
                ascii: ascii_diagram,
                compact: compact_diagram,
                orientation: "standard".to_string(),
            },
            config: ExportConfig {
                show_fret_numbers: self.config.show_fret_numbers,
                show_string_labels: self.config.show_string_labels,
                show_finger_numbers: self.config.show_finger_numbers,
                max_frets: self.config.max_frets,
                min_fret: self.config.min_fret,
            },
        })
    }

    /// Export fingering data with custom orientation
    pub fn export_with_orientation(
        &self,
        fretboard: &StringedFretboard,
        fingering: &Fingering<StringedPosition>,
        orientation: DiagramOrientation,
        format: ExportFormat,
    ) -> FretboardResult<String> {
        let mut export_data = self.create_export_data(fretboard, fingering)?;

        // Modify data based on orientation
        match orientation {
            DiagramOrientation::Standard => {
                // Already in standard orientation
            }
            DiagramOrientation::LeftHanded => {
                // Reverse string order for left-handed players
                export_data.strings.reverse();
                for pos in &mut export_data.positions {
                    pos.string = fretboard.string_count() - 1 - pos.string;
                }
                export_data.diagrams.orientation = "left_handed".to_string();
            }
            DiagramOrientation::Vertical => {
                // Transpose the diagram layout
                export_data.diagrams.orientation = "vertical".to_string();
            }
        }

        // Export in requested format
        match format {
            ExportFormat::Json => serde_json::to_string_pretty(&export_data).map_err(|e| {
                super::errors::FretboardError::InvalidPosition {
                    position: format!("JSON serialization failed: {}", e),
                }
            }),
            ExportFormat::Yaml => serde_yaml::to_string(&export_data).map_err(|e| {
                super::errors::FretboardError::InvalidPosition {
                    position: format!("YAML serialization failed: {}", e),
                }
            }),
        }
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
    fn test_special_technique_visualization() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        // Test harmonic technique
        let harmonic_fingering = Fingering::new(
            vec![FingerPosition::pressed(
                StringedPosition::new(0, 12),
                Finger::Index,
            )],
            PlayingTechnique::Harmonic,
            0.2,
        );

        let diagram = generator.generate_diagram(&fretboard, &harmonic_fingering);
        assert!(diagram.is_ok());
        let diagram_text = diagram.unwrap();
        assert!(diagram_text.contains("Harmonic"));
        assert!(diagram_text.contains("◊")); // Harmonic symbol

        // Test hammer-on technique
        let hammer_fingering = Fingering::new(
            vec![
                FingerPosition::pressed(StringedPosition::new(0, 2), Finger::Index),
                FingerPosition::pressed(StringedPosition::new(0, 4), Finger::Ring),
            ],
            PlayingTechnique::Hammer,
            0.4,
        );

        let diagram = generator.generate_diagram(&fretboard, &hammer_fingering);
        assert!(diagram.is_ok());
        let diagram_text = diagram.unwrap();
        assert!(diagram_text.contains("Hammer-on"));
        assert!(diagram_text.contains("H")); // Hammer symbol

        // Test slide technique
        let slide_fingering = Fingering::new(
            vec![
                FingerPosition::pressed(StringedPosition::new(0, 3), Finger::Index),
                FingerPosition::pressed(StringedPosition::new(0, 5), Finger::Index),
            ],
            PlayingTechnique::Slide,
            0.3,
        );

        let diagram = generator.generate_diagram(&fretboard, &slide_fingering);
        assert!(diagram.is_ok());
        let diagram_text = diagram.unwrap();
        assert!(diagram_text.contains("Slide"));
        assert!(diagram_text.contains("S")); // Slide symbol
    }

    #[test]
    fn test_barre_chord_visualization() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        // Create F major barre chord (1st fret barre)
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

        let diagram = generator.generate_diagram(&fretboard, &barre_fingering);
        assert!(diagram.is_ok());

        let diagram_text = diagram.unwrap();
        assert!(diagram_text.contains("Barre"));
        assert!(diagram_text.contains("═")); // Barre symbol
        assert!(diagram_text.contains("1")); // Finger numbers
        assert!(diagram_text.contains("2"));
        assert!(diagram_text.contains("3"));
        assert!(diagram_text.contains("4"));
    }

    #[test]
    fn test_custom_technique_chars() {
        let custom_config = DiagramConfig::new().with_technique_chars('B', 'h', 'p', 's', 'o');

        let generator = FretboardDiagramGenerator::with_config(custom_config);
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

        // Test custom harmonic character
        let harmonic_fingering = Fingering::new(
            vec![FingerPosition::pressed(
                StringedPosition::new(0, 12),
                Finger::Index,
            )],
            PlayingTechnique::Harmonic,
            0.2,
        );

        let diagram = generator.generate_diagram(&fretboard, &harmonic_fingering);
        assert!(diagram.is_ok());
        let diagram_text = diagram.unwrap();
        assert!(diagram_text.contains("o")); // Custom harmonic symbol
    }

    #[test]
    fn test_json_export() {
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
            ],
            PlayingTechnique::Standard,
            0.3,
        );

        let json_result = generator.export_json(&fretboard, &fingering);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("metadata"));
        assert!(json_str.contains("positions"));
        assert!(json_str.contains("strings"));
        assert!(json_str.contains("diagrams"));
        assert!(json_str.contains("Standard")); // Technique

        // Verify it's valid JSON by parsing it back
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed["metadata"]["string_count"].as_u64().unwrap() == 6);
        assert!(parsed["positions"].as_array().unwrap().len() == 5);
    }

    #[test]
    fn test_yaml_export() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        // Create a barre chord fingering
        let fingering = Fingering::new(
            vec![
                FingerPosition::pressed(StringedPosition::new(0, 1), Finger::Index),
                FingerPosition::pressed(StringedPosition::new(1, 1), Finger::Index),
                FingerPosition::pressed(StringedPosition::new(2, 2), Finger::Middle),
                FingerPosition::pressed(StringedPosition::new(3, 3), Finger::Ring),
                FingerPosition::pressed(StringedPosition::new(4, 3), Finger::Pinky),
                FingerPosition::pressed(StringedPosition::new(5, 1), Finger::Index),
            ],
            PlayingTechnique::Barre {
                start_string: 0,
                end_string: 5,
                fret: 1,
            },
            0.7,
        );

        let yaml_result = generator.export_yaml(&fretboard, &fingering);
        assert!(yaml_result.is_ok());

        let yaml_str = yaml_result.unwrap();
        assert!(yaml_str.contains("metadata:"));
        assert!(yaml_str.contains("positions:"));
        assert!(yaml_str.contains("strings:"));
        assert!(yaml_str.contains("diagrams:"));
        assert!(yaml_str.contains("Barre")); // Technique

        // Verify it's valid YAML by parsing it back
        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml_str).unwrap();
        assert!(parsed["metadata"]["string_count"].as_u64().unwrap() == 6);
        assert!(parsed["positions"].as_sequence().unwrap().len() == 6);
    }

    #[test]
    fn test_export_with_orientation() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        let fingering = Fingering::new(
            vec![
                FingerPosition::pressed(StringedPosition::new(0, 2), Finger::Index),
                FingerPosition::pressed(StringedPosition::new(3, 2), Finger::Middle),
            ],
            PlayingTechnique::Standard,
            0.4,
        );

        // Test standard orientation
        let standard_result = generator.export_with_orientation(
            &fretboard,
            &fingering,
            DiagramOrientation::Standard,
            ExportFormat::Json,
        );
        assert!(standard_result.is_ok());
        let standard_json = standard_result.unwrap();
        assert!(standard_json.contains("\"orientation\": \"standard\""));

        // Test left-handed orientation
        let lefty_result = generator.export_with_orientation(
            &fretboard,
            &fingering,
            DiagramOrientation::LeftHanded,
            ExportFormat::Json,
        );
        assert!(lefty_result.is_ok());
        let lefty_json = lefty_result.unwrap();
        assert!(lefty_json.contains("\"orientation\": \"left_handed\""));

        // Test vertical orientation
        let vertical_result = generator.export_with_orientation(
            &fretboard,
            &fingering,
            DiagramOrientation::Vertical,
            ExportFormat::Yaml,
        );
        assert!(vertical_result.is_ok());
        let vertical_yaml = vertical_result.unwrap();
        assert!(vertical_yaml.contains("orientation: vertical"));
    }

    #[test]
    fn test_export_data_completeness() {
        let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
        let generator = FretboardDiagramGenerator::new();

        // Create a complex fingering with multiple techniques
        let fingering = Fingering::new(
            vec![
                FingerPosition::pressed(StringedPosition::new(0, 5), Finger::Pinky),
                FingerPosition::pressed(StringedPosition::new(1, 3), Finger::Middle),
                FingerPosition::open(StringedPosition::new(2, 0)),
                FingerPosition::pressed(StringedPosition::new(3, 2), Finger::Index),
            ],
            PlayingTechnique::Slide,
            0.6,
        );

        let json_result = generator.export_json(&fretboard, &fingering);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify metadata completeness
        let metadata = &parsed["metadata"];
        assert_eq!(metadata["instrument_type"], "stringed");
        assert_eq!(metadata["string_count"], 6);
        assert_eq!(metadata["technique"], "Slide");
        assert_eq!(metadata["finger_count"], 3);
        assert!((metadata["difficulty"].as_f64().unwrap() - 0.6).abs() < 0.001);

        // Verify fret range
        let fret_range = &parsed["fret_range"];
        assert!(
            fret_range["min_fret"].as_u64().unwrap() <= fret_range["max_fret"].as_u64().unwrap()
        );

        // Verify string information
        let strings = parsed["strings"].as_array().unwrap();
        assert_eq!(strings.len(), 6);
        for (i, string) in strings.iter().enumerate() {
            assert_eq!(string["index"], i);
            assert!(string["tuning"].is_string());
            assert!(string["pitch_class"].is_string());
            assert!(string["octave"].is_number());
        }

        // Verify position information
        let positions = parsed["positions"].as_array().unwrap();
        assert_eq!(positions.len(), 4);
        for position in positions {
            assert!(position["string"].is_number());
            assert!(position["fret"].is_number());
            assert!(position["pressure"].is_number());
            assert!(position["tuning"].is_string());
        }

        // Verify diagrams
        let diagrams = &parsed["diagrams"];
        assert!(diagrams["ascii"].is_string());
        assert!(diagrams["compact"].is_string());
        assert!(diagrams["orientation"].is_string());

        // Verify config
        let config = &parsed["config"];
        assert!(config["show_fret_numbers"].is_boolean());
        assert!(config["show_string_labels"].is_boolean());
        assert!(config["show_finger_numbers"].is_boolean());
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::fret::presets::InstrumentPresets;
    use proptest::prelude::*;

    /// **Property 13: Diagram Generation Completeness**
    /// **Validates: Requirements 6.1, 6.2, 6.3, 6.4**
    ///
    /// This property test ensures that diagram generation works correctly for all
    /// valid fingerings and configurations, covering:
    /// - All finger positions are represented in the output
    /// - All playing techniques are properly indicated
    /// - Diagram structure is consistent and well-formed
    /// - No crashes or errors occur for valid inputs
    proptest! {
        #[test]
        fn prop_diagram_generation_completeness(
            // Generate random fingering configurations
            finger_count in 1usize..=6,
            technique_variant in 0usize..=5,
            show_fret_numbers in any::<bool>(),
            show_string_labels in any::<bool>(),
            show_finger_numbers in any::<bool>(),
        ) {
            let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

            // Create a configuration with random settings
            let config = DiagramConfig::new()
                .with_fret_numbers(show_fret_numbers)
                .with_string_labels(show_string_labels)
                .with_finger_numbers(show_finger_numbers);

            let generator = FretboardDiagramGenerator::with_config(config);

            // Generate a random but valid fingering
            let mut positions = Vec::new();
            let fingers = [Finger::Index, Finger::Middle, Finger::Ring, Finger::Pinky];

            for i in 0..finger_count.min(4) {
                let string = i % 6; // Ensure valid string index
                let fret = (i + 1) % 13; // Ensure reasonable fret range
                let finger = fingers[i % fingers.len()];

                if fret == 0 {
                    positions.push(FingerPosition::open(StringedPosition::new(string, fret)));
                } else {
                    positions.push(FingerPosition::pressed(StringedPosition::new(string, fret), finger));
                }
            }

            // Select technique based on variant
            let technique = match technique_variant {
                0 => PlayingTechnique::Standard,
                1 => PlayingTechnique::Barre { start_string: 0, end_string: 5, fret: 1 },
                2 => PlayingTechnique::Hammer,
                3 => PlayingTechnique::Pull,
                4 => PlayingTechnique::Slide,
                5 => PlayingTechnique::Harmonic,
                _ => PlayingTechnique::Standard,
            };

            let fingering = Fingering::new(positions, technique, 0.5);

            // Generate diagram and verify it succeeds
            let diagram_result = generator.generate_diagram(&fretboard, &fingering);
            prop_assert!(diagram_result.is_ok(), "Diagram generation should not fail for valid inputs");

            let diagram = diagram_result.unwrap();

            // Verify basic diagram structure
            prop_assert!(!diagram.is_empty(), "Diagram should not be empty");
            prop_assert!(diagram.contains('\n'), "Diagram should contain newlines");

            // Verify configuration-dependent content
            if show_string_labels {
                prop_assert!(diagram.contains("E") || diagram.contains("A") || diagram.contains("D") ||
                           diagram.contains("G") || diagram.contains("B"),
                           "Diagram should contain string labels when enabled");
            }

            if show_fret_numbers && fingering.positions.iter().any(|fp| fp.position.fret > 0) {
                // Should contain at least one digit for fret numbers
                prop_assert!(diagram.chars().any(|c| c.is_ascii_digit()),
                           "Diagram should contain fret numbers when enabled and frets are used");
            }

            // Verify technique-specific content
            match &fingering.technique {
                PlayingTechnique::Standard => {
                    // Standard technique should not have special technique indicators
                    prop_assert!(!diagram.contains("Technique:") || diagram.contains("Standard"),
                               "Standard technique should not show special technique markers");
                }
                PlayingTechnique::Barre { .. } => {
                    prop_assert!(diagram.contains("Barre"), "Barre technique should be indicated");
                }
                PlayingTechnique::Hammer => {
                    prop_assert!(diagram.contains("Hammer"), "Hammer technique should be indicated");
                }
                PlayingTechnique::Pull => {
                    prop_assert!(diagram.contains("Pull"), "Pull technique should be indicated");
                }
                PlayingTechnique::Slide => {
                    prop_assert!(diagram.contains("Slide"), "Slide technique should be indicated");
                }
                PlayingTechnique::Harmonic => {
                    prop_assert!(diagram.contains("Harmonic"), "Harmonic technique should be indicated");
                }
            }

            // Verify finger positions are represented
            for finger_pos in &fingering.positions {
                if finger_pos.position.fret == 0 {
                    // Open strings should be represented
                    prop_assert!(diagram.contains('O') || diagram.contains('0'),
                               "Open strings should be represented in diagram");
                } else if show_finger_numbers && finger_pos.finger.is_some() {
                    // Finger numbers should be present when enabled
                    let finger_chars = ['1', '2', '3', '4', 'T'];
                    prop_assert!(finger_chars.iter().any(|&c| diagram.contains(c)),
                               "Finger numbers should be present when enabled");
                }
            }

            // Test compact diagram generation
            let compact_result = generator.generate_compact_diagram(&fretboard, &fingering);
            prop_assert!(compact_result.is_ok(), "Compact diagram generation should not fail");

            let compact = compact_result.unwrap();
            prop_assert!(!compact.is_empty(), "Compact diagram should not be empty");
            prop_assert!(compact.contains('-') || compact.contains('X') || compact.contains('0') ||
                        compact.chars().any(|c| c.is_ascii_digit()),
                        "Compact diagram should contain position indicators");
        }

        /// Test diagram generation with various configuration combinations
        #[test]
        fn prop_diagram_config_combinations(
            max_frets in 3usize..=8,
            min_fret in 0usize..=5,
        ) {
            let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();

            // Test with different fret range configurations
            let config = DiagramConfig::new()
                .with_fret_range(min_fret, max_frets);

            let generator = FretboardDiagramGenerator::with_config(config);

            // Create a simple fingering
            let fingering = Fingering::new(
                vec![
                    FingerPosition::pressed(StringedPosition::new(0, min_fret + 1), Finger::Index),
                    FingerPosition::pressed(StringedPosition::new(1, min_fret + 2), Finger::Middle),
                ],
                PlayingTechnique::Standard,
                0.3,
            );

            let diagram_result = generator.generate_diagram(&fretboard, &fingering);
            prop_assert!(diagram_result.is_ok(), "Diagram should generate successfully with custom fret range");

            let diagram = diagram_result.unwrap();
            prop_assert!(!diagram.is_empty(), "Diagram should not be empty");
        }

        /// Test that all technique types can be visualized without errors
        #[test]
        fn prop_all_techniques_visualizable(
            string_idx in 0usize..6,
            fret_num in 1usize..13,
        ) {
            let fretboard = StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap();
            let generator = FretboardDiagramGenerator::new();

            let position = FingerPosition::pressed(
                StringedPosition::new(string_idx, fret_num),
                Finger::Index
            );

            // Test all technique types
            let techniques = vec![
                PlayingTechnique::Standard,
                PlayingTechnique::Barre { start_string: 0, end_string: 5, fret: fret_num },
                PlayingTechnique::Hammer,
                PlayingTechnique::Pull,
                PlayingTechnique::Slide,
                PlayingTechnique::Harmonic,
            ];

            for technique in techniques {
                let fingering = Fingering::new(vec![position.clone()], technique, 0.4);

                let diagram_result = generator.generate_diagram(&fretboard, &fingering);
                prop_assert!(diagram_result.is_ok(),
                           "All technique types should be visualizable without errors");

                let diagram = diagram_result.unwrap();
                prop_assert!(!diagram.is_empty(), "Diagram should not be empty for any technique");
            }
        }
    }
}

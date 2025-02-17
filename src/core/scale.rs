//! Scale System Module
//! Provides core functions such as scale generation, modal analysis, scale and chord derivation, and more!

use crate::interval::Interval;
use crate::tuning::{PitchClass, Tuning};
use crate::MusicError;
use std::convert::TryFrom;

/// Scale type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    // Basic scale
    /// Natural Major
    /// - 自然大调
    Major,
    /// Natural Minor
    /// - 自然小调
    NaturalMinor,
    /// HarmonicMinor
    /// - 和声小调
    HarmonicMinor,
    /// melodic minor (upward)
    /// - 旋律小调（上行）
    MelodicMinor,
    /// Dorian mode
    /// - 多利亚调式
    Dorian,
    /// Phrygian mode
    /// - 弗里几亚调式
    Phrygian,
    /// Lydian mode
    /// - 利底亚调式
    Lydian,
    /// Mixed Lydian mode
    /// - 混合利底亚调式
    Mixolydian,
    /// Locrian mode
    /// - 洛克里亚调式
    Locrian,

    // Pentatonic scale
    /// Major Pentatonic
    /// - 大调五声音阶
    PentatonicMajor,
    /// Minor Pentatonic
    /// - 小调五声音阶
    PentatonicMinor,
    /// Blues scale
    /// - 蓝调音阶
    Blues,

    // Special scale
    /// Whole Tone
    /// - 全音阶
    WholeTone,
    /// Octatonic
    /// - 八声音阶（减音阶）
    Octatonic,
    /// Chromatic
    /// - 半音阶
    Chromatic,
    /// Bebop Dominant
    /// - 比波普属音阶
    BebopDominant,

    // National scale
    /// Arabian Hijaz
    /// - 阿拉伯希贾兹音阶
    Hijaz,
    /// Japanese Hirajoshi
    /// - 日本平调子
    Hirajoshi,
    /// Japanese InSen
    /// - 日本阴旋
    InSen,

    // Custom scales
    /// Custom scale
    /// - 自定义音程模式
    Custom(&'static [i8]),
}

/// Scale System
#[derive(Debug, Clone)]
pub struct Scale {
    root: Tuning,
    scale_type: ScaleType,
    intervals: Vec<Interval>, // Scale intervals
    notes: Vec<Tuning>,       // Cache the generated notes
}

impl Scale {
    /// Create a new scale
    pub fn new(root: Tuning, scale_type: ScaleType) -> Result<Self, MusicError> {
        let intervals = Self::get_intervals(scale_type)?;
        let notes = Self::generate_notes(&root, &intervals, 3)?;

        Ok(Self {
            root,
            scale_type,
            intervals,
            notes,
        })
    }

    /// Generating note sequence
    fn generate_notes(
        root: &Tuning,
        intervals: &[Interval],
        octaves: u8,
    ) -> Result<Vec<Tuning>, MusicError> {
        let mut current = root.clone();
        let mut notes = vec![current.clone()];

        // Generate basic scales
        for interval in intervals {
            current = current.add_interval(interval);
            notes.push(current.clone());
        }

        // Extended octave
        let base_len = notes.len();
        for octave in 1..=octaves {
            for i in 0..base_len {
                let mut note = notes[i].clone();
                note.octave += octave as i8;
                notes.push(note);
            }
        }

        Ok(notes)
    }

    /// Determining whether a pitch belongs to a scale
    pub fn contains(&self, tuning: &Tuning) -> bool {
        self.notes.iter().any(|n| n.class == tuning.class)
    }

    /// Getting the Scale Degree
    /// - Get the Tuning by order
    /// - Such as in Pentatonic scale, the scale only has five notes. In major `C` the tuning is like:
    ///     - 1 -> C
    ///     - 2 -> D
    ///     - 3 -> E
    ///     - 4 -> G
    ///     - 5 -> A
    pub fn degree(&self, degree: u8) -> Result<Tuning, MusicError> {
        if degree < 1 {
            return Err(MusicError::InvalidScaleDegree(degree));
        }
        let idx = (degree - 1) as usize % self.intervals.len();
        // TODO: Dealing with a pentatonic scale where there are only five notes but the scales are not continuous
        self.notes
            .get(idx)
            .cloned()
            .ok_or(MusicError::InvalidScaleDegree(degree))
    }
}

/// # Interval pattern library
/// ## Standard scale patterns
/// - Major scale: [2, 2, 1, 2, 2, 2, 1]
/// - Natural minor scale: [2, 1, 2, 2, 1, 2, 2]
/// - Harmonic minor scale: [2, 1, 2, 2, 1, 3, 1]
/// - Melodic minor scale: [2, 1, 2, 2, 2, 2, 1]
///
/// ## Mediaeval mode
/// - Dorian mode: [2, 1, 2, 2, 2, 1, 2]
/// - Phrygian mode: [1, 2, 2, 2, 1, 2, 2]
/// - Lydian mode: [2, 2, 2, 1, 2, 2, 1]
/// - Mixolydian mode: [2, 2, 1, 2, 2, 1, 2]
/// - Locrian mode: [1, 2, 2, 1, 2, 2, 2]
///
/// ## Pentatonic scale
/// - Major pentatonic scale: [2, 2, 3, 2, 3]
/// - Minor pentatonic scale: [3, 2, 2, 3, 2]
/// - Blues scale: [3, 2, 1, 1, 3, 2]
///
/// ## Special scales
/// - Whole tone scale: [2, 2, 2, 2, 2, 2]
/// - Octatonic scale: [2, 1, 2, 1, 2, 1, 2, 1]
/// - Chromatic scale: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
/// - Bebop dominant scale: [2, 2, 1, 2, 2, 1, 1, 2]
///
/// ## National scales
/// - Arabian Hijaz scale: [1, 3, 1, 2, 1, 3, 1]
/// - Japanese Hirajoshi scale: [2, 1, 4, 1, 4]
/// - Japanese InSen scale: [1, 4, 2, 3, 2]
/// - Custom scale: [2, 1, 3, 1, 4]
impl Scale {
    /// Gets the standard interval pattern of the scale
    fn get_intervals(scale_type: ScaleType) -> Result<Vec<Interval>, MusicError> {
        match scale_type {
            // Natural scales
            ScaleType::Major => parse_intervals(&[2, 2, 1, 2, 2, 2, 1]),
            ScaleType::NaturalMinor => parse_intervals(&[2, 1, 2, 2, 1, 2, 2]),
            ScaleType::HarmonicMinor => parse_intervals(&[2, 1, 2, 2, 1, 3, 1]),
            ScaleType::MelodicMinor => parse_intervals(&[2, 1, 2, 2, 2, 2, 1]),

            // Mediaeval mode
            ScaleType::Dorian => parse_intervals(&[2, 1, 2, 2, 2, 1, 2]),
            ScaleType::Phrygian => parse_intervals(&[1, 2, 2, 2, 1, 2, 2]),
            ScaleType::Lydian => parse_intervals(&[2, 2, 2, 1, 2, 2, 1]),
            ScaleType::Mixolydian => parse_intervals(&[2, 2, 1, 2, 2, 1, 2]),
            ScaleType::Locrian => parse_intervals(&[1, 2, 2, 1, 2, 2, 2]),

            // Pentatonic scale
            ScaleType::PentatonicMajor => parse_intervals(&[2, 2, 3, 2, 3]),
            ScaleType::PentatonicMinor => parse_intervals(&[3, 2, 2, 3, 2]),
            ScaleType::Blues => parse_intervals(&[3, 2, 1, 1, 3, 2]),

            // Special scales
            ScaleType::WholeTone => parse_intervals(&[2, 2, 2, 2, 2, 2]),
            ScaleType::Octatonic => parse_intervals(&[2, 1, 2, 1, 2, 1, 2, 1]),
            ScaleType::Chromatic => parse_intervals(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            ScaleType::BebopDominant => parse_intervals(&[2, 2, 1, 2, 2, 1, 1, 2]),

            // National scales
            ScaleType::Hijaz => parse_intervals(&[1, 3, 1, 2, 1, 3, 1]),
            ScaleType::Hirajoshi => parse_intervals(&[2, 1, 4, 1, 4]),
            ScaleType::InSen => parse_intervals(&[1, 4, 2, 3, 2]),

            ScaleType::Custom(pattern) => {
                let semitones = pattern
                    .iter()
                    .map(|&s| Interval::from_semitones(s as i8))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(semitones)
            }
        }
    }
}

/// Converts semitones to a list of intervals
fn parse_intervals(semitones: &[i8]) -> Result<Vec<Interval>, MusicError> {
    semitones
        .iter()
        .map(|&s| Interval::from_semitones(s))
        .collect()
}

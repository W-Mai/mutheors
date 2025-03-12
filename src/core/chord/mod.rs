//! Chord system module
//! It includes core functions such as chord construction, analysis, inversion and voice arrangement

use crate::interval::{Interval, IntervalQuality};
use crate::tuning::Tuning;
use crate::MusicError;
use std::fmt::Display;
use std::str::FromStr;

/// Chord quality classification (basic triad)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordType {
    /// Triad
    Triad,
    /// Seventh chord
    Seventh,
    /// Extended chord (9th, 11th, 13th)
    Extended(u8),
    /// Suspended chord (sus2, sus4)
    Suspended(u8),
    /// Power chord
    Power,
    /// Altered chord
    Altered,
    /// Custom chord
    Custom,
}

/// Chord voicing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Voicing {
    // Dense arrangement (notes within an octave)
    ClosePosition,
    /// Open arrangement (notes across octaves)
    OpenPosition,
    /// Drop 2
    Drop2,
    /// Drop 3
    Drop3,
    /// Cluster (notes within a minor second)
    Cluster,
}

/// Chord inversion state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Inversion {
    /// Root position
    RootPosition,
    /// First inversion
    First,
    /// Second inversion
    Second,
    /// Third inversion (Seventh chord)
    Third,
}

/// Complete Chord Description Structure
#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    root: Tuning,
    quality: ChordQuality,
    intervals: Vec<Interval>,
    chord_type: ChordType,
    inversion: Inversion,
    voicing: Voicing,
    extensions: Vec<Interval>, // Extended sounds (9th, 11th, etc.)
}

impl Chord {
    pub fn quality(&self) -> ChordQuality {
        self.quality
    }

    pub fn root(&self) -> Tuning {
        self.root
    }
}

impl Chord {
    fn new(
        tuning: Tuning,
        intervals: Vec<Interval>,
        chord_type: ChordType,
        chord_quality: ChordQuality,
    ) -> Chord {
        Self {
            root: tuning,
            quality: chord_quality,
            intervals,
            chord_type,
            inversion: Inversion::RootPosition,
            voicing: Voicing::ClosePosition,
            extensions: Vec::new(),
        }
    }

    /// Constructive triad (musical chord)
    pub fn triad(root: Tuning, quality: ChordQuality) -> Result<Self, MusicError> {
        let intervals = match quality {
            ChordQuality::Major => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3)?,
                Interval::from_quality_degree(IntervalQuality::Perfect, 5)?,
            ],
            ChordQuality::Minor => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3)?,
                Interval::from_quality_degree(IntervalQuality::Perfect, 5)?,
            ],
            ChordQuality::Diminished => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3)?,
                Interval::from_quality_degree(IntervalQuality::Diminished, 5)?,
            ],
            ChordQuality::Augmented => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3)?,
                Interval::from_quality_degree(IntervalQuality::Augmented, 5)?,
            ],
            _ => return Err(MusicError::TheoryViolation("Invalid triad".to_owned())),
        };

        Ok(Self::new(root, intervals, ChordType::Triad, quality))
    }

    /// Construct seventh chord
    pub fn seventh(root: Tuning, quality: ChordQuality) -> Result<Self, MusicError> {
        let mut base = Self::triad(root, quality.base_quality())?.with_extension(match quality {
            ChordQuality::Major7 | ChordQuality::MinorMajor7 => {
                Interval::from_quality_degree(IntervalQuality::Major, 7)?
            }
            ChordQuality::Dominant7 | ChordQuality::HalfDiminished | ChordQuality::Minor7 => {
                Interval::from_quality_degree(IntervalQuality::Minor, 7)?
            }
            ChordQuality::FullyDiminished => {
                Interval::from_quality_degree(IntervalQuality::Diminished, 7)?
            }
            _ => return Err(MusicError::TheoryViolation("Invalid 7th chord".to_owned())),
        });
        base.chord_type = ChordType::Seventh;
        Ok(base)
    }

    /// Adding Extended interval
    pub fn with_extension(mut self, interval: Interval) -> Self {
        self.extensions.push(interval);
        self
    }

    /// TODO: Chord inversion
    pub fn invert(&mut self, inversion: Inversion) {
        self.inversion = inversion;
    }

    /// TODO: Rearrangement of voices
    pub fn revoice(&mut self, voicing: Voicing) {
        self.voicing = voicing;
    }

    /// Getting Chord composition tones
    pub fn components(&self) -> Vec<Tuning> {
        let mut notes = vec![self.root];

        // Adding basic intervals
        for interval in &self.intervals {
            let current = self.root.add_interval(interval);
            notes.push(current);
        }

        // Adding Extended Tones
        for ext in &self.extensions {
            let current = self.root.add_interval(ext);
            notes.push(current);
        }

        self.apply_voicing(&mut notes);
        notes
    }

    // TODO: Analyzing chord functions (TSD function system)
    // pub fn function(&self, key: Tuning) -> ChordFunction {
    //     // Implementing tonal analysis logic
    //     // ...
    // }

    // TODO: Parsing from chord symbols (e.g. " Cmaj7")
    // pub fn from_symbol(symbol: &str) -> Result<Self, MusicError> {
    //     // Implementing a chord symbol parser
    //     // ...
    // }

    // TODO: Generating arpeggios
    // pub fn arpeggio(&self, style: ArpeggioStyle) -> Vec<Note> {
    //     // Realization of different arpeggio patterns
    //     // ...
    // }
}

/// Classification of chord masses (basic triads)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Major7,
    Dominant7,
    Minor7,
    MinorMajor7,
    HalfDiminished,
    FullyDiminished,
}

/// Functional classification of chords (tonal analysis)
#[derive(Debug, PartialEq)]
pub enum ChordFunction {
    Tonic,
    Subdominant,
    Dominant,
    SecondaryDominant,
    Neapolitan,
    //... Other Functional Categories
}

impl ChordQuality {
    pub fn base_quality(&self) -> ChordQuality {
        match self {
            ChordQuality::Major7 => ChordQuality::Major,
            ChordQuality::Dominant7 => ChordQuality::Major,
            ChordQuality::Minor7 => ChordQuality::Minor,
            ChordQuality::MinorMajor7 => ChordQuality::Minor,
            ChordQuality::HalfDiminished => ChordQuality::Diminished,
            ChordQuality::FullyDiminished => ChordQuality::Diminished,
            _ => *self,
        }
    }
}

impl Chord {
    // Applying the rules of vocal arrangement
    fn apply_voicing(&self, notes: &mut Vec<Tuning>) {
        match self.voicing {
            Voicing::ClosePosition => self.close_voicing(notes),
            Voicing::OpenPosition => self.open_voicing(notes),
            Voicing::Drop2 | Voicing::Drop3 | Voicing::Cluster => todo!(),
            // ...
        }
    }

    /// Close arrangement algorithm
    fn close_voicing(&self, notes: &mut Vec<Tuning>) {
        // Ensure that the notes are within an octave
        let base_octave = notes[0].octave;
        for note in notes.iter_mut().skip(1) {
            while note.octave > base_octave + 1 {
                note.octave -= 1;
            }
        }
    }

    /// Open arrangement algorithm
    fn open_voicing(&self, notes: &mut Vec<Tuning>) {
        let mut current_octave = notes[0].octave;
        for (i, note) in notes.iter_mut().enumerate().skip(1) {
            if i % 2 == 0 {
                current_octave += 1;
            }
            note.octave = current_octave;
        }
    }
}

impl FromStr for Chord {
    type Err = MusicError;

    /// Eg：
    /// - "Cmaj7"   => C Major 7th chord
    /// - "G7/B"    => G Dominant 7th chord with B bass
    /// - "Dm9"     => D minor 9th chord
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("{}{}", self.root, self.quality);
        write!(f, "{}", str)
    }
}

impl Display for ChordQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChordQuality::Major => "M",
            ChordQuality::Minor => "m",
            ChordQuality::Diminished => "dim",
            ChordQuality::Augmented => "aug",
            ChordQuality::Major7 => "M7",
            ChordQuality::Dominant7 => "7",
            ChordQuality::Minor7 => "m7",
            ChordQuality::MinorMajor7 => "mM7",
            ChordQuality::HalfDiminished => "Ø",
            ChordQuality::FullyDiminished => "°7",
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PitchClass;

    #[test]
    fn test_major_triad() {
        let c_major = Chord::triad(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap();
        assert_eq!(
            c_major.components(),
            vec![
                Tuning::new(PitchClass::C, 4),
                Tuning::new(PitchClass::E, 4),
                Tuning::new(PitchClass::G, 4)
            ]
        );
    }

    #[test]
    fn test_dominant_seventh() {
        let g7 = Chord::seventh(Tuning::new(PitchClass::G, 4), ChordQuality::Dominant7).unwrap();
        assert_eq!(
            g7.components(),
            vec![
                Tuning::new(PitchClass::G, 4),
                Tuning::new(PitchClass::B, 4),
                Tuning::new(PitchClass::D, 5),
                Tuning::new(PitchClass::F, 5)
            ]
        );
    }

    #[test]
    fn test_inversion() {
        let mut cmaj = Chord::triad(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap();
        cmaj.invert(Inversion::First);
        assert_eq!(
            cmaj.components(),
            vec![
                Tuning::new(PitchClass::E, 4),
                Tuning::new(PitchClass::G, 4),
                Tuning::new(PitchClass::C, 5)
            ] // 第一转位
        );
    }
}

//! Chord system module
//! It includes core functions such as chord construction, analysis, inversion and voice arrangement

use crate::interval::{Interval, IntervalDegree, IntervalQuality};
use crate::tuning::{PitchClass, Tuning};
use crate::MusicError;
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone)]
pub struct Chord {
    root: Tuning,
    intervals: Vec<Interval>,
    chord_type: ChordType,
    inversion: Inversion,
    voicing: Voicing,
    extensions: Vec<Interval>, // Extended sounds (9th, 11th, etc.)
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
        };

        Ok(Self::new(root, intervals, ChordType::Triad, quality))
    }

    /// Construct seventh chord
    pub fn seventh(root: Tuning, quality: SeventhQuality) -> Result<Self, MusicError> {
        let mut base = Self::triad(root, quality.base_quality())?;
        base.extensions.push(match quality {
            SeventhQuality::Major7 | SeventhQuality::MinorMajor7 => {
                Interval::from_quality_degree(IntervalQuality::Major, 7)?
            }
            SeventhQuality::Dominant7 | SeventhQuality::HalfDiminished | SeventhQuality::Minor7 => {
                Interval::from_quality_degree(IntervalQuality::Minor, 7)?
            }
            SeventhQuality::FullyDiminished => {
                Interval::from_quality_degree(IntervalQuality::Diminished, 7)?
            }
        });
        base.chord_type = ChordType::Seventh;
        Ok(base)
    }
}

/// Classification of chord masses (basic triads)
#[derive(Debug, Clone, Copy)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
}

/// Subdivision of seventh chord types
#[derive(Debug, Clone, Copy)]
pub enum SeventhQuality {
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

impl SeventhQuality {
    pub fn base_quality(&self) -> ChordQuality {
        match self {
            SeventhQuality::Major7 => ChordQuality::Major,
            SeventhQuality::Dominant7 => ChordQuality::Major,
            SeventhQuality::Minor7 => ChordQuality::Minor,
            SeventhQuality::MinorMajor7 => ChordQuality::Minor,
            SeventhQuality::HalfDiminished => ChordQuality::Diminished,
            SeventhQuality::FullyDiminished => ChordQuality::Diminished,
        }
    }
}

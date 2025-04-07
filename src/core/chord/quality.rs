use crate::{Interval, IntervalQuality, MusicError};
use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

/// Classification of chord masses (basic triads)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordQuality {
    // Triad
    Major,
    Minor,
    Diminished,
    Augmented,

    // Seventh chord
    Major7,
    Dominant7,
    Minor7,
    MinorMajor7,
    HalfDiminished7,
    Diminished7,
    Augmented7,
    AugmentedMajor7,

    // Sixth chord
    Major6,
    Minor6,

    // Ninth chord
    Major9,
    Dominant9,
    Minor9,
    Add9,
    MinorAdd9,

    // Suspended chord
    Suspended2,
    Suspended4,
    Suspended7,
    Suspended9,
}

impl ChordQuality {
    pub fn base_quality(&self) -> ChordQuality {
        match self {
            ChordQuality::Major7 => ChordQuality::Major,
            ChordQuality::Dominant7 => ChordQuality::Major,
            ChordQuality::Minor7 => ChordQuality::Minor,
            ChordQuality::MinorMajor7 => ChordQuality::Minor,
            ChordQuality::HalfDiminished7 => ChordQuality::Diminished,
            ChordQuality::Diminished7 => ChordQuality::Diminished,
            ChordQuality::Augmented7 => ChordQuality::Augmented,
            ChordQuality::AugmentedMajor7 => ChordQuality::Augmented,
            _ => *self,
        }
    }

    pub fn iter() -> impl Iterator<Item = ChordQuality> {
        [
            // Triad
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Augmented,
            // Seventh chord
            ChordQuality::Major7,
            ChordQuality::Dominant7,
            ChordQuality::Minor7,
            ChordQuality::MinorMajor7,
            ChordQuality::HalfDiminished7,
            ChordQuality::Diminished7,
            ChordQuality::Augmented7,
            ChordQuality::AugmentedMajor7,
            // Sixth chord
            ChordQuality::Major6,
            ChordQuality::Minor6,
            // Ninth chord
            ChordQuality::Major9,
            ChordQuality::Dominant9,
            ChordQuality::Minor9,
            ChordQuality::Add9,
            ChordQuality::MinorAdd9,
            // Suspended chord
            ChordQuality::Suspended2,
            ChordQuality::Suspended4,
            ChordQuality::Suspended7,
            ChordQuality::Suspended9,
        ]
        .into_iter()
    }

    pub fn analyze_from(intervals: &[Interval]) -> Result<Self, MusicError> {
        let interval_set = intervals
            .iter()
            .map(|interval| interval.semitones_mod())
            .collect::<BTreeSet<_>>();
        for quality in ChordQuality::iter() {
            let base_pattern_set = quality
                .intervals()
                .iter()
                .map(|i| i.semitones_mod())
                .collect::<BTreeSet<_>>();

            if interval_set == base_pattern_set {
                return Ok(quality);
            }

            // TODO: support
            // - [ ] chord inversion
            // - [ ] chord extensions
            // - [ ] chord jazzy extensions
            // let diff = interval_set.difference(&base_pattern_set);
            // let inter = interval_set.intersection(&base_pattern_set);
        }
        Err(MusicError::InvalidChordQuality)
    }

    pub fn intervals(&self) -> Vec<Interval> {
        match self {
            ChordQuality::Major => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
            ],
            ChordQuality::Minor => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
            ],
            ChordQuality::Diminished => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Diminished, 5).unwrap(),
            ],
            ChordQuality::Augmented => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Augmented, 5).unwrap(),
            ],
            ChordQuality::Major7 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 7).unwrap(),
            ],
            ChordQuality::Dominant7 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
            ],
            ChordQuality::Minor7 => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
            ],
            ChordQuality::MinorMajor7 => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 7).unwrap(),
            ],
            ChordQuality::HalfDiminished7 => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Diminished, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
            ],
            ChordQuality::Diminished7 => vec![
                Interval::from_quality_degree(IntervalQuality::Diminished, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Diminished, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Diminished, 7).unwrap(),
            ],
            ChordQuality::Augmented7 => vec![
                Interval::from_quality_degree(IntervalQuality::Augmented, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Augmented, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
            ],
            ChordQuality::AugmentedMajor7 => vec![
                Interval::from_quality_degree(IntervalQuality::Augmented, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Augmented, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 7).unwrap(),
            ],
            ChordQuality::Major6 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 6).unwrap(),
            ],
            ChordQuality::Minor6 => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 6).unwrap(),
            ],
            ChordQuality::Major9 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 7).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 9).unwrap(),
            ],
            ChordQuality::Dominant9 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 9).unwrap(),
            ],
            ChordQuality::Minor9 => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 9).unwrap(),
            ],
            ChordQuality::Add9 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 9).unwrap(),
            ],
            ChordQuality::MinorAdd9 => vec![
                Interval::from_quality_degree(IntervalQuality::Minor, 3).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 9).unwrap(),
            ],
            ChordQuality::Suspended2 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 2).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
            ],
            ChordQuality::Suspended4 => vec![
                Interval::from_quality_degree(IntervalQuality::Perfect, 4).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
            ],
            ChordQuality::Suspended7 => vec![
                Interval::from_quality_degree(IntervalQuality::Perfect, 4).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
            ],
            ChordQuality::Suspended9 => vec![
                Interval::from_quality_degree(IntervalQuality::Perfect, 4).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Minor, 7).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Major, 9).unwrap(),
            ],
        }
    }
}

impl Display for ChordQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChordQuality::Major => "",
            ChordQuality::Minor => "m",
            ChordQuality::Diminished => "dim",
            ChordQuality::Augmented => "aug",
            ChordQuality::Major7 => "M7",
            ChordQuality::Dominant7 => "7",
            ChordQuality::Minor7 => "m7",
            ChordQuality::MinorMajor7 => "mM7",
            ChordQuality::HalfDiminished7 => "Ø",
            ChordQuality::Diminished7 => "°7",
            ChordQuality::Augmented7 => "aug7",
            ChordQuality::AugmentedMajor7 => "augM7",
            ChordQuality::Major6 => "M6",
            ChordQuality::Minor6 => "m6",
            ChordQuality::Major9 => "M9",
            ChordQuality::Dominant9 => "9",
            ChordQuality::Minor9 => "m9",
            ChordQuality::Add9 => "add9",
            ChordQuality::MinorAdd9 => "madd9",
            ChordQuality::Suspended2 => "sus2",
            ChordQuality::Suspended4 => "sus4",
            ChordQuality::Suspended7 => "7sus4",
            ChordQuality::Suspended9 => "9sus4",
        };
        write!(f, "{}", str)
    }
}

impl FromStr for ChordQuality {
    type Err = MusicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let quality_string = s.split_whitespace().collect::<String>();
        match quality_string.as_str() {
            "" | "M" | "maj" | "major" => Ok(ChordQuality::Major),
            "m" | "min" | "minor" => Ok(ChordQuality::Minor),
            "dim" | "diminished" => Ok(ChordQuality::Diminished),
            "aug" | "augmented" => Ok(ChordQuality::Augmented),
            "M7" | "maj7" | "major7" => Ok(ChordQuality::Major7),
            "7" | "dom7" | "dominant7" => Ok(ChordQuality::Dominant7),
            "m7" | "min7" | "minor7" => Ok(ChordQuality::Minor7),
            "mM7" | "minM7" | "minorMajor7" => Ok(ChordQuality::MinorMajor7),
            "Ø" | "half-diminished7" => Ok(ChordQuality::HalfDiminished7),
            "°7" | "diminished7" => Ok(ChordQuality::Diminished7),
            "aug7" | "augmented7" => Ok(ChordQuality::Augmented7),
            "augM7" | "augmentedMajor7" => Ok(ChordQuality::AugmentedMajor7),
            "M6" | "maj6" | "major6" => Ok(ChordQuality::Major6),
            "m6" | "min6" | "minor6" => Ok(ChordQuality::Minor6),
            "M9" | "maj9" | "major9" => Ok(ChordQuality::Major9),
            "9" | "dom9" | "dominant9" => Ok(ChordQuality::Dominant9),
            "m9" | "min9" | "minor9" => Ok(ChordQuality::Minor9),
            "add9" => Ok(ChordQuality::Add9),
            "madd9" => Ok(ChordQuality::MinorAdd9),
            "sus2" => Ok(ChordQuality::Suspended2),
            "sus4" => Ok(ChordQuality::Suspended4),
            "7sus4" => Ok(ChordQuality::Suspended7),
            "9sus4" => Ok(ChordQuality::Suspended9),
            _ => Err(MusicError::InvalidIntervalQuality),
        }
    }
}

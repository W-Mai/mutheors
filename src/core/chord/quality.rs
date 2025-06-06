use crate::{Interval, IntervalQuality, MusicError};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

/// Classification of chord masses (basic triads)
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
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

    // Suspended chord
    Suspended2,
    Suspended4,
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
            // Suspended chord
            ChordQuality::Suspended2,
            ChordQuality::Suspended4,
        ]
        .into_iter()
    }

    pub fn analyze_from(intervals: &[Interval]) -> Result<(Self, Vec<Interval>), MusicError> {
        let interval_map = intervals
            .iter()
            .map(|interval| (interval.semitones_mod(), interval.clone()))
            .collect::<BTreeMap<_, _>>();

        let mut matches = vec![];
        for quality in ChordQuality::iter() {
            let base_pattern_map = quality
                .intervals()
                .iter()
                .map(|i| (i.semitones_mod(), i.clone()))
                .collect::<BTreeMap<_, _>>();

            if interval_map.keys().eq(base_pattern_map.keys()) {
                return Ok((quality, vec![]));
            }

            let diff = interval_map
                .keys()
                .filter(|k| !base_pattern_map.contains_key(k))
                .filter_map(|k| interval_map.get(&k).map(|v| (k, v.clone())))
                .collect::<BTreeMap<_, _>>();

            let inter = interval_map
                .keys()
                .filter(|k| base_pattern_map.contains_key(k))
                .filter_map(|k| interval_map.get(&k).map(|v| (k, v.clone())))
                .collect::<BTreeMap<_, _>>();

            matches.push((quality, diff, inter));
        }

        matches.sort_by(|lhs, rhs| {
            let lhs_diff_len = lhs.1.len();
            let rhs_diff_len = rhs.1.len();
            if lhs_diff_len == rhs_diff_len {
                lhs.2.len().cmp(&rhs.2.len())
            } else {
                lhs_diff_len.cmp(&rhs_diff_len)
            }
        });

        let pair = matches.first().ok_or(MusicError::InvalidChordQuality)?;

        Ok((
            pair.0,
            BTreeMap::from_iter(pair.1.iter().map(|(k, v)| (*k, v.clone())))
                .values()
                .cloned()
                .collect(),
        ))
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
            ChordQuality::Suspended2 => vec![
                Interval::from_quality_degree(IntervalQuality::Major, 2).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
            ],
            ChordQuality::Suspended4 => vec![
                Interval::from_quality_degree(IntervalQuality::Perfect, 4).unwrap(),
                Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap(),
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
            ChordQuality::Suspended2 => "sus2",
            ChordQuality::Suspended4 => "sus4",
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
            "sus2" => Ok(ChordQuality::Suspended2),
            "sus4" => Ok(ChordQuality::Suspended4),
            _ => Err(MusicError::InvalidIntervalQuality),
        }
    }
}

use std::fmt::Display;

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

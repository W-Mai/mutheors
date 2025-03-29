use std::fmt::Display;

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

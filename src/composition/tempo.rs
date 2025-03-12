pub trait TempoLike {
    fn value(&self) -> f32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Tempo {
    /// Larghissimo (≤20 BPM) - Extremely slow, glacial pace
    ///
    /// 极慢板 (≤20 BPM) - 冰川般的缓慢
    Larghissimo = 20,

    /// Grave (21-40 BPM) - Solemn and grave slowness
    ///
    /// 庄板 (21-40 BPM) - 沉重庄严的慢速
    Grave,

    /// Largo (41-60 BPM) - Broad and slow rhythm
    ///
    /// 广板 (41-60 BPM) - 宽广缓慢的节奏
    Largo,

    /// Lento (61-66 BPM) - Slow flowing tempo
    ///
    /// 缓板 (61-66 BPM) - 缓慢流动的速度
    Lento,

    /// Adagio (67-76 BPM) - Leisurely slow pace
    ///
    /// 柔板 (67-76 BPM) - 从容的慢速
    Adagio,

    /// Andante (77-108 BPM) - Walking speed
    ///
    /// 行板 (77-108 BPM) - 行走般的速度
    Andante,

    /// Moderato (109-120 BPM) - Moderate speed
    ///
    /// 中板 (109-120 BPM) - 中等速度
    Moderato,

    /// Allegretto (121-156 BPM) - Moderately fast
    ///
    /// 小快板 (121-156 BPM) - 稍快的速度
    Allegretto,

    /// Allegro (157-176 BPM) - Cheerful quick tempo
    ///
    /// 快板 (157-176 BPM) - 欢快的快速
    Allegro,

    /// Vivace (177-200 BPM) - Lively and fast
    ///
    /// 活板 (177-200 BPM) - 充满活力的快速
    Vivace,

    /// Presto (201-208 BPM) - Very fast
    ///
    /// 急板 (201-208 BPM) - 极速
    Presto,

    /// Prestissimo (≥209 BPM) - Extreme speed
    ///
    /// 最急板 (≥209 BPM) - 极限速度
    Prestissimo,
}

impl Tempo {
    /// Get standard BPM range (inclusive)
    pub fn bpm_range(self) -> (u16, u16) {
        match self {
            Self::Larghissimo => (0, 20),
            Self::Grave => (21, 40),
            Self::Largo => (41, 60),
            Self::Lento => (61, 66),
            Self::Adagio => (67, 76),
            Self::Andante => (77, 108),
            Self::Moderato => (109, 120),
            Self::Allegretto => (121, 156),
            Self::Allegro => (157, 176),
            Self::Vivace => (177, 200),
            Self::Presto => (201, 208),
            Self::Prestissimo => (209, u16::MAX),
        }
    }

    /// Detect closest tempo from BPM value
    pub fn from_bpm(bpm: u16) -> Self {
        match bpm {
            0..=20 => Self::Larghissimo,
            21..=40 => Self::Grave,
            41..=60 => Self::Largo,
            61..=66 => Self::Lento,
            67..=76 => Self::Adagio,
            77..=108 => Self::Andante,
            109..=120 => Self::Moderato,
            121..=156 => Self::Allegretto,
            157..=176 => Self::Allegro,
            177..=200 => Self::Vivace,
            201..=208 => Self::Presto,
            _ => Self::Prestissimo,
        }
    }

    /// Get original Italian terminology
    pub fn italian_name(self) -> &'static str {
        match self {
            Self::Larghissimo => "Larghissimo",
            Self::Grave => "Grave",
            Self::Largo => "Largo",
            Self::Lento => "Lento",
            Self::Adagio => "Adagio",
            Self::Andante => "Andante",
            Self::Moderato => "Moderato",
            Self::Allegretto => "Allegretto",
            Self::Allegro => "Allegro",
            Self::Vivace => "Vivace",
            Self::Presto => "Presto",
            Self::Prestissimo => "Prestissimo",
        }
    }
}

impl TempoLike for f32 {
    fn value(&self) -> f32 {
        *self
    }
}

impl TempoLike for u16 {
    fn value(&self) -> f32 {
        *self as f32
    }
}

impl TempoLike for Tempo {
    fn value(&self) -> f32 {
        if self == &Tempo::Prestissimo {
            return 220.0;
        }
        ((self.bpm_range().0 + self.bpm_range().1) / 2) as f32
    }
}

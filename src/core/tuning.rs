use crate::chord::Chord;
use crate::{ChordQuality, Interval, IntervalQuality, Scale, ScaleType};
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
pub enum PitchClass {
    None = 0,
    C = 1,
    CSharpOrDFlat = 2,
    D = 3,
    DSharpOrEFlat = 4,
    E = 5,
    F = 6,
    FSharpOrGFlat = 7,
    G = 8,
    GSharpOrAFlat = 9,
    A = 10,
    ASharpOrBFlat = 11,
    B = 12,
}

impl From<u8> for PitchClass {
    fn from(value: u8) -> Self {
        match value {
            1 => PitchClass::C,
            2 => PitchClass::CSharpOrDFlat,
            3 => PitchClass::D,
            4 => PitchClass::DSharpOrEFlat,
            5 => PitchClass::E,
            6 => PitchClass::F,
            7 => PitchClass::FSharpOrGFlat,
            8 => PitchClass::G,
            9 => PitchClass::GSharpOrAFlat,
            10 => PitchClass::A,
            11 => PitchClass::ASharpOrBFlat,
            12 => PitchClass::B,
            _ => panic!("Invalid value"),
        }
    }
}

impl PitchClass {
    pub fn modulation(&self, degree: i8) -> PitchClass {
        match self {
            PitchClass::None => PitchClass::None,
            _ => PitchClass::from(((*self as i8 - 1 + 12 + degree) % 12 + 1) as u8),
        }
    }

    pub fn common_chord(&self, degree: u8, octave: i8) -> Chord {
        assert!(degree > 0 && degree < 7, "Degree must be in [1, 6]");
        const BASIC_DEGREES: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];
        let tuning = Tuning::new(*self, octave);
        let new_tuning = tuning
            .add_interval(&Interval::from_semitones(BASIC_DEGREES[(degree - 1) as usize]).unwrap());

        let quality = match degree {
            1 | 4 | 5 => ChordQuality::Major,
            2 | 3 | 6 => ChordQuality::Minor,
            _ => panic!("Invalid degree"),
        };

        Chord::triad(new_tuning, quality).unwrap()
    }

    pub fn next_basic_degree(&self, nth: i8) -> PitchClass {
        let nth = nth.rem_euclid(7) as usize;
        let basic_degrees = vec![0, 2, 4, 5, 7, 9, 11];

        match self {
            PitchClass::None => PitchClass::C,
            _ => PitchClass::from((*self as i8 + basic_degrees[nth] - 1).rem_euclid(12) as u8 + 1),
        }
    }
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PitchClass::None => " ",
            PitchClass::C => "C",
            PitchClass::CSharpOrDFlat => "C#/Db",
            PitchClass::D => "D",
            PitchClass::DSharpOrEFlat => "D#/Eb",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::FSharpOrGFlat => "F#/Gb",
            PitchClass::G => "G",
            PitchClass::GSharpOrAFlat => "G#/Ab",
            PitchClass::A => "A",
            PitchClass::ASharpOrBFlat => "A#/Bb",
            PitchClass::B => "B",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Tuning {
    pub class: PitchClass,
    pub octave: i8,
    pub freq: Option<f32>, // 自定义频率
}

impl Tuning {
    pub fn new(class: PitchClass, octave: i8) -> Self {
        Self {
            class,
            octave,
            freq: None,
        }
    }

    pub fn with_freq(self, freq: f32) -> Self {
        let mut n = self;
        n.freq = Some(freq);
        self
    }

    /// Calculation of physical frequency (A4 = 440 Hz)
    pub fn frequency(&self) -> f32 {
        self.freq.unwrap_or_else(|| {
            440.0 * 2f32.powf((self.midi_number().unwrap() as f32 - 69.0) / 12.0)
        })
    }

    pub fn scale(&self, scale_type: ScaleType) -> Scale {
        Scale::new(*self, scale_type).unwrap()
    }
}

impl Tuning {
    pub fn add_interval(&self, interval: &Interval) -> Self {
        let new_semitones = interval.semitones() + self.class as i8;
        let new_octave = self.octave + (new_semitones + 11) / 12 - 1;
        let class = PitchClass::from(((new_semitones + 11) % 12 + 1) as u8);
        Tuning::new(class, new_octave)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntervalQuality, Note};

    #[test]
    fn test_tuning() {
        let tuning = Tuning::new(PitchClass::C, 4);
        assert_eq!(tuning.class, PitchClass::C);
        assert_eq!(tuning.octave, 4);
        assert_eq!(tuning.frequency(), 440.0 * 2f32.powf((60.0 - 69.0) / 12.0));
    }

    #[test]
    fn test_modulation() {
        let pc = PitchClass::C;
        assert_eq!(pc.modulation(4), PitchClass::E);
        assert_eq!(pc.modulation(-3), PitchClass::A);
        assert_eq!(pc.modulation(12), PitchClass::C);
        assert_eq!(pc.modulation(-12), PitchClass::C);
        assert_eq!(pc.modulation(0), PitchClass::C);
        assert_eq!(PitchClass::None.modulation(1), PitchClass::None);
    }
    #[test]
    fn test_interval() {
        let tuning = Tuning::new(PitchClass::C, 4);
        let new_tuning = tuning.add_interval(&Interval::from_semitones(0).unwrap());
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::C, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(2).unwrap());
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::D, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(4).unwrap());
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::E, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(5).unwrap());
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::F, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(7).unwrap());
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::G, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(9).unwrap());
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::A, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(11).unwrap());
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::B, 4));
    }

    #[test]
    fn test_common_chord() {
        let pitch_class = PitchClass::C;
        let notes: Vec<_> = pitch_class
            .common_chord(1, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::C, PitchClass::E, PitchClass::G]);
        let notes: Vec<_> = pitch_class
            .common_chord(2, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::D, PitchClass::F, PitchClass::A]);
        let notes: Vec<_> = pitch_class
            .common_chord(3, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::E, PitchClass::G, PitchClass::B]);
        let notes: Vec<_> = pitch_class
            .common_chord(4, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::F, PitchClass::A, PitchClass::C]);
        let notes: Vec<_> = pitch_class
            .common_chord(5, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::G, PitchClass::B, PitchClass::D]);
        let notes: Vec<_> = pitch_class
            .common_chord(6, 2)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::A, PitchClass::C, PitchClass::E]);

        let pitch_class = PitchClass::D;
        let notes: Vec<_> = pitch_class
            .common_chord(1, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(
            notes,
            vec![PitchClass::D, PitchClass::FSharpOrGFlat, PitchClass::A]
        );
        let notes: Vec<_> = pitch_class
            .common_chord(2, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::E, PitchClass::G, PitchClass::B]);
        let notes: Vec<_> = pitch_class
            .common_chord(3, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::F, PitchClass::A, PitchClass::C]);
        let notes: Vec<_> = pitch_class
            .common_chord(4, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::G, PitchClass::B, PitchClass::D]);
        let notes: Vec<_> = pitch_class
            .common_chord(5, 4)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::A, PitchClass::C, PitchClass::E]);
        let notes: Vec<_> = pitch_class
            .common_chord(6, 2)
            .components()
            .iter()
            .map(|&c| c.class)
            .collect();
        assert_eq!(notes, vec![PitchClass::B, PitchClass::D, PitchClass::F]);
    }
}

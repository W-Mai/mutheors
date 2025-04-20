use crate::chord::Chord;
use crate::{Interval, MusicError, Scale, ScaleType};
use std::fmt::Display;
use std::iter::Peekable;
use std::ops::{Div, Mul};
use std::str::FromStr;

#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PitchClass {
    None,
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
    A,
    As,
    Bb,
    B,
}

impl PitchClass {
    pub fn sharp(self) -> Tuning {
        Tuning {
            class: self,
            accidentals: 1,
            octave: 0,
            freq: None,
        }
    }

    pub fn flat(self) -> Tuning {
        Tuning {
            class: self,
            accidentals: -1,
            octave: 0,
            freq: None,
        }
    }

    pub fn common_chord(&self, degree: u8, octave: i8) -> Chord {
        Tuning::from(*self)
            .with_octave(octave)
            .scale(ScaleType::Major)
            .degree_chord(degree)
            .unwrap()
    }

    /// Semitones
    pub fn semitones(&self) -> i8 {
        match &self {
            &PitchClass::C => 1,
            &PitchClass::Cs => 2,
            &PitchClass::Db => 2,
            &PitchClass::D => 3,
            &PitchClass::Ds => 4,
            &PitchClass::Eb => 4,
            &PitchClass::E => 5,
            &PitchClass::F => 6,
            &PitchClass::Fs => 7,
            &PitchClass::Gb => 7,
            &PitchClass::G => 8,
            &PitchClass::Gs => 9,
            &PitchClass::Ab => 9,
            &PitchClass::A => 10,
            &PitchClass::As => 11,
            &PitchClass::Bb => 12,
            &PitchClass::B => 12,
            &PitchClass::None => 0,
        }
    }
}

impl From<PitchClass> for i8 {
    fn from(value: PitchClass) -> Self {
        value.semitones()
    }
}

impl From<PitchClass> for Tuning {
    fn from(pc: PitchClass) -> Self {
        Tuning {
            class: pc,
            accidentals: 0,
            octave: 0,
            freq: None,
        }
    }
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PitchClass::C => "C",
            PitchClass::Cs => "C#",
            PitchClass::Db => "Db",
            PitchClass::D => "D",
            PitchClass::Ds => "D#",
            PitchClass::Eb => "Eb",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::Fs => "F#",
            PitchClass::Gs => "G#",
            PitchClass::Gb => "Gb",
            PitchClass::G => "G",
            PitchClass::Ab => "Ab",
            PitchClass::A => "A",
            PitchClass::As => "A#",
            PitchClass::Bb => "Bb",
            PitchClass::B => "B",
            PitchClass::None => "X",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

impl FromStr for Tuning {
    type Err = MusicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();

        Tuning::take(chars.by_ref())
    }
}

impl Tuning {
    pub fn take(chars: &mut Peekable<std::str::Chars>) -> Result<Tuning, MusicError> {
        let mut root = String::new();

        if let Some(&c) = chars.peek() {
            if ('A'..='G').contains(&c) {
                root.push(c);
                chars.next();
            } else {
                return Err(MusicError::InvalidPitch);
            }
        }

        let mut root = Tuning::from(match root.as_str() {
            "C" => PitchClass::C,
            "D" => PitchClass::D,
            "E" => PitchClass::E,
            "F" => PitchClass::F,
            "G" => PitchClass::G,
            "A" => PitchClass::A,
            "B" => PitchClass::B,
            _ => unreachable!("{}", MusicError::InvalidPitch),
        })
        .with_octave(4);

        while let Some(&c) = chars.peek() {
            if c == '#' {
                root = root.sharp();
                chars.next();
            } else if c == 'b' {
                root = root.flat();
                chars.next();
            } else {
                break;
            }
        }

        Ok(root)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Tuning {
    class: PitchClass,
    pub accidentals: i8,
    pub octave: i8,
    freq: Option<f32>, // 自定义频率
}

impl Tuning {
    pub fn new(class: PitchClass, octave: i8) -> Self {
        Self {
            class,
            accidentals: 0,
            octave,
            freq: None,
        }
    }

    pub fn with_octave(self, octave: i8) -> Self {
        Self { octave, ..self }
    }

    pub fn with_freq(self, freq: f32) -> Self {
        Self {
            freq: Some(freq),
            ..self
        }
    }

    /// Calculation of physical frequency (A4 = 440 Hz)
    pub fn frequency(&self) -> f32 {
        self.freq.unwrap_or_else(|| {
            440.0
                * 2f32.powf(
                    (((self.octave + 1) * 12 + self.class.semitones() - 1) as f32 - 69.0) / 12.0,
                )
        })
    }

    pub fn class(&self) -> PitchClass {
        self.class
    }

    pub fn accidentals(&self) -> i8 {
        self.accidentals
    }

    pub fn octave(&self) -> i8 {
        self.octave
    }

    pub fn scale(&self, scale_type: ScaleType) -> Scale {
        Scale::new(*self, scale_type).unwrap()
    }

    pub fn common_chord(&self, degree: u8) -> Chord {
        self.scale(ScaleType::Major).degree_chord(degree).unwrap()
    }

    pub fn class_semitones(&self) -> i8 {
        (self.class.semitones() + self.accidentals + 11) % 12
    }

    pub fn number(&self) -> i8 {
        let base = self.class.semitones();
        let num = (self.octave + 1)
            .saturating_mul(12)
            .saturating_add(base)
            .saturating_add(self.accidentals);
        num
    }
}

impl Tuning {
    pub fn add_interval(&self, interval: &Interval) -> Result<Self, MusicError> {
        let new_semitones = interval.semitones() + self.class.semitones() + self.accidentals;
        let new_octave = self.octave + (new_semitones + 11) / 12 - 1;
        if !(0..=11).contains(&new_octave) {
            Err(MusicError::InvalidOctave { octave: new_octave })
        } else {
            let semi = (new_semitones + 11) % 12 + 1;
            let is_sharp = interval.semitones() > 0;
            let tuning: Tuning = match semi {
                1 => PitchClass::C.into(),
                2 => {
                    if is_sharp {
                        PitchClass::C.sharp()
                    } else {
                        PitchClass::D.flat()
                    }
                }
                3 => PitchClass::D.into(),
                4 => {
                    if is_sharp {
                        PitchClass::D.sharp()
                    } else {
                        PitchClass::E.flat()
                    }
                }
                5 => PitchClass::E.into(),
                6 => PitchClass::F.into(),
                7 => {
                    if is_sharp {
                        PitchClass::F.sharp()
                    } else {
                        PitchClass::G.flat()
                    }
                }
                8 => PitchClass::G.into(),
                9 => {
                    if is_sharp {
                        PitchClass::G.sharp()
                    } else {
                        PitchClass::A.flat()
                    }
                }
                10 => PitchClass::A.into(),
                11 => {
                    if is_sharp {
                        PitchClass::A.sharp()
                    } else {
                        PitchClass::B.flat()
                    }
                }
                12 => PitchClass::B.into(),
                _ => unreachable!(),
            };

            Ok(Self {
                octave: new_octave,
                ..tuning
            })
        }
    }

    pub fn sharp(self) -> Self {
        Self {
            accidentals: self.accidentals.wrapping_add(1),
            ..self
        }
    }

    pub fn flat(self) -> Self {
        Self {
            accidentals: self.accidentals.wrapping_sub(1),
            ..self
        }
    }

    pub fn simple(self) -> Self {
        let accidentals = self.accidentals;
        let new_tuning = Self {
            accidentals: 0,
            ..self
        };

        new_tuning
            .add_interval(&Interval::from_semitones(accidentals).unwrap())
            .unwrap()
    }
}

impl Mul<u8> for Tuning {
    type Output = Tuning;

    fn mul(self, rhs: u8) -> Self::Output {
        self.add_interval(&Interval::from_semitones(12 * (rhs - 1) as i8).unwrap())
            .unwrap()
    }
}

impl Div<u8> for Tuning {
    type Output = Tuning;

    fn div(self, rhs: u8) -> Self::Output {
        self.add_interval(&Interval::from_semitones(-12 * (rhs - 1) as i8).unwrap())
            .unwrap()
    }
}

impl Display for Tuning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let acc_str = match self.accidentals {
            0 => String::new(),
            1 => "#".to_owned(),
            -1 => "b".to_owned(),
            2 => "##".to_owned(),
            -2 => "bb".to_owned(),
            n if n > 0 => "#".repeat(n as usize),
            n if n < 0 => "b".repeat((-n) as usize),
            _ => unreachable!(),
        };
        if f.alternate() {
            write!(f, "{}{}{}", self.class, acc_str, self.octave)
        } else {
            write!(f, "{}{}", self.class, acc_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn test_tuning_01() {
        let pc = PitchClass::C;
        let tuning1 = pc.sharp().with_octave(3) * 2;
        let tuning2 = Tuning::new(PitchClass::C, 4).sharp();
        assert_eq!(tuning1, tuning2);
    }

    #[test]
    fn test_tuning_02() {
        let pc = PitchClass::C;
        let tuning = pc.sharp().with_octave(3);
        let tuning1 = tuning
            .add_interval(&Interval::from_semitones(1).unwrap())
            .unwrap();
        let tuning2 = tuning1
            .add_interval(&Interval::from_semitones(1).unwrap())
            .unwrap();
        let tuning3 = tuning2
            .add_interval(&Interval::from_semitones(1).unwrap())
            .unwrap();
        let tuning4 = tuning3
            .add_interval(&Interval::from_semitones(-1).unwrap())
            .unwrap();

        println!("tuning1: {}", tuning1);
        println!("tuning2: {}", tuning2);
        println!("tuning2: {}", tuning3);
        println!("tuning2: {}", tuning4);
    }

    #[test]
    fn test_tuning_03() {
        let pc = PitchClass::C;
        let tuning1 = (pc.sharp().with_octave(3) * 3).flat();
        let tuning2 = Tuning::new(PitchClass::C, 6) / 2;
        assert_eq!(tuning1, tuning2);
    }

    #[test]
    fn test_tuning() {
        let tuning = tuning!(C 4);
        assert_eq!(tuning.class, PitchClass::C);
        assert_eq!(tuning.octave, 4);
        assert_eq!(tuning.frequency(), 440.0 * 2f32.powf((60.0 - 69.0) / 12.0));
    }

    #[test]
    fn test_tuning_2() {
        let pitch = PitchClass::E.flat().with_octave(3);
        for i in 1..=6 {
            let c = pitch.common_chord(i);
            println!("{}", c);
        }
    }

    #[test]
    fn test_tuning_3() -> Result<(), MusicError> {
        let tuning = Tuning::from_str("C#")?;
        assert_eq!(tuning, tuning!(# C 4));

        let tuning = Tuning::from_str("C")?;
        assert_eq!(tuning, tuning!(C 4));

        let tuning = Tuning::from_str("C##")?;
        assert_eq!(tuning, tuning!(# C 4).sharp());

        let tuning = Tuning::from_str("Cb")?;
        assert_eq!(tuning, tuning!(b C 4));

        Ok(())
    }

    #[test]
    fn test_modulation_2() {
        let tuning = tuning!(C 4);
        assert_eq!(tuning.sharp(), tuning!(# C 4));
        assert_eq!(tuning.flat(), tuning!(b C 4));
    }

    #[test]
    fn test_interval() -> Result<(), MusicError> {
        let tuning = tuning!(C 4);
        let new_tuning = tuning.add_interval(&Interval::from_semitones(0)?)?;
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::C, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(2)?)?;
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::D, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(4)?)?;
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::E, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(5)?)?;
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::F, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(7)?)?;
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::G, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(9)?)?;
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::A, 4));
        let new_tuning = tuning.add_interval(&Interval::from_semitones(11)?)?;
        assert_eq!((new_tuning.class, new_tuning.octave), (PitchClass::B, 4));

        Ok(())
    }

    #[test]
    fn test_common_chord() {
        // Test the common chord for C
        let pitch_class = tuning!(C 4);
        let notes = pitch_class.common_chord(1).components();
        assert_eq!(notes, vec![tuning!(C 4), tuning!(E 4), tuning!(G 4)]);

        let notes = pitch_class.common_chord(2).components();
        assert_eq!(notes, vec![tuning!(D 4), tuning!(F 4), tuning!(A 4)]);

        let notes = pitch_class.common_chord(3).components();
        assert_eq!(notes, vec![tuning!(E 4), tuning!(G 4), tuning!(B 4)]);

        let notes = pitch_class.common_chord(4).components();
        assert_eq!(notes, vec![tuning!(F 4), tuning!(A 4), tuning!(C 5)]);

        let notes = pitch_class.common_chord(5).components();
        assert_eq!(notes, vec![tuning!(G 4), tuning!(B 4), tuning!(D 5)]);

        let notes = pitch_class.common_chord(6).components();
        assert_eq!(notes, vec![tuning!(A 4), tuning!(C 5), tuning!(E 5)]);

        // Test the common chord for D
        let pitch_class = tuning!(D 4);
        let notes = pitch_class.common_chord(1).components();
        assert_eq!(notes, vec![tuning!(D 4), tuning!(# F 4), tuning!(A 4)]);

        let notes = pitch_class.common_chord(2).components();
        assert_eq!(notes, vec![tuning!(E 4), tuning!(G 4), tuning!(B 4)]);

        let notes = pitch_class.common_chord(3).components();
        assert_eq!(notes, vec![tuning!(# F 4), tuning!(A 4), tuning!(# C 5)]);

        let notes = pitch_class.common_chord(4).components();
        assert_eq!(notes, vec![tuning!(G 4), tuning!(B 4), tuning!(D 5)]);

        let notes = pitch_class.common_chord(5).components();
        assert_eq!(notes, vec![tuning!(A 4), tuning!(# C 5), tuning!(E 5)]);

        let notes = pitch_class.common_chord(6).components();
        assert_eq!(notes, vec![tuning!(B 4), tuning!(D 5), tuning!(# F 5)]);
    }
}

use crate::chord::Chord;
use crate::{ChordQuality, Interval, MusicError, Scale, ScaleType};
use std::fmt::Display;
use std::ops::{Div, Mul};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
pub enum PitchClass {
    None = 0,
    C = 1,
    D = 3,
    E = 5,
    F = 6,
    G = 8,
    A = 10,
    B = 12,
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
        assert!(degree > 0 && degree < 7, "Degree must be in [1, 6]");
        const BASIC_DEGREES: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];
        let tuning = Tuning::new(*self, octave);
        let new_tuning = tuning
            .add_interval(&Interval::from_semitones(BASIC_DEGREES[(degree - 1) as usize]).unwrap())
            .unwrap();

        let quality = match degree {
            1 | 4 | 5 => ChordQuality::Major,
            2 | 3 | 6 => ChordQuality::Minor,
            _ => panic!("Invalid degree"),
        };

        Chord::triad(new_tuning, quality).unwrap()
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
            PitchClass::None => " ",
            PitchClass::C => "C",
            PitchClass::D => "D",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::G => "G",
            PitchClass::A => "A",
            PitchClass::B => "B",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Tuning {
    pub class: PitchClass,
    pub accidentals: i8,
    pub octave: i8,
    pub freq: Option<f32>, // 自定义频率
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
                * 2f32.powf((((self.octave + 1) * 12 + self.class as i8 - 1) as f32 - 69.0) / 12.0)
        })
    }

    pub fn scale(&self, scale_type: ScaleType) -> Scale {
        Scale::new(*self, scale_type).unwrap()
    }

    pub fn common_chord(&self, degree: u8) -> Chord {
        self.class.common_chord(degree, self.octave)
    }
}

impl Tuning {
    pub fn add_interval(&self, interval: &Interval) -> Result<Self, MusicError> {
        let new_semitones = interval.semitones() + self.class as i8 + self.accidentals;
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
        write!(f, "{}{}{}", self.class, acc_str, self.octave)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

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

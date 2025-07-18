use crate::chord::Chord;
use crate::{Interval, IntervalQuality, MusicError, Scale, ScaleType};
use std::fmt::Display;
use std::iter::Peekable;
use std::ops::{ControlFlow, Div, Mul};
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
    pub fn sharp(&self) -> Self {
        match &self {
            PitchClass::None => PitchClass::None,
            PitchClass::C => PitchClass::Cs,
            PitchClass::Cs => PitchClass::D,
            PitchClass::Db => PitchClass::D,
            PitchClass::D => PitchClass::Ds,
            PitchClass::Ds => PitchClass::E,
            PitchClass::Eb => PitchClass::E,
            PitchClass::E => PitchClass::F,
            PitchClass::F => PitchClass::Fs,
            PitchClass::Fs => PitchClass::G,
            PitchClass::Gb => PitchClass::G,
            PitchClass::G => PitchClass::Gs,
            PitchClass::Gs => PitchClass::A,
            PitchClass::Ab => PitchClass::A,
            PitchClass::A => PitchClass::As,
            PitchClass::As => PitchClass::B,
            PitchClass::Bb => PitchClass::B,
            PitchClass::B => PitchClass::C,
        }
    }

    pub fn flat(&self) -> Self {
        match &self {
            PitchClass::None => PitchClass::None,
            PitchClass::C => PitchClass::B,
            PitchClass::Cs => PitchClass::C,
            PitchClass::Db => PitchClass::C,
            PitchClass::D => PitchClass::Db,
            PitchClass::Ds => PitchClass::D,
            PitchClass::Eb => PitchClass::D,
            PitchClass::E => PitchClass::Eb,
            PitchClass::F => PitchClass::E,
            PitchClass::Fs => PitchClass::F,
            PitchClass::Gb => PitchClass::F,
            PitchClass::G => PitchClass::Gb,
            PitchClass::Gs => PitchClass::G,
            PitchClass::Ab => PitchClass::G,
            PitchClass::A => PitchClass::Ab,
            PitchClass::As => PitchClass::A,
            PitchClass::Bb => PitchClass::A,
            PitchClass::B => PitchClass::Bb,
        }
    }

    pub fn common_chord(&self, degree: u8, octave: i8) -> Chord {
        Tuning::new(*self, octave)
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
            &PitchClass::Bb => 11,
            &PitchClass::B => 12,
            &PitchClass::None => 0,
        }
    }

    pub fn degree(&self) -> i8 {
        match &self {
            &PitchClass::C => 1,
            &PitchClass::Cs => 1,
            &PitchClass::Db => 2,
            &PitchClass::D => 2,
            &PitchClass::Ds => 2,
            &PitchClass::Eb => 3,
            &PitchClass::E => 3,
            &PitchClass::F => 4,
            &PitchClass::Fs => 4,
            &PitchClass::Gb => 5,
            &PitchClass::G => 5,
            &PitchClass::Gs => 5,
            &PitchClass::Ab => 6,
            &PitchClass::A => 6,
            &PitchClass::As => 6,
            &PitchClass::Bb => 7,
            &PitchClass::B => 7,
            &PitchClass::None => 0,
        }
    }

    pub fn from_degree(degree: i8) -> PitchClass {
        let degree = (degree - 1).rem_euclid(7) + 1;
        match degree {
            1 => PitchClass::C,
            2 => PitchClass::D,
            3 => PitchClass::E,
            4 => PitchClass::F,
            5 => PitchClass::G,
            6 => PitchClass::A,
            7 => PitchClass::B,
            _ => PitchClass::None,
        }
    }

    pub fn add_accidentals(&self, accidentals: i8) -> (Self, i8) {
        let degree = self.degree();

        match (0..accidentals.abs()).try_fold((self.clone(), accidentals), |(pc, acc), _| {
            let new_pc = if acc.is_positive() {
                pc.sharp()
            } else {
                pc.flat()
            };

            if new_pc.degree() == degree {
                ControlFlow::Continue((new_pc, acc - acc.signum()))
            } else {
                ControlFlow::Break((pc, acc))
            }
        }) {
            ControlFlow::Continue(x) | ControlFlow::Break(x) => x,
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
        Tuning::new(pc, 0)
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

        let mut root = match root.as_str() {
            "C" => PitchClass::C,
            "D" => PitchClass::D,
            "E" => PitchClass::E,
            "F" => PitchClass::F,
            "G" => PitchClass::G,
            "A" => PitchClass::A,
            "B" => PitchClass::B,
            _ => unreachable!("{}", MusicError::InvalidPitch),
        };

        let mut accidentals: i8 = 0;
        while let Some(&c) = chars.peek() {
            if c == '#' {
                accidentals += 1;
                chars.next();
            } else if c == 'b' {
                accidentals -= 1;
                chars.next();
            } else {
                break;
            }
        }

        if accidentals.abs() > 0 {
            if accidentals.is_positive() {
                root = root.sharp();
            } else {
                root = root.flat();
            }
            accidentals -= accidentals.signum();
        }

        let mut tuning = Tuning::new(root, 4);
        tuning.accidentals = accidentals;

        Ok(tuning)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Tuning {
    class: PitchClass,
    accidentals: i8,
    octave: i8,
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

    pub fn with_octave(&self, octave: i8) -> Self {
        Self { octave, ..*self }
    }

    pub fn with_freq(&self, freq: f32) -> Self {
        Self {
            freq: Some(freq),
            ..*self
        }
    }

    pub fn with_accidentals(&self, accidentals: i8) -> Self {
        Self {
            accidentals,
            ..*self
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
        if base == 0 {
            return 0;
        }
        let num = (self.octave + 1)
            .saturating_mul(12)
            .saturating_add(base - 1)
            .saturating_add(self.accidentals);
        num
    }
}

impl Tuning {
    /// TODO: handle -interval
    pub fn add_interval(&self, interval: &Interval) -> Result<Self, MusicError> {
        let new_semitones = interval.semitones() + self.class.semitones() + self.accidentals;
        let mut estimated_octave = self.octave + (new_semitones + 11) / 12 - 1;

        if !(0..=11).contains(&estimated_octave) {
            return Err(MusicError::InvalidOctave {
                octave: estimated_octave,
            });
        }

        let ori_degree = self.class().degree();
        let new_degree = ori_degree + interval.degree() * interval.semitones().signum()
            - interval.semitones().signum();
        let pitch_class = PitchClass::from_degree(new_degree);

        let mut pc_semi_diff =
            new_semitones - (pitch_class.semitones() + (estimated_octave - self.octave) * 12);

        while pc_semi_diff.abs() > Interval::minor_seventh().semitones() {
            estimated_octave += pc_semi_diff.signum();
            pc_semi_diff -= pc_semi_diff.signum() * 12;
        }

        let (pitch_class, accidental) = pitch_class.add_accidentals(pc_semi_diff);

        let mut tuning = Tuning::new(pitch_class, estimated_octave);
        tuning.accidentals = accidental;

        Ok(tuning)
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
        let new_tuning = Tuning::new(self.class, self.octave);

        new_tuning
            .add_interval(&Interval::from_semitones(accidentals).unwrap())
            .unwrap()
    }
}

impl Tuning {
    pub fn dom(&self, n: u8) -> Vec<Self> {
        let scale_root = self.add_interval(&-Interval::perfect_fifth()).unwrap();
        let scale = scale_root.scale(ScaleType::Major);

        (7..=n).step_by(2).map(|i| scale(i + 4)).collect()
    }

    pub fn maj(&self, n: u8) -> Vec<Self> {
        let scale = self.scale(ScaleType::Major);

        (7..=n).step_by(2).map(|i| scale(i)).collect()
    }

    pub fn min(&self, n: u8) -> Vec<Self> {
        let scale = self.scale(ScaleType::NaturalMinor);

        (7..=n).step_by(2).map(|i| scale(i)).collect()
    }
}

impl Mul<u8> for Tuning {
    type Output = Tuning;

    fn mul(self, rhs: u8) -> Self::Output {
        self.add_interval(
            &Interval::from_quality_degree(IntervalQuality::Perfect, 1 + 7 * (rhs - 1)).unwrap(),
        )
        .unwrap()
    }
}

impl Div<u8> for Tuning {
    type Output = Tuning;

    fn div(self, rhs: u8) -> Self::Output {
        self.add_interval(
            &Interval::from_quality_degree(IntervalQuality::Perfect, 1 + 7 * (rhs - 1))
                .unwrap()
                .invert(),
        )
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
        let tuning1 = Tuning::new(PitchClass::C.sharp(), 3) * 2;
        let tuning2 = Tuning::new(PitchClass::C, 4).sharp();
        assert_eq!(tuning1.number(), tuning2.number());
    }

    #[test]
    fn test_tuning_02() {
        let pc = PitchClass::C;
        let tuning = Tuning::new(pc.sharp(), 3);
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
        let tuning1 = (Tuning::new(pc.sharp(), 3) * 3).flat();
        let tuning2 = Tuning::new(PitchClass::C, 6) / 2;
        assert_eq!(tuning1.number(), tuning2.number());
    }

    #[test]
    fn test_tuning_04() {
        let pc = PitchClass::C;
        let interval_1 = Interval::from_quality_degree(IntervalQuality::Augmented, 4).unwrap();
        let interval_2 = Interval::from_quality_degree(IntervalQuality::Diminished, 5).unwrap();
        let tuning_1 = Tuning::new(pc, 4);
        let tuning_2 = tuning_1.add_interval(&interval_1).unwrap();
        let tuning_3 = tuning_1.add_interval(&interval_2).unwrap();

        assert_eq!(Tuning::new(PitchClass::Fs, 4), tuning_2);
        assert_eq!(Tuning::new(PitchClass::Gb, 4), tuning_3);
    }

    #[test]
    fn test_tuning_05() {
        let pc = PitchClass::Cs;
        let interval_1 = Interval::from_quality_degree(IntervalQuality::Augmented, 4).unwrap();
        let interval_2 = Interval::from_quality_degree(IntervalQuality::Diminished, 5).unwrap();
        let tuning_1 = Tuning::new(pc, 4);
        let tuning_2 = tuning_1.add_interval(&interval_1).unwrap();
        let tuning_3 = tuning_1.add_interval(&interval_2).unwrap();

        assert_eq!(Tuning::new(PitchClass::Fs, 4).sharp(), tuning_2);
        assert_eq!(Tuning::new(PitchClass::G, 4), tuning_3);
    }

    #[test]
    fn test_tuning_06() {
        let pc = PitchClass::As;
        let interval_1 = Interval::from_quality_degree(IntervalQuality::Major, 2).unwrap();
        let tuning_1 = Tuning::new(pc, 4);
        let tuning_2 = tuning_1.add_interval(&interval_1).unwrap();

        assert_eq!(Tuning::new(PitchClass::B, 4).sharp(), tuning_2);
    }

    #[test]
    fn test_tuning_07() {
        let pc = PitchClass::Bb;
        let interval_1 = Interval::from_quality_degree(IntervalQuality::Major, 2).unwrap();
        let tuning_1 = Tuning::new(pc, 4);
        let tuning_2 = tuning_1.add_interval(&interval_1).unwrap();
        assert_eq!(Tuning::new(PitchClass::C, 5), tuning_2);
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
        let pitch = Tuning::new(PitchClass::E.flat(), 3);
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
        assert_eq!(tuning.flat().number(), tuning!(b C 3).number());
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

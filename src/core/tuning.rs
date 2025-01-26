use std::fmt::Display;
use crate::chord::Chord;
use crate::tonality::Tonality;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq)]
pub enum Tuning {
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

impl From<u8> for Tuning {
    fn from(value: u8) -> Self {
        match value {
            1 => Tuning::C,
            2 => Tuning::CSharpOrDFlat,
            3 => Tuning::D,
            4 => Tuning::DSharpOrEFlat,
            5 => Tuning::E,
            6 => Tuning::F,
            7 => Tuning::FSharpOrGFlat,
            8 => Tuning::G,
            9 => Tuning::GSharpOrAFlat,
            10 => Tuning::A,
            11 => Tuning::ASharpOrBFlat,
            12 => Tuning::B,
            _ => panic!("Invalid value"),
        }
    }
}

impl Tuning {
    pub fn modulation(&self, degree: i8) -> Tuning {
        match self {
            Tuning::None => { Tuning::None }
            _ => Tuning::from(((*self as i8 - 1 + 12 + degree) % 12 + 1) as u8)
        }
    }

    pub fn common_chord(&self, degree: u8) -> Chord {
        assert!(degree > 0 && degree < 7, "Degree must be in [1, 6]");
        let new_tuning = self.next_basic_degree((degree - 1) as i8);
        let tonality = match degree {
            1 | 4 | 5 => Tonality::Major,
            2 | 3 | 6 => Tonality::Minor,
            _ => panic!("Invalid degree"),
        };

        Chord { tuning: new_tuning, tonality }
    }

    pub fn next_basic_degree(&self, nth: i8) -> Tuning {
        let nth = nth.rem_euclid(7) as usize;
        let basic_degrees = vec![0, 2, 4, 5, 7, 9, 11];

        match self {
            Tuning::None => Tuning::C,
            _ => Tuning::from((*self as i8 + basic_degrees[nth] - 1).rem_euclid(12) as u8 + 1)
        }
    }
}

impl Display for Tuning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Tuning::None => " ",
            Tuning::C => "C",
            Tuning::CSharpOrDFlat => "C#/Db",
            Tuning::D => "D",
            Tuning::DSharpOrEFlat => "D#/Eb",
            Tuning::E => "E",
            Tuning::F => "F",
            Tuning::FSharpOrGFlat => "F#/Gb",
            Tuning::G => "G",
            Tuning::GSharpOrAFlat => "G#/Ab",
            Tuning::A => "A",
            Tuning::ASharpOrBFlat => "A#/Bb",
            Tuning::B => "B",
        }.to_string();
        write!(f, "{}", str)
    }
}

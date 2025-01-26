use std::fmt::Display;
use crate::note::Note;
use crate::tonality::Tonality;
use crate::tuning::Tuning;

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq)]
pub struct Chord {
    pub tuning: Tuning,
    pub tonality: Tonality,
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            tuning: Tuning::C,
            tonality: Tonality::Major,
        }
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuning_str = self.tuning.to_string();
        let tonality_str = match self.tonality {
            Tonality::Major => "",
            Tonality::Minor => "m",
            _ => "?"
        };

        write!(f, "{}", format!("{}{}", tuning_str, tonality_str))
    }
}

impl Chord {
    pub fn breakdown(&self, octave: u8) -> Vec<Note> {
        fn octave_eval(octave: u8, base_degree: Tuning, delta_degree: i8) -> u8 {
            let new_degree = base_degree as i8 + delta_degree;
            (octave as i8 + new_degree / 12 + (new_degree % 12 != 0) as i8 - 1) as u8
        }

        match self.tonality {
            Tonality::Major => {
                vec![
                    Note { tuning: self.tuning, octave, duration: 0.5, velocity: 0.5 },
                    Note { tuning: self.tuning.modulation(4), octave: octave_eval(octave, self.tuning, 4), duration: 0.5, velocity: 0.5 },
                    Note { tuning: self.tuning.modulation(7), octave: octave_eval(octave, self.tuning, 7), duration: 0.5, velocity: 0.5 },
                ]
            }
            Tonality::Minor => {
                vec![
                    Note { tuning: self.tuning, octave, duration: 0.5, velocity: 0.5 },
                    Note { tuning: self.tuning.modulation(3), octave: octave_eval(octave, self.tuning, 3), duration: 0.5, velocity: 0.5 },
                    Note { tuning: self.tuning.modulation(7), octave: octave_eval(octave, self.tuning, 7), duration: 0.5, velocity: 0.5 },
                ]
            }
            _ => vec![]
        }
    }
}

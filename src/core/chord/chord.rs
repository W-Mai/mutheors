use crate::note::Note;
use crate::tonality::Tonality;
use crate::tuning::PitchClass;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Chord {
    pub pitch_class: PitchClass,
    pub tonality: Tonality,
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            pitch_class: PitchClass::C,
            tonality: Tonality::Major,
        }
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pc_str = self.pitch_class.to_string();
        let tonality_str = match self.tonality {
            Tonality::Major => "",
            Tonality::Minor => "m",
            _ => "?",
        };

        write!(f, "{}", format!("{}{}", pc_str, tonality_str))
    }
}

impl Chord {
    pub fn breakdown(&self, octave: u8) -> Vec<Note> {
        fn octave_eval(octave: u8, base_degree: PitchClass, delta_degree: i8) -> u8 {
            let new_degree = base_degree as i8 + delta_degree;
            (octave as i8 + new_degree / 12 + (new_degree % 12 != 0) as i8 - 1) as u8
        }

        match self.tonality {
            Tonality::Major => {
                vec![
                    Note {
                        pitch_class: self.pitch_class,
                        octave,
                        duration: 0.5,
                        velocity: 0.5,
                    },
                    Note {
                        pitch_class: self.pitch_class.modulation(4),
                        octave: octave_eval(octave, self.pitch_class, 4),
                        duration: 0.5,
                        velocity: 0.5,
                    },
                    Note {
                        pitch_class: self.pitch_class.modulation(7),
                        octave: octave_eval(octave, self.pitch_class, 7),
                        duration: 0.5,
                        velocity: 0.5,
                    },
                ]
            }
            Tonality::Minor => {
                vec![
                    Note {
                        pitch_class: self.pitch_class,
                        octave,
                        duration: 0.5,
                        velocity: 0.5,
                    },
                    Note {
                        pitch_class: self.pitch_class.modulation(3),
                        octave: octave_eval(octave, self.pitch_class, 3),
                        duration: 0.5,
                        velocity: 0.5,
                    },
                    Note {
                        pitch_class: self.pitch_class.modulation(7),
                        octave: octave_eval(octave, self.pitch_class, 7),
                        duration: 0.5,
                        velocity: 0.5,
                    },
                ]
            }
            _ => vec![],
        }
    }
}

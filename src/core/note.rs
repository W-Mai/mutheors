use crate::tuning::Tuning;
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub struct Note {
    pub tuning: Tuning,
    pub octave: u8,
    pub duration: f32,
    pub velocity: f32,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chord_str = self.tuning.to_string();
        let octave_str = self.octave.to_string();

        write!(f, "{}{}", chord_str, octave_str)
    }
}

impl Note {
    pub fn with_duration(self, duration: f32) -> Note {
        Note {
            tuning: self.tuning,
            octave: self.octave,
            duration,
            velocity: self.velocity,
        }
    }

    pub fn with_velocity(self, velocity: f32) -> Note {
        Note {
            tuning: self.tuning,
            octave: self.octave,
            duration: self.duration,
            velocity,
        }
    }
}

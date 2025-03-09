use crate::tuning::PitchClass;
use crate::Duration;
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: i8,
    pub duration: Duration,
    pub velocity: f32,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chord_str = self.pitch_class.to_string();
        let octave_str = self.octave.to_string();

        write!(f, "{}{}", chord_str, octave_str)
    }
}

impl Note {
    pub fn new(pitch_class: PitchClass, octave: i8) -> Self {
        Note {
            pitch_class,
            octave,
            duration: Duration::from_quarters(1.0),
            velocity: 0.0,
        }
    }

    pub fn with_duration(self, duration: Duration) -> Note {
        Note {
            pitch_class: self.pitch_class,
            octave: self.octave,
            duration,
            velocity: self.velocity,
        }
    }

    pub fn with_velocity(self, velocity: f32) -> Note {
        Note {
            pitch_class: self.pitch_class,
            octave: self.octave,
            duration: self.duration,
            velocity,
        }
    }

    pub fn pitch_class(&self) -> PitchClass {
        self.pitch_class
    }

    pub fn octave(&self) -> i8 {
        self.octave
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }
}

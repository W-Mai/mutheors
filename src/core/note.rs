use crate::{Duration, Tuning};
use std::fmt::Display;

#[cfg_attr(feature = "bindgen", derive(uniffi::Object))]
#[derive(Copy, Clone, Debug)]
pub struct Note {
    tuning: Tuning,
    duration: Duration,
    velocity: f32,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.tuning, self.duration)
    }
}

impl Note {
    pub fn new(tuning: Tuning) -> Self {
        Note {
            tuning,
            duration: Duration::from_quarters(1.0),
            velocity: 0.0,
        }
    }

    pub fn with_duration(self, duration: Duration) -> Note {
        Note { duration, ..self }
    }

    pub fn with_velocity(self, velocity: f32) -> Note {
        Note { velocity, ..self }
    }

    pub fn tuning(&self) -> Tuning {
        self.tuning
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }
}

impl From<Tuning> for Note {
    fn from(tuning: Tuning) -> Self {
        Note::new(tuning)
    }
}

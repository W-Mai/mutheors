//! Interval calculation system
//! Provides core functions such as definition, calculation, and conversion of intervals.

use super::errors::MusicError;
use super::tuning::PitchClass;
use std::convert::TryFrom;

/// Interval quality (consonance/dissonance)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntervalQuality {
    Perfect,    // Pure intervals (1,4,5,8 degrees)
    Major,      // Major intervals (2,3,6,7 degrees)
    Minor,      // Minor intervals (to be used with Major)
    Augmented,  // Augmented interval
    Diminished, // Diminished interval
}

/// Degree of an interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalDegree(pub u8);

/// Interval
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    quality: IntervalQuality,
    degree: IntervalDegree,
    semitones: i8,       // Actual number of semitones
    is_descending: bool, // Is the interval descending (relative to the root)
}

impl IntervalDegree {
    pub fn new(degree: u8) -> Result<Self, MusicError> {
        if degree < 1 || degree > 13 {
            return Err(MusicError::InvalidIntervalDegree { degree });
        }
        Ok(Self(degree))
    }
}

impl Interval {
    pub fn from_semitones(semitones: i8) -> Result<Self, MusicError> {
        let abs_semi = semitones.abs() % 12;
        let octaves = semitones.abs() / 12;

        let (quality, degree) = match abs_semi {
            0 => (IntervalQuality::Perfect, 1),
            1 => (IntervalQuality::Minor, 2),
            2 => (IntervalQuality::Major, 2),
            3 => (IntervalQuality::Minor, 3),
            4 => (IntervalQuality::Major, 3),
            5 => (IntervalQuality::Perfect, 4),
            6 => (IntervalQuality::Augmented, 4),
            7 => (IntervalQuality::Perfect, 5),
            8 => (IntervalQuality::Minor, 6),
            9 => (IntervalQuality::Major, 6),
            10 => (IntervalQuality::Minor, 7),
            11 => (IntervalQuality::Major, 7),
            _ => unreachable!(),
        };

        Ok(Self {
            quality,
            degree: IntervalDegree(degree + octaves as u8 * 7),
            semitones,
            is_descending: semitones < 0,
        })
    }
}

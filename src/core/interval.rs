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

/// Consonance of an interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consonance {
    Consonant,
    Imperfect,
    Dissonant,
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
        let is_descending = semitones < 0;

        let (quality, degree) = match abs_semi {
            0 => (IntervalQuality::Perfect, 1),
            1 => (IntervalQuality::Minor, 2),
            2 => (IntervalQuality::Major, 2),
            3 => (IntervalQuality::Minor, 3),
            4 => (IntervalQuality::Major, 3),
            5 => (IntervalQuality::Perfect, 4),
            6 => {
                if is_descending {
                    (IntervalQuality::Diminished, 5)
                } else {
                    (IntervalQuality::Augmented, 4)
                }
            } // or Diminished 5th (depending on direction)
            7 => (IntervalQuality::Perfect, 5),
            8 => (IntervalQuality::Minor, 6),
            9 => (IntervalQuality::Major, 6),
            10 => (IntervalQuality::Minor, 7),
            11 => (IntervalQuality::Major, 7),
            _ => unreachable!(),
        };

        Ok(Self {
            quality,
            degree: IntervalDegree::new(degree + octaves as u8 * 7)?,
            semitones,
            is_descending,
        })
    }

    pub fn between(start: PitchClass, end: PitchClass) -> Self {
        let semitones = end as i8 - start as i8;
        Self::from_semitones(semitones).unwrap()
    }

    /// Interstitial inversion (e.g. Major 3rd -> minor 6th)
    pub fn invert(&mut self) {
        self.degree.0 = 9 - self.degree.0 % 7;
        self.semitones = 12 - self.semitones.abs();
        self.quality = match self.quality {
            IntervalQuality::Perfect => IntervalQuality::Perfect,
            IntervalQuality::Major => IntervalQuality::Minor,
            IntervalQuality::Minor => IntervalQuality::Major,
            IntervalQuality::Augmented => IntervalQuality::Diminished,
            IntervalQuality::Diminished => IntervalQuality::Augmented,
        };
    }

    /// Consonance of the interval
    pub fn consonance(&self) -> Consonance {
        match (self.degree.0 % 7, self.quality) {
            (0, _) => Consonance::Consonant, // Same quality
            (3, IntervalQuality::Perfect) => Consonance::Consonant, // 4th
            (4, IntervalQuality::Perfect) => Consonance::Consonant, // 5th
            (_, IntervalQuality::Perfect) => Consonance::Consonant,
            (1 | 2 | 5, q) if matches!(q, IntervalQuality::Major | IntervalQuality::Minor) => {
                Consonance::Imperfect
            }
            _ => Consonance::Dissonant,
        }
    }
}

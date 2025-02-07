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

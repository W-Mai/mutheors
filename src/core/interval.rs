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

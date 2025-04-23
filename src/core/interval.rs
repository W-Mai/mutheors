//! # Interval System
//!
//! This module provides core functionality for working with musical intervals:
//! - Interval representation (quality, degree, semitones)
//! - Interval calculation and conversion
//! - Interval operations (addition, subtraction, inversion)
//! - Common interval factories
//! - Consonance analysis
//!
//! Intervals are essential for describing relationships between notes, chord structure,
//! and scale construction in music theory.

use super::errors::MusicError;
use super::tuning::PitchClass;
use std::convert::TryFrom;
use std::ops::{Add, Mul, Neg, Sub};

// Common interval semitone sizes
pub const UNISON: i8 = 0;
pub const MINOR_SECOND: i8 = 1;
pub const MAJOR_SECOND: i8 = 2;
pub const MINOR_THIRD: i8 = 3;
pub const MAJOR_THIRD: i8 = 4;
pub const PERFECT_FOURTH: i8 = 5;
pub const TRITONE: i8 = 6;
pub const PERFECT_FIFTH: i8 = 7;
pub const MINOR_SIXTH: i8 = 8;
pub const MAJOR_SIXTH: i8 = 9;
pub const MINOR_SEVENTH: i8 = 10;
pub const MAJOR_SEVENTH: i8 = 11;
pub const OCTAVE: i8 = 12;

/// The quality component of an interval, indicating its specific type
///
/// In music theory, intervals are classified both by their degree (second, third, etc.)
/// and their quality (perfect, major, minor, etc.).
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum IntervalQuality {
    /// Perfect intervals (unison, fourth, fifth, octave)
    Perfect,
    /// Major intervals (seconds, thirds, sixths, sevenths)
    Major,
    /// Minor intervals (one semitone smaller than major)
    Minor,
    /// Augmented intervals (one semitone larger than perfect or major)
    Augmented,
    /// Diminished intervals (one semitone smaller than perfect or minor)
    Diminished,
}

/// Consonance category of an interval, representing its harmonic quality
///
/// In music theory, intervals are classified based on how "pleasing" they sound:
/// - Consonant intervals sound stable and resolved
/// - Imperfect consonances have a mild tension but still sound harmonious
/// - Dissonant intervals create tension and instability
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consonance {
    /// Perfect consonances (unison, fifth, octave) - highly stable
    Consonant,
    /// Imperfect consonances (thirds, sixths) - moderately stable
    Imperfect,
    /// Dissonances (seconds, sevenths, tritone) - unstable
    Dissonant,
}

/// Degree of an interval
#[cfg_attr(feature = "bindgen", derive(uniffi::Object))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct IntervalDegree(pub u8);

/// Interval
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct Interval {
    quality: IntervalQuality,
    degree: IntervalDegree,
    semitones: i8,       // Actual number of semitones
    is_descending: bool, // Is the interval descending (relative to the root)
}

impl IntervalDegree {
    pub fn new(degree: u8) -> Result<Self, MusicError> {
        if degree < 1 || degree > 127 {
            return Err(MusicError::InvalidIntervalDegree { degree });
        }
        Ok(Self(degree))
    }
}

impl Interval {
    pub fn semitones(&self) -> i8 {
        self.semitones
    }

    pub fn semitones_mod(&self) -> i8 {
        self.semitones.rem_euclid(12)
    }
}

impl Interval {
    /// Create an interval from a quality and degree combination
    ///
    /// # Arguments
    /// * `quality` - The interval quality (Perfect, Major, Minor, Augmented, Diminished)
    /// * `degree` - The degree of the interval
    ///
    /// # Returns
    /// * The interval or an error if the combination is invalid
    pub fn from_quality_degree(quality: IntervalQuality, degree: u8) -> Result<Self, MusicError> {
        let degree = IntervalDegree::new(degree)?;
        let semitones = calculate_semitones(quality, degree)?;

        Ok(Self {
            quality,
            degree,
            semitones: semitones as i8,
            is_descending: false,
        })
    }

    /// Create a perfect unison (0 semitones)
    pub fn unison() -> Self {
        Self::from_quality_degree(IntervalQuality::Perfect, 1).unwrap()
    }

    /// Create a minor second (1 semitone)
    pub fn minor_second() -> Self {
        Self::from_quality_degree(IntervalQuality::Minor, 2).unwrap()
    }

    /// Create a major second (2 semitones)
    pub fn major_second() -> Self {
        Self::from_quality_degree(IntervalQuality::Major, 2).unwrap()
    }

    /// Create a minor third (3 semitones)
    pub fn minor_third() -> Self {
        Self::from_quality_degree(IntervalQuality::Minor, 3).unwrap()
    }

    /// Create a major third (4 semitones)
    pub fn major_third() -> Self {
        Self::from_quality_degree(IntervalQuality::Major, 3).unwrap()
    }

    /// Create a perfect fourth (5 semitones)
    pub fn perfect_fourth() -> Self {
        Self::from_quality_degree(IntervalQuality::Perfect, 4).unwrap()
    }

    /// Create a tritone as augmented fourth (6 semitones)
    pub fn augmented_fourth() -> Self {
        Self::from_quality_degree(IntervalQuality::Augmented, 4).unwrap()
    }

    /// Create a tritone as diminished fifth (6 semitones)
    pub fn diminished_fifth() -> Self {
        Self::from_quality_degree(IntervalQuality::Diminished, 5).unwrap()
    }

    /// Create a perfect fifth (7 semitones)
    pub fn perfect_fifth() -> Self {
        Self::from_quality_degree(IntervalQuality::Perfect, 5).unwrap()
    }

    /// Create a minor sixth (8 semitones)
    pub fn minor_sixth() -> Self {
        Self::from_quality_degree(IntervalQuality::Minor, 6).unwrap()
    }

    /// Create a major sixth (9 semitones)
    pub fn major_sixth() -> Self {
        Self::from_quality_degree(IntervalQuality::Major, 6).unwrap()
    }

    /// Create a minor seventh (10 semitones)
    pub fn minor_seventh() -> Self {
        Self::from_quality_degree(IntervalQuality::Minor, 7).unwrap()
    }

    /// Create a major seventh (11 semitones)
    pub fn major_seventh() -> Self {
        Self::from_quality_degree(IntervalQuality::Major, 7).unwrap()
    }

    /// Create a perfect octave (12 semitones)
    pub fn octave() -> Self {
        Self::from_quality_degree(IntervalQuality::Perfect, 8).unwrap()
    }

    /// Create a tritone interval (6 semitones)
    /// Returns the augmented fourth by default
    pub fn tritone() -> Self {
        Self::augmented_fourth()
    }

    /// Create an interval from a number of semitones, handling errors
    pub fn from_semitones_unchecked(semitones: i8) -> Self {
        Self::from_semitones(semitones).unwrap_or_else(|_| Self::unison())
    }

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

    /// Calculate the interval between two pitch classes
    ///
    /// # Arguments
    /// * `start` - The starting pitch class
    /// * `end` - The ending pitch class
    ///
    /// # Returns
    /// * The interval between the two pitch classes
    pub fn between(start: PitchClass, end: PitchClass) -> Self {
        let semitones = end as i8 - start as i8;
        Self::from_semitones(semitones).unwrap_or_else(|_| Self::unison())
    }

    /// Perform an interstitial inversion of the interval
    /// For example,
    /// - Major 3rd -> minor 6th
    /// - Perfect 5th -> Perfect 4th
    /// - Augmented 4th -> Diminished 5th
    pub fn invert(&self) -> Self {
        let inverted_quality = match self.quality {
            IntervalQuality::Perfect => IntervalQuality::Perfect,
            IntervalQuality::Major => IntervalQuality::Minor,
            IntervalQuality::Minor => IntervalQuality::Major,
            IntervalQuality::Augmented => IntervalQuality::Diminished,
            IntervalQuality::Diminished => IntervalQuality::Augmented,
        };

        // The complementary degree: 9 - original (mod 7)
        let new_degree = 9 - (self.degree.0 % 7);

        Self {
            quality: inverted_quality,
            degree: IntervalDegree(new_degree),
            semitones: -self.semitones,
            is_descending: !self.is_descending,
        }
    }

    /// Create a new interval with the same properties but in the opposite direction
    /// For example, an ascending Major 3rd becomes a descending Major 3rd
    pub fn negate(&self) -> Self {
        Self {
            semitones: -self.semitones,
            is_descending: !self.is_descending,
            ..*self
        }
    }

    /// Determine the consonance category of the interval
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

    /// Check if the interval is a perfect consonance (unison, perfect fifth, perfect octave)
    pub fn is_perfect_consonance(&self) -> bool {
        matches!(self.consonance(), Consonance::Consonant)
            && matches!(self.quality, IntervalQuality::Perfect)
            && matches!(self.degree.0 % 7, 1 | 5 | 0)
    }

    /// Check if the interval is an imperfect consonance (major/minor third, major/minor sixth)
    pub fn is_imperfect_consonance(&self) -> bool {
        matches!(self.consonance(), Consonance::Imperfect)
    }

    /// Check if the interval is a dissonance
    pub fn is_dissonant(&self) -> bool {
        matches!(self.consonance(), Consonance::Dissonant)
    }

    /// Check if the interval is a perfect interval (unison, fourth, fifth, octave)
    pub fn is_perfect(&self) -> bool {
        matches!(self.quality, IntervalQuality::Perfect)
    }

    /// Check if the interval is a major interval
    pub fn is_major(&self) -> bool {
        matches!(self.quality, IntervalQuality::Major)
    }

    /// Check if the interval is a minor interval
    pub fn is_minor(&self) -> bool {
        matches!(self.quality, IntervalQuality::Minor)
    }

    /// Check if the interval is an augmented interval
    pub fn is_augmented(&self) -> bool {
        matches!(self.quality, IntervalQuality::Augmented)
    }

    /// Check if the interval is a diminished interval
    pub fn is_diminished(&self) -> bool {
        matches!(self.quality, IntervalQuality::Diminished)
    }

    /// Check if the interval is a simple interval (within an octave)
    pub fn is_simple(&self) -> bool {
        self.semitones.abs() <= OCTAVE
    }

    /// Check if the interval is a compound interval (larger than an octave)
    pub fn is_compound(&self) -> bool {
        self.semitones.abs() > OCTAVE
    }

    /// Get a collection of all perfect consonance intervals within an octave
    pub fn perfect_consonances() -> Vec<Self> {
        vec![Self::unison(), Self::perfect_fifth(), Self::octave()]
    }

    /// Get a collection of all imperfect consonance intervals within an octave
    pub fn imperfect_consonances() -> Vec<Self> {
        vec![
            Self::minor_third(),
            Self::major_third(),
            Self::minor_sixth(),
            Self::major_sixth(),
        ]
    }

    /// Get a collection of all consonant intervals within an octave
    pub fn consonant_intervals() -> Vec<Self> {
        let mut intervals = Self::perfect_consonances();
        intervals.extend(Self::imperfect_consonances());
        intervals
    }

    /// Get a collection of all dissonant intervals within an octave
    pub fn dissonant_intervals() -> Vec<Self> {
        vec![
            Self::minor_second(),
            Self::major_second(),
            Self::perfect_fourth(), // Perfect fourth is sometimes considered dissonant in certain contexts
            Self::tritone(),
            Self::minor_seventh(),
            Self::major_seventh(),
        ]
    }

    /// Get the interval name
    /// e.g.
    /// - M3 (major third)
    /// - m6 (minor sixth)
    /// - Aug4 (augmented fourth)
    /// - Dim5 (diminished fifth)
    pub fn name(&self) -> String {
        let quality_str = match self.quality {
            IntervalQuality::Perfect => "P",
            IntervalQuality::Major => "M",
            IntervalQuality::Minor => "m",
            IntervalQuality::Augmented => "Aug",
            IntervalQuality::Diminished => "Dim",
        };

        format!("{}{}", quality_str, self.degree.0)
    }
}

/// Calculate the number of semitones for a given interval quality and degree
///
/// # Arguments
/// * `quality` - The interval quality (Perfect, Major, Minor, Augmented, Diminished)
/// * `degree` - The degree of the interval (1-8 for simple intervals)
///
/// # Returns
/// * The number of semitones or an error if the combination is invalid
fn calculate_semitones(quality: IntervalQuality, degree: IntervalDegree) -> Result<u8, MusicError> {
    let degree_num = (degree.0 - 1) % 7 + 1;
    let octaves = (degree.0 - 1) / 7;

    // Standard semitones for each degree (based on major and perfect intervals)
    const BASE_SEMITONES: [u8; 8] = [0, 0, 2, 4, 5, 7, 9, 11];
    let base_semitones = BASE_SEMITONES[degree_num as usize];

    // Determine if this degree can be "perfect" (1, 4, 5, 8)
    let is_perfect_degree = matches!(degree_num, 1 | 4 | 5 | 8);

    // Calculate adjustment based on quality
    let adjustment = match (quality, is_perfect_degree) {
        (IntervalQuality::Perfect, true) => 0,
        (IntervalQuality::Major, false) => 0,
        (IntervalQuality::Minor, false) => -1,
        (IntervalQuality::Augmented, _) => 1,
        (IntervalQuality::Diminished, true) => -1,
        (IntervalQuality::Diminished, false) => -2,
        _ => return Err(MusicError::InvalidIntervalQuality),
    };

    // Calculate final semitone count
    Ok(((12 * octaves + base_semitones) as i8 + adjustment) as u8)
}

impl TryFrom<&str> for Interval {
    type Error = MusicError;

    /// Parse an interval name string into an `Interval` object
    ///
    /// # Format examples
    /// - "P1" - Perfect unison
    /// - "M3" - Major third
    /// - "m6" - Minor sixth
    /// - "Aug4" - Augmented fourth
    /// - "Dim5" - Diminished fifth
    ///
    /// # Returns
    /// * The interval or an error if the string is invalid
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.trim();
        let mut quality = None;
        let mut degree = None;

        if s.starts_with('P') {
            quality = Some(IntervalQuality::Perfect);
            degree = s[1..].parse().ok();
        } else if let Some(remainder) = s.strip_prefix("Aug") {
            quality = Some(IntervalQuality::Augmented);
            degree = remainder.parse().ok();
        } else if let Some(remainder) = s.strip_prefix("Dim") {
            quality = Some(IntervalQuality::Diminished);
            degree = remainder.parse().ok();
        } else if s.starts_with('M') {
            quality = Some(IntervalQuality::Major);
            degree = s[1..].parse().ok();
        } else if s.starts_with('m') {
            quality = Some(IntervalQuality::Minor);
            degree = s[1..].parse().ok();
        }

        match (quality, degree) {
            (Some(q), Some(d)) => Self::from_quality_degree(q, d),
            _ => Err(MusicError::IntervalParseError { name: s.to_owned() }),
        }
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = if self.is_descending { "-" } else { "" };
        write!(
            f,
            "{}{} ({}{})",
            direction,
            self.name(),
            direction,
            self.semitones
        )
    }
}

impl From<i8> for Interval {
    /// Create an interval from a semitone count, defaulting to unison on error
    fn from(semitones: i8) -> Self {
        Self::from_semitones_unchecked(semitones)
    }
}

impl Add for Interval {
    type Output = Self;

    /// Add two intervals together
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_semitones_unchecked(self.semitones + rhs.semitones)
    }
}

impl Sub for Interval {
    type Output = Self;

    /// Subtract one interval from another
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_semitones_unchecked(self.semitones - rhs.semitones)
    }
}

impl Neg for Interval {
    type Output = Self;

    /// Negate an interval (change direction)
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Mul<i8> for Interval {
    type Output = Self;

    /// Multiply an interval by a scalar
    /// For example: major_third * 2 = major sixth
    fn mul(self, rhs: i8) -> Self::Output {
        Self::from_semitones_unchecked(self.semitones * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_perfect_fifth() {
        let interval = Interval::from_quality_degree(IntervalQuality::Perfect, 5).unwrap();
        assert_eq!(interval.semitones, 7);
        assert_eq!(interval.name(), "P5");
    }

    #[test]
    fn test_major_third() {
        let interval = Interval::from_semitones(4).unwrap();
        assert_eq!(interval.quality, IntervalQuality::Major);
        assert_eq!(interval.degree.0, 3);
    }

    #[test]
    fn test_inversion() {
        let interval = Interval::from_semitones(4).unwrap(); // Major 3rd
        let interval = interval.invert();
        assert_eq!(interval.semitones, -4); // Minor 6th
        assert_eq!(interval.quality, IntervalQuality::Minor);
        assert_eq!(interval.degree.0, 6);
    }

    #[test]
    fn test_descending_interval() -> Result<(), MusicError> {
        let interval = Interval::from_semitones(-24)?;
        assert_eq!(interval.quality, IntervalQuality::Perfect);

        let tuning = tuning!(C 3);
        let new_tuning = tuning.add_interval(&interval)?;
        assert_eq!(new_tuning.class(), PitchClass::C);
        assert_eq!(new_tuning.octave(), 1);

        Ok(())
    }

    #[test]
    fn test_perfect_interval() -> Result<(), MusicError> {
        let intervals = [
            Interval::from_quality_degree(IntervalQuality::Perfect, 1)?,
            Interval::from_quality_degree(IntervalQuality::Perfect, 4)?,
            Interval::from_quality_degree(IntervalQuality::Perfect, 5)?,
            Interval::from_quality_degree(IntervalQuality::Perfect, 8)?,
        ];

        let interval_numbers = [0, 5, 7, 12];
        let intervals = intervals.map(|i| i.semitones());
        assert_eq!(intervals, interval_numbers);

        Ok(())
    }

    #[test]
    fn test_major_interval() -> Result<(), MusicError> {
        let intervals = [
            Interval::from_quality_degree(IntervalQuality::Major, 2)?,
            Interval::from_quality_degree(IntervalQuality::Major, 3)?,
            Interval::from_quality_degree(IntervalQuality::Major, 6)?,
            Interval::from_quality_degree(IntervalQuality::Major, 7)?,
            Interval::from_quality_degree(IntervalQuality::Major, 9)?,
        ];

        let interval_numbers = [2, 4, 9, 11, 14];
        let intervals = intervals.map(|i| i.semitones());
        assert_eq!(intervals, interval_numbers);
        Ok(())
    }

    #[test]
    fn test_interval_factory_methods() {
        assert_eq!(Interval::unison().semitones(), 0);
        assert_eq!(Interval::minor_second().semitones(), 1);
        assert_eq!(Interval::major_second().semitones(), 2);
        assert_eq!(Interval::minor_third().semitones(), 3);
        assert_eq!(Interval::major_third().semitones(), 4);
        assert_eq!(Interval::perfect_fourth().semitones(), 5);
        assert_eq!(Interval::tritone().semitones(), 6);
        assert_eq!(Interval::perfect_fifth().semitones(), 7);
        assert_eq!(Interval::minor_sixth().semitones(), 8);
        assert_eq!(Interval::major_sixth().semitones(), 9);
        assert_eq!(Interval::minor_seventh().semitones(), 10);
        assert_eq!(Interval::major_seventh().semitones(), 11);
        assert_eq!(Interval::octave().semitones(), 12);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_interval_operations() {
        let m3 = Interval::minor_third();
        let M3 = Interval::major_third();
        let P5 = Interval::perfect_fifth();

        assert_eq!((m3 + M3).semitones(), 7);

        let m7 = Interval::minor_seventh();
        let P4 = Interval::perfect_fourth();

        assert_eq!((m7 - P4).semitones(), 5);

        assert_eq!((M3 * 2).semitones(), 8);
        assert_eq!((P5 * 2).semitones(), 14);
    }

    #[test]
    fn test_interval_from_string() -> Result<(), MusicError> {
        assert_eq!(Interval::try_from("P1")?.semitones(), 0);
        assert_eq!(Interval::try_from("m3")?.semitones(), 3);
        assert_eq!(Interval::try_from("M6")?.semitones(), 9);
        assert_eq!(Interval::try_from("Aug4")?.semitones(), 6);
        assert_eq!(Interval::try_from("Dim5")?.semitones(), 6);

        // error cases
        assert!(Interval::try_from("X5").is_err());
        assert!(Interval::try_from("P9").is_err());

        Ok(())
    }
}

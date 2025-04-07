//! Interval calculation system
//! Provides core functions such as definition, calculation, and conversion of intervals.

use super::errors::MusicError;
use super::tuning::PitchClass;
use std::convert::TryFrom;

/// Interval quality (consonance/dissonance)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct IntervalDegree(pub u8);

/// Interval
#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
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

fn calculate_semitones(quality: IntervalQuality, degree: IntervalDegree) -> Result<u8, MusicError> {
    let degree_num = (degree.0 - 1) % 7 + 1;
    let octaves = (degree.0 - 1) / 7;
    let base_semitones = match degree_num {
        1 => 0,  // 1 degree standard interval
        2 => 2,  // Major 2nd standard interval
        3 => 4,  // Major 3rd standard interval
        4 => 5,  // Perfect 4th
        5 => 7,  // Perfect 5th
        6 => 9,  // Major 6th
        7 => 11, // Major 7th
        _ => unreachable!(),
    };

    let adjustment = match quality {
        IntervalQuality::Perfect if [1, 4, 5, 8].contains(&degree_num) => 0,
        IntervalQuality::Major if [2, 3, 6, 7].contains(&degree_num) => 0,
        IntervalQuality::Minor if [2, 3, 6, 7].contains(&degree_num) => -1,
        IntervalQuality::Augmented => 1,
        IntervalQuality::Diminished if [2, 3, 6, 7].contains(&degree_num) => -2,
        IntervalQuality::Diminished if [1, 4, 5, 8].contains(&degree_num) => -1,
        _ => 
            return Err(MusicError::InvalidIntervalQuality),
    };

    Ok(((12 * octaves + base_semitones) as i8 + adjustment) as u8)
}

impl TryFrom<&str> for Interval {
    type Error = MusicError;

    /// Parse an interval name
    /// e.g.
    /// - "M3"
    /// - "m6"
    /// - "Aug4"
    /// - "Dim5"
    ///
    /// into an `Interval` object.
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut quality = None;
        let mut degree = None;

        if s.starts_with("P") {
            quality = Some(IntervalQuality::Perfect);
            degree = s[1..].parse().ok();
        } else if let Some(remainder) = s.strip_prefix("Aug") {
            quality = Some(IntervalQuality::Augmented);
            degree = remainder.parse().ok();
        } else if let Some(remainder) = s.strip_prefix("Dim") {
            quality = Some(IntervalQuality::Diminished);
            degree = remainder.parse().ok();
        } else if s.starts_with("M") {
            quality = Some(IntervalQuality::Major);
            degree = s[1..].parse().ok();
        } else if s.starts_with("m") {
            quality = Some(IntervalQuality::Minor);
            degree = s[1..].parse().ok();
        }

        match (quality, degree) {
            (Some(q), Some(d)) => Self::from_quality_degree(q, d),
            _ => Err(MusicError::IntervalParseError { name: s.to_owned() }),
        }
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
        let mut interval = Interval::from_semitones(4).unwrap(); // Major 3rd
        interval.invert();
        assert_eq!(interval.semitones, 8); // Minor 6th
        assert_eq!(interval.quality, IntervalQuality::Minor);
        assert_eq!(interval.degree.0, 6);
    }

    #[test]
    fn test_descending_interval() -> Result<(), MusicError> {
        let interval = Interval::from_semitones(-24)?;
        assert_eq!(interval.quality, IntervalQuality::Perfect);

        let tuning = tuning!(C 3);
        let new_tuning = tuning.add_interval(&interval)?;
        assert_eq!(new_tuning.class, PitchClass::C);
        assert_eq!(new_tuning.octave, 1);

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
}

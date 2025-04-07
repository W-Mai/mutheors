//! This module contains the parser for the Chord struct.
//! It is responsible for converting a string representation of a chord into a Chord object.
//!
//! The parser supports the following formats:
//! - "C" for C major
//! - "Cm" for C minor
//! - "C7" for C dominant 7th
//! - "Cmaj7" for C major 7th
//! - "C#7" for C# dominant 7th
//! - "C#m" for C# minor
//! - "C#m7" for C# minor 7th
//! - "C#maj7" for C# major 7th
//! - "Dbdim" for Db diminished
//! - "Db dim7" for Db diminished 7th

use crate::{Chord, ChordQuality, Interval, MusicError, Tuning};
use std::collections::BTreeSet;
use std::str::FromStr;

impl FromStr for Chord {
    type Err = MusicError;

    /// Eg：
    /// - "Cmaj7"   => C Major 7th chord
    /// - "G7/B"    => G Dominant 7th chord with B bass
    /// - "Dm9"     => D minor 9th chord
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.split_whitespace().collect::<String>();
        let mut chars = chars.chars().peekable();
        let root = Tuning::take(chars.by_ref())?;
        let quality = ChordQuality::from_str(&chars.collect::<String>())?;

        Chord::new(root, quality)
    }
}

impl Chord {
    pub fn analyze_from(tunings: &[Tuning]) -> Result<Self, MusicError> {
        if tunings.is_empty() {
            return Err(MusicError::InvalidPitch);
        }

        let number_set = tunings.iter().map(|t| t.number()).collect::<BTreeSet<_>>();
        let min_tuning = *tunings
            .iter()
            .min_by(|t0, t1| t0.number().cmp(&t1.number()))
            .ok_or(MusicError::InvalidPitch)?;
        let tuning_classes = number_set.iter().map(|&t| t % 12).collect::<BTreeSet<_>>();

        for root_class in tuning_classes.iter().by_ref() {
            let intervals_sorted = tuning_classes
                .iter()
                .skip(1)
                .by_ref()
                .filter_map(|&t| Interval::from_semitones(t - root_class).ok())
                .collect::<Vec<_>>();

            let chord_quality = ChordQuality::analyze_from(&intervals_sorted)?;

            return Chord::new(min_tuning, chord_quality);
        }

        Err(MusicError::UnsupportedChord {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_chord_parser() -> Result<(), MusicError> {
        let chord = Chord::from_str("C")?;
        println!("Parsed chord: {}", chord);

        let chord = Chord::from_str("C#m")?;
        println!("Parsed chord: {}", chord);

        let chord = Chord::from_str("CmM7")?;
        println!("Parsed chord: {}", chord);

        let chord = Chord::from_str("Gbsus2")?;
        println!("Parsed chord: {}", chord);

        Ok(())
    }

    #[test]
    fn test_chord_analyze_from() -> Result<(), MusicError> {
        let tunings = vec![tuning!(C 5), tuning!(b E 4), tuning!(G 4), tuning!(B 4)];

        let chord = Chord::analyze_from(&tunings)?;
        println!("Analyzed chord: {}", chord);

        Ok(())
    }

    #[test]
    fn test_chord_analyze_supported() {
        for quality in ChordQuality::iter() {
            let c = Chord::new(tuning!(C 4), quality).unwrap();
            println!("Chord: {}, components: {:?}", c, c.components());
            let c2 = Chord::analyze_from(&c.components()).unwrap();

            assert_eq!(c.simple(), c2.simple());
        }
    }

    #[test]
    fn test_chord_from_symbol() -> Result<(), MusicError> {
        let c = Chord::new(tuning!(C 4), ChordQuality::Add9)?;
        println!("Chord: {}, components: {:?}", c, c.components());
        let c2 = Chord::analyze_from(&c.components())?;

        assert_eq!(c, c2);
        Ok(())
    }
}

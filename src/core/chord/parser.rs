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

use crate::{Chord, ChordQuality, MusicError, Tuning};
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

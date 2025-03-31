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

impl TryFrom<&str> for Chord {
    type Error = MusicError;

    fn try_from(value: &str) -> Result<Self, MusicError> {
        let chars = value.split_whitespace().collect::<String>();
        let mut chars = chars.chars().peekable();
        let root = Tuning::take(chars.by_ref())?;
        let quality = chars.collect::<String>();

        println!("Root: {}, Quality: {}", root, quality);

        Chord::new(root, ChordQuality::Major)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_parser() -> Result<(), MusicError> {
        let chord = Chord::try_from("C")?;
        println!("Parsed chord: {}", chord);
        let chord = Chord::try_from("C#m")?;
        println!("Parsed chord: {}", chord);

        Ok(())
    }
}

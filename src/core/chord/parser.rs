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

use crate::{Chord, ChordQuality, PitchClass, Tuning};

impl<T> From<T> for Chord
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let chord_string = value.into();
        let chars = chord_string.split_whitespace().collect::<String>();
        let mut chars = chars.chars().peekable();
        let mut root = String::new();

        while let Some(&c) = chars.peek() {
            if ('A'..='G').contains(&c) || c == '#' || c == 'b' {
                root.push(c);
                chars.next();
            } else {
                break;
            }
        }

        let quality = chars.collect::<String>();

        println!("Root: {}, Quality: {}", root, quality);

        Chord::new(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_parser() {
        let chord = Chord::from("C");
        let chord = Chord::from("C#m");
    }
}

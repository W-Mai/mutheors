use crate::{Chord, Note};
use std::fmt::Display;

#[derive(Clone)]
pub enum Measure {
    Rest,
    Chords(Vec<Chord>),
    Note(Vec<Note>),
}

impl Measure {
    pub fn new() -> Self {
        Self::Rest
    }

    pub fn rest(&mut self) {
        *self = Self::Rest;
    }

    pub fn chord(&mut self, chord: Chord) {
        *self = Self::Chords(vec![chord]);
    }

    pub fn chords(&mut self, chords: Vec<Chord>) {
        *self = Self::Chords(chords);
    }

    pub fn note(&mut self, notes: Vec<Note>) {
        *self = Self::Note(notes);
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Measure::Rest => write!(f, "{}", "Rest"),
            Measure::Chords(chords) => write!(
                f,
                "{}",
                chords
                    .iter()
                    .map(|chord| format!("{}{}", chord.root(), chord.quality()))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Measure::Note(notes) => {
                let notes_str: Vec<String> = notes.iter().map(|n| n.to_string()).collect();
                write!(f, "{}", notes_str.join(" "))
            }
        }
    }
}

impl<const NOTE_COUNT: usize> From<[Note; NOTE_COUNT]> for Measure {
    fn from(value: [Note; NOTE_COUNT]) -> Self {
        Measure::Note(value.to_vec())
    }
}

impl From<Vec<Note>> for Measure {
    fn from(value: Vec<Note>) -> Self {
        Measure::Note(value)
    }
}

impl From<Chord> for Measure {
    fn from(value: Chord) -> Self {
        let mut m = Measure::new();
        m.chord(value);
        m
    }
}

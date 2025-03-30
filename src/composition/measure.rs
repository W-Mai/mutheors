use crate::{Chord, Note};
use std::fmt::Display;

#[derive(Clone)]
pub enum Measure {
    Rest,
    Chord(Chord),
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
        *self = Self::Chord(chord);
    }

    pub fn note(&mut self, notes: Vec<Note>) {
        *self = Self::Note(notes);
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Measure::Rest => write!(f, "{}", "Rest"),
            Measure::Chord(chord) => write!(f, "{}{}", chord.root(), chord.quality()),
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
        Measure::Chord(value)
    }
}

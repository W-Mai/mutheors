use crate::{Chord, Note};

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

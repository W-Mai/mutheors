#[cfg(test)]
mod tests {
    use std::ops::Rem;
    use simple_compose::*;

    #[test]
    fn test_modulation() {
        assert_eq!(Tuning::C.modulation(4), Tuning::E);
        assert_eq!(Tuning::C.modulation(-3), Tuning::A);
        assert_eq!(Tuning::C.modulation(12), Tuning::C);
        assert_eq!(Tuning::C.modulation(-12), Tuning::C);
        assert_eq!(Tuning::C.modulation(0), Tuning::C);
        assert_eq!(Tuning::None.modulation(1), Tuning::None);
    }

    #[test]
    fn test_breakdown() {
        let tuning = Tuning::C;
        let chord = tuning.common_chord(1);
        assert_eq!(chord.tonality, Tonality::Major);
        let notes = chord.breakdown(4);
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].chord, Tuning::C);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].chord, Tuning::E);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].chord, Tuning::G);
        assert_eq!(notes[2].octave, 4);

        let chord = tuning.common_chord(2);
        assert_eq!(chord.tonality, Tonality::Minor);
        let notes = chord.breakdown(4);
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].chord, Tuning::D);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].chord, Tuning::F);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].chord, Tuning::A);
        assert_eq!(notes[2].octave, 4);

        let chord = tuning.common_chord(6);
        assert_eq!(chord.tonality, Tonality::Minor);
    }
}

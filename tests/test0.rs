#[cfg(test)]
mod tests {
    use mutheors::*;

    #[test]
    fn test_modulation() {
        assert_eq!(PitchClass::C.modulation(4), PitchClass::E);
        assert_eq!(PitchClass::C.modulation(-3), PitchClass::A);
        assert_eq!(PitchClass::C.modulation(12), PitchClass::C);
        assert_eq!(PitchClass::C.modulation(-12), PitchClass::C);
        assert_eq!(PitchClass::C.modulation(0), PitchClass::C);
        assert_eq!(PitchClass::None.modulation(1), PitchClass::None);
    }

    #[test]
    fn test_breakdown() {
        let pitch_class = PitchClass::C;
        let chord = pitch_class.common_chord(1, 4);
        assert_eq!(chord.quality(), ChordQuality::Major);
        let notes = chord.components();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].class, PitchClass::C);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].class, PitchClass::E);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].class, PitchClass::G);
        assert_eq!(notes[2].octave, 4);

        let chord = pitch_class.common_chord(2, 4);
        assert_eq!(chord.quality(), ChordQuality::Minor);
        let notes = chord.components();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].class, PitchClass::D);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].class, PitchClass::F);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].class, PitchClass::A);
        assert_eq!(notes[2].octave, 4);

        let chord = pitch_class.common_chord(6, 2);
        assert_eq!(chord.quality(), ChordQuality::Minor);
    }

    #[test]
    fn test_breakdown_2() {
        let pitch_class = PitchClass::CSharpOrDFlat;
        let chord = pitch_class.common_chord(1, 4);
        let notes = chord.components();

        assert_eq!(
            chord,
            Chord::triad(
                Tuning::new(PitchClass::CSharpOrDFlat, 4),
                ChordQuality::Major
            )
            .unwrap()
        );
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].class, PitchClass::CSharpOrDFlat);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].class, PitchClass::F);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].class, PitchClass::GSharpOrAFlat);
        assert_eq!(notes[2].octave, 4);
    }

    #[test]
    fn test_pitch_class_1() {
        let pitch_class = PitchClass::CSharpOrDFlat;
        assert_eq!(pitch_class.next_basic_degree(0), PitchClass::CSharpOrDFlat);
        assert_eq!(pitch_class.next_basic_degree(1), PitchClass::DSharpOrEFlat);
        assert_eq!(pitch_class.next_basic_degree(2), PitchClass::F);
        assert_eq!(pitch_class.next_basic_degree(3), PitchClass::FSharpOrGFlat);
        assert_eq!(pitch_class.next_basic_degree(4), PitchClass::GSharpOrAFlat);
        assert_eq!(pitch_class.next_basic_degree(5), PitchClass::ASharpOrBFlat);
        assert_eq!(pitch_class.next_basic_degree(6), PitchClass::C);
        assert_eq!(pitch_class.next_basic_degree(7), PitchClass::CSharpOrDFlat);
    }
}

#[cfg(test)]
mod tests {
    use simple_compose::*;

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
        let chord = pitch_class.common_chord(1);
        assert_eq!(chord.tonality, Tonality::Major);
        let notes = chord.breakdown(4);
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch_class, PitchClass::C);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].pitch_class, PitchClass::E);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].pitch_class, PitchClass::G);
        assert_eq!(notes[2].octave, 4);

        let chord = pitch_class.common_chord(2);
        assert_eq!(chord.tonality, Tonality::Minor);
        let notes = chord.breakdown(4);
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch_class, PitchClass::D);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].pitch_class, PitchClass::F);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].pitch_class, PitchClass::A);
        assert_eq!(notes[2].octave, 4);

        let chord = pitch_class.common_chord(6);
        assert_eq!(chord.tonality, Tonality::Minor);
    }

    #[test]
    fn test_breakdown_2() {
        let pitch_class = PitchClass::CSharpOrDFlat;
        let chord = pitch_class.common_chord(1);
        let notes = chord.breakdown(4);

        assert_eq!(
            chord,
            Chord {
                pitch_class: PitchClass::CSharpOrDFlat,
                tonality: Tonality::Major
            }
        );
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch_class, PitchClass::CSharpOrDFlat);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].pitch_class, PitchClass::F);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].pitch_class, PitchClass::GSharpOrAFlat);
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

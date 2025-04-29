#[cfg(test)]
mod tests {
    use mutheors::*;

    #[test]
    fn test_breakdown() {
        let pitch_class = PitchClass::C;
        let chord = pitch_class.common_chord(1, 4);
        assert_eq!(chord.quality(), ChordQuality::Major);
        let notes = chord.components();
        assert_eq!(notes, [tuning!(C 4), tuning!(E 4), tuning!(G 4)]);

        let chord = pitch_class.common_chord(2, 4);
        assert_eq!(chord.quality(), ChordQuality::Minor);
        let notes = chord.components();
        assert_eq!(notes, [tuning!(D 4), tuning!(F 4), tuning!(A 4)]);

        let chord = pitch_class.common_chord(6, 2);
        assert_eq!(chord.quality(), ChordQuality::Minor);
    }

    #[test]
    fn test_breakdown_2() {
        let tuning = Tuning::new(PitchClass::C.sharp(), 4);
        let chord = tuning.common_chord(1);

        assert_eq!(
            chord,
            Chord::new(Tuning::new(PitchClass::C, 4).sharp(), ChordQuality::Major).unwrap()
        );
    }
}

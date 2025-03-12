#[cfg(test)]
mod tests {
    use mutheors::*;

    fn sum_duration(dg: &DurationGenerator, ds: &Vec<Note>) -> f32 {
        ds.iter().fold(0.0f32, |acc, x| acc + x.duration().in_beats(dg))
    }

    fn do_a_measure_test(beat: u8) {
        let dg = DurationGenerator::new(DurationBase::Quarter);
        let chord = Chord::triad(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap();
        let measure = duration_utils::generate_one_measure(&dg, chord, beat);
        match measure {
            Measure::Note(notes) => {
                println!(
                    "{}",
                    (&notes)
                        .iter()
                        .fold("".to_owned(), |acc, x| format!("{} {}", acc, x))
                );
                assert_eq!(sum_duration(&dg, &notes), beat as f32);
            }
            _ => {
                assert!(false, "Expected a Note measure");
            }
        }
    }

    #[test]
    fn test_duration_1() {
        assert_eq!(
            Duration::new(DurationBase::Whole) + Duration::new(DurationBase::Half),
            1.5
        );
        assert_eq!(
            <Duration as Into<f32>>::into(Duration::new(DurationBase::Half)),
            0.5
        );
        assert_eq!(Duration::from(0.5).base, DurationBase::Half);
    }

    #[test]
    fn test_duration_2() {
        assert_eq!(Duration::new(DurationBase::Half).to_string(), "𝅗𝅥");
    }

    #[test]
    fn test_measure_1() {
        for i in 1..16 {
            for _ in 0..8 {
                do_a_measure_test(i);
            }
        }
    }
}

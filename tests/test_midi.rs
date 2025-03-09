#[cfg(all(test))]
mod tests {
    use super::*;
    use mutheors::*;

    #[test]
    fn test_score_with_midi_player() {
        let mut score = Score::<2>::new()
            .with_tempo(140.0)
            .with_time_signature((4, 4));
        score.new_measures(|m| {
            m[0].rest();
            m[1].note(vec![
                Note::new(Tuning::new(PitchClass::C, 4))
                    .with_duration(Duration::new(DurationBase::Quarter)),
                Note::new(Tuning::new(PitchClass::E, 4))
                    .with_duration(Duration::new(DurationBase::Eighth)),
                Note::new(Tuning::new(PitchClass::G, 4))
                    .with_duration(Duration::new(DurationBase::Eighth)),
                Note::new(Tuning::new(PitchClass::B, 4))
                    .with_duration(Duration::new(DurationBase::Eighth)),
                Note::new(Tuning::new(PitchClass::D, 5))
                    .with_duration(Duration::new(DurationBase::Eighth)),
            ]);
        });
        score.new_measures(|m| {
            m[0].chord(Chord::triad(Tuning::new(PitchClass::G, 4), ChordQuality::Major).unwrap());
            m[1].chord(Chord::triad(Tuning::new(PitchClass::B, 3), ChordQuality::Major).unwrap());
        });
        score.new_measures(|m| {
            m[0].note(vec![
                Note::new(Tuning::new(PitchClass::C, 4))
                    .with_duration(Duration::new(DurationBase::Quarter)),
                Note::new(Tuning::new(PitchClass::E, 4))
                    .with_duration(Duration::new(DurationBase::Quarter)),
                Note::new(Tuning::new(PitchClass::B, 4))
                    .with_duration(Duration::new(DurationBase::Eighth)),
                Note::new(Tuning::new(PitchClass::G, 4))
                    .with_duration(Duration::new(DurationBase::Eighth)),
                Note::new(Tuning::new(PitchClass::E, 4))
                    .with_duration(Duration::new(DurationBase::Eighth)),
                Note::new(Tuning::new(PitchClass::C, 5))
                    .with_duration(Duration::new(DurationBase::Eighth)),
            ]);
            m[1].rest();
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();
    }
}

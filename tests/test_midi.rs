macro_rules! degrees {
    ($($degree:expr)*) => {
        [$($degree),*]
    };
}

#[cfg(all(test))]
mod tests {
    use mutheors::*;
    use rand::prelude::*;
    use rand::rng;

    #[test]
    fn test_score_with_midi_player() {
        let mut score = Score::<2>::new()
            .with_tempo(140.0)
            .with_time_signature(4, DurationBase::Quarter);

        let dg = score.duration_generator();

        score.new_measures(|m| {
            m[0].rest();
            m[1].note(beats!(dg;
                1.0 => tuning!(C 4),
                0.5 => tuning!(E 4),
                0.5 => tuning!(G 4),
                0.5 => tuning!(B 4),
                0.5 => tuning!(D 5)
            ));
        });
        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(G 4), ChordQuality::Major).unwrap());
            m[1].chord(Chord::triad(tuning!(B 3), ChordQuality::Major).unwrap());
        });
        score.new_measures(|m| {
            m[0].note(beats!(dg;
                1.0 => tuning!(C 4),
                1.0 => tuning!(E 4),
                0.5 => tuning!(B 4),
                0.5 => tuning!(G 4),
                0.5 => tuning!(E 4),
                0.5 => tuning!(C 5)
            ));
            m[1].rest();
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();
    }

    #[test]
    fn test_score_with_midi_player_2() {
        let mut score = Score::<2>::new()
            .with_tempo(180.0)
            .with_time_signature(4, DurationBase::Quarter);

        let dg = score.duration_generator().clone();

        score.new_measures(|m| {
            m[0].rest();
            m[1].note(beats!(dg;
                1.0 => tuning!(C 4),
                1.0 => tuning!(E 4),
                1.0 => tuning!(G 4),
                1.0 => tuning!(C 5)
            ));
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();
        midi_player.close();
    }

    #[test]
    fn test_random_measure() {
        let pitch_class = PitchClass::G;
        let deg = degrees!(1 5 6 2 4 1 4 5 1 5 6 2 4 1 4 5 1 5 6 2 4 1 4 5 1 5 6 2 4 1 4 5 1 1);
        let chords = deg.map(|degree| pitch_class.common_chord(degree, 4));

        let mut score = Score::<2>::new().with_tempo(140);
        let mut rng = rng();

        (0..deg.len()).for_each(|i| {
            score.new_measures(|m| {
                m[0].chord(chords[i].clone());

                let chord_notes = chords[i].components();
                let durations = duration_utils::generate_one_measure(4);
                let note_iter = durations
                    .iter()
                    .map(|duration| {
                        let tuning = chord_notes.choose(&mut rng).unwrap().clone();
                        Note::new(tuning.add_interval(&Interval::from_semitones(12).unwrap()))
                            .with_duration(duration.clone())
                    })
                    .collect();

                m[1].note(note_iter);
            })
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();

        midi_player.close();
    }

    #[test]
    fn test_two_tigers() {
        let mut score = Score::<2>::new()
            .with_tempo(Tempo::Vivace)
            .with_time_signature(4, DurationBase::Quarter);

        let dg = score.duration_generator();

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(C 4), ChordQuality::Major).unwrap());
            m[1].note(beats!(dg;
                1.0 => tuning!(C 4),
                1.0 => tuning!(D 4),
                1.0 => tuning!(E 4),
                1.0 => tuning!(C 4),
            ));
        });

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(C 4), ChordQuality::Major).unwrap());
            m[1].note(beats!(dg;
                1.0 => tuning!(C 4),
                1.0 => tuning!(D 4),
                1.0 => tuning!(E 4),
                1.0 => tuning!(C 4),
            ));
        });

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(G 4), ChordQuality::Major).unwrap());
            m[1].note(beats!(dg;
                1.0 => tuning!(E 4),
                1.0 => tuning!(F 4),
                2.0 => tuning!(G 4),
            ));
        });

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(G 4), ChordQuality::Major).unwrap());
            m[1].note(beats!(dg;
                1.0 => tuning!(E 4),
                1.0 => tuning!(F 4),
                2.0 => tuning!(G 4),
            ));
        });

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(A 4), ChordQuality::Minor).unwrap());
            m[1].note(beats!(dg;
                0.5 => tuning!(G 4),
                0.5 => tuning!(A 4),
                0.5 => tuning!(G 4),
                0.5 => tuning!(F 4),
                1.0 => tuning!(E 4),
                1.0 => tuning!(C 4),
            ));
        });

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(A 4), ChordQuality::Minor).unwrap());
            m[1].note(beats!(dg;
                0.5 => tuning!(G 4),
                0.5 => tuning!(A 4),
                0.5 => tuning!(G 4),
                0.5 => tuning!(F 4),
                1.0 => tuning!(E 4),
                1.0 => tuning!(C 4),
            ));
        });

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(E 4), ChordQuality::Minor).unwrap());
            m[1].note(beats!(dg;
                1.0 => tuning!(E 4),
                1.0 => tuning!(G 3),
                2.0 => tuning!(C 4),
            ));
        });

        score.new_measures(|m| {
            m[0].chord(Chord::triad(tuning!(E 4), ChordQuality::Minor).unwrap());
            m[1].note(beats!(dg;
                1.0 => tuning!(E 4),
                1.0 => tuning!(G 3),
                2.0 => tuning!(C 4),
            ));
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();
    }
}

macro_rules! degrees {
    ($($degree:expr)*) => {
        [$($degree),*]
    };
}

#[cfg(all(test))]
mod tests {
    use mutheors::*;

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
        let dg = score.duration_generator();

        (0..deg.len()).for_each(|i| {
            score.new_measures(|m| {
                m[0].chord(chords[i].clone());
                m[1] = duration_utils::generate_one_measure(&dg, chords[i].clone(), 4);
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

    #[test]
    fn test_two_tigers_with_diff_scale_type() {
        let mut score = Score::<2>::new()
            .with_tempo(Tempo::Vivace)
            .with_time_signature(4, DurationBase::Quarter);

        let scale = Scale::new(tuning!(C 4), ScaleType::Hirajoshi).unwrap();
        let dg = score.duration_generator();

        (0..2).for_each(|_| {
            score.new_measures(|m| {
                m[0].chord(Chord::seventh(tuning!(C 4), ChordQuality::Dominant7).unwrap());
                m[1].note(beats!(dg;
                    1.0 => scale.degree(1).unwrap(),
                    1.0 => scale.degree(2).unwrap(),
                    1.0 => scale.degree(3).unwrap(),
                    1.0 => scale.degree(1).unwrap(),
                ));
            });
        });

        (0..2).for_each(|_| {
            score.new_measures(|m| {
                m[0].chord(Chord::triad(tuning!(G 4), ChordQuality::Diminished).unwrap());
                m[1].note(beats!(dg;
                    1.0 => scale.degree(3).unwrap(),
                    1.0 => scale.degree(4).unwrap(),
                    2.0 => scale.degree(5).unwrap(),
                ));
            });
        });

        (0..2).for_each(|_| {
            score.new_measures(|m| {
                m[0].chord(Chord::seventh(tuning!(A 4), ChordQuality::FullyDiminished).unwrap());
                m[1].note(beats!(dg;
                    0.5 => scale.degree(5).unwrap(),
                    0.5 => scale.degree(6).unwrap(),
                    0.5 => scale.degree(5).unwrap(),
                    0.5 => scale.degree(4).unwrap(),
                    1.0 => scale.degree(3).unwrap(),
                    1.0 => scale.degree(1).unwrap(),
                ));
            });
        });

        (0..2).for_each(|_| {
            score.new_measures(|m| {
                m[0].chord(Chord::triad(tuning!(E 4), ChordQuality::Major).unwrap());
                m[1].note(beats!(dg;
                    1.0 => scale.degree(3).unwrap(),
                    1.0 => scale.degree(5).unwrap() / 2,
                    2.0 => scale.degree(1).unwrap(),
                ));
            });
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();
    }

    #[test]
    fn test_degrees() {
        let mut score = Score::<1>::new()
            .with_tempo(Tempo::Vivace)
            .with_time_signature(14, DurationBase::Quarter);

        let s = Scale::new(tuning!(C 4), ScaleType::PentatonicMajor).unwrap();
        let dg = score.duration_generator();

        score.new_measures(|m| {
            m[0].note(beats!(dg;
                1.0 => s - 6,
                1.0 => s - 5,
                1.0 => s - 4,
                1.0 => s - 3,
                1.0 => s - 2,
                1.0 => s - 1,
                1.0 => s,
                1.0 => s + 1,
                1.0 => s + 2,
                1.0 => s + 3,
                1.0 => s + 4,
                1.0 => s + 5,
                1.0 => s + 6,
            ));
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();
        midi_player.close();
    }
}

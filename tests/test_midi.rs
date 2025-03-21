#![feature(iter_array_chunks)]

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

        score.play(func!()).unwrap()
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

        score.play(func!()).unwrap()
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

        score.play(func!()).unwrap()
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

        score.play(func!()).unwrap()
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
                    1.0 => scale(1),
                    1.0 => scale(2),
                    1.0 => scale(3),
                    1.0 => scale(1),
                ));
            });
        });

        (0..2).for_each(|_| {
            score.new_measures(|m| {
                m[0].chord(Chord::triad(tuning!(G 4), ChordQuality::Diminished).unwrap());
                m[1].note(beats!(dg;
                    1.0 => scale(3),
                    1.0 => scale(4),
                    2.0 => scale(5),
                ));
            });
        });

        (0..2).for_each(|_| {
            score.new_measures(|m| {
                m[0].chord(Chord::seventh(tuning!(A 4), ChordQuality::FullyDiminished).unwrap());
                m[1].note(beats!(dg;
                    0.5 => scale(5),
                    0.5 => scale(6),
                    0.5 => scale(5),
                    0.5 => scale(4),
                    1.0 => scale(3),
                    1.0 => scale(1),
                ));
            });
        });

        (0..2).for_each(|_| {
            score.new_measures(|m| {
                m[0].chord(Chord::triad(tuning!(E 4), ChordQuality::Major).unwrap());
                m[1].note(beats!(dg;
                    1.0 => scale(3),
                    1.0 => scale(5) / 2,
                    2.0 => scale(1),
                ));
            });
        });

        score.play(func!()).unwrap()
    }

    #[test]
    fn test_degrees() {
        let mut score = Score::<1>::new()
            .with_tempo(Tempo::Vivace)
            .with_time_signature(16, DurationBase::Quarter);

        let s = Scale::new(tuning!(C 4), ScaleType::PentatonicMajor).unwrap();
        let dg = score.duration_generator();

        score.new_measures(|m| {
            m[0].note(beats!(dg;
                1.0 => s - 8,
                1.0 => s - 7,
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
                1.0 => s + 7,
            ));
        });

        score.play(func!()).unwrap()
    }

    #[test]
    fn test_degree_scale_iter() {
        let mut score = Score::<1>::new()
            .with_tempo(480)
            .with_time_signature(16, DurationBase::Quarter);

        let s = Scale::new(tuning!(C 0), ScaleType::Blues).unwrap();
        let dg = score.duration_generator();

        for chunk in s.into_iter().array_chunks::<4>() {
            score.new_measures(|m| {
                m[0].note(beats!(dg;
                    0.75 => chunk[0],
                    0.25 => chunk[1],
                    2.00 => chunk[1],

                    0.75 => chunk[1],
                    0.25 => chunk[2],
                    2.00 => chunk[2],

                    1.0 => chunk[2],
                    2.0 => chunk[3],
                    1.0 => chunk[3],

                    1.0 => chunk[2],
                    1.0 => chunk[3],
                    1.0 => chunk[2],
                    3.0 => chunk[0],
                ));
            });
        }

        score.play(func!()).unwrap()
    }
}

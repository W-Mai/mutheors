mod measure;
mod score;
mod track;

pub use measure::*;
pub use score::*;
pub use track::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Chord, ChordQuality, DurationBase, PitchClass, Tuning};

    #[test]
    fn test_score_creation() {
        let score = Score::<4>::new();
        assert_eq!(score.get_tracks().len(), 4);
        assert_eq!(score.tempo(), 120.0);
        assert_eq!(
            score.time_signature(),
            &TimeSignature::new(4, DurationBase::Quarter)
        );
    }

    #[test]
    fn test_score_with_tempo() {
        let score = Score::<4>::new().with_tempo(140.0);
        assert_eq!(score.tempo(), 140.0);
    }

    #[test]
    fn test_score_with_time_signature() {
        let mut score = Score::<4>::new().with_time_signature(3, DurationBase::Quarter);
        assert_eq!(
            score.time_signature(),
            &TimeSignature::new(3, DurationBase::Quarter)
        );

        score.new_measures(|ms| {
            ms[0].chord(Chord::triad(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap());
            ms[1].chord(Chord::triad(Tuning::new(PitchClass::G, 4), ChordQuality::Major).unwrap());
            ms[2].chord(Chord::triad(Tuning::new(PitchClass::A, 4), ChordQuality::Major).unwrap());
            ms[3].chord(Chord::triad(Tuning::new(PitchClass::F, 4), ChordQuality::Major).unwrap());
        });

        score.new_measures(|ms| {
            ms[0].chord(Chord::triad(Tuning::new(PitchClass::D, 4), ChordQuality::Major).unwrap());
            ms[1].rest();
            ms[2].rest();
            ms[3].chord(Chord::triad(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap());
        })
    }
}

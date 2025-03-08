mod measure;
mod score;
mod track;

pub use measure::*;
pub use score::*;
pub use track::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_creation() {
        let score = Score::<4>::new();
        assert_eq!(score.tracks.len(), 4);
        assert_eq!(score.tempo, 120.0);
        assert_eq!(score.time_signature, (4, 4));
    }

    #[test]
    fn test_score_with_tempo() {
        let score = Score::<4>::new().with_tempo(140.0);
        assert_eq!(score.tempo, 140.0);
    }

    #[test]
    fn test_score_with_time_signature() {
        let mut score = Score::<4>::new().with_time_signature((3, 4));
        assert_eq!(score.time_signature, (3, 4));

        score.new_measures(|ms| {
            ms[0].test();
            ms[1].test();
            ms[2].test();
            ms[3].test();
        });
    }
}

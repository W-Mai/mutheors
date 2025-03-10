use crate::composition::measure::Measure;
use crate::composition::track::Track;
use std::array;
use std::fmt::Display;

pub struct Score<const TRACK_COUNT: usize> {
    pub tracks: [Track; TRACK_COUNT],
    pub tempo: f32,
    pub time_signature: (u8, u8),
}

impl<const TRACK_COUNT: usize> Score<TRACK_COUNT> {
    pub fn new() -> Self {
        Score {
            tracks: array::from_fn(|_| Track::new()),
            tempo: 120.0,
            time_signature: (4, 4),
        }
    }

    pub fn with_tempo(self, tempo: f32) -> Self {
        Score { tempo, ..self }
    }

    pub fn with_time_signature(self, time_signature: (u8, u8)) -> Self {
        Score {
            time_signature,
            ..self
        }
    }

    pub fn push_measures(&mut self, measures: [Measure; TRACK_COUNT]) {
        self.tracks
            .iter_mut()
            .zip(measures.into_iter())
            .for_each(|(track, measure)| {
                track.push(measure);
            })
    }

    pub fn new_measures<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Measure; TRACK_COUNT]),
    {
        let mut new_measure: [Measure; TRACK_COUNT] = array::from_fn(|_| Measure::new());
        f(&mut new_measure);
        self.push_measures(new_measure);
    }

    pub fn get_tracks(&self) -> &[Track; TRACK_COUNT] {
        &self.tracks
    }

    pub fn tempo(&self) -> f32 {
        self.tempo
    }

    pub fn time_signature(&self) -> (u8, u8) {
        self.time_signature
    }
}

impl<const TRACK_COUNT: usize> Display for Score<TRACK_COUNT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Tempo: {}\n {}\n---\n {}",
            self.tempo, self.time_signature.0, self.time_signature.1
        )?;

        for (_i, track) in self.tracks.iter().enumerate() {
            for measure in track.get_measures() {
                write!(f, "{} ", measure)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

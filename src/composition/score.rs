use crate::composition::measure::Measure;
use crate::composition::track::Track;
use crate::DurationBase;
use std::array;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct TimeSignature {
    beats_per_measure: u8,
    beat_type: DurationBase,
}

pub struct Score<const TRACK_COUNT: usize> {
    tracks: [Track; TRACK_COUNT],
    tempo: f32,
    time_signature: TimeSignature,
}

impl<const TRACK_COUNT: usize> Score<TRACK_COUNT> {
    pub fn new() -> Self {
        Score {
            tracks: array::from_fn(|_| Track::new()),
            tempo: 120.0,
            time_signature: TimeSignature::new(4, DurationBase::Quarter),
        }
    }

    pub fn with_tempo(self, tempo: f32) -> Self {
        Score { tempo, ..self }
    }

    pub fn with_time_signature(self, beats_per_measure: u8, beat_type: DurationBase) -> Self {
        Score {
            time_signature: TimeSignature::new(beats_per_measure, beat_type),
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

        let measure_check = new_measure.iter().enumerate().filter_map(|(i, measure)| {
            return match measure {
                Measure::Note(notes) => {
                    let total = notes.iter().fold(0.0f32, |acc, note| {
                        let duration = note.duration().in_quarters();
                        let new_duration: f32 = duration + acc;
                        new_duration
                    });

                    if total > self.time_signature.beats_per_measure as f32 {
                        Some((i, total))
                    } else {
                        None
                    }
                }
                _ => None,
            };
        });

        measure_check.for_each(|track| {
            eprintln!(
                "Track {}: measure [{}] that exceeds the time signature please check the measures ",
                track.0, track.1
            );
        });
        self.push_measures(new_measure);
    }

    pub fn get_tracks(&self) -> &[Track; TRACK_COUNT] {
        &self.tracks
    }

    pub fn tempo(&self) -> f32 {
        self.tempo
    }

    pub fn time_signature(&self) -> &TimeSignature {
        &self.time_signature
    }
}

impl<const TRACK_COUNT: usize> Display for Score<TRACK_COUNT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Tempo: {}\n {}\n---\n {}",
            self.tempo, self.time_signature.beats_per_measure, self.time_signature.beat_type as u8
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

impl TimeSignature {
    pub fn new(beats_per_measure: u8, beat_type: DurationBase) -> Self {
        TimeSignature {
            beats_per_measure,
            beat_type,
        }
    }

    pub fn beats_per_measure(&self) -> u8 {
        self.beats_per_measure
    }

    pub fn beat_type(&self) -> DurationBase {
        self.beat_type
    }
}

//! Audio analysis module — pitch detection, chroma, chord detection, onset/BPM, key detection
//!
//! Feature-gated under `audio`. Requires `rustfft`.

mod chord_detect;
mod chroma;
mod cqt;
mod fft;
mod key;
mod onset;
mod yin;

pub use chord_detect::{ChordDetectionResult, ChordDetector};
pub use chroma::{cosine_similarity, Chroma};
pub use cqt::{Cqt, CqtResult};
pub use fft::Fft;
pub use key::{detect_key, KeyResult};
pub use onset::{OnsetDetector, OnsetResult};
pub use yin::{detect_pitch, detect_pitch_with_config, PitchResult, YinConfig, YinDetector};

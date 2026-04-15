//! Audio analysis module — pitch detection, chroma extraction, chord detection
//!
//! Feature-gated under `audio`. Requires `rustfft`.

mod fft;
mod yin;
mod cqt;
mod chroma;
mod chord_detect;

pub use fft::Fft;
pub use yin::{detect_pitch, detect_pitch_with_config, PitchResult, YinConfig, YinDetector};
pub use cqt::{Cqt, CqtResult};
pub use chroma::{Chroma, cosine_similarity};
pub use chord_detect::{ChordDetector, ChordDetectionResult};

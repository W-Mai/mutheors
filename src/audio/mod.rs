//! Audio analysis module — pitch detection, chroma extraction, chord detection
//!
//! Feature-gated under `audio`. Requires `rustfft`.

mod fft;
mod yin;
mod chroma;
mod chord_detect;

pub use fft::Fft;
pub use yin::{detect_pitch, detect_pitch_with_config, PitchResult, YinConfig, YinDetector};
pub use chroma::{Chroma, ChromaExtractor, cosine_similarity};
pub use chord_detect::{ChordDetector, ChordDetectionResult};

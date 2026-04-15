//! Audio analysis module — pitch detection, FFT utilities
//!
//! Feature-gated under `audio`. Requires `rustfft`.

mod fft;
mod yin;

pub use fft::Fft;
pub use yin::{detect_pitch, detect_pitch_with_config, PitchResult, YinConfig, YinDetector};

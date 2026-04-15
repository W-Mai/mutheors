//! Chord detection from audio via chroma template matching
//!
//! Two backends:
//! - `ChordDetector::realtime()` — FFT chroma, works with short frames (~100ms)
//! - `ChordDetector::high_quality()` — CQT chroma, better accuracy especially in low register

use super::chroma::{cosine_similarity, Chroma};
use super::cqt::Cqt;
use super::fft::Fft;
use crate::{Chord, ChordQuality, PitchClass, Tuning};

/// Result of chord detection
#[derive(Clone, Debug)]
pub struct ChordDetectionResult {
    pub chord: Chord,
    pub confidence: f32,
    pub chroma: Chroma,
}

/// Chroma extraction backend
enum Backend {
    /// FFT-based: fast, works with any frame size, lower accuracy in bass register
    Realtime { fft: Fft, sample_rate: f32 },
    /// CQT-based: precomputed kernels, needs longer frames, accurate across full range
    HighQuality { cqt: Cqt },
}

impl Backend {
    fn extract_chroma(&mut self, samples: &[f32]) -> Chroma {
        match self {
            Backend::Realtime { fft, sample_rate } => fft_chroma(fft, samples, *sample_rate),
            Backend::HighQuality { cqt } => cqt.transform(samples).to_chroma(),
        }
    }
}

/// Chord detector with unified API, configurable backend.
pub struct ChordDetector {
    backend: Backend,
    templates: Vec<(PitchClass, ChordQuality, Chroma)>,
}

const ROOTS: [PitchClass; 12] = [
    PitchClass::C,
    PitchClass::Cs,
    PitchClass::D,
    PitchClass::Ds,
    PitchClass::E,
    PitchClass::F,
    PitchClass::Fs,
    PitchClass::G,
    PitchClass::Gs,
    PitchClass::A,
    PitchClass::As,
    PitchClass::B,
];

impl ChordDetector {
    /// Fast detector for real-time use. Works with short frames (~100ms+).
    /// Lower accuracy in bass register (below C3).
    pub fn realtime(sample_rate: f32) -> Self {
        Self {
            backend: Backend::Realtime {
                fft: Fft::new(),
                sample_rate,
            },
            templates: Self::build_templates(ChordQuality::iter()),
        }
    }

    /// High-quality detector using CQT. Accurate across full piano range.
    /// Requires frames ≥ CQT fft_size (~0.7s at 44100Hz). One-time init cost ~50ms.
    pub fn high_quality(sample_rate: f32) -> Self {
        Self {
            backend: Backend::HighQuality {
                cqt: Cqt::new(sample_rate, 12),
            },
            templates: Self::build_templates(ChordQuality::iter()),
        }
    }

    /// Create a realtime detector with only triad templates (faster matching).
    pub fn realtime_triads(sample_rate: f32) -> Self {
        Self {
            backend: Backend::Realtime {
                fft: Fft::new(),
                sample_rate,
            },
            templates: Self::build_triads(),
        }
    }

    /// Create a high-quality detector with only triad templates.
    pub fn high_quality_triads(sample_rate: f32) -> Self {
        Self {
            backend: Backend::HighQuality {
                cqt: Cqt::new(sample_rate, 12),
            },
            templates: Self::build_triads(),
        }
    }

    /// Detect chord from audio samples.
    pub fn detect(&mut self, samples: &[f32]) -> Option<ChordDetectionResult> {
        let chroma = self.backend.extract_chroma(samples);
        let energy: f32 = chroma.iter().sum();
        if energy < 1e-6 {
            return None;
        }
        self.match_chroma(&chroma)
    }

    /// Match a precomputed chroma vector against templates.
    pub fn match_chroma(&self, chroma: &Chroma) -> Option<ChordDetectionResult> {
        let (root, quality, confidence) = self
            .templates
            .iter()
            .map(|(r, q, tmpl)| (*r, *q, cosine_similarity(chroma, tmpl)))
            .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())?;

        if confidence < 0.3 {
            return None;
        }

        let chord = Chord::new(Tuning::new(root, 4), quality).ok()?;
        Some(ChordDetectionResult {
            chord,
            confidence,
            chroma: *chroma,
        })
    }

    fn build_triads() -> Vec<(PitchClass, ChordQuality, Chroma)> {
        Self::build_templates(
            [
                ChordQuality::Major,
                ChordQuality::Minor,
                ChordQuality::Diminished,
                ChordQuality::Augmented,
            ]
            .into_iter(),
        )
    }

    fn build_templates(
        qualities: impl Iterator<Item = ChordQuality>,
    ) -> Vec<(PitchClass, ChordQuality, Chroma)> {
        let mut templates = Vec::new();
        let qualities: Vec<_> = qualities.collect();

        for &root in &ROOTS {
            let root_semitone = root.semitones() - 1; // 1-based → 0-based
            for &quality in &qualities {
                let intervals = quality.intervals();
                let mut chroma = [0.0f32; 12];
                chroma[root_semitone as usize % 12] = 1.0;
                for interval in &intervals {
                    let idx = (root_semitone as i16 + interval.semitones() as i16) as usize % 12;
                    chroma[idx] = 1.0;
                }
                let norm: f32 = chroma.iter().map(|x| x * x).sum::<f32>().sqrt();
                if norm > 0.0 {
                    for c in &mut chroma {
                        *c /= norm;
                    }
                }
                templates.push((root, quality, chroma));
            }
        }
        templates
    }
}

/// FFT-based chroma extraction (inline, not publicly exposed)
fn fft_chroma(fft: &mut Fft, samples: &[f32], sample_rate: f32) -> Chroma {
    let n = samples.len();
    let magnitudes = fft.magnitude_spectrum(samples);
    let mut chroma = [0.0f32; 12];
    let freq_min = 27.5f32;
    let freq_max = 4200.0f32;

    for (i, &mag) in magnitudes.iter().enumerate() {
        let freq = i as f32 * sample_rate / n as f32;
        if freq < freq_min || freq > freq_max {
            continue;
        }
        let midi = 12.0 * (freq / 32.703).log2();
        let pitch = ((midi.round() as i32 % 12) + 12) as usize % 12;
        chroma[pitch] += mag;
    }

    let norm: f32 = chroma.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-8 {
        for c in &mut chroma {
            *c /= norm;
        }
    }
    chroma
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn generate_chord_signal(freqs: &[f32], sample_rate: f32, duration: f32) -> Vec<f32> {
        let n = (sample_rate * duration) as usize;
        (0..n)
            .map(|i| {
                freqs
                    .iter()
                    .map(|&f| (2.0 * PI * f * i as f32 / sample_rate).sin())
                    .sum::<f32>()
                    / freqs.len() as f32
            })
            .collect()
    }

    #[test]
    fn realtime_detect_c_major() {
        let signal = generate_chord_signal(&[261.63, 329.63, 392.00], 44100.0, 0.2);
        let mut det = ChordDetector::realtime_triads(44100.0);
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::C);
        assert_eq!(result.chord.quality(), ChordQuality::Major);
    }

    #[test]
    fn realtime_detect_a_minor() {
        let signal = generate_chord_signal(&[440.0, 523.25, 659.25], 44100.0, 0.2);
        let mut det = ChordDetector::realtime_triads(44100.0);
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::A);
        assert_eq!(result.chord.quality(), ChordQuality::Minor);
    }

    #[test]
    fn hq_detect_c2_major() {
        // Low register — FFT would fail here, CQT should succeed
        let mut det = ChordDetector::high_quality_triads(44100.0);
        let n = match &det.backend {
            Backend::HighQuality { cqt } => cqt.fft_size(),
            _ => unreachable!(),
        };
        let signal: Vec<f32> = (0..n)
            .map(|i| {
                let t = i as f32 / 44100.0;
                (2.0 * PI * 65.41 * t).sin()
                    + (2.0 * PI * 82.41 * t).sin()
                    + (2.0 * PI * 98.00 * t).sin()
            })
            .collect();
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::C);
        assert_eq!(result.chord.quality(), ChordQuality::Major);
    }

    #[test]
    fn silence_returns_none() {
        let mut det = ChordDetector::realtime(44100.0);
        assert!(det.detect(&vec![0.0f32; 4096]).is_none());
    }

    #[test]
    fn template_count() {
        let det = ChordDetector::realtime(44100.0);
        assert_eq!(det.templates.len(), 12 * 16);
        let det2 = ChordDetector::realtime_triads(44100.0);
        assert_eq!(det2.templates.len(), 12 * 4);
    }

    #[test]
    fn hq_detect_g_major() {
        let signal = generate_chord_signal(&[196.0, 246.94, 293.66], 44100.0, 0.8);
        let mut det = ChordDetector::high_quality_triads(44100.0);
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::G);
        assert_eq!(result.chord.quality(), ChordQuality::Major);
    }
}

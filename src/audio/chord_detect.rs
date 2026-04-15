//! Chord detection from audio via chroma template matching

use super::chroma::{cosine_similarity, Chroma, ChromaExtractor};
use crate::{Chord, ChordQuality, PitchClass, Tuning};

/// Result of chord detection
#[derive(Clone, Debug)]
pub struct ChordDetectionResult {
    /// Detected chord
    pub chord: Chord,
    /// Detection confidence (0.0–1.0)
    pub confidence: f32,
    /// The chroma vector extracted from the audio
    pub chroma: Chroma,
}

/// Chord detector using chroma template matching.
///
/// Generates templates from all root × quality combinations using mutheors' `ChordQuality::intervals()`,
/// then matches input chroma via cosine similarity.
pub struct ChordDetector {
    extractor: ChromaExtractor,
    templates: Vec<(PitchClass, ChordQuality, Chroma)>,
}

/// All 12 root pitch classes for template generation
const ROOTS: [PitchClass; 12] = [
    PitchClass::C, PitchClass::Cs, PitchClass::D, PitchClass::Ds,
    PitchClass::E, PitchClass::F, PitchClass::Fs, PitchClass::G,
    PitchClass::Gs, PitchClass::A, PitchClass::As, PitchClass::B,
];

impl ChordDetector {
    /// Create a detector with default chord qualities (triads + 7ths + sus).
    pub fn new(sample_rate: f32) -> Self {
        let templates = Self::build_templates(ChordQuality::iter());
        Self {
            extractor: ChromaExtractor::new(sample_rate),
            templates,
        }
    }

    /// Create a detector with only triad templates (faster, fewer false positives).
    pub fn triads_only(sample_rate: f32) -> Self {
        let qualities = [
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Augmented,
        ];
        let templates = Self::build_templates(qualities.into_iter());
        Self {
            extractor: ChromaExtractor::new(sample_rate),
            templates,
        }
    }

    /// Detect chord from audio samples (FFT-based chroma).
    pub fn detect(&mut self, samples: &[f32]) -> Option<ChordDetectionResult> {
        let chroma = self.extractor.extract(samples);
        let energy: f32 = chroma.iter().sum();
        if energy < 1e-6 {
            return None;
        }
        self.match_chroma(&chroma)
    }

    /// Detect chord using CQT-based chroma (better low-frequency resolution).
    pub fn detect_with_cqt(
        &self,
        cqt: &mut super::cqt::Cqt,
        samples: &[f32],
    ) -> Option<ChordDetectionResult> {
        let result = cqt.transform(samples);
        let chroma = result.to_chroma();
        let energy: f32 = chroma.iter().sum();
        if energy < 1e-6 {
            return None;
        }
        self.match_chroma(&chroma)
    }

    /// Match a chroma vector against templates. Useful if you already have a chroma.
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

    /// Build chroma templates from chord qualities.
    fn build_templates(
        qualities: impl Iterator<Item = ChordQuality>,
    ) -> Vec<(PitchClass, ChordQuality, Chroma)> {
        let mut templates = Vec::new();
        let qualities: Vec<_> = qualities.collect();

        for &root in &ROOTS {
            let root_semitone = root.semitones() - 1; // PitchClass semitones are 1-based
            for &quality in &qualities {
                let intervals = quality.intervals();
                let mut chroma = [0.0f32; 12];

                // Root
                chroma[root_semitone as usize % 12] = 1.0;
                // Intervals above root
                for interval in &intervals {
                    let idx = (root_semitone as i16 + interval.semitones() as i16) as usize % 12;
                    chroma[idx] = 1.0;
                }

                // Normalize
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
    fn detect_c_major() {
        let signal = generate_chord_signal(&[261.63, 329.63, 392.00], 44100.0, 0.2);
        let mut det = ChordDetector::triads_only(44100.0);
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::C);
        assert_eq!(result.chord.quality(), ChordQuality::Major);
        assert!(result.confidence > 0.5, "confidence={}", result.confidence);
    }

    #[test]
    fn detect_a_minor() {
        // A4=440, C5=523.25, E5=659.25
        let signal = generate_chord_signal(&[440.0, 523.25, 659.25], 44100.0, 0.2);
        let mut det = ChordDetector::triads_only(44100.0);
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::A);
        assert_eq!(result.chord.quality(), ChordQuality::Minor);
    }

    #[test]
    fn detect_g_major() {
        // G3=196, B3=246.94, D4=293.66
        let signal = generate_chord_signal(&[196.0, 246.94, 293.66], 44100.0, 0.2);
        let mut det = ChordDetector::triads_only(44100.0);
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::G);
        assert_eq!(result.chord.quality(), ChordQuality::Major);
    }

    #[test]
    fn detect_d_minor() {
        // D4=293.66, F4=349.23, A4=440
        let signal = generate_chord_signal(&[293.66, 349.23, 440.0], 44100.0, 0.2);
        let mut det = ChordDetector::triads_only(44100.0);
        let result = det.detect(&signal).unwrap();
        assert_eq!(result.chord.root().class(), PitchClass::D);
        assert_eq!(result.chord.quality(), ChordQuality::Minor);
    }

    #[test]
    fn detect_silence_returns_none() {
        let signal = vec![0.0f32; 4096];
        let mut det = ChordDetector::triads_only(44100.0);
        assert!(det.detect(&signal).is_none());
    }

    #[test]
    fn template_count() {
        let det = ChordDetector::new(44100.0);
        // 12 roots × 16 qualities
        assert_eq!(det.templates.len(), 12 * 16);

        let det2 = ChordDetector::triads_only(44100.0);
        assert_eq!(det2.templates.len(), 12 * 4);
    }
}

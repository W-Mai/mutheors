//! Chroma vector extraction — fold FFT spectrum into 12 pitch classes

use super::fft::Fft;

/// 12-dimensional chroma vector, one bin per pitch class (C=0, C#=1, ..., B=11)
pub type Chroma = [f32; 12];

/// Chroma feature extractor
pub struct ChromaExtractor {
    fft: Fft,
    sample_rate: f32,
    /// Minimum frequency to consider (Hz)
    freq_min: f32,
    /// Maximum frequency to consider (Hz)
    freq_max: f32,
}

impl ChromaExtractor {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            fft: Fft::new(),
            sample_rate,
            freq_min: 65.0,  // ~C2
            freq_max: 2100.0, // ~C7
        }
    }

    /// Extract chroma vector from audio samples.
    ///
    /// FFT magnitude spectrum bins are folded into 12 pitch classes by mapping
    /// each bin's frequency to the nearest semitone (mod 12).
    pub fn extract(&mut self, samples: &[f32]) -> Chroma {
        let n = samples.len();
        let magnitudes = self.fft.magnitude_spectrum(samples);
        let mut chroma = [0.0f32; 12];

        for (i, &mag) in magnitudes.iter().enumerate() {
            let freq = i as f32 * self.sample_rate / n as f32;
            if freq < self.freq_min || freq > self.freq_max {
                continue;
            }
            // Map frequency to pitch class: C=0, C#=1, ..., B=11
            let pitch = (12.0 * (freq / self.freq_min).log2()).round() as usize % 12;
            chroma[pitch] += mag;
        }

        // Normalize to unit vector
        let norm: f32 = chroma.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 1e-8 {
            for c in &mut chroma {
                *c /= norm;
            }
        }
        chroma
    }
}

/// Cosine similarity between two chroma vectors
pub fn cosine_similarity(a: &Chroma, b: &Chroma) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na < 1e-8 || nb < 1e-8 {
        return 0.0;
    }
    dot / (na * nb)
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
    fn c_major_chord_chroma() {
        // C4=261.63, E4=329.63, G4=392.00
        let signal = generate_chord_signal(&[261.63, 329.63, 392.00], 44100.0, 0.1);
        let mut ext = ChromaExtractor::new(44100.0);
        let chroma = ext.extract(&signal);

        // C=0, E=4, G=7 should be the dominant bins
        let c = chroma[0];
        let e = chroma[4];
        let g = chroma[7];

        assert!(c > 0.1, "C should be strong: {}", c);
        assert!(e > 0.1, "E should be strong: {}", e);
        assert!(g > 0.1, "G should be strong: {}", g);
    }

    #[test]
    fn cosine_similarity_identical() {
        let a = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &a) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn cosine_similarity_orthogonal() {
        let a = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert!(cosine_similarity(&a, &b).abs() < 1e-5);
    }
}

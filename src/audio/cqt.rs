//! Constant-Q Transform (CQT) — logarithmic frequency resolution
//!
//! Based on Brown & Puckette (1992): precompute spectral kernels from FFT,
//! then apply sparse kernel multiplication per frame.
//!
//! Each bin corresponds to one semitone (12 bins/octave), giving uniform
//! resolution across the entire musical range.

use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;

/// A single CQT kernel entry: which FFT bin, and what (complex) weight
#[derive(Clone)]
struct KernelEntry {
    fft_bin: usize,
    weight: Complex<f32>,
}

/// Precomputed CQT kernels + FFT planner
pub struct Cqt {
    /// One kernel per CQT bin (semitone)
    kernels: Vec<Vec<KernelEntry>>,
    /// FFT size (must be power of 2, large enough for lowest frequency)
    fft_size: usize,
    sample_rate: f32,
    /// MIDI note number of the lowest bin
    midi_min: i32,
    planner: FftPlanner<f32>,
}

/// CQT output: magnitude per semitone bin
pub struct CqtResult {
    /// Magnitude for each semitone bin, from midi_min upward
    pub magnitudes: Vec<f32>,
    /// MIDI note number of the first bin
    pub midi_min: i32,
}

impl CqtResult {
    /// Fold CQT magnitudes into a 12-dimensional chroma vector
    pub fn to_chroma(&self) -> [f32; 12] {
        let mut chroma = [0.0f32; 12];
        for (i, &mag) in self.magnitudes.iter().enumerate() {
            let pitch_class = (self.midi_min as usize + i) % 12;
            chroma[pitch_class] += mag;
        }
        // Normalize
        let norm: f32 = chroma.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 1e-8 {
            for c in &mut chroma {
                *c /= norm;
            }
        }
        chroma
    }
}

impl Cqt {
    /// Create a CQT covering the full piano range (A0=27.5Hz to C8=4186Hz).
    ///
    /// `bins_per_octave` is typically 12 (semitone resolution) or 24 (quarter-tone).
    pub fn new(sample_rate: f32, bins_per_octave: u32) -> Self {
        // MIDI 21 = A0 (27.5 Hz), MIDI 108 = C8 (4186 Hz)
        let midi_min = 21i32;
        let midi_max = 108i32;
        let num_bins = (midi_max - midi_min + 1) as usize;

        // Q factor: for 12 bins/octave, Q ≈ 17
        let q = 1.0 / (2.0f32.powf(1.0 / bins_per_octave as f32) - 1.0);

        // FFT size: must accommodate the longest window (lowest frequency)
        let freq_min = 440.0 * 2.0f32.powf((midi_min as f32 - 69.0) / 12.0);
        let longest_window = (q * sample_rate / freq_min).ceil() as usize;
        let fft_size = longest_window.next_power_of_two();

        // Precompute spectral kernels
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);

        let mut kernels = Vec::with_capacity(num_bins);
        let min_val = 0.0054f32; // Threshold for sparse kernel (Brown & Puckette: 0.0054)

        for bin in 0..num_bins {
            let midi = midi_min + bin as i32;
            let freq = 440.0 * 2.0f32.powf((midi as f32 - 69.0) / 12.0);
            let window_len = (q * sample_rate / freq).ceil() as usize;

            // Build temporal kernel: windowed complex exponential
            let mut temporal = vec![Complex::new(0.0, 0.0); fft_size];
            let center = fft_size / 2;
            let half = window_len / 2;

            for n in 0..window_len {
                let pos = center + n - half;
                if pos < fft_size {
                    // Hamming window
                    let w = 0.54 - 0.46 * (2.0 * PI * n as f32 / window_len as f32).cos();
                    let phase = 2.0 * PI * freq * n as f32 / sample_rate;
                    temporal[pos] = Complex::new(
                        w * phase.cos() / window_len as f32,
                        w * phase.sin() / window_len as f32,
                    );
                }
            }

            // FFT of temporal kernel → spectral kernel
            fft.process(&mut temporal);

            // Keep only significant entries (sparse)
            let kernel: Vec<KernelEntry> = temporal
                .iter()
                .enumerate()
                .filter(|(_, c)| c.norm() > min_val)
                .map(|(k, c)| KernelEntry {
                    fft_bin: k,
                    weight: c.conj(), // conjugate for correlation
                })
                .collect();

            kernels.push(kernel);
        }

        Self {
            kernels,
            fft_size,
            sample_rate,
            midi_min,
            planner,
        }
    }

    /// Compute CQT of a signal frame.
    ///
    /// Input can be any length; it will be zero-padded or truncated to fft_size.
    pub fn transform(&mut self, samples: &[f32]) -> CqtResult {
        // Zero-pad input to fft_size
        let mut buf: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); self.fft_size];
        let copy_len = samples.len().min(self.fft_size);
        for i in 0..copy_len {
            buf[i] = Complex::new(samples[i], 0.0);
        }

        // Forward FFT of input
        let fft = self.planner.plan_fft_forward(self.fft_size);
        fft.process(&mut buf);

        // Sparse kernel multiplication: CQT[k] = (1/N) * Σ X[j] * K*[j,k]
        let n_inv = 1.0 / self.fft_size as f32;
        let magnitudes: Vec<f32> = self
            .kernels
            .iter()
            .map(|kernel| {
                let sum: Complex<f32> = kernel
                    .iter()
                    .map(|entry| buf[entry.fft_bin] * entry.weight)
                    .sum();
                (sum * n_inv).norm()
            })
            .collect();

        CqtResult {
            magnitudes,
            midi_min: self.midi_min,
        }
    }

    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn num_bins(&self) -> usize {
        self.kernels.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine(freq: f32, sample_rate: f32, n: usize) -> Vec<f32> {
        (0..n)
            .map(|i| (2.0 * PI * freq * i as f32 / sample_rate).sin())
            .collect()
    }

    #[test]
    fn a440_peak_at_correct_bin() {
        let sr = 44100.0;
        let mut cqt = Cqt::new(sr, 12);
        let signal = generate_sine(440.0, sr, cqt.fft_size());
        let result = cqt.transform(&signal);

        // A4 = MIDI 69, bin index = 69 - 21 = 48
        let a4_bin = (69 - result.midi_min) as usize;
        let peak_bin = result
            .magnitudes
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        assert_eq!(peak_bin, a4_bin, "A440 should peak at MIDI 69 bin");
    }

    #[test]
    fn low_c2_resolved() {
        let sr = 44100.0;
        let mut cqt = Cqt::new(sr, 12);
        let signal = generate_sine(65.41, sr, cqt.fft_size()); // C2
        let result = cqt.transform(&signal);

        // C2 = MIDI 36, bin = 36 - 21 = 15
        let c2_bin = (36 - result.midi_min) as usize;
        let peak_bin = result
            .magnitudes
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        assert_eq!(peak_bin, c2_bin, "C2 should peak at MIDI 36 bin");
    }

    #[test]
    fn chroma_from_c_major_chord() {
        let sr = 44100.0;
        let mut cqt = Cqt::new(sr, 12);
        let n = cqt.fft_size();
        // C4 + E4 + G4
        let signal: Vec<f32> = (0..n)
            .map(|i| {
                let t = i as f32 / sr;
                (2.0 * PI * 261.63 * t).sin()
                    + (2.0 * PI * 329.63 * t).sin()
                    + (2.0 * PI * 392.00 * t).sin()
            })
            .collect();

        let result = cqt.transform(&signal);
        let chroma = result.to_chroma();

        // C=0, E=4, G=7 should dominate
        assert!(chroma[0] > 0.1, "C should be strong: {}", chroma[0]);
        assert!(chroma[4] > 0.1, "E should be strong: {}", chroma[4]);
        assert!(chroma[7] > 0.1, "G should be strong: {}", chroma[7]);
    }

    #[test]
    fn low_c2_major_chord_resolved() {
        let sr = 44100.0;
        let mut cqt = Cqt::new(sr, 12);
        let n = cqt.fft_size();
        // C2(65.41) + E2(82.41) + G2(98.00)
        let signal: Vec<f32> = (0..n)
            .map(|i| {
                let t = i as f32 / sr;
                (2.0 * PI * 65.41 * t).sin()
                    + (2.0 * PI * 82.41 * t).sin()
                    + (2.0 * PI * 98.00 * t).sin()
            })
            .collect();

        let result = cqt.transform(&signal);

        // C2=MIDI36, E2=MIDI40, G2=MIDI43
        let c2 = result.magnitudes[(36 - result.midi_min) as usize];
        let ds2 = result.magnitudes[(39 - result.midi_min) as usize]; // D#2 — should be low
        let e2 = result.magnitudes[(40 - result.midi_min) as usize];
        let g2 = result.magnitudes[(43 - result.midi_min) as usize];

        assert!(c2 > ds2 * 2.0, "C2 ({}) should be much stronger than D#2 ({})", c2, ds2);
        assert!(e2 > ds2 * 2.0, "E2 ({}) should be much stronger than D#2 ({})", e2, ds2);
        assert!(g2 > ds2, "G2 ({}) should be stronger than D#2 ({})", g2, ds2);
    }
}

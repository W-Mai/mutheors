//! FFT wrapper around rustfft

use rustfft::{num_complex::Complex, FftPlanner};

/// Thin FFT wrapper with cached planner
pub struct Fft {
    planner: FftPlanner<f32>,
}

impl Fft {
    pub fn new() -> Self {
        Self {
            planner: FftPlanner::new(),
        }
    }

    /// Real-to-complex forward FFT. Returns magnitude spectrum (first N/2+1 bins).
    pub fn magnitude_spectrum(&mut self, signal: &[f32]) -> Vec<f32> {
        let n = signal.len();
        let fft = self.planner.plan_fft_forward(n);
        let mut buf: Vec<Complex<f32>> = signal.iter().map(|&s| Complex::new(s, 0.0)).collect();
        fft.process(&mut buf);
        buf[..n / 2 + 1].iter().map(|c| c.norm()).collect()
    }
}

impl Default for Fft {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn sine_peak_at_correct_bin() {
        let sr = 1024.0;
        let freq = 100.0;
        let n = 1024;
        let signal: Vec<f32> = (0..n)
            .map(|i| (2.0 * PI * freq * i as f32 / sr).sin())
            .collect();

        let mut fft = Fft::new();
        let mag = fft.magnitude_spectrum(&signal);

        let peak_bin = mag
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        let peak_freq = peak_bin as f32 * sr / n as f32;
        assert!((peak_freq - freq).abs() < 2.0);
    }
}

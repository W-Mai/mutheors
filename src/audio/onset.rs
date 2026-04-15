//! Onset detection and BPM estimation
//!
//! Spectral flux peak detection → onset times → autocorrelation → BPM

use super::fft::Fft;

/// Onset detection result
#[derive(Clone, Debug)]
pub struct OnsetResult {
    /// Onset times in seconds
    pub onsets: Vec<f32>,
    /// Estimated BPM (None if too few onsets)
    pub bpm: Option<f32>,
}

/// Onset detector using spectral flux
pub struct OnsetDetector {
    sample_rate: f32,
    hop_size: usize,
    frame_size: usize,
    /// Peak-picking threshold multiplier over median
    threshold: f32,
    /// Minimum time between onsets in seconds (prevents double-triggers)
    min_onset_interval: f32,
}

impl OnsetDetector {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            hop_size: 512,
            frame_size: 1024,
            threshold: 3.0,
            min_onset_interval: 0.10, // 100ms minimum between onsets
        }
    }

    pub fn with_hop_size(mut self, hop: usize) -> Self {
        self.hop_size = hop;
        self
    }

    pub fn with_threshold(mut self, t: f32) -> Self {
        self.threshold = t;
        self
    }

    pub fn with_min_interval(mut self, secs: f32) -> Self {
        self.min_onset_interval = secs;
        self
    }

    /// Detect onsets and estimate BPM from audio samples.
    pub fn detect(&self, samples: &[f32]) -> OnsetResult {
        let flux = self.spectral_flux(samples);
        let onsets = self.pick_peaks(&flux);
        let bpm = self.estimate_bpm(&onsets);
        OnsetResult { onsets, bpm }
    }

    /// Compute spectral flux: half-wave rectified difference of magnitude spectra
    fn spectral_flux(&self, samples: &[f32]) -> Vec<f32> {
        let mut fft = Fft::new();
        let mut prev_mag: Option<Vec<f32>> = None;
        let mut flux = Vec::new();

        let mut pos = 0;
        while pos + self.frame_size <= samples.len() {
            let frame = &samples[pos..pos + self.frame_size];
            let mag = fft.magnitude_spectrum(frame);

            if let Some(ref prev) = prev_mag {
                // Half-wave rectified spectral flux
                let f: f32 = mag
                    .iter()
                    .zip(prev.iter())
                    .map(|(&curr, &prev)| (curr - prev).max(0.0))
                    .sum();
                flux.push(f);
            } else {
                flux.push(0.0);
            }

            prev_mag = Some(mag);
            pos += self.hop_size;
        }
        flux
    }

    /// Pick peaks from spectral flux using global adaptive threshold + minimum interval
    fn pick_peaks(&self, flux: &[f32]) -> Vec<f32> {
        if flux.is_empty() {
            return vec![];
        }

        // Global threshold: mean + threshold * stddev
        let mean = flux.iter().sum::<f32>() / flux.len() as f32;
        let variance = flux.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / flux.len() as f32;
        let stddev = variance.sqrt();
        let adaptive_threshold = mean + self.threshold * stddev;

        let min_frames =
            (self.min_onset_interval * self.sample_rate / self.hop_size as f32) as usize;
        let mut onsets = Vec::new();
        let mut last_onset_frame = 0usize;

        for i in 1..flux.len().saturating_sub(1) {
            if flux[i] > adaptive_threshold
                && flux[i] > flux[i - 1]
                && flux[i] >= flux[i + 1]
                && (onsets.is_empty() || i - last_onset_frame >= min_frames)
            {
                let time = i as f32 * self.hop_size as f32 / self.sample_rate;
                onsets.push(time);
                last_onset_frame = i;
            }
        }
        onsets
    }

    /// Estimate BPM from onset intervals via autocorrelation
    fn estimate_bpm(&self, onsets: &[f32]) -> Option<f32> {
        if onsets.len() < 4 {
            return None;
        }

        // Inter-onset intervals
        let intervals: Vec<f32> = onsets.windows(2).map(|w| w[1] - w[0]).collect();

        // Autocorrelation of intervals to find dominant periodicity
        // Search BPM range: 40–240 BPM → interval 0.25–1.5s
        let min_interval = 0.25f32; // 240 BPM
        let max_interval = 1.5f32; // 40 BPM

        // Histogram approach: bin intervals and find mode
        let bin_size = 0.02f32; // 20ms bins
        let num_bins = ((max_interval - min_interval) / bin_size) as usize + 1;
        let mut histogram = vec![0u32; num_bins];

        for &ioi in &intervals {
            if ioi >= min_interval && ioi <= max_interval {
                let bin = ((ioi - min_interval) / bin_size) as usize;
                if bin < num_bins {
                    histogram[bin] += 1;
                }
            }
        }

        // Find peak bin
        let (peak_bin, &peak_count) = histogram
            .iter()
            .enumerate()
            .max_by_key(|(_, &c)| c)?;

        if peak_count < 2 {
            return None;
        }

        let dominant_interval = min_interval + (peak_bin as f32 + 0.5) * bin_size;
        Some(60.0 / dominant_interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    /// Generate a click track: short bursts at regular intervals
    fn generate_click_track(bpm: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
        let n = (sample_rate * duration) as usize;
        let interval_samples = (60.0 / bpm * sample_rate) as usize;
        let click_len = (sample_rate * 0.005) as usize; // 5ms click

        let mut samples = vec![0.0f32; n];
        let mut pos = 0;
        while pos < n {
            for i in 0..click_len.min(n - pos) {
                samples[pos + i] = (2.0 * PI * 1000.0 * i as f32 / sample_rate).sin();
            }
            pos += interval_samples;
        }
        samples
    }

    #[test]
    fn detect_onsets_in_click_track() {
        let samples = generate_click_track(120.0, 44100.0, 3.0);
        let det = OnsetDetector::new(44100.0);
        let result = det.detect(&samples);

        // 120 BPM for 3 seconds = ~6 beats
        assert!(
            result.onsets.len() >= 4,
            "Should detect at least 4 onsets, got {}",
            result.onsets.len()
        );
    }

    #[test]
    fn bpm_estimation_120() {
        let samples = generate_click_track(120.0, 44100.0, 5.0);
        let det = OnsetDetector::new(44100.0);
        let result = det.detect(&samples);

        let bpm = result.bpm.expect("Should estimate BPM");
        assert!(
            (bpm - 120.0).abs() < 10.0,
            "Expected ~120 BPM, got {:.1}",
            bpm
        );
    }

    #[test]
    fn bpm_estimation_90() {
        let samples = generate_click_track(90.0, 44100.0, 5.0);
        let det = OnsetDetector::new(44100.0);
        let result = det.detect(&samples);

        let bpm = result.bpm.expect("Should estimate BPM");
        assert!(
            (bpm - 90.0).abs() < 10.0,
            "Expected ~90 BPM, got {:.1}",
            bpm
        );
    }

    #[test]
    fn silence_no_onsets() {
        let samples = vec![0.0f32; 44100 * 2];
        let det = OnsetDetector::new(44100.0);
        let result = det.detect(&samples);
        assert!(result.onsets.is_empty());
        assert!(result.bpm.is_none());
    }
}

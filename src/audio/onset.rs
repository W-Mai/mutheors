//! Onset detection and BPM estimation
//!
//! Multi-band spectral flux with adaptive thresholding for robust beat detection.

use super::fft::Fft;

/// Onset detection result
#[derive(Clone, Debug)]
pub struct OnsetResult {
    /// Onset times in seconds
    pub onsets: Vec<f32>,
    /// Estimated BPM (None if too few onsets)
    pub bpm: Option<f32>,
}

/// Frequency band for multi-band onset detection
struct Band {
    /// FFT bin range [lo, hi)
    lo: usize,
    hi: usize,
}

/// Onset detector using multi-band spectral flux
pub struct OnsetDetector {
    sample_rate: f32,
    hop_size: usize,
    frame_size: usize,
    /// Threshold: number of standard deviations above local mean
    threshold: f32,
    /// Minimum time between onsets in seconds
    min_onset_interval: f32,
    /// Local adaptive window size in frames
    adaptive_window: usize,
}

impl OnsetDetector {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            hop_size: 512,
            frame_size: 1024,
            threshold: 2.0,
            min_onset_interval: 0.03, // 30ms — allows fast hihat patterns
            adaptive_window: 15,      // ~170ms context at hop=512/44100
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

    /// Detect onsets and estimate BPM.
    pub fn detect(&self, samples: &[f32]) -> OnsetResult {
        let flux = self.multiband_flux(samples);
        let onsets = self.pick_peaks(&flux);
        let bpm = Self::estimate_bpm(&onsets);
        OnsetResult { onsets, bpm }
    }

    /// Compute multi-band spectral flux, then merge.
    ///
    /// Three bands: low (kick ~20-300Hz), mid (snare ~300-2000Hz), high (hihat ~2000Hz+)
    fn multiband_flux(&self, samples: &[f32]) -> Vec<f32> {
        let mut fft = Fft::new();
        let bin_hz = self.sample_rate / self.frame_size as f32;

        let bands = [
            Band {
                lo: (20.0 / bin_hz) as usize,
                hi: (300.0 / bin_hz) as usize,
            },
            Band {
                lo: (300.0 / bin_hz) as usize,
                hi: (2000.0 / bin_hz) as usize,
            },
            Band {
                lo: (2000.0 / bin_hz) as usize,
                hi: (self.sample_rate / 2.0 / bin_hz) as usize,
            },
        ];

        let num_frames = samples.len().saturating_sub(self.frame_size) / self.hop_size + 1;
        // Per-band flux
        let mut band_flux: Vec<Vec<f32>> = vec![vec![0.0; num_frames]; bands.len()];
        let mut prev_mag: Option<Vec<f32>> = None;

        #[allow(clippy::needless_range_loop)]
        for frame_idx in 0..num_frames {
            let pos = frame_idx * self.hop_size;
            if pos + self.frame_size > samples.len() {
                break;
            }
            let frame = &samples[pos..pos + self.frame_size];
            let mag = fft.magnitude_spectrum(frame);

            if let Some(ref prev) = prev_mag {
                for (b, band) in bands.iter().enumerate() {
                    let lo = band.lo.min(mag.len());
                    let hi = band.hi.min(mag.len());
                    let f: f32 = mag[lo..hi]
                        .iter()
                        .zip(prev[lo..hi].iter())
                        .map(|(&c, &p)| (c - p).max(0.0))
                        .sum();
                    band_flux[b][frame_idx] = f;
                }
            }
            prev_mag = Some(mag);
        }

        // Normalize each band independently, then sum
        let mut combined = vec![0.0f32; num_frames];
        for band in &band_flux {
            let max = band.iter().cloned().fold(0.0f32, f32::max);
            if max > 1e-8 {
                for (i, &v) in band.iter().enumerate() {
                    combined[i] += v / max;
                }
            }
        }
        combined
    }

    /// Pick peaks with local adaptive threshold + minimum interval
    fn pick_peaks(&self, flux: &[f32]) -> Vec<f32> {
        if flux.len() < 3 {
            return vec![];
        }

        let half_w = self.adaptive_window / 2;
        let min_frames =
            (self.min_onset_interval * self.sample_rate / self.hop_size as f32) as usize;
        let mut onsets = Vec::new();
        let mut last_frame = 0usize;

        for i in 1..flux.len() - 1 {
            // Local window stats
            let start = i.saturating_sub(half_w);
            let end = (i + half_w + 1).min(flux.len());
            let window = &flux[start..end];
            let n = window.len() as f32;
            let mean = window.iter().sum::<f32>() / n;
            let var = window.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;
            let threshold = mean + self.threshold * var.sqrt();

            if flux[i] > threshold
                && flux[i] > flux[i - 1]
                && flux[i] >= flux[i + 1]
                && (onsets.is_empty() || i - last_frame >= min_frames)
            {
                onsets.push(i as f32 * self.hop_size as f32 / self.sample_rate);
                last_frame = i;
            }
        }
        onsets
    }

    /// Estimate BPM from onset intervals
    fn estimate_bpm(onsets: &[f32]) -> Option<f32> {
        if onsets.len() < 4 {
            return None;
        }

        let intervals: Vec<f32> = onsets.windows(2).map(|w| w[1] - w[0]).collect();

        // Histogram: 40–240 BPM → 0.25–1.5s intervals
        let min_ioi = 0.25f32;
        let max_ioi = 1.5f32;
        let bin_size = 0.02f32;
        let num_bins = ((max_ioi - min_ioi) / bin_size) as usize + 1;
        let mut histogram = vec![0u32; num_bins];

        for &ioi in &intervals {
            if ioi >= min_ioi && ioi <= max_ioi {
                let bin = ((ioi - min_ioi) / bin_size) as usize;
                if bin < num_bins {
                    histogram[bin] += 1;
                }
            }
        }

        let (peak_bin, &peak_count) = histogram.iter().enumerate().max_by_key(|(_, &c)| c)?;

        if peak_count < 2 {
            return None;
        }

        let dominant_interval = min_ioi + (peak_bin as f32 + 0.5) * bin_size;
        Some(60.0 / dominant_interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn click_track(bpm: f32, sr: f32, duration: f32) -> Vec<f32> {
        let n = (sr * duration) as usize;
        let interval = (60.0 / bpm * sr) as usize;
        let click_len = (sr * 0.005) as usize;
        let mut out = vec![0.0f32; n];
        let mut pos = 0;
        while pos < n {
            for i in 0..click_len.min(n - pos) {
                out[pos + i] = (2.0 * PI * 1000.0 * i as f32 / sr).sin();
            }
            pos += interval;
        }
        out
    }

    /// Mixed drum pattern: kick(low) + snare(mid) + hihat(high) at different velocities
    fn drum_pattern(bpm: f32, sr: f32, measures: usize) -> Vec<f32> {
        let beat_samples = (60.0 / bpm * sr) as usize;
        let eighth = beat_samples / 2;
        let total = measures * 4 * beat_samples; // 4/4 time
        let mut out = vec![0.0f32; total];

        for measure in 0..measures {
            for beat in 0..4 {
                let base = measure * 4 * beat_samples + beat * beat_samples;

                // Kick on beats 1 and 3
                if beat == 0 || beat == 2 {
                    add_hit(&mut out, base, sr, 55.0, 0.9, 0.01);
                }
                // Snare on beats 2 and 4
                if beat == 1 || beat == 3 {
                    add_hit(&mut out, base, sr, 200.0, 0.7, 0.008);
                }
                // Hihat on every eighth note
                add_hit(&mut out, base, sr, 8000.0, 0.3, 0.003);
                if base + eighth < total {
                    add_hit(&mut out, base + eighth, sr, 8000.0, 0.2, 0.003);
                }
            }
        }
        out
    }

    fn add_hit(buf: &mut [f32], pos: usize, sr: f32, freq: f32, amp: f32, decay: f32) {
        let len = (sr * 0.05) as usize; // 50ms max
        for i in 0..len.min(buf.len().saturating_sub(pos)) {
            let env = amp * (-(i as f32) / (sr * decay)).exp();
            buf[pos + i] += env * (2.0 * PI * freq * i as f32 / sr).sin();
        }
    }

    #[test]
    fn click_track_120bpm() {
        let audio = click_track(120.0, 44100.0, 5.0);
        let r = OnsetDetector::new(44100.0).detect(&audio);
        // 5s at 120BPM = 10 beats
        assert!(r.onsets.len() >= 8, "got {} onsets", r.onsets.len());
        let bpm = r.bpm.expect("should detect BPM");
        assert!((bpm - 120.0).abs() < 10.0, "bpm={:.1}", bpm);
    }

    #[test]
    fn drum_pattern_detects_all_hits() {
        let audio = drum_pattern(120.0, 44100.0, 4);
        let r = OnsetDetector::new(44100.0).detect(&audio);

        // 4 measures × (4 kicks/snares + 8 hihats) = 48 events
        // But some overlap. Expect at least the 16 kick+snare hits + some hihats
        assert!(
            r.onsets.len() >= 20,
            "Should detect most drum hits, got {}",
            r.onsets.len()
        );
    }

    #[test]
    fn drum_pattern_bpm() {
        let audio = drum_pattern(100.0, 44100.0, 8);
        let r = OnsetDetector::new(44100.0).detect(&audio);
        if let Some(bpm) = r.bpm {
            // Could detect quarter (100) or eighth (200) pulse
            assert!(
                (bpm - 100.0).abs() < 15.0 || (bpm - 200.0).abs() < 15.0,
                "bpm={:.1}",
                bpm
            );
        }
    }

    #[test]
    fn silence() {
        let r = OnsetDetector::new(44100.0).detect(&vec![0.0f32; 44100 * 3]);
        assert!(r.onsets.is_empty());
        assert!(r.bpm.is_none());
    }
}

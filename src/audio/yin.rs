//! YIN pitch detection algorithm
//!
//! Reference: de Cheveigné & Kawahara (2002), "YIN, a fundamental frequency estimator for speech and music"

use crate::Tuning;

/// YIN configuration parameters
#[derive(Clone, Debug)]
pub struct YinConfig {
    /// Detection threshold (lower = stricter). Typical: 0.10–0.20
    pub threshold: f32,
    /// Minimum detectable frequency in Hz
    pub freq_min: f32,
    /// Maximum detectable frequency in Hz
    pub freq_max: f32,
    /// RMS energy gate — below this, signal is considered silent
    pub silence_threshold: f32,
}

impl Default for YinConfig {
    fn default() -> Self {
        Self {
            threshold: 0.15,
            freq_min: 60.0,
            freq_max: 1200.0,
            silence_threshold: 0.01,
        }
    }
}

impl YinConfig {
    pub fn guitar_tuner() -> Self {
        Self {
            threshold: 0.10,
            freq_min: 60.0,
            freq_max: 1200.0,
            silence_threshold: 0.02,
        }
    }

    pub fn vocal() -> Self {
        Self {
            threshold: 0.20,
            freq_min: 80.0,
            freq_max: 1000.0,
            silence_threshold: 0.01,
        }
    }
}

/// Pitch detection result
#[derive(Clone, Debug)]
pub struct PitchResult {
    /// Nearest musical pitch
    pub tuning: Tuning,
    /// Deviation from nearest pitch in cents (-50 to +50)
    pub cents: f64,
    /// Detected fundamental frequency in Hz
    pub frequency: f64,
    /// Detection confidence (0.0–1.0, higher = more confident)
    pub confidence: f32,
    /// Whether the signal is voiced (has a clear pitch)
    pub is_voiced: bool,
}

/// Detect pitch from audio samples using the YIN algorithm.
///
/// Returns `None` if the signal is silent or no clear pitch is found.
pub fn detect_pitch(samples: &[f32], sample_rate: f32) -> Option<PitchResult> {
    detect_pitch_with_config(samples, sample_rate, &YinConfig::default())
}

/// Detect pitch with custom configuration.
pub fn detect_pitch_with_config(
    samples: &[f32],
    sample_rate: f32,
    config: &YinConfig,
) -> Option<PitchResult> {
    // Silence gate
    let rms = (samples.iter().map(|&s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
    if rms < config.silence_threshold {
        return None;
    }

    let tau_min = (sample_rate / config.freq_max).ceil() as usize;
    let tau_max = (sample_rate / config.freq_min).floor() as usize;
    let w = samples.len() / 2;

    if tau_max >= w || tau_min >= tau_max {
        return None;
    }

    // Step 1-2: Difference function + cumulative mean normalized difference
    let cmnd = cumulative_mean_normalized_difference(samples, w);

    // Step 3: Absolute threshold — find first tau where cmnd < threshold
    let mut tau = None;
    for t in tau_min..=tau_max {
        if t >= cmnd.len() {
            break;
        }
        if cmnd[t] < config.threshold {
            // Find the local minimum after crossing threshold
            let mut best = t;
            for t2 in (t + 1)..=tau_max.min(cmnd.len() - 1) {
                if cmnd[t2] < cmnd[best] {
                    best = t2;
                } else {
                    break;
                }
            }
            tau = Some(best);
            break;
        }
    }

    let tau = tau?;

    // Step 4: Parabolic interpolation for sub-sample accuracy
    let refined_tau = parabolic_interpolation(&cmnd, tau);

    // Step 5: Period → frequency
    let frequency = sample_rate as f64 / refined_tau;
    let confidence = 1.0 - cmnd[tau];

    // Frequency → nearest Tuning + cent offset
    let (tuning, cents) = Tuning::from_frequency(frequency);

    Some(PitchResult {
        tuning,
        cents,
        frequency,
        confidence,
        is_voiced: confidence > 0.5,
    })
}

/// Compute cumulative mean normalized difference function (steps 1+2 of YIN)
fn cumulative_mean_normalized_difference(samples: &[f32], w: usize) -> Vec<f32> {
    // Step 1: Difference function
    let mut d = vec![0.0f32; w];
    for tau in 1..w {
        let mut sum = 0.0f32;
        for i in 0..w {
            let diff = samples[i] - samples[i + tau];
            sum += diff * diff;
        }
        d[tau] = sum;
    }

    // Step 2: Cumulative mean normalized difference
    let mut cmnd = vec![0.0f32; w];
    cmnd[0] = 1.0;
    let mut running_sum = 0.0f32;
    for tau in 1..w {
        running_sum += d[tau];
        cmnd[tau] = if running_sum > 0.0 {
            d[tau] * tau as f32 / running_sum
        } else {
            1.0
        };
    }
    cmnd
}

/// Parabolic interpolation around index `tau` for sub-sample accuracy
fn parabolic_interpolation(cmnd: &[f32], tau: usize) -> f64 {
    if tau < 1 || tau >= cmnd.len() - 1 {
        return tau as f64;
    }
    let s0 = cmnd[tau - 1] as f64;
    let s1 = cmnd[tau] as f64;
    let s2 = cmnd[tau + 1] as f64;
    let shift = (s0 - s2) / (2.0 * (s0 - 2.0 * s1 + s2));
    tau as f64 + shift
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn generate_sine(freq: f32, sample_rate: f32, duration_secs: f32) -> Vec<f32> {
        let n = (sample_rate * duration_secs) as usize;
        (0..n)
            .map(|i| (2.0 * PI * freq * i as f32 / sample_rate).sin())
            .collect()
    }

    #[test]
    fn detect_a440() {
        let samples = generate_sine(440.0, 44100.0, 0.1);
        let result = detect_pitch(&samples, 44100.0).unwrap();
        assert!((result.frequency - 440.0).abs() < 1.0, "freq={}", result.frequency);
        assert!(result.cents.abs() < 5.0, "cents={}", result.cents);
        assert!(result.confidence > 0.8);
        assert!(result.is_voiced);
    }

    #[test]
    fn detect_e2_guitar_low() {
        let samples = generate_sine(82.41, 44100.0, 0.1);
        let result = detect_pitch(&samples, 44100.0).unwrap();
        assert!((result.frequency - 82.41).abs() < 1.0, "freq={}", result.frequency);
    }

    #[test]
    fn detect_c4() {
        let samples = generate_sine(261.63, 44100.0, 0.1);
        let result = detect_pitch(&samples, 44100.0).unwrap();
        assert!((result.frequency - 261.63).abs() < 1.0, "freq={}", result.frequency);
        assert_eq!(result.tuning.class(), crate::PitchClass::C);
    }

    #[test]
    fn silence_returns_none() {
        let samples = vec![0.0f32; 4096];
        assert!(detect_pitch(&samples, 44100.0).is_none());
    }

    #[test]
    fn too_short_returns_none() {
        let samples = generate_sine(440.0, 44100.0, 0.001); // ~44 samples
        assert!(detect_pitch(&samples, 44100.0).is_none());
    }

    #[test]
    fn guitar_tuner_config() {
        let samples = generate_sine(329.63, 44100.0, 0.1); // E4
        let result = detect_pitch_with_config(&samples, 44100.0, &YinConfig::guitar_tuner()).unwrap();
        assert!((result.frequency - 329.63).abs() < 1.0);
    }

    #[test]
    fn various_frequencies() {
        for &(freq, expected_class) in &[
            (261.63, crate::PitchClass::C),
            (293.66, crate::PitchClass::D),
            (329.63, crate::PitchClass::E),
            (349.23, crate::PitchClass::F),
            (392.00, crate::PitchClass::G),
            (440.00, crate::PitchClass::A),
            (493.88, crate::PitchClass::B),
        ] {
            let samples = generate_sine(freq, 44100.0, 0.1);
            let result = detect_pitch(&samples, 44100.0).unwrap();
            assert!(
                (result.frequency - freq as f64).abs() < 2.0,
                "freq={}, detected={}",
                freq,
                result.frequency
            );
            assert_eq!(
                result.tuning.class(),
                expected_class,
                "freq={}, expected {:?}, got {:?}",
                freq,
                expected_class,
                result.tuning.class()
            );
        }
    }
}

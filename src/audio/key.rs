//! Key detection using the Krumhansl-Schmuckler algorithm
//!
//! Correlates a chroma vector with 24 key profiles (12 major + 12 minor)
//! to estimate the most likely musical key.

use super::chroma::Chroma;
use crate::{PitchClass, Scale, ScaleType, Tuning};

/// Krumhansl-Kessler key profiles (major and minor)
/// Index 0 = tonic, 1 = semitone above tonic, etc.
const MAJOR_PROFILE: [f32; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
];

const MINOR_PROFILE: [f32; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
];

/// Key detection result
#[derive(Clone, Debug)]
pub struct KeyResult {
    /// Detected root pitch class
    pub root: PitchClass,
    /// Major or minor
    pub scale_type: ScaleType,
    /// Correlation score (higher = more confident)
    pub confidence: f32,
    /// The corresponding Scale object
    pub scale: Scale,
}

const PITCH_CLASSES: [PitchClass; 12] = [
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

/// Detect the musical key from a chroma vector.
pub fn detect_key(chroma: &Chroma) -> Option<KeyResult> {
    let mut best: Option<(PitchClass, ScaleType, f32)> = None;

    for (shift, &root) in PITCH_CLASSES.iter().enumerate() {
        // Rotate profile to match this root
        for (profile, scale_type) in [
            (&MAJOR_PROFILE, ScaleType::Major),
            (&MINOR_PROFILE, ScaleType::NaturalMinor),
        ] {
            let score = correlate(chroma, profile, shift);
            if best.is_none() || score > best.unwrap().2 {
                best = Some((root, scale_type, score));
            }
        }
    }

    let (root, scale_type, confidence) = best?;
    let scale = Scale::new(Tuning::new(root, 4), scale_type).ok()?;
    Some(KeyResult {
        root,
        scale_type,
        confidence,
        scale,
    })
}

/// Pearson correlation between chroma and a rotated key profile
fn correlate(chroma: &Chroma, profile: &[f32; 12], shift: usize) -> f32 {
    let n = 12.0f32;
    let mut sum_xy = 0.0f32;
    let mut sum_x = 0.0f32;
    let mut sum_y = 0.0f32;
    let mut sum_x2 = 0.0f32;
    let mut sum_y2 = 0.0f32;

    for i in 0..12 {
        let x = chroma[(i + shift) % 12];
        let y = profile[i];
        sum_xy += x * y;
        sum_x += x;
        sum_y += y;
        sum_x2 += x * x;
        sum_y2 += y * y;
    }

    let num = n * sum_xy - sum_x * sum_y;
    let den = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();
    if den < 1e-8 {
        return 0.0;
    }
    num / den
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a chroma vector with energy only on the given pitch classes
    fn chroma_from_notes(notes: &[usize]) -> Chroma {
        let mut c = [0.0f32; 12];
        for &n in notes {
            c[n % 12] = 1.0;
        }
        c
    }

    #[test]
    fn detect_c_major() {
        // C major scale: C D E F G A B = 0 2 4 5 7 9 11
        let chroma = chroma_from_notes(&[0, 2, 4, 5, 7, 9, 11]);
        let result = detect_key(&chroma).unwrap();
        assert_eq!(result.root, PitchClass::C);
        assert_eq!(result.scale_type, ScaleType::Major);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn detect_a_minor() {
        // A natural minor: A B C D E F G = 9 11 0 2 4 5 7
        let chroma = chroma_from_notes(&[9, 11, 0, 2, 4, 5, 7]);
        let result = detect_key(&chroma).unwrap();
        // C major and A minor are relative keys — both valid
        let is_a_minor =
            result.root == PitchClass::A && result.scale_type == ScaleType::NaturalMinor;
        let is_c_major = result.root == PitchClass::C && result.scale_type == ScaleType::Major;
        assert!(
            is_a_minor || is_c_major,
            "Expected A minor or C major, got {:?} {:?}",
            result.root,
            result.scale_type
        );
    }

    #[test]
    fn detect_g_major() {
        // G major: G A B C D E F# = 7 9 11 0 2 4 6
        let chroma = chroma_from_notes(&[7, 9, 11, 0, 2, 4, 6]);
        let result = detect_key(&chroma).unwrap();
        assert_eq!(result.root, PitchClass::G);
        assert_eq!(result.scale_type, ScaleType::Major);
    }

    #[test]
    fn detect_d_minor() {
        // D natural minor: D E F G A Bb C = 2 4 5 7 9 10 0
        // Relative major is F major — both are valid detections
        let chroma = chroma_from_notes(&[2, 4, 5, 7, 9, 10, 0]);
        let result = detect_key(&chroma).unwrap();
        let is_d_minor =
            result.root == PitchClass::D && result.scale_type == ScaleType::NaturalMinor;
        let is_f_major = result.root == PitchClass::F && result.scale_type == ScaleType::Major;
        assert!(
            is_d_minor || is_f_major,
            "Expected D minor or F major, got {:?} {:?}",
            result.root,
            result.scale_type
        );
    }
}

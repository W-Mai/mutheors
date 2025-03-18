//! Scale System Module
//! Provides core functions such as scale generation, modal analysis, scale and chord derivation, and more!
//!
//! # Interval pattern library
//! ## Standard scale patterns
//! - Major scale: [2, 2, 1, 2, 2, 2, 1]
//! - Natural minor scale: [2, 1, 2, 2, 1, 2, 2]
//! - Harmonic minor scale: [2, 1, 2, 2, 1, 3, 1]
//! - Melodic minor scale: [2, 1, 2, 2, 2, 2, 1]
//!
//! ## Mediaeval mode
//! - Ionian mode: [2, 2, 1, 2, 2, 2, 1]
//! - Dorian mode: [2, 1, 2, 2, 2, 1, 2]      Ionian mode shifted by 1
//! - Phrygian mode: [1, 2, 2, 2, 1, 2, 2]    Ionian mode shifted by 2
//! - Lydian mode: [2, 2, 2, 1, 2, 2, 1]      Ionian mode shifted by 3
//! - Mixolydian mode: [2, 2, 1, 2, 2, 1, 2]  Ionian mode shifted by 4
//! - Aeolian mode: [2, 1, 2, 2, 1, 2, 2]     Ionian mode shifted by 5
//! - Locrian mode: [1, 2, 2, 1, 2, 2, 2]     Ionian mode shifted by 6
//!
//! ## Pentatonic scale
//! - Major pentatonic scale: [2, 2, 3, 2, 3]
//! - Minor pentatonic scale: [3, 2, 2, 3, 2]
//! - Blues scale: [3, 2, 1, 1, 3, 2]
//!
//! ## Special scales
//! - Whole tone scale: [2, 2, 2, 2, 2, 2]
//! - Octatonic scale: [2, 1, 2, 1, 2, 1, 2, 1]
//! - Chromatic scale: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
//! - Bebop dominant scale: [2, 2, 1, 2, 2, 1, 1, 2]
//!
//! ## National scales
//! - Arabian Hijaz scale: [1, 3, 1, 2, 1, 3, 1]
//! - Japanese Hirajoshi scale: [2, 1, 4, 1, 4]
//! - Japanese InSen scale: [1, 4, 2, 3, 2]
//! - Custom scale: [2, 1, 3, 1, 4]

use crate::interval::Interval;
use crate::tuning::Tuning;
use crate::MusicError;
use std::ops::{Add, Div, Mul, Sub};

/// Scale type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    // Basic scale
    /// Natural Major
    /// - 自然大调
    Major,
    /// Natural Minor
    /// - 自然小调
    NaturalMinor,
    /// HarmonicMinor
    /// - 和声小调
    HarmonicMinor,
    /// melodic minor (upward)
    /// - 旋律小调（上行）
    MelodicMinor,
    /// Ionian mode (natural major)
    /// - 伊奥尼亚调式（自然大调）
    Ionian,
    /// Dorian mode
    /// - 多利亚调式
    Dorian,
    /// Phrygian mode
    /// - 弗里几亚调式
    Phrygian,
    /// Lydian mode
    /// - 利底亚调式
    Lydian,
    /// Mixed Lydian mode
    /// - 混合利底亚调式
    Mixolydian,
    /// Aeolian mode (natural minor)
    /// - 艾奥利亚调式（自然小调）
    Aeolian,
    /// Locrian mode
    /// - 洛克里亚调式
    Locrian,

    // Pentatonic scale
    /// Major Pentatonic
    /// - 大调五声音阶
    PentatonicMajor,
    /// Minor Pentatonic
    /// - 小调五声音阶
    PentatonicMinor,
    /// Blues scale
    /// - 蓝调音阶
    Blues,

    // Special scale
    /// Whole Tone
    /// - 全音阶
    WholeTone,
    /// Octatonic
    /// - 八声音阶（减音阶）
    Octatonic,
    /// Chromatic
    /// - 半音阶
    Chromatic,
    /// Bebop Dominant
    /// - 比波普属音阶
    BebopDominant,

    // National scale
    /// Arabian Hijaz
    /// - 阿拉伯希贾兹音阶
    Hijaz,
    /// Japanese Hirajoshi
    /// - 日本平调子
    Hirajoshi,
    /// Japanese InSen
    /// - 日本阴旋
    InSen,

    // Custom scales
    /// Custom scale
    /// - 自定义音程模式
    Custom(&'static [i8]),
}

/// Scale System
#[derive(Debug, Clone, Copy)]
pub struct Scale {
    root: Tuning,
    scale_type: ScaleType,
}

impl Scale {
    /// Create a new scale
    pub fn new(root: Tuning, scale_type: ScaleType) -> Result<Self, MusicError> {
        Ok(Self { root, scale_type })
    }

    /// Generating note sequence
    pub fn generate_tunings(&self, octaves: u8) -> Result<Vec<Tuning>, MusicError> {
        let mut current = self.root.clone();
        let mut tunings = vec![current.clone()];
        let intervals = Self::get_intervals(self.scale_type)?;

        for _ in 0..=octaves {
            for interval in intervals.iter() {
                current = current.add_interval(interval)?;
                tunings.push(current.clone());
            }
        }

        Ok(tunings)
    }

    /// Determining whether a pitch belongs to a scale
    pub fn contains(&self, tuning: &Tuning) -> bool {
        let tunings = self.generate_tunings(1).unwrap();
        tunings.iter().any(|n| n.class == tuning.class)
    }

    /// Getting the Scale Degree
    /// - Get the Tuning by order
    /// - Such as in Pentatonic scale, the scale only has five notes. In major `C` the tuning is like:
    ///     - 1 -> C
    ///     - 2 -> D
    ///     - 3 -> E
    ///     - 4 -> G
    ///     - 5 -> A
    pub fn degree(&self, degree: u8) -> Result<Tuning, MusicError> {
        if degree < 1 {
            return Err(MusicError::InvalidScaleDegree(degree));
        }
        let intervals = Self::get_intervals(self.scale_type)?;
        let octave = (degree - 1) / intervals.len() as u8;
        // TODO: Dealing with a pentatonic scale where there are only five notes but the scales are not continuous
        let tunings = self.generate_tunings(octave + 1)?;
        tunings
            .get(degree as usize - 1)
            .cloned()
            .ok_or(MusicError::InvalidScaleDegree(degree))
    }

    pub fn interval_count(&self) -> u8 {
        let intervals = Self::get_intervals(self.scale_type).unwrap();
        intervals.len() as u8
    }

    pub fn semitone_count(&self) -> u8 {
        let intervals = Self::get_intervals(self.scale_type).unwrap();
        intervals.iter().map(|i| i.semitones()).sum::<i8>() as u8
    }

    // TODO: 生成音阶和弦
    // pub fn chord(&self, degree: u8, quality: ChordQuality) -> Result<Chord, MusicError> {}

    // TODO: 分析调式特征音程
    // pub fn characteristic_interval(&self) -> Option<Interval> {}

    // TODO: 获取音阶调式主音
    // pub fn modal_tonic(&self) -> Option<Tuning> {}
}

impl Scale {
    /// Gets the standard interval pattern of the scale
    fn get_intervals(scale_type: ScaleType) -> Result<Vec<Interval>, MusicError> {
        const NATURE_MAJOR: [i8; 7] = [2, 2, 1, 2, 2, 2, 1];
        fn shift_major(shift: i8) -> Vec<i8> {
            let mut major = NATURE_MAJOR.to_vec();
            major.rotate_left(shift as usize);
            major
        }

        /// Converts semitones to a list of intervals
        fn parse_intervals(semitones: &[i8]) -> Result<Vec<Interval>, MusicError> {
            semitones
                .iter()
                .map(|&s| Interval::from_semitones(s))
                .collect()
        }

        match scale_type {
            // Natural scales
            ScaleType::Major => parse_intervals(&shift_major(0)),
            ScaleType::NaturalMinor => parse_intervals(&shift_major(6)),
            ScaleType::HarmonicMinor => parse_intervals(&[2, 1, 2, 2, 1, 3, 1]),
            ScaleType::MelodicMinor => parse_intervals(&[2, 1, 2, 2, 2, 2, 1]),

            // Mediaeval mode
            ScaleType::Ionian => parse_intervals(&shift_major(0)),
            ScaleType::Dorian => parse_intervals(&shift_major(1)),
            ScaleType::Phrygian => parse_intervals(&shift_major(2)),
            ScaleType::Lydian => parse_intervals(&shift_major(3)),
            ScaleType::Mixolydian => parse_intervals(&shift_major(4)),
            ScaleType::Aeolian => parse_intervals(&shift_major(5)),
            ScaleType::Locrian => parse_intervals(&shift_major(6)),

            // Pentatonic scale
            ScaleType::PentatonicMajor => parse_intervals(&[2, 2, 3, 2, 3]),
            ScaleType::PentatonicMinor => parse_intervals(&[3, 2, 2, 3, 2]),
            ScaleType::Blues => parse_intervals(&[3, 2, 1, 1, 3, 2]),

            // Special scales
            ScaleType::WholeTone => parse_intervals(&[2, 2, 2, 2, 2, 2]),
            ScaleType::Octatonic => parse_intervals(&[2, 1, 2, 1, 2, 1, 2, 1]),
            ScaleType::Chromatic => parse_intervals(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            ScaleType::BebopDominant => parse_intervals(&[2, 2, 1, 2, 2, 1, 1, 2]),

            // National scales
            ScaleType::Hijaz => parse_intervals(&[1, 3, 1, 2, 1, 3, 1]),
            ScaleType::Hirajoshi => parse_intervals(&[2, 1, 4, 1, 4]),
            ScaleType::InSen => parse_intervals(&[1, 4, 2, 3, 2]),

            ScaleType::Custom(pattern) => {
                let semitones = pattern
                    .iter()
                    .map(|&s| Interval::from_semitones(s as i8))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(semitones)
            }
        }
    }
}

impl FnOnce<(u8,)> for Scale {
    type Output = Tuning;

    extern "rust-call" fn call_once(self, args: (u8,)) -> Self::Output {
        self.degree(args.0).unwrap()
    }
}

impl Add<u8> for Scale {
    type Output = Scale;

    fn add(self, rhs: u8) -> Self::Output {
        Self {
            root: self.degree(rhs).unwrap(),
            ..self
        }
    }
}

impl Sub<u8> for Scale {
    type Output = Scale;

    fn sub(self, rhs: u8) -> Self::Output {
        let interval_count = self.interval_count();
        let octave = rhs / interval_count + 2;
        let scale = self / octave;
        Scale {
            root: scale.degree(interval_count - rhs % interval_count).unwrap(),
            ..scale
        }
    }
}

impl Mul<u8> for Scale {
    type Output = Scale;

    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            root: self
                .root
                .add_interval(
                    &Interval::from_semitones(((rhs - 1) * self.semitone_count()) as i8).unwrap(),
                )
                .unwrap(),
            ..self
        }
    }
}

impl Div<u8> for Scale {
    type Output = Scale;

    fn div(self, rhs: u8) -> Self::Output {
        Self {
            root: self
                .root
                .add_interval(
                    &Interval::from_semitones(-(((rhs - 1) * self.semitone_count()) as i8))
                        .unwrap(),
                )
                .unwrap(),
            ..self
        }
    }
}

impl From<Scale> for Tuning {
    fn from(scale: Scale) -> Self {
        scale.root
    }
}

pub struct IntoIter {
    scale: Scale,
    current_degree: u8,
}

impl IntoIterator for Scale {
    type Item = Tuning;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            scale: self,
            current_degree: 1,
        }
    }
}

impl Iterator for IntoIter {
    type Item = Tuning;

    fn next(&mut self) -> Option<Self::Item> {
        let tuning = self.scale.degree(self.current_degree).ok();
        self.current_degree += 1;
        tuning
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_major_scale() {
        let c = Tuning::new(PitchClass::G, 4);
        let scale = c.scale(ScaleType::Major);
        let tunings = scale.generate_tunings(0).unwrap();
        assert_eq!(
            tunings,
            vec![
                Tuning::new(PitchClass::G, 4),
                Tuning::new(PitchClass::A, 4),
                Tuning::new(PitchClass::B, 4),
                Tuning::new(PitchClass::C, 5),
                Tuning::new(PitchClass::D, 5),
                Tuning::new(PitchClass::E, 5),
                Tuning::new(PitchClass::FSharpOrGFlat, 5),
                Tuning::new(PitchClass::G, 5),
            ]
        );
    }

    #[test]
    fn test_blues_scale() {
        let a = Tuning::new(PitchClass::A, 4);
        let scale = a.scale(ScaleType::Blues);
        assert!(scale.contains(&Tuning::new(PitchClass::C, 5)));
        assert!(scale.contains(&Tuning::new(PitchClass::DSharpOrEFlat, 5)));
    }

    #[test]
    fn test_scale_iter() {
        let s = Scale::new(tuning!(C 4), ScaleType::PentatonicMajor).unwrap();

        for t in s {
            println!("{:?}", t);
        }
    }
}

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
use crate::{Chord, ChordFunction, ChordQuality, IntervalQuality, MusicError};
use std::ops::{Add, Div, Mul, Sub};

/// Scale type classification
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
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
    #[cfg(not(feature = "bindgen"))]
    Custom(&'static [i8]),
}

impl ScaleType {
    pub fn iter() -> impl Iterator<Item = ScaleType> {
        [
            // Basic scale
            ScaleType::Major,
            ScaleType::NaturalMinor,
            ScaleType::HarmonicMinor,
            ScaleType::MelodicMinor,
            ScaleType::Ionian,
            ScaleType::Dorian,
            ScaleType::Phrygian,
            ScaleType::Lydian,
            ScaleType::Mixolydian,
            ScaleType::Aeolian,
            ScaleType::Locrian,
            // Pentatonic scale
            ScaleType::PentatonicMajor,
            ScaleType::PentatonicMinor,
            ScaleType::Blues,
            // Special scale
            ScaleType::WholeTone,
            ScaleType::Octatonic,
            ScaleType::Chromatic,
            ScaleType::BebopDominant,
            // National scale
            ScaleType::Hijaz,
            ScaleType::Hirajoshi,
            ScaleType::InSen,
        ]
        .into_iter()
    }
}

/// Scale System
#[derive(Debug, Clone, Copy)]
pub struct Scale {
    root: Tuning,
    scale_type: ScaleType,
}

impl Scale {
    pub fn scale_type(&self) -> ScaleType {
        self.scale_type
    }

    pub fn root(&self) -> Tuning {
        self.root
    }
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
        tunings
            .iter()
            .any(|n| n.class().semitones() == tuning.class().semitones())
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

    /// Get the chord of the scale in the given degree
    pub fn chord(&self, degree: u8, quality: ChordQuality) -> Result<Chord, MusicError> {
        Chord::new(self.degree(degree)?, quality)
    }

    /// Get the chord of the scale in the given degree
    /// - The chord is based on the scale degree (In-key chord)
    /// TODO: This function is incomplete. Some chords maybe wrong.
    pub fn degree_chord(&self, degree: u8) -> Result<Chord, MusicError> {
        const NATURE_MAJOR: [ChordQuality; 7] = [
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
        ];

        fn shift_major(shift: i8) -> Vec<ChordQuality> {
            let mut major = NATURE_MAJOR.to_vec();
            major.rotate_left(shift as usize);
            major
        }

        let scale_qualities = match self.scale_type {
            // Natural scales
            ScaleType::Major => shift_major(0),
            ScaleType::NaturalMinor => shift_major(5),
            // i (m), ii° (d), III+ (aug), iv (m), V (M), VI (M), vii° (d)
            ScaleType::HarmonicMinor => vec![
                ChordQuality::Minor,
                ChordQuality::Diminished,
                ChordQuality::Augmented,
                ChordQuality::Minor,
                ChordQuality::Major,
                ChordQuality::Major,
                ChordQuality::Diminished,
            ],
            // i (m), ii (m), III+ (aug), IV (M), V (M), vi° (d), vii° (d)
            ScaleType::MelodicMinor => vec![
                ChordQuality::Minor,
                ChordQuality::Minor,
                ChordQuality::Augmented,
                ChordQuality::Major,
                ChordQuality::Major,
                ChordQuality::Diminished,
                ChordQuality::Diminished,
            ],

            // Mediaeval mode
            ScaleType::Ionian => shift_major(0),
            ScaleType::Dorian => shift_major(1),
            ScaleType::Phrygian => shift_major(2),
            ScaleType::Lydian => shift_major(3),
            ScaleType::Mixolydian => shift_major(4),
            ScaleType::Aeolian => shift_major(5),
            ScaleType::Locrian => shift_major(6),

            // Pentatonic scale
            // I (M), ii (m), iii (m), V (M), vi (m)
            ScaleType::PentatonicMajor => vec![
                ChordQuality::Major,
                ChordQuality::Suspended2,
                ChordQuality::Suspended4,
                ChordQuality::Suspended2,
                ChordQuality::Suspended4,
            ],
            // i (m), III (M), IV (m), V (M), VII (M)
            ScaleType::PentatonicMinor => vec![
                ChordQuality::Minor,
                ChordQuality::Major,
                ChordQuality::Minor,
                ChordQuality::Major,
                ChordQuality::Major,
            ],
            // i7, IV7, V7
            ScaleType::Blues => vec![
                ChordQuality::Diminished,
                ChordQuality::Major,
                ChordQuality::Major,
            ],

            // Special scales
            ScaleType::WholeTone => vec![
                ChordQuality::Augmented,
                ChordQuality::Augmented,
                ChordQuality::Augmented,
                ChordQuality::Augmented,
                ChordQuality::Augmented,
                ChordQuality::Augmented,
            ],
            ScaleType::Octatonic => vec![
                ChordQuality::Diminished,
                ChordQuality::Diminished,
                ChordQuality::Diminished,
                ChordQuality::Diminished,
                ChordQuality::Diminished,
                ChordQuality::Diminished,
                ChordQuality::Diminished,
                ChordQuality::Diminished,
            ],
            ScaleType::BebopDominant => vec![
                ChordQuality::Dominant7,
                ChordQuality::Minor7,
                ChordQuality::HalfDiminished7,
                ChordQuality::Major7,
                ChordQuality::Dominant7,
                ChordQuality::Minor7,
                ChordQuality::Minor7,
                ChordQuality::Diminished7,
            ],

            // National scales
            // i (m), II (M), III+ (aug), iv (m), V (M), VI (M), vii° (d)
            ScaleType::Hijaz => vec![
                ChordQuality::Minor,
                ChordQuality::Major,
                ChordQuality::Augmented,
                ChordQuality::Minor,
                ChordQuality::Major,
                ChordQuality::Major,
                ChordQuality::Diminished,
            ],
            ScaleType::Hirajoshi => vec![
                ChordQuality::Major,
                ChordQuality::Minor,
                ChordQuality::Augmented,
                ChordQuality::Minor,
                ChordQuality::Augmented,
            ],
            // i (m), IV (M), V (m)
            ScaleType::InSen => vec![
                ChordQuality::Minor,
                ChordQuality::Major,
                ChordQuality::Minor,
            ],
            ScaleType::Chromatic => vec![],

            #[cfg(not(feature = "bindgen"))]
            ScaleType::Custom(_) => vec![],
        };

        if degree < 1 || degree > scale_qualities.len() as u8 {
            Err(MusicError::InvalidScaleDegree(degree))?
        }
        let quality = scale_qualities[(degree - 1) as usize];
        self.chord(degree, quality)
    }

    // Get the characteristic interval of the scale
    pub fn characteristic_interval(&self) -> Option<Interval> {
        match self.scale_type {
            ScaleType::Dorian => Interval::from_quality_degree(IntervalQuality::Major, 6).ok(),
            ScaleType::Phrygian => Interval::from_quality_degree(IntervalQuality::Minor, 2).ok(),
            ScaleType::Lydian => Interval::from_quality_degree(IntervalQuality::Augmented, 4).ok(),
            ScaleType::Mixolydian => Interval::from_quality_degree(IntervalQuality::Minor, 7).ok(),
            ScaleType::Aeolian | ScaleType::NaturalMinor => {
                Interval::from_quality_degree(IntervalQuality::Minor, 6).ok()
            }
            ScaleType::Locrian => {
                Interval::from_quality_degree(IntervalQuality::Diminished, 5).ok()
            }

            ScaleType::HarmonicMinor => {
                Interval::from_quality_degree(IntervalQuality::Augmented, 7).ok()
            }
            ScaleType::MelodicMinor => {
                Interval::from_quality_degree(IntervalQuality::Major, 6).ok()
            }

            ScaleType::Blues => Interval::from_quality_degree(IntervalQuality::Diminished, 5).ok(),
            ScaleType::WholeTone => {
                Interval::from_quality_degree(IntervalQuality::Augmented, 4).ok()
            }
            ScaleType::Octatonic => Interval::from_quality_degree(IntervalQuality::Minor, 3).ok(),
            ScaleType::BebopDominant => {
                Interval::from_quality_degree(IntervalQuality::Major, 7).ok()
            }
            ScaleType::Hijaz => Interval::from_quality_degree(IntervalQuality::Augmented, 2).ok(),

            ScaleType::Hirajoshi => Interval::from_quality_degree(IntervalQuality::Perfect, 4).ok(),
            ScaleType::InSen => Interval::from_quality_degree(IntervalQuality::Perfect, 4).ok(),

            ScaleType::Major
            | ScaleType::Ionian
            | ScaleType::PentatonicMajor
            | ScaleType::PentatonicMinor
            | ScaleType::Chromatic => None,
            #[cfg(not(feature = "bindgen"))]
            ScaleType::Custom(_) => None,
        }
    }

    pub fn characteristic_tuning(&self) -> Option<Tuning> {
        self.characteristic_interval()
            .and_then(|i| self.root.add_interval(&i).ok())
    }

    // Get the modal tonic
    pub fn modal_tonic(&self) -> Tuning {
        self.root
    }
}

/// Function analysis
/// - Analyze the scale based on the modal tonic
impl Scale {
    pub fn function(&self, chord: &Chord) -> ChordFunction {
        let root = chord.root().with_octave(0);
        let t = self.degree(1).unwrap().with_octave(0);
        let s = self.degree(4).unwrap().with_octave(0);
        let d = self.degree(5).unwrap().with_octave(0);

        if root == t {
            ChordFunction::Tonic
        } else if root == s {
            ChordFunction::Subdominant
        } else if root == d {
            ChordFunction::Dominant
        } else {
            ChordFunction::Unknown
        }
    }

    pub fn function_root(&self, chord_function: ChordFunction) -> Option<Tuning> {
        match chord_function {
            ChordFunction::Tonic => self.degree(1).ok(),
            ChordFunction::Subdominant => self.degree(4).ok(),
            ChordFunction::Dominant => self.degree(5).ok(),
            _ => None,
        }
    }

    pub fn tonic(&self) -> Tuning {
        self.function_root(ChordFunction::Tonic).unwrap()
    }

    pub fn subdominant(&self) -> Tuning {
        self.function_root(ChordFunction::Subdominant).unwrap()
    }

    pub fn dominant(&self) -> Tuning {
        self.function_root(ChordFunction::Dominant).unwrap()
    }
}

impl Scale {
    pub fn sharp(self) -> Scale {
        Self {
            root: self.root.sharp(),
            ..self
        }
    }

    pub fn flat(self) -> Scale {
        Self {
            root: self.root.flat(),
            ..self
        }
    }
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
            ScaleType::NaturalMinor => parse_intervals(&shift_major(5)),
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

            #[cfg(not(feature = "bindgen"))]
            ScaleType::Custom(pattern) => {
                let semitones = pattern
                    .iter()
                    .map(|&s| Interval::from_semitones(s))
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
            root: self.degree(rhs + 1).unwrap(),
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
                Tuning::new(PitchClass::F.sharp(), 5),
                Tuning::new(PitchClass::G, 5),
            ]
        );
    }

    #[test]
    fn test_blues_scale() {
        let a = Tuning::new(PitchClass::A, 4);
        let scale = a.scale(ScaleType::Blues);
        assert!(scale.contains(&Tuning::new(PitchClass::C, 5)));
        assert!(scale.contains(&Tuning::new(PitchClass::D, 5).sharp()));
        assert_eq!(
            scale.characteristic_tuning(),
            Some(Tuning::new(PitchClass::E.flat(), 5))
        );
    }

    #[test]
    fn test_scale_iter() {
        let s = Scale::new(tuning!(C 4), ScaleType::PentatonicMajor).unwrap();

        for t in s {
            println!("{}", t);
        }
    }

    #[test]
    fn test_scale_1() {
        let s = Scale::new(tuning!(C 4), ScaleType::Major).unwrap();

        assert_eq!(s.sharp().sharp()(1).simple(), tuning!(D 4));
        assert_eq!(s.flat().sharp()(1), tuning!(C 4));
        assert_eq!(s.flat().sharp()(1), s(1).sharp().flat());
    }

    #[test]
    fn test_scale_2() {
        let s = Scale::new(tuning!(C 4), ScaleType::Major).unwrap();

        let c = s.chord(1, ChordQuality::Major).ok();
        assert_eq!(c, Chord::new(tuning!(C 4), ChordQuality::Major).ok());

        let d = s.chord(2, ChordQuality::Major).ok();
        assert_eq!(d, Chord::new(tuning!(D 4), ChordQuality::Major).ok());

        let e = s.chord(3, ChordQuality::Major).ok();
        assert_eq!(e, Chord::new(tuning!(E 4), ChordQuality::Major).ok());
    }

    #[test]
    fn test_scale_3() {
        let s = Scale::new(tuning!(C 4), ScaleType::Major).unwrap();

        assert_eq!(
            s.degree_chord(1).ok(),
            Chord::new(tuning!(C 4), ChordQuality::Major).ok()
        );
        assert_eq!(
            s.degree_chord(2).ok(),
            Chord::new(tuning!(D 4), ChordQuality::Minor).ok()
        );
        assert_eq!(
            s.degree_chord(3).ok(),
            Chord::new(tuning!(E 4), ChordQuality::Minor).ok()
        );
        assert_eq!(
            s.degree_chord(4).ok(),
            Chord::new(tuning!(F 4), ChordQuality::Major).ok()
        );
        assert_eq!(
            s.degree_chord(5).ok(),
            Chord::new(tuning!(G 4), ChordQuality::Major).ok()
        );
        assert_eq!(
            s.degree_chord(6).ok(),
            Chord::new(tuning!(A 4), ChordQuality::Minor).ok()
        );
        assert_eq!(
            s.degree_chord(7).ok(),
            Chord::new(tuning!(B 4), ChordQuality::Diminished).ok()
        );
    }

    #[test]
    fn test_scale_4() {
        let s = Scale::new(tuning!(b E 4), ScaleType::Major).unwrap();
        assert_eq!(s(6), tuning!(C 5));
    }
}

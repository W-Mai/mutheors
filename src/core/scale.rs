//! Scale System Module
//! Provides core functions such as scale generation, modal analysis, scale and chord derivation, and more!

use crate::interval::Interval;
use crate::tuning::{PitchClass, Tuning};
use crate::MusicError;
use std::convert::TryFrom;

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
#[derive(Debug, Clone)]
pub struct Scale {
    root: Tuning,
    scale_type: ScaleType,
    intervals: Vec<Interval>, // Scale intervals
    notes: Vec<Tuning>,       // Cache the generated notes
}

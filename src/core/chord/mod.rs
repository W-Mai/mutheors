//! Chord system module
//! It includes core functions such as chord construction, analysis, inversion and voice arrangement

mod parser;
mod quality;

use crate::interval::Interval;
use crate::pitch_tuning;
use crate::tuning::Tuning;
use crate::{tuning, MusicError, PitchClass, Scale, ScaleType};
pub use quality::*;
use std::collections::{BTreeSet, HashMap};
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Chord quality classification (basic triad)
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordType {
    /// Triad
    Triad,
    /// Seventh chord
    Seventh,
    /// Extended chord (9th, 11th, 13th)
    Extended(u8),
    /// Suspended chord (sus2, sus4)
    Suspended(u8),
    /// Power chord
    Power,
    /// Altered chord
    Altered,
    /// Custom chord
    Custom,
}

/// Chord voicing
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Voicing {
    // Dense arrangement (notes within an octave)
    ClosePosition,
    /// Open arrangement (notes across octaves)
    OpenPosition,
    /// Drop 2
    Drop2,
    /// Drop 3
    Drop3,
    /// Cluster (notes within a minor second)
    Cluster,
}

/// Chord inversion state
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Inversion {
    /// Root position
    RootPosition,
    /// First inversion
    First,
    /// Second inversion
    Second,
    /// Third inversion (Seventh chord)
    Third,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionAlter {
    Add(Tuning),
    No(Tuning),
}

/// Complete Chord Description Structure
#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    root: Tuning,
    quality: ChordQuality,
    chord_type: ChordType,
    inversion: Inversion,
    voicing: Voicing,
    extensions: Vec<ExtensionAlter>, // Extended sounds (9th, 11th, etc.)
}

impl Deref for ExtensionAlter {
    type Target = Tuning;

    fn deref(&self) -> &Self::Target {
        match self {
            ExtensionAlter::Add(t) => t,
            ExtensionAlter::No(t) => t,
        }
    }
}

impl DerefMut for ExtensionAlter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ExtensionAlter::Add(t) => t,
            ExtensionAlter::No(t) => t,
        }
    }
}

impl ExtensionAlter {
    pub fn simple(&self) -> Self {
        match self {
            ExtensionAlter::Add(t) => ExtensionAlter::Add(t.simple()),
            ExtensionAlter::No(t) => ExtensionAlter::No(t.simple()),
        }
    }

    pub fn is_add(&self) -> bool {
        matches!(self, ExtensionAlter::Add(_))
    }

    pub fn is_no(&self) -> bool {
        matches!(self, ExtensionAlter::No(_))
    }
}

impl Chord {
    pub fn quality(&self) -> ChordQuality {
        self.quality
    }

    pub fn root(&self) -> Tuning {
        self.root
    }

    pub fn with_root(self, root: Tuning) -> Self {
        Self { root, ..self }
    }

    pub fn with_octave(self, octave: i8) -> Self {
        Self {
            root: self.root.with_octave(octave),
            ..self
        }
    }
}

impl Chord {
    fn construct(tuning: Tuning, chord_type: ChordType, chord_quality: ChordQuality) -> Chord {
        Self {
            root: tuning,
            quality: chord_quality,
            chord_type,
            inversion: Inversion::RootPosition,
            voicing: Voicing::ClosePosition,
            extensions: Vec::new(),
        }
    }

    /// Constructive triad (musical chord)
    pub fn new(root: Tuning, quality: ChordQuality) -> Result<Self, MusicError> {
        Ok(Self::construct(root, ChordType::Triad, quality))
    }

    /// Adding Extended interval
    pub fn with_extension(&self, tunings: &[ExtensionAlter]) -> Self {
        let mut s = self.clone();
        s.extensions.extend_from_slice(tunings);
        s
    }

    pub fn push(&mut self, ext: ExtensionAlter) {
        self.extensions.push(ext);
    }

    pub fn extend(&mut self, ext: impl IntoIterator<Item = ExtensionAlter>) {
        self.extensions.extend(ext)
    }

    /// Chord inversion
    /// - Root position
    /// - First inversion
    /// - Second inversion
    /// - Third inversion (Seventh chord)
    pub fn invert(&mut self, inversion: Inversion) {
        self.inversion = inversion;
    }

    /// TODO: Rearrangement of voices
    pub fn revoice(&mut self, voicing: Voicing) {
        self.voicing = voicing;
    }

    pub fn intervals(&self) -> Vec<Interval> {
        let mut intervals = self.quality.intervals().to_vec();
        let mut conv_intervals = vec![];
        let mut pop_intervals = vec![];
        self.extensions.iter().for_each(|t| {
            let interval = Interval::from_semitones_unchecked(t.number() - self.root.number());
            if !(intervals.contains(&interval) || conv_intervals.contains(&interval)) && t.is_add()
            {
                conv_intervals.push(interval);
                return;
            }

            if t.is_no() {
                pop_intervals.push(interval);
            }
        });
        intervals.extend(conv_intervals);
        pop_intervals.into_iter().for_each(|i| {
            if let Some(pos) = intervals.iter().position(|x| x.degree() == i.degree()) {
                intervals.remove(pos);
            }
        });
        intervals
    }

    /// Getting Chord composition tones
    pub fn components(&self) -> Vec<Tuning> {
        let mut notes = vec![self.root];

        // Adding basic intervals
        for interval in &self.intervals() {
            let current = self.root.add_interval(interval).unwrap();
            notes.push(current);
        }

        // Applying inversion
        self.apply_inversion(&mut notes);
        // Applying voicing
        self.apply_voicing(&mut notes);
        notes
    }

    pub fn simple(self) -> Self {
        Self {
            extensions: self
                .extensions
                .iter()
                .map(|i| i.simple())
                .collect::<Vec<_>>(),
            ..self
        }
    }

    // Analyzing chord functions (TSD function system)
    // - Tonic
    // - Subdominant
    // - Dominant
    // TODO: Add more functions
    pub fn function(&self, scale: &Scale) -> ChordFunction {
        scale.function(&self)
    }

    // Parsing from chord symbols (e.g. " Cmaj7")
    #[cfg_attr(feature = "bindgen", uniffi::constructor)]
    pub fn from_symbol(symbol: &str) -> Result<Self, MusicError> {
        Chord::from_str(symbol)
    }

    // TODO: Generating arpeggios
    // pub fn arpeggio(&self, style: ArpeggioStyle) -> Vec<Note> {
    //     // Realization of different arpeggio patterns
    //     // ...
    // }
}

/// Functional classification of chords (tonal analysis)
#[cfg_attr(feature = "bindgen", derive(uniffi::Enum))]
#[derive(Debug, PartialEq)]
pub enum ChordFunction {
    Unknown,
    Tonic,
    Subdominant,
    Dominant,
    SecondaryDominant,
    Neapolitan,
    //... Other Functional Categories
}

impl Chord {
    // Applying the chord inversion
    fn apply_inversion(&self, notes: &mut Vec<Tuning>) {
        match self.inversion {
            Inversion::RootPosition => return, // No inversion
            Inversion::First => {
                notes.rotate_left(1);
            }
            Inversion::Second => {
                notes.rotate_left(2);
            }
            Inversion::Third => {
                notes.rotate_left(3);
            }
        }

        let last = notes.last_mut().unwrap();
        *last = last.with_octave(last.octave() + 1);
    }

    // Applying the rules of vocal arrangement
    fn apply_voicing(&self, notes: &mut Vec<Tuning>) {
        match self.voicing {
            Voicing::ClosePosition => self.close_voicing(notes),
            Voicing::OpenPosition => self.open_voicing(notes),
            Voicing::Drop2 | Voicing::Drop3 | Voicing::Cluster => todo!(),
            // ...
        }
    }

    /// Close arrangement algorithm
    fn close_voicing(&self, notes: &mut Vec<Tuning>) {
        // Ensure that the notes are within an octave
        let base_octave = notes[0].octave();
        for note in notes.iter_mut().skip(1) {
            while note.octave() > base_octave + 1 {
                *note = note.with_octave(note.octave() - 1);
            }
        }
    }

    /// Open arrangement algorithm
    fn open_voicing(&self, notes: &mut Vec<Tuning>) {
        let mut current_octave = notes[0].octave();
        for (i, note) in notes.iter_mut().enumerate().skip(1) {
            if i % 2 == 0 {
                current_octave += 1;
            }
            *note = note.with_octave(current_octave);
        }
    }
}

impl Chord {
    pub fn add(&self, n: u8) -> Self {
        let root = self.root();
        let scale = root.scale(ScaleType::Major);
        let mut c = self.clone();

        c.push(ExtensionAlter::Add(scale(n)));

        c
    }

    pub fn no(&self, n: u8) -> Self {
        let root = self.root();
        let scale = root.scale(ScaleType::Major);
        let mut c = self.clone();

        c.push(ExtensionAlter::No(scale(n)));

        c
    }

    // Major dominant
    pub fn dom(&self, n: u8) -> Self {
        let root = self.root();
        let mut c = self.clone();

        c.extend(root.dom(n).into_iter().map(|t| ExtensionAlter::Add(t)));

        c
    }

    pub fn maj(&self, n: u8) -> Self {
        let root = self.root();
        let mut c = self.clone();

        c.extend(root.maj(n).into_iter().map(|t| ExtensionAlter::Add(t)));

        c
    }

    pub fn min(&self, n: u8) -> Self {
        let root = self.root();
        let mut c = self.clone();

        c.extend(root.min(n).into_iter().map(|t| ExtensionAlter::Add(t)));

        c
    }
}

impl Chord {
    pub fn in_scales(&self) -> Vec<Scale> {
        let chord_tunings = self
            .components()
            .iter()
            .map(|t| t.class_semitones())
            .collect::<BTreeSet<_>>();
        let tunings = [
            tuning!(C 0),
            tuning!(#C 0),
            tuning!(D 0),
            tuning!(#D 0),
            tuning!(E 0),
            tuning!(F 0),
            tuning!(#F 0),
            tuning!(G 0),
            tuning!(#G 0),
            tuning!(A 0),
            tuning!(#A 0),
            tuning!(B 0),
        ];

        let mut scales = Vec::new();
        for scale_type in ScaleType::iter().filter(|t| ![ScaleType::Chromatic].contains(t)) {
            for t in tunings.iter() {
                let scale = t.scale(scale_type);
                let scale_tunings_set = scale
                    .generate_tunings(0)
                    .unwrap()
                    .iter()
                    .map(|t| t.class_semitones())
                    .collect::<BTreeSet<_>>();

                if chord_tunings.is_subset(&scale_tunings_set) {
                    scales.push(scale);
                }
            }
        }

        scales
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut degree_alter = HashMap::new();
        // TODO: Add extensions support
        for ext in &self.extensions {
            let diff = ext.number() - self.root.number();
            let interval = Interval::from_semitones_unchecked(diff);
            let deg = interval.degree();
            let s = self.root().scale(ScaleType::Major);
            let new_deg = s(deg as u8);
            let new_number = new_deg.number();
            let acc = ext.number() - new_number;

            degree_alter.insert(deg, (ext, acc));
        }

        let mut degree_alter = degree_alter.into_iter().collect::<Vec<_>>();

        #[derive(Debug)]
        enum ExtensionMode {
            Dom,
            Major,
            Minor,
            MinorMajor,
        }
        fn match_extension_chord<'a>(
            quality: ChordQuality,
            root: &Tuning,
            degree_alter: &[(i8, (&'a ExtensionAlter, i8))],
        ) -> Option<(ExtensionMode, i8, Vec<(i8, (&'a ExtensionAlter, i8))>)> {
            let add_alters = degree_alter
                .iter()
                .filter(|(_, (ext, _))| ext.is_add())
                .map(|(i, _)| *i)
                .collect::<Vec<_>>();
            let max_add = *add_alters.iter().max()? as u8;
            let min_add = *add_alters.iter().min()?;
            let doms = root.dom(max_add);
            let majs = root.maj(max_add);
            let mins = root.min(max_add);
            let mut dom_count = 0;
            let mut maj_count = 0;
            let mut min_count = 0;
            let mut remove_list = vec![];
            for (_, (ext, _)) in degree_alter {
                let mut r = false;
                if doms.contains(*ext) {
                    dom_count += 1;
                    r = true;
                }
                if majs.contains(*ext) {
                    maj_count += 1;
                    r = true;
                }
                if mins.contains(*ext) {
                    min_count += 1;
                    r = true;
                }
                if r {
                    remove_list.push(*ext);
                }
            }
            let max_count = dom_count.max(maj_count).max(min_count);
            let max_degree = max_count * 2 + min_add - 2;
            Some((
                if dom_count == max_count
                    && (quality == ChordQuality::Major || quality == ChordQuality::Dominant7)
                {
                    ExtensionMode::Dom
                } else if maj_count == max_count
                    && (quality == ChordQuality::Major || quality == ChordQuality::Major7)
                {
                    ExtensionMode::Major
                } else if min_count == max_count
                    && (quality == ChordQuality::Minor || quality == ChordQuality::Minor7)
                {
                    ExtensionMode::Minor
                } else if maj_count == max_count
                    && (quality == ChordQuality::Minor || quality == ChordQuality::MinorMajor7)
                {
                    ExtensionMode::MinorMajor
                } else {
                    return None;
                },
                max_degree,
                degree_alter
                    .iter()
                    .filter(|(_, (alter, _))| !remove_list.contains(alter))
                    .cloned()
                    .collect(),
            ))
        }

        degree_alter.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));

        let matched = match_extension_chord(self.quality(), &self.root(), &degree_alter);

        let alter_quality = if let Some(ref matched) = matched {
            match matched.0 {
                ExtensionMode::Dom => format!("{}", matched.1),
                ExtensionMode::Major => format!("M{}", matched.1),
                ExtensionMode::Minor => format!("m{}", matched.1),
                ExtensionMode::MinorMajor => format!("mM{}", matched.1),
            }
        } else {
            Default::default()
        };

        let quality_str = if alter_quality.is_empty() {
            self.quality().to_string()
        } else {
            alter_quality
        };

        let str = if f.alternate() {
            format!("{:#}{}", self.root, quality_str)
        } else {
            format!("{}{}", self.root, quality_str)
        };
        write!(f, "{}", str)?;

        let degree_alter = if let Some(acc) = matched.map(|(_, _, acc)| acc) {
            acc
        } else {
            degree_alter
        };

        for (deg, (ext, acc)) in degree_alter {
            let acc_str = match acc {
                v if v == 0 => "",
                v if v > 0 => &"#".repeat(v as usize),
                v if v < 0 => &"b".repeat(v.abs() as usize),
                _ => "",
            };

            match ext {
                ExtensionAlter::Add(_) => write!(f, "({}{})", acc_str, deg)?,
                ExtensionAlter::No(_) => write!(f, "(no {}{})", acc_str, deg)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_chord() {
        let tuning = tuning!(# D 4);
        let scale = tuning.scale(ScaleType::Major);
        assert_eq!(scale.degree_chord(1).unwrap().root(), tuning!(# D 4));
        assert_eq!(scale.degree_chord(2).unwrap().root(), tuning!(E 4).sharp());
        assert_eq!(
            scale.degree_chord(3).unwrap().root(),
            tuning!(# F 4).sharp()
        );
        assert_eq!(scale.degree_chord(4).unwrap().root(), tuning!(# G 4));
        assert_eq!(scale.degree_chord(5).unwrap().root(), tuning!(# A 4));
        assert_eq!(scale.degree_chord(6).unwrap().root(), tuning!(B 4).sharp());
        assert_eq!(
            scale.degree_chord(7).unwrap().root(),
            tuning!(# C 5).sharp()
        );
    }

    #[test]
    fn test_major_triad() {
        let c_major = Chord::new(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap();
        assert_eq!(
            c_major.components(),
            vec![
                Tuning::new(PitchClass::C, 4),
                Tuning::new(PitchClass::E, 4),
                Tuning::new(PitchClass::G, 4)
            ]
        );
    }

    #[test]
    fn test_chord_01() {
        let seventh = ["C", "D", "E", "F", "G", "A", "B", "Eb"];
        for sym in seventh {
            let c7 = Chord::from_symbol(&(sym.to_owned() + "7")).unwrap();

            let c = Chord::from_symbol(sym).unwrap();
            let c = c.dom(7);

            let c = Chord::analyze_from(&c.components()).unwrap();

            assert_eq!(c7, c);
            println!("Checked: {}", c);
        }
    }

    #[test]
    fn test_chord_02() {
        // They are not equal but have the same components.
        let c = Chord::from_symbol("C").unwrap();
        let c = c.add(7);
        let c_maj7 = Chord::from_symbol("Cmaj7").unwrap();

        assert_ne!(c, c_maj7);
        assert_eq!(c.components(), c_maj7.components());
    }

    #[test]
    fn test_chord_03() {
        let c = Chord::from_symbol("C").unwrap();
        let c = c.maj(9);

        // Add additional notes for Cmaj9, which includes the 9th degree.
        let c = c.add(7);
        let c = c.add(7);
        let c = c.add(7);

        //TODO: Parsing Cmaj9 is not supported yet.

        println!("{}", c);

        assert_eq!(
            c.components(),
            [
                tuning!(C 4),
                tuning!(E 4),
                tuning!(G 4),
                tuning!(B 4),
                tuning!(D 5)
            ]
        );
    }

    #[test]
    fn test_chord_04() {
        let c = Chord::from_symbol("C7").unwrap();

        let c = c.no(7);

        let c_maj = Chord::from_symbol("C").unwrap();

        assert_eq!(c.components(), c_maj.components());
    }

    #[test]
    fn test_chord_05() {
        let mut tunings = vec![
            tuning!(G 4),
            tuning!(B 4),
            tuning!(D 5),
            tuning!(F 5),
            tuning!(A 5),
        ];

        let c = Chord::analyze_from(&tunings).unwrap();

        assert_eq!(c.to_string(), "G9");

        tunings.push(tuning!(C 6));

        let c = Chord::analyze_from(&tunings).unwrap();

        assert_eq!(c.to_string(), "G11");
    }

    #[test]
    fn test_chord_06() {
        let c = Chord::from_symbol("Gm").unwrap();
        let c = c.maj(9);

        assert_eq!(c.to_string(), "GmM9");

        assert_eq!(
            c.components(),
            [
                tuning!(G 4),
                tuning!(b B 4),
                tuning!(D 5),
                tuning!(# F 5),
                tuning!(A 5)
            ]
        );
    }

    #[test]
    fn test_dominant_seventh() {
        let g7 = Chord::new(Tuning::new(PitchClass::G, 4), ChordQuality::Dominant7).unwrap();
        assert_eq!(
            g7.components(),
            vec![
                Tuning::new(PitchClass::G, 4),
                Tuning::new(PitchClass::B, 4),
                Tuning::new(PitchClass::D, 5),
                Tuning::new(PitchClass::F, 5)
            ]
        );
    }

    #[test]
    fn test_chord_extension_1() -> Result<(), MusicError> {
        let s = tuning!(C 4).scale(ScaleType::Major);
        let c = s.degree_chord(1)?;
        let c9 = c.dom(9);

        println!("Chord 1: {:?}", c9);

        Ok(())
    }

    #[test]
    fn test_inversion() {
        let mut cmaj = Chord::new(Tuning::new(PitchClass::C, 4), ChordQuality::Major).unwrap();
        cmaj.invert(Inversion::First);
        assert_eq!(
            cmaj.components(),
            vec![
                Tuning::new(PitchClass::E, 4),
                Tuning::new(PitchClass::G, 4),
                Tuning::new(PitchClass::C, 5)
            ] // 第一转位
        );
    }

    #[test]
    fn test_chord_function_1() {
        let scale = Scale::new(tuning!(C 4), ScaleType::Major).unwrap();
        let chord = Chord::new(tuning!(C 4), ChordQuality::Major).unwrap();

        assert_eq!(chord.function(&scale), ChordFunction::Tonic);

        let chord = Chord::new(tuning!(C 3), ChordQuality::Major).unwrap();
        assert_eq!(chord.function(&scale), ChordFunction::Tonic);
    }

    #[test]
    fn test_in_chords() {
        let c = Chord::from_symbol("C").unwrap();
        let r = c.root();
        let ss = c.in_scales();

        for s in ss.iter() {
            println!(
                "{}{:?} {:?} {}",
                s.root(),
                s.scale_type(),
                c.function(s),
                s.generate_tunings(0)
                    .unwrap()
                    .iter()
                    .map(|t| { t.with_octave(4) })
                    .enumerate()
                    .find(|x| { x.1.class_semitones() == r.class_semitones() })
                    .unwrap()
                    .0
                    + 1
            );
        }
    }
}

#[macro_export]
macro_rules! tuning {
    (# $note:ident $octave:expr) => {
        Tuning {
            class: pitch_class!(# $note),
            octave: $octave,
            freq: None,
        }
    };
    (b $note:ident $octave:expr) => {
        Tuning {
            class: pitch_class!(b $note),
            octave: $octave,
            freq: None,
        }
    };
    ($note:ident $octave:expr) => {
        Tuning {
            class: pitch_class!($note),
            octave: $octave,
            freq: None,
        }
    };
}

#[macro_export]
macro_rules! pitch_class {
    (# C) => {
        PitchClass::CSharpOrDFlat
    };
    (b C) => {
        PitchClass::B
    };
    (# D) => {
        PitchClass::DSharpOrEFlat
    };
    (b D) => {
        PitchClass::CSharpOrDFlat
    };
    (# E) => {
        PitchClass::F
    };
    (b E) => {
        PitchClass::DSharpOrEFlat
    };
    (# F) => {
        PitchClass::FSharpOrGFlat
    };
    (b F) => {
        PitchClass::E
    };
    (# G) => {
        PitchClass::GSharpOrAFlat
    };
    (b G) => {
        PitchClass::FSharpOrGFlat
    };
    (# A) => {
        PitchClass::ASharpOrBFlat
    };
    (b A) => {
        PitchClass::GSharpOrAFlat
    };
    (# B) => {
        PitchClass::C
    };
    (b B) => {
        PitchClass::ASharpOrBFlat
    };
    (C) => {
        PitchClass::C
    };
    (D) => {
        PitchClass::D
    };
    (E) => {
        PitchClass::E
    };
    (F) => {
        PitchClass::F
    };
    (G) => {
        PitchClass::G
    };
    (A) => {
        PitchClass::A
    };
    (B) => {
        PitchClass::B
    };
}

#[macro_export]
macro_rules! beats {
    ($dg:expr; $($bts:expr),+ ; with $($tunings:expr),+) => {{
        let bts = [$($bts),+];
        let tunings = [$($tunings),+];
        
        assert_eq!(bts.len(), tunings.len());

        bts.iter().enumerate().map(|(i, &bt)| {
            $dg.beat(bt).with_note(tunings[i].into())
        }).collect::<Vec<_>>()
    }};
}

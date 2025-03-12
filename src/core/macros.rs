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
    ($dg:expr; $($bt:expr => $tuning:expr),+ $(,)?) => {{
        let mappings = [$(($bt, $tuning)),+];

        mappings.iter().map(|(beat, tune)| {
            $dg.beat(*beat).with_note((*tune).into())
        }).collect::<Vec<_>>()
    }};
}

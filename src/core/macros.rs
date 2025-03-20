#[macro_export]
macro_rules! tuning {
    (# $note:ident $octave:expr) => {
        Into::<Tuning>::into(pitch_tuning!(# $note)).with_octave($octave)
    };
    (b $note:ident $octave:expr) => {
        Into::<Tuning>::into(pitch_tuning!(b $note)).with_octave($octave)
    };
    ($note:ident $octave:expr) => {
        Into::<Tuning>::into(pitch_tuning!($note)).with_octave($octave)
    };
}

#[macro_export]
macro_rules! pitch_tuning {
    (# C) => {
        PitchClass::C.sharp()
    };
    (b C) => {
        PitchClass::B
    };
    (# D) => {
        PitchClass::D.sharp()
    };
    (b D) => {
        PitchClass::D.flat()
    };
    (# E) => {
        PitchClass::F
    };
    (b E) => {
        PitchClass::E.flat()
    };
    (# F) => {
        PitchClass::F.sharp()
    };
    (b F) => {
        PitchClass::F.flat()
    };
    (# G) => {
        PitchClass::G.sharp()
    };
    (b G) => {
        PitchClass::G.flat()
    };
    (# A) => {
        PitchClass::A.sharp()
    };
    (b A) => {
        PitchClass::A.flat()
    };
    (# B) => {
        PitchClass::B.sharp()
    };
    (b B) => {
        PitchClass::B.flat()
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
            let tune: Tuning = (*tune).into();
            $dg.beat(*beat).with_note(tune.into())
        }).collect::<Vec<Note>>()
    }};
}

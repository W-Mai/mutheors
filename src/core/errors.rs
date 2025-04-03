#[derive(Debug, thiserror::Error)]
pub enum MusicError {
    #[error("Invalid pitch parameter")]
    InvalidPitch,

    #[error("Out of MIDI range: {0}")]
    MidiOutOfRange(u8),

    #[error("Unsupported chord types")]
    UnsupportedChord,

    #[error("Music Theory Conflict: {0}")]
    TheoryViolation(String),

    #[error("MIDI operation failed: {0}")]
    MidiError(String),

    #[error("Invalid Duration {0}")]
    InvalidDuration(f32),

    #[error("Ratio of invalid legato: {actual}:{base}")]
    InvalidTupletRatio { actual: u8, base: u8 },

    #[error("Unsupported tuplet types")]
    UnsupportedTuplet,

    #[error("The duration of the tied notes does not match the note values.")]
    TupletDurationMismatch,

    #[error("Invalid interval degree: {degree}")]
    InvalidIntervalDegree { degree: u8 },

    #[error("Invalid interval quality: {name}")]
    IntervalParseError { name: String },

    #[error("Invalid interval quality")]
    InvalidIntervalQuality,

    #[error("Invalid scale degree {0}")]
    InvalidScaleDegree(u8),

    #[error(
        "Invalid Octave {octave}, IT MUST BE IN [0, 10]. Because Humans can only hear 10 octaves."
    )]
    InvalidOctave { octave: i8 },

    #[error("Invalid chord quality")]
    InvalidChordQuality,
}

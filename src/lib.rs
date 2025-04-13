#![feature(unboxed_closures, fn_traits)]

//! # MuTheoRS
//! > This crate provides a set of tools for working with music theory concepts,
//!
//! - PitchClass: C/D/E/F/G/A/B...
//! - Tuning: C4/C#4/D4/E4/F4/G4/A4/B4...
//! - Scale: C major, C minor, C# pentatonic...
//! - Duration: quarter, eighth, half...
//! - Note: C4 quarter, C4 eighth, C4 half...
//! - Chord: C major, C minor, C7...
//! - Measure: bundle of notes and chords
//! - Track: bundle of measures
//! - Score: bundle of tracks
//!
//! - Midi: play the score using midi
//!
//! Other Abilities:
//! - Interval: describe the distance between two `Tuning`s

#[cfg(feature = "midi_io")]
mod midi;
#[cfg(feature = "midi_io")]
pub use midi::*;

mod composition;
mod core;
mod tests;

pub use composition::*;
pub use core::*;
pub use tests::*;

#[cfg(feature = "bindgen")]
mod uniffi_binding;
#[cfg(feature = "bindgen")]
uniffi::setup_scaffolding!();

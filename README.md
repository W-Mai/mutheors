# MuTheoRS

> This crate provides a set of tools for working with music theory concepts,

## Features

- [x] Chord
- [x] Scale
- [x] Interval
- [x] Note
- [x] MIDI IO

## Modules and Types

- PitchClass: C/D/E/F/G/A/B...
- Tuning: C4/C#4/D4/E4/F4/G4/A4/B4...
- Duration: quarter, eighth, half...
- Note: C4 quarter, C4 eighth, C4 half...
- Scale: C major, C minor, C# pentatonic...
- Chord: C major, C minor, C7...
- Measure: bundle of notes and chords
- Track: bundle of measures
- Score: bundle of tracks
- Midi: play the score using midi
  Other Abilities:
- Interval: describe the distance between two `Tuning`s

## Simple Usage

1. Pick a PitchClass
2. From PitchClass, generate a Tuning
3. Generate a Chord from Tuning

## Bindgen

### Swift

#### Build & Generate

```bash
cargo build --release --features bindgen
cargo run --features bindgen --bin uniffi-bindgen -- generate --library target/release/libmutheors.a --language swift --out-dir generated_bindgen
```

#### Generate Dynamic Library

```bash
pushd generated_bindgen && swiftc \
    -module-name mutheors \
    -emit-library -o libmutheors.dylib \
    -emit-module -emit-module-path ./ \
    -parse-as-library \
    -L ../target/release/ \
    -lmutheors \
    -Xcc -fmodule-map-file=mutheorsFFI.modulemap \
    mutheors.swift
popd
```

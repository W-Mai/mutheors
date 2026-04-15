//! E2E tests: composition module → audio synthesis → onset detection

use mutheors::audio::OnsetDetector;
use mutheors::*;
use std::f32::consts::PI;

/// Synthesize audio from notes at a given tempo
fn synthesize_notes(notes: &[Note], tempo: f32, sample_rate: f32) -> Vec<f32> {
    let mut samples = Vec::new();
    for note in notes {
        let freq = note.tuning().frequency();
        let dur_secs = note.duration().in_seconds(tempo);
        let n = (sample_rate * dur_secs) as usize;
        let attack = (sample_rate * 0.002) as usize; // 2ms attack
        let decay_time = sample_rate * 0.03; // 30ms decay — percussive envelope

        for i in 0..n {
            let env = if i < attack {
                i as f32 / attack as f32
            } else {
                (-(i as f32 - attack as f32) / decay_time).exp()
            };
            samples.push(env * note.velocity() * (2.0 * PI * freq * i as f32 / sample_rate).sin());
        }
    }
    samples
}

#[test]
fn e2e_4_4_kick_120bpm() {
    // 8 quarter-note kicks at 120 BPM → expect ~8 onsets, BPM ≈ 120
    let q = Duration::new(DurationBase::Quarter);
    let kick = Note::new(Tuning::new(PitchClass::A, 1))
        .with_duration(q)
        .with_velocity(0.9);
    let notes: Vec<_> = (0..8).map(|_| kick.clone()).collect();

    let audio = synthesize_notes(&notes, 120.0, 44100.0);
    let result = OnsetDetector::new(44100.0).detect(&audio);

    // Should detect most kicks (allow ≤ 2 missed)
    assert!(
        result.onsets.len() >= 6 && result.onsets.len() <= 10,
        "Expected 6-10 onsets from 8 kicks, got {}",
        result.onsets.len()
    );

    // BPM should be close to 120
    let bpm = result.bpm.expect("Should estimate BPM from 8 kicks");
    assert!(
        (bpm - 120.0).abs() < 15.0,
        "Expected ~120 BPM, got {:.1}",
        bpm
    );
}

#[test]
fn e2e_rock_beat_100bpm() {
    // kick-hihat-snare-hihat at eighth notes, 100 BPM, 4 measures
    let e = Duration::new(DurationBase::Eighth);
    let kick = Note::new(Tuning::new(PitchClass::A, 1)).with_duration(e).with_velocity(0.9);
    let snare = Note::new(Tuning::new(PitchClass::D, 3)).with_duration(e).with_velocity(0.8);
    let hihat = Note::new(Tuning::new(PitchClass::C, 6)).with_duration(e).with_velocity(0.4);

    let pattern: Vec<_> = [&kick, &hihat, &snare, &hihat]
        .iter()
        .cycle()
        .take(32)
        .map(|n| (*n).clone())
        .collect();

    let audio = synthesize_notes(&pattern, 100.0, 44100.0);
    let result = OnsetDetector::new(44100.0).detect(&audio);

    // 32 events but hihats are quiet — expect mostly kicks and snares detected
    assert!(
        result.onsets.len() >= 10,
        "Should detect significant onsets from rock beat, got {}",
        result.onsets.len()
    );

    // BPM: eighth note pulse ~200 or quarter note pulse ~100
    if let Some(bpm) = result.bpm {
        assert!(
            (bpm - 100.0).abs() < 15.0 || (bpm - 200.0).abs() < 15.0,
            "Expected ~100 or ~200 BPM, got {:.1}",
            bpm
        );
    }
}

#[test]
fn e2e_tempo_roundtrip() {
    // Set tempo in Score → synthesize → detect BPM → should match
    for target_bpm in [90.0f32, 120.0, 150.0] {
        let q = Duration::new(DurationBase::Quarter);
        let note = Note::new(Tuning::new(PitchClass::A, 2))
            .with_duration(q)
            .with_velocity(0.9);
        let notes: Vec<_> = (0..16).map(|_| note.clone()).collect();

        let audio = synthesize_notes(&notes, target_bpm, 44100.0);
        let result = OnsetDetector::new(44100.0).detect(&audio);

        let bpm = result
            .bpm
            .unwrap_or_else(|| panic!("No BPM detected for target {}BPM", target_bpm));
        assert!(
            (bpm - target_bpm).abs() < 15.0,
            "Target {}BPM: detected {:.1}BPM",
            target_bpm,
            bpm
        );
    }
}

#[test]
fn e2e_silence_no_onsets() {
    let audio = vec![0.0f32; 44100 * 3];
    let result = OnsetDetector::new(44100.0).detect(&audio);
    assert!(result.onsets.is_empty());
    assert!(result.bpm.is_none());
}

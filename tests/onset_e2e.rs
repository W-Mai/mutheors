//! E2E tests: composition module → audio synthesis → onset detection

use mutheors::audio::OnsetDetector;
use mutheors::*;
use std::f32::consts::PI;

/// Synthesize audio from a Score: each Note becomes a short sine burst at its pitch.
fn synthesize_score(score: &Score<1>, sample_rate: f32) -> Vec<f32> {
    let tempo = score.tempo();
    let track = &score.get_tracks()[0];
    let mut samples = Vec::new();

    for measure in track.get_measures() {
        match measure {
            Measure::Note(notes) => {
                for note in notes {
                    let freq = note.tuning().frequency();
                    let dur_secs = note.duration().in_seconds(tempo);
                    let n = (sample_rate * dur_secs) as usize;
                    let attack = (sample_rate * 0.005) as usize; // 5ms attack

                    for i in 0..n {
                        let env = if i < attack {
                            i as f32 / attack as f32
                        } else {
                            (-(i as f32 - attack as f32) / (n as f32 * 0.3)).exp()
                        };
                        samples.push(
                            env * note.velocity()
                                * (2.0 * PI * freq * i as f32 / sample_rate).sin(),
                        );
                    }
                }
            }
            Measure::Rest => {
                // Quarter rest duration
                let dur_secs = Duration::new(DurationBase::Quarter).in_seconds(tempo);
                let n = (sample_rate * dur_secs) as usize;
                samples.extend(vec![0.0f32; n]);
            }
            _ => {}
        }
    }
    samples
}

#[test]
fn e2e_basic_4_4_beat_detection() {
    // Build a simple 4/4 drum pattern: kick on every quarter note
    // 2 measures at 120 BPM = 8 quarter notes = 4 seconds
    let mut score: Score<1> = Score::new()
        .with_tempo(120.0)
        .with_time_signature(4, DurationBase::Quarter);

    // Kick drum ≈ low frequency burst (A1 = 55Hz)
    let kick = Note::new(Tuning::new(PitchClass::A, 1))
        .with_duration(Duration::new(DurationBase::Quarter))
        .with_velocity(0.9);

    for _ in 0..2 {
        let mut m = Measure::new();
        m.note(vec![kick.clone(), kick.clone(), kick.clone(), kick.clone()]);
        score.push_measures([m]);
    }

    let audio = synthesize_score(&score, 44100.0);
    let detector = OnsetDetector::new(44100.0);
    let result = detector.detect(&audio);

    // 8 quarter notes → expect ~8 onsets
    assert!(
        result.onsets.len() >= 6,
        "Should detect at least 6 onsets from 8 kicks, got {}",
        result.onsets.len()
    );

    // BPM should be close to 120
    if let Some(bpm) = result.bpm {
        assert!(
            (bpm - 120.0).abs() < 15.0,
            "Expected ~120 BPM, got {:.1}",
            bpm
        );
    }
}

#[test]
fn e2e_rock_beat_pattern() {
    // Classic rock beat: kick-hihat-snare-hihat pattern
    // Kick = A1(55Hz), Snare = D3(147Hz), HiHat = noise-like high freq
    let mut score: Score<1> = Score::new()
        .with_tempo(100.0)
        .with_time_signature(4, DurationBase::Quarter);

    let eighth = Duration::new(DurationBase::Eighth);
    let kick = Note::new(Tuning::new(PitchClass::A, 1)).with_duration(eighth).with_velocity(0.9);
    let snare = Note::new(Tuning::new(PitchClass::D, 3)).with_duration(eighth).with_velocity(0.8);
    let hihat = Note::new(Tuning::new(PitchClass::C, 6)).with_duration(eighth).with_velocity(0.4);

    // 4 measures of: kick-hihat-snare-hihat-kick-hihat-snare-hihat
    for _ in 0..4 {
        let mut m = Measure::new();
        m.note(vec![
            kick.clone(), hihat.clone(), snare.clone(), hihat.clone(),
            kick.clone(), hihat.clone(), snare.clone(), hihat.clone(),
        ]);
        score.push_measures([m]);
    }

    let audio = synthesize_score(&score, 44100.0);
    let detector = OnsetDetector::new(44100.0).with_threshold(1.3);
    let result = detector.detect(&audio);

    // 4 measures × 8 eighth notes = 32 events
    assert!(
        result.onsets.len() >= 16,
        "Should detect many onsets from rock beat, got {}",
        result.onsets.len()
    );

    // BPM: 100 BPM with eighth notes → onset rate is 200/min
    // But BPM estimator should find the quarter note pulse (~100 BPM)
    // or the eighth note pulse (~200 BPM)
    if let Some(bpm) = result.bpm {
        let close_to_100 = (bpm - 100.0).abs() < 15.0;
        let close_to_200 = (bpm - 200.0).abs() < 15.0;
        assert!(
            close_to_100 || close_to_200,
            "Expected ~100 or ~200 BPM, got {:.1}",
            bpm
        );
    }
}

#[test]
fn e2e_waltz_3_4_time() {
    // 3/4 waltz: strong-weak-weak pattern
    let mut score: Score<1> = Score::new()
        .with_tempo(140.0)
        .with_time_signature(3, DurationBase::Quarter);

    let quarter = Duration::new(DurationBase::Quarter);
    let strong = Note::new(Tuning::new(PitchClass::A, 1)).with_duration(quarter).with_velocity(0.9);
    let weak = Note::new(Tuning::new(PitchClass::E, 3)).with_duration(quarter).with_velocity(0.4);

    for _ in 0..4 {
        let mut m = Measure::new();
        m.note(vec![strong.clone(), weak.clone(), weak.clone()]);
        score.push_measures([m]);
    }

    let audio = synthesize_score(&score, 44100.0);
    let detector = OnsetDetector::new(44100.0);
    let result = detector.detect(&audio);

    // 4 measures × 3 beats = 12 events
    assert!(
        result.onsets.len() >= 8,
        "Should detect onsets from waltz, got {}",
        result.onsets.len()
    );
}

#[test]
fn e2e_tempo_from_score_matches_detection() {
    // Verify that the tempo we set in Score matches what onset detection finds
    for target_bpm in [80.0f32, 120.0, 160.0] {
        let mut score: Score<1> = Score::new()
            .with_tempo(target_bpm)
            .with_time_signature(4, DurationBase::Quarter);

        let quarter = Duration::new(DurationBase::Quarter);
        let note = Note::new(Tuning::new(PitchClass::A, 2))
            .with_duration(quarter)
            .with_velocity(0.9);

        // 4 measures of quarter notes
        for _ in 0..4 {
            let mut m = Measure::new();
            m.note(vec![note.clone(), note.clone(), note.clone(), note.clone()]);
            score.push_measures([m]);
        }

        assert_eq!(score.tempo(), target_bpm);

        let audio = synthesize_score(&score, 44100.0);
        let detector = OnsetDetector::new(44100.0);
        let result = detector.detect(&audio);

        if let Some(detected_bpm) = result.bpm {
            assert!(
                (detected_bpm - target_bpm).abs() < 15.0,
                "Target {}BPM: detected {:.1}BPM",
                target_bpm,
                detected_bpm
            );
        }
    }
}

use mutheors::*;

fn main() {
    println!("═══════════════════════════════════════════");
    println!("  MuTheoRS Fretboard System Demo");
    println!("═══════════════════════════════════════════\n");

    guitar_chord_fingerings();
    voice_leading_demo();
    fretboard_diagram_demo();
    multi_instrument_demo();
}

fn chord(root: PitchClass, quality: ChordQuality) -> Chord {
    Chord::new(Tuning::new(root, 3), quality).unwrap()
}

fn guitar_fretboard() -> StringedFretboard {
    StringedFretboard::new(InstrumentPresets::guitar_standard()).unwrap()
}

fn guitar_chord_fingerings() {
    println!("【1】吉他和弦指法生成");
    println!("─────────────────────────────────────────\n");

    let fb = guitar_fretboard();
    let gen = ChordFingeringGenerator::new();

    let chords = [
        ("C",  chord(PitchClass::C, ChordQuality::Major)),
        ("Am", chord(PitchClass::A, ChordQuality::Minor)),
        ("G",  chord(PitchClass::G, ChordQuality::Major)),
        ("Em", chord(PitchClass::E, ChordQuality::Minor)),
        ("D",  chord(PitchClass::D, ChordQuality::Major)),
    ];

    for (name, c) in &chords {
        match gen.generate_chord_fingerings(&fb, c) {
            Ok(f) => println!(
                "  {} — {} 种指法，最佳难度: {:.2}",
                name, f.len(), f.first().map(|x| x.difficulty).unwrap_or(0.0)
            ),
            Err(e) => println!("  {} — 生成失败: {}", name, e),
        }
    }
    println!();
}

fn voice_leading_demo() {
    println!("【2】声部进行优化 (I-V-vi-IV)");
    println!("─────────────────────────────────────────\n");

    let fb = guitar_fretboard();
    let gen = ChordFingeringGenerator::new();
    let opt = VoiceLeadingOptimizer::new();

    let names = ["C", "G", "Am", "F"];
    let progression = vec![
        chord(PitchClass::C, ChordQuality::Major),
        chord(PitchClass::G, ChordQuality::Major),
        chord(PitchClass::A, ChordQuality::Minor),
        chord(PitchClass::F, ChordQuality::Major),
    ];

    println!("  和弦进行: C → G → Am → F\n");

    match opt.optimize_progression(&fb, &progression, &gen) {
        Ok(seq) => {
            for (i, (name, fg)) in names.iter().zip(seq.iter()).enumerate() {
                let frets: Vec<String> = fg.positions.iter()
                    .map(|fp| format!("{}弦{}品", fp.position.string + 1, fp.position.fret))
                    .collect();
                println!("  {}. {} (难度 {:.2}): {}", i + 1, name, fg.difficulty, frets.join(", "));
            }

            let a = opt.analyze_sequence(&seq);
            println!("\n  📊 序列分析:");
            println!("     总成本: {:.2}", a.total_cost);
            println!("     平均难度: {:.2}", a.average_difficulty);
            println!("     最大转换成本: {:.2}", a.max_transition_cost);
            for s in &a.suggestions {
                println!("     💡 {}", s);
            }
        }
        Err(e) => println!("  优化失败: {}", e),
    }
    println!();
}

fn fretboard_diagram_demo() {
    println!("【3】指板图可视化");
    println!("─────────────────────────────────────────\n");

    let fb = guitar_fretboard();
    let gen = ChordFingeringGenerator::new();
    let diag = FretboardDiagramGenerator::new();

    let c = chord(PitchClass::C, ChordQuality::Major);
    if let Ok(fingerings) = gen.generate_chord_fingerings(&fb, &c) {
        if let Some(best) = fingerings.first() {
            println!("  C Major 指板图:\n");
            if let Ok(diagram) = diag.generate_diagram(&fb, best) {
                for line in diagram.lines() {
                    println!("    {}", line);
                }
            }
        }
    }
    println!();
}

fn multi_instrument_demo() {
    println!("【4】多乐器支持");
    println!("─────────────────────────────────────────\n");

    let instruments: Vec<(&str, StringedInstrumentConfig)> = vec![
        ("吉他 (标准)", InstrumentPresets::guitar_standard()),
        ("贝斯 (4弦)",  InstrumentPresets::bass_4_string()),
        ("尤克里里",     InstrumentPresets::ukulele_soprano()),
        ("曼陀林",       InstrumentPresets::mandolin_standard()),
    ];

    for (name, cfg) in instruments {
        let fb = StringedFretboard::new(cfg).unwrap();
        println!("  {} — {}弦, {}品", name, fb.string_count(), fb.fret_count());
    }

    let piano = KeyboardFretboard::new(InstrumentPresets::piano_88_key()).unwrap();
    println!("  钢琴 (88键) — {}键", piano.key_count());
    println!();
}

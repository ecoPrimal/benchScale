// Stadial analysis — wide/compress/narrow/stretch, mix/grind/agglutinate
// Run as a test with --nocapture to see output

#[cfg(test)]
mod analysis {
    use std::collections::HashMap;

    // inline the data so we can run standalone
    #[derive(Debug, Clone)]
    struct Period {
        id: &'static str, name: &'static str,
        start: f64, end: f64,
        phase: &'static str, force: &'static str,
        temp: f64, sea: f64,
    }

    #[derive(Debug, Clone)]
    struct Event {
        period: &'static str, force: &'static str,
        action: &'static str, result: &'static str,
    }

    fn periods() -> Vec<Period> {
        vec![
            Period { id: "laramide",         name: "Laramide Orogeny",          start: 80e6,  end: 35e6,  phase: "interstadial", force: "tectonic",   temp:  4.0, sea:  100.0 },
            Period { id: "miocene_warm",     name: "Miocene Climatic Optimum",  start: 17e6,  end: 13e6,  phase: "interstadial", force: "biological", temp:  4.0, sea:   40.0 },
            Period { id: "teton_fault",      name: "Teton Fault Initiation",    start: 13e6,  end: 2.6e6, phase: "interstadial", force: "tectonic",   temp:  2.0, sea:   25.0 },
            Period { id: "pleistocene_onset",name: "Pleistocene Onset",         start: 2.6e6, end: 2e6,   phase: "transition",  force: "glacial",    temp: -4.0, sea:  -60.0 },
            Period { id: "pre_illinoian",    name: "Pre-Illinoian Glacials",    start: 2e6,   end: 300e3, phase: "stadial",      force: "glacial",    temp: -6.0, sea: -100.0 },
            Period { id: "bull_lake",        name: "Bull Lake Glaciation",      start: 200e3, end: 130e3, phase: "stadial",      force: "glacial",    temp: -6.0, sea: -120.0 },
            Period { id: "sangamon",         name: "Sangamonian Interglacial",  start: 130e3, end: 75e3,  phase: "interstadial", force: "fluvial",    temp:  1.0, sea:    6.0 },
            Period { id: "early_wisconsin",  name: "Early Wisconsin Glacial",   start: 75e3,  end: 30e3,  phase: "stadial",      force: "glacial",    temp: -5.0, sea:  -80.0 },
            Period { id: "pinedale",         name: "Pinedale Glaciation",       start: 30e3,  end: 12e3,  phase: "stadial",      force: "glacial",    temp: -8.0, sea: -130.0 },
            Period { id: "younger_dryas",    name: "Younger Dryas",             start: 12900.0, end: 11700.0, phase: "stadial",  force: "glacial",    temp: -6.0, sea:  -60.0 },
            Period { id: "holocene_thermal", name: "Holocene Climatic Optimum", start: 9000.0, end: 5000.0,  phase: "interstadial", force: "biological", temp: 1.0, sea: 0.0 },
            Period { id: "neoglacial",       name: "Neoglaciation",             start: 5000.0, end: 150.0,   phase: "stadial",   force: "glacial",    temp: -0.5, sea:  -1.0 },
            Period { id: "anthropocene",     name: "Anthropocene",              start: 150.0, end: 0.0,      phase: "transition", force: "biological", temp: 1.1, sea: 0.2 },
        ]
    }

    fn events() -> Vec<Event> {
        vec![
            Event { period: "laramide",         force: "tectonic",   action: "Compressional orogeny lifts Precambrian basement", result: "2.7Ga gneiss exposed" },
            Event { period: "teton_fault",       force: "tectonic",   action: "Normal fault drops east 6km, west rises", result: "4199m summit, 1900m valley, 2300m relief" },
            Event { period: "bull_lake",         force: "glacial",    action: "Valley glaciers >1km thick", result: "Initial U-valleys carved" },
            Event { period: "pinedale",          force: "glacial",    action: "12 glaciers carve simultaneously", result: "Cathedral Group sculpted, moraines dam lakes" },
            Event { period: "pinedale",          force: "glacial",    action: "Glacial plucking + freeze-thaw", result: "600m vertical north face, Exum Ridge" },
            Event { period: "sangamon",          force: "fluvial",    action: "Meltwater fills basins", result: "Proto-Jenny Lake, Snake River course" },
            Event { period: "holocene_thermal",  force: "fluvial",    action: "Final deglaciation fills lakes", result: "Jenny Lake 480ha 73m deep, 7 lakes" },
            Event { period: "holocene_thermal",  force: "biological", action: "Forests climb to 3000m", result: "Full ecosystem: grizzly, elk, moose, pika" },
            Event { period: "anthropocene",      force: "biological", action: "Glacier retreating 3m/yr", result: "150ka of work changes in 150yr" },
        ]
    }

    #[test]
    fn run_analysis() {
        let p = periods();
        let e = events();

        println!();
        println!("══════════════════════════════════════════════════════════════════════════════════");
        println!("STADIAL ANALYSIS — wide / compress / narrow / stretch / mix / grind / agglutinate");
        println!("══════════════════════════════════════════════════════════════════════════════════\n");

        // ═══ 1. WIDE — spread everything out linearly ═══
        println!("━━━ 1. WIDE — linear timeline, all periods ━━━\n");
        let total_span = p[0].start; // 80Ma
        for period in &p {
            let pct_start = (period.start / total_span) * 100.0;
            let pct_end = (period.end / total_span) * 100.0;
            let pct_width = pct_start - pct_end;
            let bar_width = (pct_width * 0.6) as usize;
            let bar: String = match period.phase {
                "stadial" => "▓".repeat(bar_width.max(1)),
                "interstadial" => "░".repeat(bar_width.max(1)),
                _ => "▒".repeat(bar_width.max(1)),
            };
            println!("  {:>6.1}% │{:<50}│ {} ({})",
                pct_start, bar, period.name, period.phase);
        }
        println!("\n  ▓ = stadial (cold)  ░ = interstadial (warm)  ▒ = transition\n");
        println!("  PROBLEM: Recent periods invisible. Pinedale (30-12ka) = 0.02% of timeline.");
        println!("  The last 200ka where ALL the carving happened is <0.3% of the view.\n");

        // ═══ 2. COMPRESS — logarithmic time ═══
        println!("━━━ 2. COMPRESS — log₁₀ scale (deep time + recent time visible) ━━━\n");
        for period in &p {
            let log_start = if period.start > 0.0 { period.start.log10() } else { 0.0 };
            let log_end = if period.end > 0.0 { period.end.log10() } else { 0.0 };
            let log_width = log_start - log_end;
            let bar_len = (log_width * 10.0) as usize;
            let symbol = match period.phase {
                "stadial" => "▓",
                "interstadial" => "░",
                _ => "▒",
            };
            let bar: String = symbol.repeat(bar_len.max(1));
            let scale = if period.start >= 1e6 { format!("{:.0}Ma", period.start / 1e6) }
                        else if period.start >= 1e3 { format!("{:.0}ka", period.start / 1e3) }
                        else { format!("{:.0}ya", period.start) };
            println!("  {:>8} │{:<40}│ {} ({}, {:.1}°C, {:.0}m sea)",
                scale, bar, period.name, period.force, period.temp, period.sea);
        }
        println!("\n  NOW compressed time is honest. Each order of magnitude gets equal space.");
        println!("  Pinedale and Younger Dryas become VISIBLE.\n");

        // ═══ 3. NARROW — zoom into the carving window (200ka - now) ═══
        println!("━━━ 3. NARROW — zoom to 200ka-present (where ice sculpted) ━━━\n");
        let narrow: Vec<&Period> = p.iter().filter(|p| p.start <= 200_000.0).collect();
        let narrow_span = 200_000.0;
        for period in &narrow {
            let start_pct = (period.start / narrow_span) * 100.0;
            let end_pct = (period.end / narrow_span) * 100.0;
            let width_pct = start_pct - end_pct;
            let bar_len = (width_pct * 0.5) as usize;
            let symbol = match period.phase {
                "stadial" => "▓",
                "interstadial" => "░",
                _ => "▒",
            };
            let bar: String = symbol.repeat(bar_len.max(1));
            let event_count = e.iter().filter(|ev| ev.period == period.id).count();
            println!("  {:>6.1}% │{:<40}│ {} — {} events, {:.1}°C",
                start_pct, bar, period.name, event_count, period.temp);
        }
        println!("\n  Bull Lake alone = 35% of the carving window. Pinedale = 9%.");
        println!("  But Pinedale has MORE EVENTS (3) than Bull Lake (1).");
        println!("  Intensity ≠ duration. The sculptor works FASTER as it learns.\n");

        // ═══ 4. STRETCH — expand short periods to see internal structure ═══
        println!("━━━ 4. STRETCH — periods normalized to equal width ━━━\n");
        println!("  {:30} {:>12} {:>8} {:>8} {:>8} {:>6} {:>6}",
            "Period", "Duration", "Phase", "Force", "Events", "°C", "Sea m");
        println!("  {:-<30} {:->12} {:->8} {:->8} {:->8} {:->6} {:->6}",
            "", "", "", "", "", "", "");
        for period in &p {
            let duration = period.start - period.end;
            let event_count = e.iter().filter(|ev| ev.period == period.id).count();
            let dur_str = if duration >= 1e6 { format!("{:.1} Ma", duration / 1e6) }
                          else if duration >= 1e3 { format!("{:.1} ka", duration / 1e3) }
                          else { format!("{:.0} ya", duration) };
            println!("  {:30} {:>12} {:>8} {:>8} {:>8} {:>6.1} {:>6.0}",
                period.name, dur_str, period.phase, period.force, event_count, period.temp, period.sea);
        }

        // ═══ 5. MIX — overlay temp + sea level + events ═══
        println!("\n━━━ 5. MIX — overlay temperature, sea level, and event density ━━━\n");
        println!("  temp °C   sea m    events/Ma   period");
        println!("  ────────  ──────   ─────────   ──────");
        for period in &p {
            let duration_ma = (period.start - period.end) / 1e6;
            let event_count = e.iter().filter(|ev| ev.period == period.id).count() as f64;
            let events_per_ma = if duration_ma > 0.0 { event_count / duration_ma } else { f64::INFINITY };
            let temp_bar: String = if period.temp >= 0.0 {
                format!("{:>4.1} {}", period.temp, "█".repeat((period.temp * 3.0) as usize))
            } else {
                format!("{:>5.1} {}", period.temp, "▄".repeat((-period.temp * 2.0) as usize))
            };
            let sea_bar: String = if period.sea >= 0.0 {
                format!("{:>5.0} {}", period.sea, "~".repeat((period.sea / 10.0).min(10.0) as usize))
            } else {
                format!("{:>5.0} {}", period.sea, "v".repeat((-period.sea / 15.0).min(10.0) as usize))
            };
            let epm = if events_per_ma.is_infinite() { "∞".to_string() }
                      else if events_per_ma > 100.0 { format!("{:.0}", events_per_ma) }
                      else { format!("{:.2}", events_per_ma) };
            println!("  {:16} {:16} {:>9}   {}",
                temp_bar, sea_bar, epm, period.name);
        }

        // ═══ 6. GRIND — break into granular units ═══
        println!("\n━━━ 6. GRIND — rate of change (derivatives) ━━━\n");
        println!("  transition              Δtemp    Δsea     Δtemp/ka   phase shift");
        println!("  ───────────             ──────   ──────   ────────   ───────────");
        for w in p.windows(2) {
            let dt = w[0].end - w[1].start; // gap (should be ~0 or overlap)
            let d_temp = w[1].temp - w[0].temp;
            let d_sea = w[1].sea - w[0].sea;
            let duration_ka = (w[0].start - w[0].end) / 1e3;
            let rate = if duration_ka > 0.0 { d_temp / duration_ka } else { 0.0 };
            let shift = if w[0].phase != w[1].phase {
                format!("{} → {}", w[0].phase, w[1].phase)
            } else {
                "—".to_string()
            };
            println!("  {:25} {:>+6.1}°C {:>+7.0}m {:>+9.4}  {}",
                format!("{} → {}", w[0].id, w[1].id),
                d_temp, d_sea, rate, shift);
        }

        // ═══ 7. AGGLUTINATE — cluster by force, phase, pattern ═══
        println!("\n━━━ 7. AGGLUTINATE — clusters ━━━\n");

        // by force
        println!("  BY SHAPING FORCE:");
        let mut force_groups: HashMap<&str, Vec<&Period>> = HashMap::new();
        for period in &p {
            force_groups.entry(period.force).or_default().push(period);
        }
        for (force, group) in &force_groups {
            let total_dur: f64 = group.iter().map(|p| p.start - p.end).sum();
            let avg_temp: f64 = group.iter().map(|p| p.temp).sum::<f64>() / group.len() as f64;
            let event_count: usize = group.iter()
                .map(|p| e.iter().filter(|ev| ev.period == p.id).count())
                .sum();
            let dur_str = if total_dur >= 1e6 { format!("{:.1} Ma", total_dur / 1e6) }
                          else if total_dur >= 1e3 { format!("{:.1} ka", total_dur / 1e3) }
                          else { format!("{:.0} ya", total_dur) };
            println!("    {:12} │ {} periods │ {} total │ avg {:.1}°C │ {} events",
                force, group.len(), dur_str, avg_temp, event_count);
        }

        // by phase
        println!("\n  BY CLIMATE PHASE:");
        let mut phase_groups: HashMap<&str, Vec<&Period>> = HashMap::new();
        for period in &p {
            phase_groups.entry(period.phase).or_default().push(period);
        }
        for (phase, group) in &phase_groups {
            let total_dur: f64 = group.iter().map(|p| p.start - p.end).sum();
            let avg_temp: f64 = group.iter().map(|p| p.temp).sum::<f64>() / group.len() as f64;
            let avg_sea: f64 = group.iter().map(|p| p.sea).sum::<f64>() / group.len() as f64;
            let dur_str = if total_dur >= 1e6 { format!("{:.1} Ma", total_dur / 1e6) }
                          else if total_dur >= 1e3 { format!("{:.1} ka", total_dur / 1e3) }
                          else { format!("{:.0} ya", total_dur) };
            println!("    {:14} │ {} periods │ {} │ avg {:.1}°C │ avg {:.0}m sea",
                phase, group.len(), dur_str, avg_temp, avg_sea);
        }

        // force sequence pattern
        println!("\n  FORCE SEQUENCE (temporal order):");
        let sorted: Vec<&Period> = {
            let mut v: Vec<&Period> = p.iter().collect();
            v.sort_by(|a, b| b.start.partial_cmp(&a.start).unwrap());
            v
        };
        let force_seq: Vec<&str> = sorted.iter().map(|p| p.force).collect();
        print!("    ");
        for (i, f) in force_seq.iter().enumerate() {
            if i > 0 { print!(" → "); }
            match *f {
                "tectonic" => print!("T"),
                "glacial" => print!("G"),
                "fluvial" => print!("F"),
                "biological" => print!("B"),
                "aeolian" => print!("A"),
                _ => print!("?"),
            }
        }
        println!();
        println!("    T=tectonic G=glacial F=fluvial B=biological\n");
        println!("    Pattern: T → B → T → G → G → G → F → G → G → G → B → G → B");
        println!("    Simplified: TECTONIC sets stage → GLACIAL dominates → BIOLOGICAL caps\n");

        // ═══ 8. WHAT CHANGES — acceleration, intensity, compression ═══
        println!("━━━ 8. WHAT CHANGES — the signal in the noise ━━━\n");

        // Event density accelerates
        println!("  EVENT ACCELERATION:");
        let windows: Vec<(&str, f64, f64, usize)> = vec![
            ("80Ma-2.6Ma (deep time)", 80e6, 2.6e6, e.iter().filter(|ev| {
                let per = p.iter().find(|p| p.id == ev.period).unwrap();
                per.start >= 2.6e6
            }).count()),
            ("2.6Ma-200ka (early ice)", 2.6e6, 200e3, e.iter().filter(|ev| {
                let per = p.iter().find(|p| p.id == ev.period).unwrap();
                per.start < 2.6e6 && per.start >= 200e3
            }).count()),
            ("200ka-12ka (main carving)", 200e3, 12e3, e.iter().filter(|ev| {
                let per = p.iter().find(|p| p.id == ev.period).unwrap();
                per.start < 200e3 && per.start >= 12e3
            }).count()),
            ("12ka-now (filling+bloom)", 12e3, 0.0, e.iter().filter(|ev| {
                let per = p.iter().find(|p| p.id == ev.period).unwrap();
                per.start < 12e3
            }).count()),
        ];
        for (label, start, end, count) in &windows {
            let span_ka = (start - end) / 1e3;
            let density = if span_ka > 0.0 { *count as f64 / span_ka * 1000.0 } else { f64::INFINITY };
            println!("    {:30} {:>2} events in {:>10.0} ka = {:.4} events/ka",
                label, count, span_ka, *count as f64 / span_ka);
        }

        println!("\n  ⚡ ACCELERATION FACTOR:");
        println!("    Deep time:    2 events  / 77,400 ka = 0.000026 events/ka");
        println!("    Main carving: 4 events  /    188 ka = 0.021    events/ka  (800× faster)");
        println!("    Fill+bloom:   3 events  /     12 ka = 0.250    events/ka  (9600× faster)");

        // Temperature oscillation amplitude
        println!("\n  TEMPERATURE OSCILLATION:");
        let max_temp = p.iter().map(|p| p.temp).fold(f64::MIN, f64::max);
        let min_temp = p.iter().map(|p| p.temp).fold(f64::MAX, f64::min);
        println!("    Range: {:.1}°C to {:.1}°C = {:.1}°C amplitude",
            min_temp, max_temp, max_temp - min_temp);
        println!("    Pinedale (coldest): {:.1}°C | Holocene optimum (warmest recent): {:.1}°C",
            p.iter().find(|p| p.id == "pinedale").unwrap().temp,
            p.iter().find(|p| p.id == "holocene_thermal").unwrap().temp);
        println!("    Swing from Pinedale to Holocene: {:.1}°C in {:.0} ka",
            1.0 - (-8.0), 30.0 - 5.0);

        // Period compression
        println!("\n  PERIOD COMPRESSION (durations getting shorter):");
        for period in &p {
            let dur = period.start - period.end;
            let dur_str = if dur >= 1e6 { format!("{:>8.1} Ma", dur / 1e6) }
                          else if dur >= 1e3 { format!("{:>8.1} ka", dur / 1e3) }
                          else { format!("{:>8.0} ya", dur) };
            let bar_len = (dur.log10() * 3.0) as usize;
            let bar = "█".repeat(bar_len.min(30));
            println!("    {:30} {} │{}│", period.name, dur_str, bar);
        }
        println!("\n  Periods COMPRESS as we approach now. Laramide = 45Ma. Younger Dryas = 1.2ka.");
        println!("  The resolution increases. Deep time is blurry. Recent time is sharp.");
        println!("  Same as: old fossils are vague, recent git commits are precise.\n");

        // Final synthesis
        println!("━━━ SYNTHESIS — what the grinding reveals ━━━\n");
        println!("  1. ACCELERATION: Events per unit time increases 10000× from deep time to now.");
        println!("     The sculptor works faster as material hardens. More happens per ka.");
        println!("  2. COMPRESSION: Periods shrink from 45Ma → 1.2ka → 150ya.");
        println!("     Time resolution increases approaching the observer.");
        println!("  3. FORCE SEQUENCE: T→G→F→B (tectonic→glacial→fluvial→biological).");
        println!("     Infrastructure → defense → data → ecosystem. Always in this order.");
        println!("  4. OSCILLATION: stadial↔interstadial breathing. 12°C amplitude.");
        println!("     The membrane breathes. Cold carves. Warm fills. Cold carves deeper.");
        println!("  5. ASYMMETRY: Cooling is slow (75ka → Pinedale), warming is fast (12ka → Holocene).");
        println!("     Stadials are long. Interstadials are short. Defense takes longer than bloom.");
        println!("  6. YOUNGER DRYAS: The cold snap. 1200 years of reversal AFTER deglaciation started.");
        println!("     The system can snap back. Progress is not monotonic. Watch for reversals.");
        println!("  7. ANTHROPOCENE: 150 years. Matching in 150yr what took 150ka.");
        println!("     We are in a phase transition. Not stadial, not interstadial. Transition.");
    }
}

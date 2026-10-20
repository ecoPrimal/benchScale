// SPDX-License-Identifier: AGPL-3.0-or-later
//! Stadial — Deep time orientation. Earth breathes on geological time.

use serde::{Deserialize, Serialize};

use super::pi_orient::{Landmark, LandmarkKind, landmarks};

// Stadial — Deep time orientation
//
// The Earth breathes on geological time. Stadials (cold, ice advance) and
// interstadials (warm, bloom) are the membrane's inhale/exhale at planetary scale.
//
// The Grand Tetons are the case study:
//   13 Ma: Teton Fault initiates. Block uplift begins.
//   2.6 Ma: Pleistocene. Ice ages begin. The sculptor arrives.
//   150 ka: Bull Lake glaciation carves the first U-valleys.
//   30-12 ka: Pinedale glaciation. Maximum ice. The Tetons as we know them.
//   12 ka: Deglaciation. Moraines dam Jenny Lake, Leigh Lake, etc.
//   Now: Holocene interstadial. The bloom on the ice's work.
//
// Pattern: tectonic sets the stage, ice carves the form, water fills the result.
// Same as: infrastructure sets the stage, defense carves the channels, data fills them.
// ═══════════════════════════════════════════════════════════════════════════

/// Whether a geological period is cold (stadial) or warm (interstadial).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClimatePhase {
    /// Cold period — ice advances, contraction, defense, carving.
    Stadial,
    /// Warm period — ice retreats, bloom, expansion, filling.
    Interstadial,
    /// Transitional — between states.
    Transition,
}

/// What force shaped the landscape during this period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapingForce {
    /// Tectonic — faulting, uplift, volcanism. Sets the stage.
    Tectonic,
    /// Glacial — ice carving, U-valleys, cirques, arêtes. Sculpts.
    Glacial,
    /// Fluvial — water erosion, deposition, lake filling. Fills.
    Fluvial,
    /// Aeolian — wind erosion/deposition. Scatters.
    Aeolian,
    /// Biological — soil formation, vegetation, coral. Blooms.
    Biological,
}

/// A geological period in deep time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPeriod {
    pub id: String,
    pub name: String,
    /// Start time in years before present (ya). Negative = future.
    pub start_ya: f64,
    /// End time in years before present (ya). 0 = now.
    pub end_ya: f64,
    pub phase: ClimatePhase,
    pub primary_force: ShapingForce,
    /// Global mean temperature offset from present (°C, negative = colder)
    pub temp_offset_c: f64,
    /// Sea level offset from present (m, negative = lower)
    pub sea_level_m: f64,
    pub note: String,
}

/// A formation event — something that happened to shape a specific place.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationEvent {
    pub period_id: String,
    pub landmark_id: String,
    pub force: ShapingForce,
    /// What specifically happened
    pub action: String,
    /// What resulted
    pub result: String,
}

/// The major geological periods relevant to landscape formation.
pub fn geo_periods() -> Vec<GeoPeriod> {
    let p = |id: &str, name: &str, start: f64, end: f64, phase: ClimatePhase,
             force: ShapingForce, temp: f64, sea: f64, note: &str| {
        GeoPeriod {
            id: id.into(), name: name.into(), start_ya: start, end_ya: end,
            phase, primary_force: force, temp_offset_c: temp, sea_level_m: sea,
            note: note.into(),
        }
    };
    use ClimatePhase::*;
    use ShapingForce::*;
    vec![
        // Deep time — tectonic stage-setting
        p("laramide", "Laramide Orogeny", 80_000_000.0, 35_000_000.0,
          Interstadial, Tectonic, 4.0, 100.0,
          "Rocky Mountain uplift. Ancestral Tetons begin. Cretaceous interior seaway drains."),
        p("teton_fault", "Teton Fault Initiation", 13_000_000.0, 2_600_000.0,
          Interstadial, Tectonic, 2.0, 25.0,
          "Normal fault. West block rises (Tetons), east block drops (Jackson Hole). No foothills — raw uplift from valley floor."),
        p("miocene_warm", "Miocene Climatic Optimum", 17_000_000.0, 13_000_000.0,
          Interstadial, Biological, 4.0, 40.0,
          "Warmest since Eocene. Grasslands spread. Grazing mammals diversify. Setting stage for the ice."),

        // Pleistocene — the sculptor arrives
        p("pleistocene_onset", "Pleistocene Onset", 2_600_000.0, 2_000_000.0,
          Transition, Glacial, -4.0, -60.0,
          "First major Northern Hemisphere glaciation. Teton uplift accelerates under ice load differential."),
        p("pre_illinoian", "Pre-Illinoian Glacials", 2_000_000.0, 300_000.0,
          Stadial, Glacial, -6.0, -100.0,
          "Multiple glacial advances. Early carving of Teton canyons. Evidence mostly overwritten by later ice."),
        p("bull_lake", "Bull Lake Glaciation", 200_000.0, 130_000.0,
          Stadial, Glacial, -6.0, -120.0,
          "Named for moraines at Bull Lake, WY. Carved the initial U-valleys: Cascade, Death, Paintbrush canyons. Ice >1km thick in Jackson Hole."),
        p("sangamon", "Sangamonian Interglacial", 130_000.0, 75_000.0,
          Interstadial, Fluvial, 1.0, 6.0,
          "Warmer than today. Sea level higher. Teton valleys fill with sediment. Rivers re-establish."),
        p("early_wisconsin", "Early Wisconsin Glacial", 75_000.0, 30_000.0,
          Stadial, Glacial, -5.0, -80.0,
          "Ice rebuilds. Cascade Canyon reoccupied. Cirques deepen at Cathedral Group."),
        p("pinedale", "Pinedale Glaciation", 30_000.0, 12_000.0,
          Stadial, Glacial, -8.0, -130.0,
          "Maximum ice. The Tetons as we know them carved here. 12 named glaciers. Moraines dam future lakes. Last Glacial Maximum at 21ka: ice 2km thick in valleys."),
        p("younger_dryas", "Younger Dryas", 12_900.0, 11_700.0,
          Stadial, Glacial, -6.0, -60.0,
          "Sudden return to near-glacial cold. 1300 years. Meltwater pulse stops. North Atlantic circulation disrupted. Teton glaciers briefly readvance."),
        p("holocene_thermal", "Holocene Climatic Optimum", 9_000.0, 5_000.0,
          Interstadial, Biological, 1.0, 0.0,
          "Warmest Holocene. Teton glaciers nearly gone. Forests climb. Jenny Lake, Leigh Lake, Phelps Lake reach current form."),
        p("neoglacial", "Neoglaciation", 5_000.0, 150.0,
          Stadial, Glacial, -0.5, -1.0,
          "Cool periods. Teton glaciers re-form. Little Ice Age 1300-1850 CE. Teton Glacier, Schoolroom Glacier at maximum extent ~1850."),
        p("anthropocene", "Anthropocene", 150.0, 0.0,
          Transition, Biological, 1.1, 0.2,
          "Teton Glacier retreating. Schoolroom Glacier may disappear by 2030. The bloom on 13 million years of work."),
    ]
}

/// Formation events — what shaped the Grand Tetons specifically.
pub fn teton_formation() -> Vec<FormationEvent> {
    let e = |period: &str, landmark: &str, force: ShapingForce, action: &str, result: &str| {
        FormationEvent {
            period_id: period.into(), landmark_id: landmark.into(),
            force, action: action.into(), result: result.into(),
        }
    };
    use ShapingForce::*;
    vec![
        // Tectonic — sets the stage
        e("laramide", "grand_tetons", Tectonic,
          "Compressional orogeny lifts Precambrian basement rock",
          "Ancestral Rocky Mountain block. Gneiss and granite 2.7 billion years old exposed."),
        e("teton_fault", "grand_tetons", Tectonic,
          "Normal fault drops east block 6km. West block rises. No foothills.",
          "Teton Range: 64km long, 21km wide. Grand Teton summit 4,199m. Jackson Hole floor 1,900m. 2,300m of vertical relief."),
        // Glacial — sculpts
        e("bull_lake", "grand_tetons", Glacial,
          "Valley glaciers fill Cascade, Death, Paintbrush canyons. Ice >1km thick.",
          "Initial U-shaped valleys. Teton peaks become nunataks above the ice."),
        e("pinedale", "grand_tetons", Glacial,
          "Maximum ice extent. 12 glaciers carve simultaneously. Cirques deepen. Arêtes sharpen.",
          "Cathedral Group sculpted: Grand Teton, Mt Owen, Teewinot. Terminal moraines dam future lakes. Jenny Lake moraine 100m high."),
        e("pinedale", "grand_tetons", Glacial,
          "Glacial plucking on north/east faces. Freeze-thaw shatters ridgelines.",
          "The Grand's north face: 600m vertical granite. The Cathedral Traverse. The Exum Ridge."),
        // Fluvial — fills
        e("sangamon", "grand_tetons", Fluvial,
          "Meltwater fills moraine-dammed basins. Rivers cut through outwash.",
          "Proto-Jenny Lake. Snake River establishes course through Jackson Hole."),
        e("holocene_thermal", "grand_tetons", Fluvial,
          "Final deglaciation fills lakes to current levels. Streams cascade from hanging valleys.",
          "Jenny Lake (surface 480 ha, depth 73m), Leigh Lake, Bradley Lake, Taggart Lake, Phelps Lake. 7 moraine-dammed lakes."),
        // Biological — blooms
        e("holocene_thermal", "grand_tetons", Biological,
          "Forests climb to 3000m. Meadows fill valley floors. Whitebark pine colonizes.",
          "The ecosystem as seen today. Grizzly, elk, moose, pika, trumpeter swan."),
        e("anthropocene", "grand_tetons", Biological,
          "Teton Glacier retreating 3m/yr. Whitebark pine dying from blister rust + beetles.",
          "The bloom is shifting. What ice built in 150,000 years changes in 150."),
    ]
}

/// Add the Grand Tetons as a landmark.
pub fn landmarks_with_tetons() -> Vec<Landmark> {
    let mut lm = landmarks();
    lm.push(Landmark {
        id: "grand_tetons".into(),
        name: "Grand Teton".into(),
        lat: 43.7413,
        lon: -110.8024,
        kind: LandmarkKind::Stone,
        era: Some(-13_000_000),
        note: "Teton Fault. No foothills. 2.7Ga gneiss lifted 2300m above Jackson Hole. Ice carved, water filled.".into(),
    });
    lm.push(Landmark {
        id: "jackson_hole".into(),
        name: "Jackson Hole".into(),
        lat: 43.4799,
        lon: -110.7624,
        kind: LandmarkKind::Water,
        era: Some(-13_000_000),
        note: "The down-thrown block. What drops makes the valley. Snake River runs through.".into(),
    });
    lm
}

/// Trace the formation of a landmark through geological time.
/// Returns periods + events sorted oldest to youngest.
pub fn trace_formation(landmark_id: &str) -> Vec<(GeoPeriod, Vec<FormationEvent>)> {
    let periods = geo_periods();
    let events = teton_formation();
    let mut result = Vec::new();
    for period in &periods {
        let matching: Vec<FormationEvent> = events.iter()
            .filter(|e| e.period_id == period.id && e.landmark_id == landmark_id)
            .cloned()
            .collect();
        if !matching.is_empty() {
            result.push((period.clone(), matching));
        }
    }
    result.sort_by(|a, b| b.0.start_ya.partial_cmp(&a.0.start_ya).unwrap());
    result
}

/// Phase map — returns (stadial_count, interstadial_count, transition_count)
/// for all geological periods.
pub fn phase_counts() -> (usize, usize, usize) {
    let periods = geo_periods();
    let s = periods.iter().filter(|p| p.phase == ClimatePhase::Stadial).count();
    let i = periods.iter().filter(|p| p.phase == ClimatePhase::Interstadial).count();
    let t = periods.iter().filter(|p| p.phase == ClimatePhase::Transition).count();
    (s, i, t)
}

/// Time depth — how much of Earth's history a period spans, as a fraction.
/// Earth age = 4.54 billion years.
pub fn time_depth(period: &GeoPeriod) -> f64 {
    let earth_age: f64 = 4_540_000_000.0;
    (period.start_ya - period.end_ya) / earth_age
}

/// The stadial/interstadial rhythm: returns periods as a sequence of
/// (phase, duration_ya) pairs, oldest first.
pub fn climate_rhythm() -> Vec<(ClimatePhase, f64)> {
    let mut periods = geo_periods();
    periods.sort_by(|a, b| b.start_ya.partial_cmp(&a.start_ya).unwrap());
    periods.iter().map(|p| (p.phase, p.start_ya - p.end_ya)).collect()
}

#[cfg(test)]
mod stadial_tests {
    use crate::io_surface::*;

    #[test]
    fn test_geo_periods_chronological() {
        let periods = geo_periods();
        assert!(periods.len() >= 13, "should have at least 13 periods");
        // all start_ya > end_ya
        for p in &periods {
            assert!(p.start_ya > p.end_ya,
                "{} start {} should be > end {}", p.id, p.start_ya, p.end_ya);
        }
    }

    #[test]
    fn test_pinedale_is_stadial() {
        let periods = geo_periods();
        let pine = periods.iter().find(|p| p.id == "pinedale").unwrap();
        assert_eq!(pine.phase, ClimatePhase::Stadial);
        assert!(pine.temp_offset_c < -5.0, "Pinedale should be very cold");
        assert!(pine.sea_level_m < -100.0, "sea level should be >100m lower");
    }

    #[test]
    fn test_holocene_is_interstadial() {
        let periods = geo_periods();
        let hol = periods.iter().find(|p| p.id == "holocene_thermal").unwrap();
        assert_eq!(hol.phase, ClimatePhase::Interstadial);
        assert!(hol.temp_offset_c > 0.0);
    }

    #[test]
    fn test_younger_dryas_cold_snap() {
        let periods = geo_periods();
        let yd = periods.iter().find(|p| p.id == "younger_dryas").unwrap();
        assert_eq!(yd.phase, ClimatePhase::Stadial);
        let duration = yd.start_ya - yd.end_ya;
        assert!((duration - 1200.0).abs() < 10.0, "YD should be ~1200 years");
    }

    #[test]
    fn test_teton_formation_trace() {
        let trace = trace_formation("grand_tetons");
        assert!(trace.len() >= 5, "Grand Tetons should have at least 5 formation periods");
        // oldest event should be Laramide
        assert_eq!(trace[0].0.id, "laramide");
        // newest should be anthropocene
        let last = &trace[trace.len() - 1];
        assert_eq!(last.0.id, "anthropocene");
    }

    #[test]
    fn test_teton_formation_pattern() {
        // The pattern: tectonic → glacial → fluvial → biological
        let trace = trace_formation("grand_tetons");
        let forces: Vec<ShapingForce> = trace.iter()
            .flat_map(|(_, events)| events.iter().map(|e| e.force))
            .collect();
        // first force should be tectonic
        assert_eq!(forces[0], ShapingForce::Tectonic);
        // should contain all four main forces
        assert!(forces.contains(&ShapingForce::Tectonic));
        assert!(forces.contains(&ShapingForce::Glacial));
        assert!(forces.contains(&ShapingForce::Fluvial));
        assert!(forces.contains(&ShapingForce::Biological));
    }

    #[test]
    fn test_phase_counts() {
        let (s, i, t) = phase_counts();
        assert!(s >= 5, "should have at least 5 stadial periods, got {}", s);
        assert!(i >= 4, "should have at least 4 interstadial periods, got {}", i);
        assert!(t >= 2, "should have at least 2 transition periods, got {}", t);
    }

    #[test]
    fn test_teton_fault_vertical_relief() {
        // Grand Teton summit 4199m, Jackson Hole floor ~1900m
        // Teton Fault drop = ~6km total displacement over 13Ma
        let trace = trace_formation("grand_tetons");
        let fault = trace.iter().find(|(p, _)| p.id == "teton_fault").unwrap();
        assert!(fault.1[0].result.contains("2,300m"), "should mention 2300m relief");
    }

    #[test]
    fn test_grand_teton_landmark() {
        let lm = landmarks_with_tetons();
        let gt = lm.iter().find(|l| l.id == "grand_tetons").unwrap();
        assert_eq!(gt.kind, LandmarkKind::Stone);
        assert!(gt.lat > 43.0 && gt.lat < 44.0);
        let jh = lm.iter().find(|l| l.id == "jackson_hole").unwrap();
        assert_eq!(jh.kind, LandmarkKind::Water);
    }

    #[test]
    fn test_climate_rhythm_alternation() {
        let rhythm = climate_rhythm();
        // should have stadial and interstadial alternating (roughly)
        let phases: Vec<ClimatePhase> = rhythm.iter().map(|(p, _)| *p).collect();
        let has_s = phases.contains(&ClimatePhase::Stadial);
        let has_i = phases.contains(&ClimatePhase::Interstadial);
        assert!(has_s && has_i, "rhythm should contain both stadial and interstadial");
    }

    #[test]
    fn test_time_depth_pinedale() {
        let periods = geo_periods();
        let pine = periods.iter().find(|p| p.id == "pinedale").unwrap();
        let depth = time_depth(pine);
        // 18000 years / 4.54 billion = ~4e-6
        assert!(depth < 0.00001, "Pinedale should be a tiny fraction of Earth's age, got {}", depth);
        assert!(depth > 0.0);
    }

    #[test]
    fn test_jenny_lake_from_pinedale() {
        let trace = trace_formation("grand_tetons");
        // find holocene events
        let holocene = trace.iter().find(|(p, _)| p.id == "holocene_thermal").unwrap();
        let has_jenny = holocene.1.iter().any(|e| e.result.contains("Jenny Lake"));
        assert!(has_jenny, "holocene should mention Jenny Lake formation");
    }

    #[test]
    fn test_detroit_to_tetons_photon_line() {
        let lm = landmarks_with_tetons();
        let det = lm.iter().find(|l| l.id == "detroit").unwrap();
        let gt = lm.iter().find(|l| l.id == "grand_tetons").unwrap();
        let line = photon_line(det, gt);
        // Detroit to Grand Teton ~2250 km (great circle)
        assert!(line.arc_km > 2000.0 && line.arc_km < 2600.0,
            "Detroit→Grand Teton should be ~2250km, got {:.0}km", line.arc_km);
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later
//! CNS — Central Nervous System surface classification.

#[derive(Debug, Clone, PartialEq)]
pub enum Gear {
    /// 1st gear — heavy torque, slow speed. Investigation.
    Investigation,
    /// 2nd gear — torque-to-speed. Science/research.
    Science,
    /// 3rd gear — speed. Real-time observation.
    Observation,
    /// 4th gear — cruise. Hidden infrastructure.
    Infrastructure,
    /// Reverse — defense. Honeycomb maze.
    Immune,
}

/// CNS type — nervous system classification for routing behavior.
#[derive(Debug, Clone, PartialEq)]
pub enum CnsType {
    /// Executive function — integrates everything, decides action.
    Prefrontal,
    /// Thinking — processes information, generates connections.
    Cortex,
    /// Observation — real-time, no memory trap.
    Sensory,
    /// Action — evidence pages, filing guides.
    Motor,
    /// Breathing — runs without consciousness.
    Autonomic,
    /// Defense — absorbs threats, routes into maze.
    Immune,
}

// ═══════════════════════════════════════════════════════════════
// BUS GRADIENT — VITAL/SENSORY/CORTICAL/MOTOR dispatch tiers
// Maps CnsType → membrane-bus.py tier for pressure-routed dispatch.
// ═══════════════════════════════════════════════════════════════

/// Bus tier — pressure-routed dispatch level (from membrane-bus.py).
///
/// The bus routes scripts based on gradient pressure:
/// - **Vital**: always flows, no gradient check (cardiovascular, wake-cycle)
/// - **Sensory**: flows when staleness > 30s (campfire, pollen, HUD feeds)
/// - **Cortical**: flows when staleness > 120s (scan-sweep-sort, taxonomy, braids)
/// - **Motor**: flows when staleness > 300s AND aperture open (deploy, pull, build)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BusTier {
    /// Always flows. Keeps the organism alive.
    Vital,
    /// Flows when gradient > 30s. Observation, real-time feeds.
    Sensory,
    /// Flows when gradient > 120s. Deep processing, braids, classification.
    Cortical,
    /// Flows when gradient > 300s + aperture open. Deploy, build, git pull.
    Motor,
}

impl BusTier {
    /// Gradient threshold in seconds — minimum staleness before this tier fires.
    pub const fn threshold_secs(&self) -> u64 {
        match self {
            Self::Vital => 0,
            Self::Sensory => 30,
            Self::Cortical => 120,
            Self::Motor => 300,
        }
    }

    /// Max concurrent scripts per tier at full aperture.
    pub const fn max_concurrent(&self) -> u8 {
        match self {
            Self::Vital => 99,
            Self::Sensory => 12,
            Self::Cortical => 6,
            Self::Motor => 2,
        }
    }

    /// Max concurrent at a given aperture (0.0 = shut, 1.0 = wide open).
    pub fn concurrent_at_aperture(&self, aperture: f64) -> u8 {
        let base = self.max_concurrent();
        if *self == Self::Vital {
            return base;
        }
        let scaled = if aperture < 0.3 {
            base / 3
        } else if aperture < 0.6 {
            base / 2
        } else {
            base
        };
        scaled.max(1)
    }

    /// Tier label as used by membrane-bus.py.
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Vital => "vital",
            Self::Sensory => "sensory",
            Self::Cortical => "cortical",
            Self::Motor => "motor",
        }
    }
}

/// Map CNS type to bus gradient tier.
///
/// The mapping follows northGate's architecture:
/// - Autonomic + Immune → Vital (always flows — keepalive, defense)
/// - Sensory → Sensory (real-time observation)
/// - Cortex → Cortical (science, deep processing)
/// - Motor + Prefrontal → Motor (action — deploy, pull, build)
pub fn cns_to_bus_tier(cns: &CnsType) -> BusTier {
    match cns {
        CnsType::Autonomic | CnsType::Immune => BusTier::Vital,
        CnsType::Sensory => BusTier::Sensory,
        CnsType::Cortex => BusTier::Cortical,
        CnsType::Prefrontal | CnsType::Motor => BusTier::Motor,
    }
}

/// Compute aperture from CPU load. Non-Newtonian: thick under shear, never a brick wall.
/// Sigmoid around load=1.2, floor 0.12 so cortical/motor can still drip.
pub fn aperture(load_normalized: f64) -> f64 {
    (1.0 / (1.0 + (2.2 * (load_normalized - 1.2)).exp())).max(0.12).min(1.0)
}

/// Compute gradient pressure: output staleness minus freshest input age.
/// Positive = should run. Negative = outputs are fresh, skip.
pub fn gradient_pressure(input_ages_secs: &[f64], output_ages_secs: &[f64]) -> f64 {
    if output_ages_secs.is_empty() {
        return 9999.0; // no outputs = motor tier, always due
    }
    let freshest_in = input_ages_secs
        .iter()
        .copied()
        .reduce(f64::min)
        .unwrap_or(0.0);
    let stalest_out = output_ages_secs
        .iter()
        .copied()
        .reduce(f64::max)
        .unwrap_or(0.0);
    stalest_out - freshest_in
}

/// Check if a tier should fire given its gradient pressure and aperture.
pub fn should_fire(tier: &BusTier, gradient: f64, aperture_val: f64) -> bool {
    match tier {
        BusTier::Vital => true, // vital always flows
        BusTier::Motor => gradient >= tier.threshold_secs() as f64 && aperture_val >= 0.10,
        _ => gradient >= tier.threshold_secs() as f64,
    }
}

/// Get all surfaces in a given bus tier.
pub fn surfaces_by_bus_tier(tier: &BusTier) -> Vec<Surface> {
    surfaces()
        .into_iter()
        .filter(|s| cns_to_bus_tier(&s.cns) == *tier)
        .collect()
}

/// A surface in the primals.eco network.
#[derive(Debug, Clone)]
pub struct Surface {
    pub id: String,
    pub domain: String,
    pub gear: Gear,
    pub cns: CnsType,
    pub depth: u8,
    pub pages: u32,
    pub topo_tier: u8,
}

/// Feed relationship between surfaces.
#[derive(Debug, Clone)]
pub struct SurfaceFeed {
    pub from: String,
    pub to: String,
}

/// Gear shift — how to transition between gears.
#[derive(Debug, Clone)]
pub struct GearShift {
    pub from_gear: Gear,
    pub to_gear: Gear,
    pub action: String,
    pub path: String,
}

/// All surfaces in the primals.eco network.
pub fn surfaces() -> Vec<Surface> {
    vec![
        // tier 0 — autonomic (body)
        Surface { id: "ca".into(),        domain: "ca.primals.eco".into(),          gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 0 },
        Surface { id: "nestgate".into(),   domain: "nestgate.io".into(),             gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 0 },
        Surface { id: "depot".into(),      domain: "depot.primals.eco".into(),       gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 0 },
        // tier 1 — nervous (spinal cord)
        Surface { id: "membrane".into(),   domain: "membrane.primals.eco".into(),    gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 1 },
        Surface { id: "neutron".into(),    domain: "neutron.primals.eco".into(),     gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 1 },
        Surface { id: "mesh".into(),       domain: "mesh.primals.eco".into(),        gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 1 },
        Surface { id: "notify".into(),     domain: "notify.primals.eco".into(),      gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 1 },
        // tier 3 — sensory (observation)
        Surface { id: "hud".into(),        domain: "hud.primals.eco".into(),         gear: Gear::Observation,    cns: CnsType::Sensory,   depth: 2, pages: 20,   topo_tier: 3 },
        Surface { id: "water".into(),      domain: "water.primals.eco".into(),       gear: Gear::Observation,    cns: CnsType::Sensory,   depth: 1, pages: 1,    topo_tier: 3 },
        Surface { id: "signal".into(),     domain: "signal.primals.eco".into(),      gear: Gear::Observation,    cns: CnsType::Sensory,   depth: 1, pages: 3,    topo_tier: 3 },
        Surface { id: "gorilla".into(),    domain: "gorilla.primals.eco".into(),     gear: Gear::Observation,    cns: CnsType::Sensory,   depth: 2, pages: 22,   topo_tier: 3 },
        Surface { id: "live".into(),       domain: "live.primals.eco".into(),        gear: Gear::Observation,    cns: CnsType::Sensory,   depth: 0, pages: 0,    topo_tier: 3 },
        // tier 4 — motor (action)
        Surface { id: "detroit".into(),    domain: "detroit.primals.eco".into(),     gear: Gear::Investigation,  cns: CnsType::Motor,     depth: 4, pages: 424,  topo_tier: 4 },
        Surface { id: "barry".into(),      domain: "barry.primals.eco".into(),       gear: Gear::Investigation,  cns: CnsType::Motor,     depth: 2, pages: 35,   topo_tier: 4 },
        Surface { id: "lansing".into(),    domain: "lansing.primals.eco".into(),     gear: Gear::Investigation,  cns: CnsType::Motor,     depth: 0, pages: 0,    topo_tier: 4 },
        // tier 5 — cortex (science)
        Surface { id: "sporeprint".into(), domain: "sporeprint.primals.eco".into(),  gear: Gear::Science,        cns: CnsType::Cortex,    depth: 4, pages: 510,  topo_tier: 5 },
        Surface { id: "thesis".into(),     domain: "thesis.primals.eco".into(),      gear: Gear::Science,        cns: CnsType::Cortex,    depth: 1, pages: 2,    topo_tier: 5 },
        Surface { id: "hypothesis".into(), domain: "hypothesis.primals.eco".into(),  gear: Gear::Science,        cns: CnsType::Cortex,    depth: 0, pages: 0,    topo_tier: 5 },
        // tier 6 — prefrontal (executive)
        Surface { id: "clutch".into(),     domain: "clutch.primals.eco".into(),      gear: Gear::Investigation,  cns: CnsType::Prefrontal,depth: 1, pages: 10,   topo_tier: 6 },
        Surface { id: "tuebor".into(),     domain: "tuebor.primals.eco".into(),      gear: Gear::Investigation,  cns: CnsType::Prefrontal,depth: 5, pages: 1219, topo_tier: 6 },
        // tier 7 — source (reproduction)
        Surface { id: "git".into(),        domain: "git.primals.eco".into(),         gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 7 },
        Surface { id: "pollen".into(),     domain: "pollination.primals.eco".into(), gear: Gear::Infrastructure, cns: CnsType::Autonomic, depth: 0, pages: 0,    topo_tier: 7 },
    ]
}

/// Feed relationships — what flows where.
pub fn surface_feeds() -> Vec<SurfaceFeed> {
    let pairs = vec![
        ("detroit","clutch"), ("detroit","barry"), ("detroit","tuebor"), ("detroit","signal"),
        ("barry","clutch"), ("barry","detroit"),
        ("clutch","detroit"), ("clutch","barry"), ("clutch","tuebor"), ("clutch","sporeprint"), ("clutch","signal"),
        ("tuebor","detroit"), ("tuebor","barry"), ("tuebor","clutch"), ("tuebor","signal"), ("tuebor","sporeprint"),
        ("sporeprint","thesis"), ("sporeprint","hypothesis"), ("sporeprint","clutch"), ("sporeprint","detroit"), ("sporeprint","tuebor"),
        ("thesis","sporeprint"), ("thesis","hypothesis"),
        ("hypothesis","sporeprint"), ("hypothesis","thesis"),
        ("hud","water"), ("hud","signal"), ("hud","gorilla"), ("hud","neutron"),
        ("water","hud"), ("water","neutron"),
        ("signal","gorilla"), ("signal","clutch"), ("signal","detroit"),
        ("gorilla","signal"), ("gorilla","hud"), ("gorilla","clutch"),
        ("live","hud"), ("live","signal"),
        ("membrane","hud"), ("membrane","water"), ("membrane","signal"), ("membrane","live"),
        ("neutron","hud"), ("neutron","water"), ("neutron","membrane"),
    ];
    pairs.into_iter().map(|(f,t)| SurfaceFeed { from: f.into(), to: t.into() }).collect()
}

/// Available gear shifts.
pub fn gear_shifts() -> Vec<GearShift> {
    vec![
        GearShift { from_gear: Gear::Investigation, to_gear: Gear::Science,       action: "press".into(),     path: "clutch.contribute → sporeprint.methodology".into() },
        GearShift { from_gear: Gear::Science,        to_gear: Gear::Investigation, action: "release".into(),   path: "sporeprint.methodology → clutch.graph".into() },
        GearShift { from_gear: Gear::Observation,    to_gear: Gear::Investigation, action: "downshift".into(), path: "signal → clutch.graph".into() },
        GearShift { from_gear: Gear::Investigation,  to_gear: Gear::Observation,   action: "upshift".into(),   path: "tuebor → signal OR hud".into() },
    ]
}

/// Topo-sorted surface list (dependency order).
pub fn topo_sorted() -> Vec<Surface> {
    let mut all = surfaces();
    all.sort_by_key(|s| s.topo_tier);
    all
}

/// Find a surface by id.
pub fn surface_by_id(id: &str) -> Option<Surface> {
    surfaces().into_iter().find(|s| s.id == id)
}

/// Find surfaces by gear.
pub fn surfaces_by_gear(gear: &Gear) -> Vec<Surface> {
    surfaces().into_iter().filter(|s| &s.gear == gear).collect()
}

/// Find surfaces by CNS type.
pub fn surfaces_by_cns(cns: &CnsType) -> Vec<Surface> {
    surfaces().into_iter().filter(|s| &s.cns == cns).collect()
}

/// Check if navigation from one surface to another requires a gear shift.
pub fn gear_shift_needed(from_id: &str, to_id: &str) -> Option<GearShift> {
    let from = surface_by_id(from_id)?;
    let to = surface_by_id(to_id)?;
    if from.gear == to.gear { return None; }
    gear_shifts().into_iter().find(|s| s.from_gear == from.gear && s.to_gear == to.gear)
}

/// Depth guard — returns true if the target depth exceeds max_safe_depth.
pub fn is_black_hole(surface_id: &str, max_safe_depth: u8) -> bool {
    surface_by_id(surface_id).map(|s| s.depth > max_safe_depth).unwrap_or(false)
}

/// Surfaces that feed into the given surface (upstream).
pub fn upstream(surface_id: &str) -> Vec<String> {
    surface_feeds().into_iter()
        .filter(|f| f.to == surface_id)
        .map(|f| f.from)
        .collect()
}

/// Surfaces that the given surface feeds into (downstream).
pub fn downstream(surface_id: &str) -> Vec<String> {
    surface_feeds().into_iter()
        .filter(|f| f.from == surface_id)
        .map(|f| f.to)
        .collect()
}

#[cfg(test)]
mod cns_tests {
    use crate::io_surface::*;

    #[test]
    fn test_surface_count() {
        let all = surfaces();
        assert!(all.len() >= 22, "should have at least 22 surfaces, got {}", all.len());
    }

    #[test]
    fn test_topo_sort_order() {
        let sorted = topo_sorted();
        for w in sorted.windows(2) {
            assert!(w[0].topo_tier <= w[1].topo_tier,
                "{} (tier {}) should come before {} (tier {})",
                w[0].id, w[0].topo_tier, w[1].id, w[1].topo_tier);
        }
    }

    #[test]
    fn test_clutch_is_prefrontal() {
        let c = surface_by_id("clutch").unwrap();
        assert_eq!(c.cns, CnsType::Prefrontal);
        assert_eq!(c.gear, Gear::Investigation);
        assert_eq!(c.topo_tier, 6);
    }

    #[test]
    fn test_detroit_is_motor() {
        let d = surface_by_id("detroit").unwrap();
        assert_eq!(d.cns, CnsType::Motor);
        assert_eq!(d.pages, 424);
    }

    #[test]
    fn test_tuebor_is_black_hole() {
        assert!(is_black_hole("tuebor", 2));
        assert!(!is_black_hole("clutch", 2));
    }

    #[test]
    fn test_gear_shift_investigation_to_science() {
        let shift = gear_shift_needed("detroit", "sporeprint").unwrap();
        assert_eq!(shift.action, "press");
    }

    #[test]
    fn test_no_shift_within_gear() {
        assert!(gear_shift_needed("detroit", "barry").is_none());
    }

    #[test]
    fn test_feeds_bidirectional() {
        let feeds = surface_feeds();
        let det_to_clutch = feeds.iter().any(|f| f.from == "detroit" && f.to == "clutch");
        let clutch_to_det = feeds.iter().any(|f| f.from == "clutch" && f.to == "detroit");
        assert!(det_to_clutch && clutch_to_det, "detroit ↔ clutch should be bidirectional");
    }

    #[test]
    fn test_upstream_downstream() {
        let up = upstream("clutch");
        assert!(up.contains(&"detroit".to_string()));
        assert!(up.contains(&"tuebor".to_string()));
        let down = downstream("clutch");
        assert!(down.contains(&"detroit".to_string()));
        assert!(down.contains(&"sporeprint".to_string()));
    }

    #[test]
    fn test_surfaces_by_gear() {
        let obs = surfaces_by_gear(&Gear::Observation);
        assert_eq!(obs.len(), 5);
        assert!(obs.iter().all(|s| s.cns == CnsType::Sensory));
    }

    #[test]
    fn test_autonomic_has_no_pages() {
        let auto = surfaces_by_cns(&CnsType::Autonomic);
        assert!(auto.iter().all(|s| s.pages == 0), "autonomic surfaces should have 0 pages");
    }

    #[test]
    fn test_all_cns_types_present() {
        let all = surfaces();
        let types: Vec<&CnsType> = all.iter().map(|s| &s.cns).collect();
        assert!(types.contains(&&CnsType::Prefrontal));
        assert!(types.contains(&&CnsType::Cortex));
        assert!(types.contains(&&CnsType::Sensory));
        assert!(types.contains(&&CnsType::Motor));
        assert!(types.contains(&&CnsType::Autonomic));
    }

    #[test]
    fn test_total_pages() {
        let total: u32 = surfaces().iter().map(|s| s.pages).sum();
        assert!(total > 2000, "total pages should be > 2000, got {}", total);
    }

    // ── Bus Gradient Tests ──

    #[test]
    fn test_bus_tier_thresholds() {
        assert_eq!(BusTier::Vital.threshold_secs(), 0);
        assert_eq!(BusTier::Sensory.threshold_secs(), 30);
        assert_eq!(BusTier::Cortical.threshold_secs(), 120);
        assert_eq!(BusTier::Motor.threshold_secs(), 300);
    }

    #[test]
    fn test_cns_to_bus_mapping() {
        assert_eq!(cns_to_bus_tier(&CnsType::Autonomic), BusTier::Vital);
        assert_eq!(cns_to_bus_tier(&CnsType::Immune), BusTier::Vital);
        assert_eq!(cns_to_bus_tier(&CnsType::Sensory), BusTier::Sensory);
        assert_eq!(cns_to_bus_tier(&CnsType::Cortex), BusTier::Cortical);
        assert_eq!(cns_to_bus_tier(&CnsType::Motor), BusTier::Motor);
        assert_eq!(cns_to_bus_tier(&CnsType::Prefrontal), BusTier::Motor);
    }

    #[test]
    fn test_aperture_sigmoid() {
        // Low load → wide open
        assert!(aperture(0.2) > 0.9);
        // High load → nearly shut but not zero
        assert!(aperture(3.0) >= 0.12);
        // At load 1.2 → sigmoid midpoint ≈ 0.5
        let mid = aperture(1.2);
        assert!(mid > 0.4 && mid < 0.6, "aperture at load 1.2 should be ~0.5, got {mid}");
    }

    #[test]
    fn test_gradient_pressure() {
        // Outputs staler than inputs → positive gradient → should run
        assert!(gradient_pressure(&[10.0], &[120.0]) > 0.0);
        // Outputs fresh → negative gradient → skip
        assert!(gradient_pressure(&[120.0], &[10.0]) < 0.0);
        // No outputs → motor tier → always due
        assert_eq!(gradient_pressure(&[10.0], &[]), 9999.0);
    }

    #[test]
    fn test_should_fire() {
        // Vital always fires
        assert!(should_fire(&BusTier::Vital, 0.0, 0.0));
        // Sensory needs gradient > 30
        assert!(!should_fire(&BusTier::Sensory, 20.0, 1.0));
        assert!(should_fire(&BusTier::Sensory, 50.0, 1.0));
        // Motor needs gradient > 300 AND aperture ≥ 0.10
        assert!(!should_fire(&BusTier::Motor, 400.0, 0.05));
        assert!(should_fire(&BusTier::Motor, 400.0, 0.5));
    }

    #[test]
    fn test_surfaces_by_bus_tier() {
        let vital = surfaces_by_bus_tier(&BusTier::Vital);
        assert!(vital.iter().all(|s| matches!(s.cns, CnsType::Autonomic | CnsType::Immune)));

        let sensory = surfaces_by_bus_tier(&BusTier::Sensory);
        assert_eq!(sensory.len(), 5); // hud, water, signal, gorilla, live

        let motor = surfaces_by_bus_tier(&BusTier::Motor);
        assert!(motor.iter().any(|s| s.id == "detroit"));
        assert!(motor.iter().any(|s| s.id == "tuebor"));
    }

    #[test]
    fn test_concurrent_at_aperture() {
        // Vital always 99
        assert_eq!(BusTier::Vital.concurrent_at_aperture(0.1), 99);
        // Sensory at narrow aperture
        assert_eq!(BusTier::Sensory.concurrent_at_aperture(0.2), 4); // 12/3
        assert_eq!(BusTier::Sensory.concurrent_at_aperture(0.5), 6); // 12/2
        assert_eq!(BusTier::Sensory.concurrent_at_aperture(0.8), 12); // full
    }
}

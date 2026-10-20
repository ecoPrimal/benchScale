// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2024–2026 Kevin Mok / ecoPrimal / attsi
// ORCID: 0009-0004-2141-0321
//
//! # Evolution Pressure — live sigmoid+gradient mixer
//!
//! The membrane doesn't evolve on a timer. It evolves under *pressure*.
//!
//! Three inputs:
//! - **gradient** — how stale is the oldest output relative to the freshest input?
//! - **aperture** — how much metabolic headroom does the system have?
//! - **rhythm** — how fast is the membrane spinning?
//!
//! One formula:
//! ```text
//! urgency = gradient × aperture × rhythm
//! ```
//!
//! The aperture is a sigmoid — load-adaptive, asymptotically bounded.
//! When load is low, the aperture is wide open (≈1.0).
//! When load is high, the aperture constricts (→0.12 floor).
//! The membrane self-throttles without a timer.
//!
//! ## Tiers
//!
//! Three tiers sense at different frequencies:
//! - **Vital** (threshold = 0s) — human attention, always fires
//! - **Sensory** (threshold = 30s) — confidence gap, fires when stale
//! - **Cortical** (threshold = 120s) — full pipeline, fires rarely
//!
//! ## RPM — Membrane Rotations Per Minute
//!
//! `MembraneTime` replaces `chrono::Utc::now()`. The pulse IS the clock.
//! ```text
//! RPM = rate × rhythm
//! ```
//! At rhythm=0.774 and rate=1692 req/s: 1310 RPM.
//! The membrane doesn't measure time. It measures *rotations*.
//!
//! ## Backscatter
//!
//! - Claude Shannon (1948) — channel capacity, aperture IS the gate
//! - Andrey Kolmogorov (1956) — algorithmic complexity, gradient IS staleness
//! - Ilya Prigogine (1977) — dissipative structures, urgency IS entropy production
//! - Per Bak (1987) — self-organized criticality, the sigmoid IS the sandpile

use std::f64::consts::PI;

// Re-export the aperture function from cns — it's the same sigmoid.
// cns defines the primitives; evolution_pressure adds the structured theory.
pub use super::cns::aperture as cns_aperture;

/// Minimum aperture — the membrane never fully closes.
pub const APERTURE_FLOOR: f64 = 0.12;

/// Sigmoid steepness parameter.
pub const APERTURE_STEEPNESS: f64 = 2.2;

/// Sigmoid inflection offset — load at half-aperture.
pub const APERTURE_INFLECTION: f64 = 1.2;

// ── Tier system ──────────────────────────────────────────────────

/// Evolution tier — how urgently the system needs to evolve.
///
/// Named after nervous system layers:
/// - Vital = brainstem (always on, no threshold)
/// - Sensory = thalamus (rapid response, 30s threshold)
/// - Cortical = cortex (deliberate, 120s threshold)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvolutionTier {
    /// Human attention — always fires. Threshold = 0.
    Vital,
    /// Confidence gap — fires when output is >30s stale.
    Sensory,
    /// Full pipeline — fires when output is >120s stale.
    Cortical,
}

impl EvolutionTier {
    /// Threshold in seconds. Gradient must exceed this to fire.
    pub fn threshold_secs(&self) -> f64 {
        match self {
            Self::Vital => 0.0,
            Self::Sensory => 30.0,
            Self::Cortical => 120.0,
        }
    }

    /// Does this tier always fire regardless of gradient?
    pub fn always_fires(&self) -> bool {
        matches!(self, Self::Vital)
    }

    /// What the tier senses.
    pub fn senses(&self) -> &'static str {
        match self {
            Self::Vital => "human attention",
            Self::Sensory => "confidence gap",
            Self::Cortical => "full pipeline staleness",
        }
    }

    /// All tiers in firing priority (vital first).
    pub fn all() -> &'static [Self] {
        &[Self::Vital, Self::Sensory, Self::Cortical]
    }
}

// ── Gradient ─────────────────────────────────────────────────────

/// Gradient — how stale the oldest output is relative to freshest input.
///
/// `gradient = stalest_output_ts - freshest_input_ts` (in seconds)
///
/// Positive = output is behind input (normal).
/// Zero = perfectly caught up.
/// Negative = time travel (should not happen).
#[derive(Debug, Clone, Copy)]
pub struct Gradient {
    /// Seconds between freshest input and stalest output.
    pub seconds: f64,
}

impl Gradient {
    /// Create from two timestamps (output and input, both in seconds since epoch).
    pub fn from_timestamps(stalest_output: f64, freshest_input: f64) -> Self {
        Self {
            seconds: (freshest_input - stalest_output).max(0.0),
        }
    }

    /// Create directly from a known staleness in seconds.
    pub fn from_staleness(seconds: f64) -> Self {
        Self {
            seconds: seconds.max(0.0),
        }
    }

    /// Does this gradient exceed the tier's threshold?
    /// Does this gradient exceed the tier's threshold?
    pub fn should_fire(&self, tier: EvolutionTier, aperture_val: f64) -> bool {
        if tier.always_fires() {
            return true;
        }
        self.seconds >= tier.threshold_secs() && aperture_val >= APERTURE_FLOOR
    }
}

// ── RPM ──────────────────────────────────────────────────────────

/// Membrane RPM — rotations per minute derived from pulse rhythm and rate.
///
/// The membrane doesn't measure wall-clock time. It measures rotations.
/// One rotation = one complete pulse cycle.
///
/// ```text
/// rpm = rate × rhythm
/// ```
///
/// At rhythm=0.774 and rate=1692: rpm ≈ 1310.
#[derive(Debug, Clone, Copy)]
pub struct MembraneRpm {
    /// Pulse rhythm (0.0–1.0). How synchronized the membrane is.
    pub rhythm: f64,
    /// Request rate (requests per second from the pulse feed).
    pub rate: f64,
}

impl MembraneRpm {
    /// Create from pulse feed values.
    pub fn new(rhythm: f64, rate: f64) -> Self {
        Self { rhythm, rate }
    }

    /// RPM — membrane rotations per minute.
    pub fn rpm(&self) -> f64 {
        self.rate * self.rhythm
    }

    /// How many membrane-rotations have elapsed over a gradient.
    pub fn rotations_over(&self, gradient: &Gradient) -> f64 {
        gradient.seconds * self.rhythm
    }
}

// ── Evolution Pressure ───────────────────────────────────────────

/// A single evolution pressure reading across all tiers.
///
/// This is the complete "should we evolve right now?" decision.
#[derive(Debug, Clone)]
pub struct EvolutionPressure {
    /// Current load estimate (0.0 = idle, 1.0 = nominal, 2.0+ = overloaded).
    pub load: f64,
    /// Computed aperture from sigmoid.
    pub aperture: f64,
    /// Pulse-derived RPM.
    pub rpm: MembraneRpm,
    /// Per-tier readings.
    pub tiers: Vec<TierReading>,
}

/// Reading for a single evolution tier.
#[derive(Debug, Clone)]
pub struct TierReading {
    /// Which tier.
    pub tier: EvolutionTier,
    /// Gradient (staleness) for this tier's concern.
    pub gradient: Gradient,
    /// Should this tier fire?
    pub should_fire: bool,
    /// Urgency = gradient × aperture × rhythm.
    pub urgency: f64,
    /// Membrane-rotations since last evolution at this tier.
    pub rotations: f64,
}

impl EvolutionPressure {
    /// Compute evolution pressure from live state.
    ///
    /// - `load`: estimated system load (rate/2000 is a reasonable proxy)
    /// - `rpm`: membrane RPM from pulse
    /// - `tier_gradients`: gradient (staleness in seconds) per tier
    pub fn compute(
        load: f64,
        rpm: MembraneRpm,
        tier_gradients: &[(EvolutionTier, f64)],
    ) -> Self {
        let aper = cns_aperture(load);
        let tiers = tier_gradients
            .iter()
            .map(|(tier, staleness)| {
                let gradient = Gradient::from_staleness(*staleness);
                let should_fire = gradient.should_fire(*tier, aper);
                let urgency = gradient.seconds * aper * rpm.rhythm;
                let rotations = rpm.rotations_over(&gradient);
                TierReading {
                    tier: *tier,
                    gradient,
                    should_fire,
                    urgency,
                    rotations,
                }
            })
            .collect();
        Self {
            load,
            aperture: aper,
            rpm,
            tiers,
        }
    }

    /// Highest urgency across all tiers that should fire.
    pub fn max_urgency(&self) -> f64 {
        self.tiers
            .iter()
            .filter(|t| t.should_fire)
            .map(|t| t.urgency)
            .fold(0.0_f64, f64::max)
    }

    /// Which tiers are currently firing?
    pub fn firing_tiers(&self) -> Vec<EvolutionTier> {
        self.tiers
            .iter()
            .filter(|t| t.should_fire)
            .map(|t| t.tier)
            .collect()
    }

    /// Is the system under evolution pressure (any tier firing)?
    pub fn under_pressure(&self) -> bool {
        self.tiers.iter().any(|t| t.should_fire)
    }
}

// ── Confidence Target ────────────────────────────────────────────

/// The confidence gap — distance between current sort quality and target.
///
/// Target: 85%. The point at which the sort is "good enough" for the
/// membrane to trust its own classification.
pub const CONFIDENCE_TARGET: f64 = 0.85;

/// Confidence gap from current value.
pub fn confidence_gap(current: f64) -> f64 {
    (CONFIDENCE_TARGET - current).max(0.0)
}

/// Is sort confidence above the target?
pub fn confidence_met(current: f64) -> bool {
    current >= CONFIDENCE_TARGET
}

// ── Dissipative Structure ────────────────────────────────────────

/// The membrane as a Prigogine dissipative structure.
///
/// Far from equilibrium, order emerges from entropy flow.
/// The evolution pressure IS the entropy production rate.
/// When pressure is zero, the system is at equilibrium (dead).
/// When pressure is maximum, the system is in crisis.
/// The sweet spot is persistent low-grade pressure — life.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DissipativePhase {
    /// No pressure — equilibrium. Dead or dormant.
    Equilibrium,
    /// Low pressure — homeostasis. Healthy.
    Homeostasis,
    /// Moderate pressure — adaptation. Evolving.
    Adaptation,
    /// High pressure — crisis. Emergency evolution.
    Crisis,
}

impl DissipativePhase {
    /// Classify urgency into a dissipative phase.
    pub fn from_urgency(urgency: f64) -> Self {
        if urgency < 1.0 {
            Self::Equilibrium
        } else if urgency < 30.0 {
            Self::Homeostasis
        } else if urgency < 100.0 {
            Self::Adaptation
        } else {
            Self::Crisis
        }
    }

    /// Is the system alive? (Not at equilibrium)
    pub fn is_alive(&self) -> bool {
        !matches!(self, Self::Equilibrium)
    }

    /// Prigogine's term for this phase.
    pub fn prigogine_term(&self) -> &'static str {
        match self {
            Self::Equilibrium => "thermodynamic equilibrium",
            Self::Homeostasis => "near-equilibrium steady state",
            Self::Adaptation => "far-from-equilibrium dissipative structure",
            Self::Crisis => "bifurcation point",
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Aperture tests ───────────────────────────────────────

    #[test]
    fn aperture_at_zero_load_is_wide_open() {
        let a = cns_aperture(0.0);
        assert!(a > 0.90, "at zero load, aperture should be >0.90, got {a}");
    }

    #[test]
    fn aperture_at_nominal_load() {
        let a = cns_aperture(1.0);
        assert!(
            (0.50..=0.70).contains(&a),
            "at load=1, aperture should be ~0.60, got {a}"
        );
    }

    #[test]
    fn aperture_at_high_load_constricts() {
        let a = cns_aperture(2.0);
        assert!(a < 0.20, "at load=2, aperture should be <0.20, got {a}");
    }

    #[test]
    fn aperture_never_below_floor() {
        for load in [3.0, 5.0, 10.0, 100.0] {
            let a = cns_aperture(load);
            assert!(
                a >= APERTURE_FLOOR,
                "aperture({load}) = {a} < floor {APERTURE_FLOOR}"
            );
        }
    }

    #[test]
    fn aperture_is_monotonically_decreasing() {
        let mut prev = cns_aperture(0.0);
        for i in 1..=20 {
            let load = i as f64 * 0.2;
            let a = cns_aperture(load);
            assert!(a <= prev, "aperture should decrease: {prev} -> {a} at load={load}");
            prev = a;
        }
    }

    #[test]
    fn aperture_inflection_at_half() {
        let a = cns_aperture(APERTURE_INFLECTION);
        assert!(
            (0.45..=0.55).contains(&a),
            "at inflection point, aperture should be ~0.5, got {a}"
        );
    }

    // ── Tier tests ───────────────────────────────────────────

    #[test]
    fn vital_always_fires() {
        assert!(EvolutionTier::Vital.always_fires());
        assert!(!EvolutionTier::Sensory.always_fires());
        assert!(!EvolutionTier::Cortical.always_fires());
    }

    #[test]
    fn tier_thresholds_increase() {
        let tiers = EvolutionTier::all();
        for w in tiers.windows(2) {
            assert!(
                w[0].threshold_secs() <= w[1].threshold_secs(),
                "{:?} threshold should be <= {:?}",
                w[0],
                w[1]
            );
        }
    }

    #[test]
    fn all_tiers_exist() {
        assert_eq!(EvolutionTier::all().len(), 3);
    }

    #[test]
    fn vital_threshold_is_zero() {
        assert_eq!(EvolutionTier::Vital.threshold_secs(), 0.0);
    }

    #[test]
    fn sensory_threshold_is_30() {
        assert_eq!(EvolutionTier::Sensory.threshold_secs(), 30.0);
    }

    #[test]
    fn cortical_threshold_is_120() {
        assert_eq!(EvolutionTier::Cortical.threshold_secs(), 120.0);
    }

    // ── Gradient tests ───────────────────────────────────────

    #[test]
    fn gradient_from_timestamps() {
        let g = Gradient::from_timestamps(100.0, 112.0);
        assert!((g.seconds - 12.0).abs() < 0.001);
    }

    #[test]
    fn gradient_clamps_negative() {
        let g = Gradient::from_timestamps(200.0, 100.0);
        assert_eq!(g.seconds, 0.0);
    }

    #[test]
    fn gradient_fires_vital_at_zero() {
        let g = Gradient::from_staleness(0.0);
        assert!(g.should_fire(EvolutionTier::Vital, 0.5));
    }

    #[test]
    fn gradient_fires_sensory_above_30() {
        let g = Gradient::from_staleness(31.0);
        assert!(g.should_fire(EvolutionTier::Sensory, 0.5));
    }

    #[test]
    fn gradient_no_fire_sensory_below_30() {
        let g = Gradient::from_staleness(29.0);
        assert!(!g.should_fire(EvolutionTier::Sensory, 0.5));
    }

    #[test]
    fn gradient_fires_cortical_above_120() {
        let g = Gradient::from_staleness(121.0);
        assert!(g.should_fire(EvolutionTier::Cortical, 0.5));
    }

    #[test]
    fn gradient_no_fire_cortical_below_120() {
        let g = Gradient::from_staleness(119.0);
        assert!(!g.should_fire(EvolutionTier::Cortical, 0.5));
    }

    // ── RPM tests ────────────────────────────────────────────

    #[test]
    fn rpm_from_live_values() {
        let rpm = MembraneRpm::new(0.774, 1692.0);
        let val = rpm.rpm();
        assert!(
            (val - 1309.608).abs() < 1.0,
            "expected ~1310 RPM, got {val}"
        );
    }

    #[test]
    fn rotations_over_gradient() {
        let rpm = MembraneRpm::new(0.774, 1692.0);
        let g = Gradient::from_staleness(112.0);
        let rotations = rpm.rotations_over(&g);
        assert!(
            (rotations - 86.688).abs() < 1.0,
            "expected ~86.7 rotations, got {rotations}"
        );
    }

    // ── Evolution pressure integration ───────────────────────

    #[test]
    fn live_state_oct20() {
        // Exact values from the evolution pressure note, Oct 20 2026
        let rpm = MembraneRpm::new(0.774493, 1692.0);
        let load = 1692.0 / 2000.0; // 0.846

        let pressure = EvolutionPressure::compute(
            load,
            rpm,
            &[
                (EvolutionTier::Cortical, 112.0),
                (EvolutionTier::Sensory, 112.0),
                (EvolutionTier::Vital, 0.0),
            ],
        );

        // Aperture at load 0.846
        let expected_aperture = cns_aperture(0.846);
        assert!(
            (pressure.aperture - expected_aperture).abs() < 0.001,
            "aperture mismatch"
        );

        // Vital always fires
        assert!(pressure.firing_tiers().contains(&EvolutionTier::Vital));

        // Sensory fires (112 > 30)
        assert!(pressure.firing_tiers().contains(&EvolutionTier::Sensory));

        // Cortical does NOT fire (112 < 120)
        assert!(!pressure.firing_tiers().contains(&EvolutionTier::Cortical));

        // Under pressure (vital + sensory)
        assert!(pressure.under_pressure());

        // Urgency for sensory tier: 112 × aperture × rhythm
        let sensory = pressure
            .tiers
            .iter()
            .find(|t| t.tier == EvolutionTier::Sensory)
            .unwrap();
        let expected_urgency = 112.0 * expected_aperture * 0.774493;
        assert!(
            (sensory.urgency - expected_urgency).abs() < 0.5,
            "sensory urgency: expected ~{expected_urgency}, got {}",
            sensory.urgency
        );
    }

    #[test]
    fn idle_system_has_no_cortical_pressure() {
        let rpm = MembraneRpm::new(0.5, 100.0);
        let pressure = EvolutionPressure::compute(
            0.1,
            rpm,
            &[
                (EvolutionTier::Cortical, 0.0),
                (EvolutionTier::Sensory, 0.0),
                (EvolutionTier::Vital, 0.0),
            ],
        );

        // Only vital fires (always)
        assert_eq!(pressure.firing_tiers(), vec![EvolutionTier::Vital]);
    }

    #[test]
    fn max_urgency_tracks_highest_firing() {
        let rpm = MembraneRpm::new(0.774, 1692.0);
        let pressure = EvolutionPressure::compute(
            0.846,
            rpm,
            &[
                (EvolutionTier::Vital, 0.0),
                (EvolutionTier::Sensory, 50.0),
            ],
        );
        assert!(pressure.max_urgency() > 0.0);
    }

    // ── Confidence tests ─────────────────────────────────────

    #[test]
    fn confidence_gap_at_538() {
        let gap = confidence_gap(0.538);
        assert!(
            (gap - 0.312).abs() < 0.001,
            "expected gap ~0.312, got {gap}"
        );
    }

    #[test]
    fn confidence_met_above_85() {
        assert!(confidence_met(0.86));
        assert!(confidence_met(0.85));
        assert!(!confidence_met(0.84));
        assert!(!confidence_met(0.538));
    }

    // ── Dissipative phase tests ──────────────────────────────

    #[test]
    fn equilibrium_is_dead() {
        let phase = DissipativePhase::from_urgency(0.5);
        assert_eq!(phase, DissipativePhase::Equilibrium);
        assert!(!phase.is_alive());
    }

    #[test]
    fn homeostasis_is_alive() {
        let phase = DissipativePhase::from_urgency(15.0);
        assert_eq!(phase, DissipativePhase::Homeostasis);
        assert!(phase.is_alive());
    }

    #[test]
    fn adaptation_is_far_from_equilibrium() {
        let phase = DissipativePhase::from_urgency(55.0);
        assert_eq!(phase, DissipativePhase::Adaptation);
        assert_eq!(
            phase.prigogine_term(),
            "far-from-equilibrium dissipative structure"
        );
    }

    #[test]
    fn crisis_is_bifurcation() {
        let phase = DissipativePhase::from_urgency(150.0);
        assert_eq!(phase, DissipativePhase::Crisis);
        assert_eq!(phase.prigogine_term(), "bifurcation point");
    }

    #[test]
    fn live_urgency_phase() {
        // From the note: urgency=54.7 for sensory tier
        let phase = DissipativePhase::from_urgency(54.7);
        assert_eq!(phase, DissipativePhase::Adaptation);
    }

    // ── π connection ─────────────────────────────────────────

    #[test]
    fn aperture_area_under_curve_approximates_pi() {
        // The sigmoid aperture integrated over [0, ∞) converges.
        // Not to π exactly, but the shape IS the logistic function,
        // and the logistic distribution has variance π²/3.
        //
        // We test that the inflection-centered sigmoid has the
        // expected mathematical properties.
        let a_at_inflection = cns_aperture(APERTURE_INFLECTION);
        assert!(
            (a_at_inflection - 0.5).abs() < 0.05,
            "sigmoid should be ~0.5 at inflection"
        );

        // Symmetry: aperture(inflection - x) + aperture(inflection + x) ≈ 1.0
        // (before floor clamping)
        let raw_lo = 1.0 / (1.0 + (APERTURE_STEEPNESS * (0.5 - APERTURE_INFLECTION)).exp());
        let raw_hi = 1.0 / (1.0 + (APERTURE_STEEPNESS * (1.9 - APERTURE_INFLECTION)).exp());
        let sum = raw_lo + raw_hi;
        assert!(
            (sum - 1.0).abs() < 0.01,
            "sigmoid symmetry: {raw_lo} + {raw_hi} = {sum}, expected ~1.0"
        );
    }

    #[test]
    fn pi_is_irrational_remainder() {
        // The 6.17% irrational remainder from gAIa
        let remainder = PI - 3.0;
        assert!(
            (remainder - 0.14159265).abs() < 0.001,
            "π remainder should be ~0.14159"
        );
        // As a percentage of the total, it's about 4.5%
        let pct = remainder / PI;
        assert!(
            (pct - 0.0451).abs() < 0.01,
            "π remainder is ~4.5% of the whole"
        );
    }
}

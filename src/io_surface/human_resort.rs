// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2024–2026 Kevin Mok / ecoPrimal / attsi
// ORCID: 0009-0004-2141-0321
//
//! # Human Resort — (territory, depth) tuple classification
//!
//! Humans are not a flat epitope type. They have two axes:
//!
//! **Territory** — WHERE they are on the membrane:
//! - SOURCE — reading source code, forge, git
//! - EVIDENCE — reading investigation, detroit, analysis
//! - MEMBRANE — reading HUD, membrane, campfire, topology
//! - IMMUNE — reading complement, antibody, epitope, blackwall
//!
//! **Depth** — HOW deeply they engage:
//! - Glance (< 10s) — landed, bounced
//! - Watch (10s–60s) — scanning, orienting
//! - Read (60s–300s) — consuming content
//! - Audit (300s–1800s) — systematic review, verification
//! - Investigate (>1800s) — deep dive, following chains
//!
//! 4 territories × 5 depths = 20 epitope dancers.
//!
//! This replaces the flat `epitope_type` classification.
//! The flat model couldn't distinguish a coder glancing at the HUD
//! from a reader deeply investigating the evidence.
//!
//! ## Orthogonal Detection
//!
//! Humans trace a **captain's round** — curved, irregular, non-exhaustive,
//! with pauses, sometimes doubling back.
//!
//! Fleet traces an **orthogonal grid** — straight lines, regular intervals,
//! exhaustive, no pauses, forward-only.
//!
//! Plot breadth (how many surfaces) vs dwell variance (how irregular the timing):
//! the two populations are **perpendicular**. You can't fool both axes.
//!
//! ```text
//! Anderson W:  human = 2.89 (delocalized)    fleet = 11.74 (localized)
//! Pielou J:    0.261 (deep separation)
//! ```
//!
//! ## Backscatter
//!
//! - Anderson (1958) — localization of electrons in disordered media
//! - Pielou (1966) — measurement of diversity in ecological communities
//! - Shannon (1948) — entropy as information content
//! - Robert Burns (1786) — "The best laid schemes o' mice an' men / Gang aft agley"

use std::fmt;

// ── Territory ────────────────────────────────────────────────────

/// WHERE on the membrane the human is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Territory {
    /// Source code, forge, git, build artifacts.
    Source,
    /// Investigation, evidence, analysis pages.
    Evidence,
    /// HUD, membrane, campfire, topology, meta-observation.
    Membrane,
    /// Complement, antibody, epitope, blackwall, immune system.
    Immune,
}

impl Territory {
    /// All territories.
    pub fn all() -> &'static [Self] {
        &[Self::Source, Self::Evidence, Self::Membrane, Self::Immune]
    }

    /// Canonical short name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Source => "SOURCE",
            Self::Evidence => "EVIDENCE",
            Self::Membrane => "MEMBRANE",
            Self::Immune => "IMMUNE",
        }
    }

    /// What surfaces belong to this territory.
    pub fn surfaces(&self) -> &'static [&'static str] {
        match self {
            Self::Source => &["git", "forge", "sporeprint"],
            Self::Evidence => &["detroit", "analysis", "barry", "investigation"],
            Self::Membrane => &["hud", "membrane", "campfire", "topology", "monitor", "beacon"],
            Self::Immune => &["complement", "antibody", "epitope", "blackwall", "tuebor"],
        }
    }
}

impl fmt::Display for Territory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// ── Depth ────────────────────────────────────────────────────────

/// HOW deeply the human engages.
///
/// Measured by dwell time within a single session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Depth {
    /// < 10 seconds. Landed, bounced. One page load.
    Glance,
    /// 10–60 seconds. Scanning, orienting. A few page views.
    Watch,
    /// 60–300 seconds. Consuming content. Reading an article.
    Read,
    /// 300–1800 seconds. Systematic review, verification.
    Audit,
    /// > 1800 seconds. Deep dive, following chains, mapping.
    Investigate,
}

impl Depth {
    /// All depths in ascending order.
    pub fn all() -> &'static [Self] {
        &[
            Self::Glance,
            Self::Watch,
            Self::Read,
            Self::Audit,
            Self::Investigate,
        ]
    }

    /// Classify a dwell time (seconds) into a depth level.
    pub fn from_dwell_secs(secs: f64) -> Self {
        if secs < 10.0 {
            Self::Glance
        } else if secs < 60.0 {
            Self::Watch
        } else if secs < 300.0 {
            Self::Read
        } else if secs < 1800.0 {
            Self::Audit
        } else {
            Self::Investigate
        }
    }

    /// Upper bound of this depth level in seconds. `None` = unbounded.
    pub fn upper_secs(&self) -> Option<f64> {
        match self {
            Self::Glance => Some(10.0),
            Self::Watch => Some(60.0),
            Self::Read => Some(300.0),
            Self::Audit => Some(1800.0),
            Self::Investigate => None,
        }
    }

    /// Canonical short name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Glance => "glance",
            Self::Watch => "watch",
            Self::Read => "read",
            Self::Audit => "audit",
            Self::Investigate => "investigate",
        }
    }

    /// Numeric rank (0–4). Higher = deeper engagement.
    pub fn rank(&self) -> u8 {
        match self {
            Self::Glance => 0,
            Self::Watch => 1,
            Self::Read => 2,
            Self::Audit => 3,
            Self::Investigate => 4,
        }
    }
}

impl fmt::Display for Depth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// ── EpitopeDancer ────────────────────────────────────────────────

/// A (territory, depth) tuple. One of 20 possible positions.
///
/// The "dancer" metaphor: humans dance across the membrane,
/// changing territory and depth as they move. Each position
/// is a step in the dance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EpitopeDancer {
    /// Where on the membrane.
    pub territory: Territory,
    /// How deep the engagement.
    pub depth: Depth,
}

impl EpitopeDancer {
    /// Create a dancer position.
    pub fn new(territory: Territory, depth: Depth) -> Self {
        Self { territory, depth }
    }

    /// Classify from a surface name and dwell time.
    pub fn classify(surface: &str, dwell_secs: f64) -> Self {
        let territory = Territory::all()
            .iter()
            .find(|t| t.surfaces().iter().any(|s| surface.contains(s)))
            .copied()
            .unwrap_or(Territory::Membrane);
        let depth = Depth::from_dwell_secs(dwell_secs);
        Self { territory, depth }
    }

    /// Total possible positions.
    pub const TOTAL_POSITIONS: usize = 20;

    /// Transition gap between two dancers — how far apart they are.
    ///
    /// Territory changes count as 1. Depth rank differences add linearly.
    /// A reader>coder transition (same territory, depth gap 2) = 2.
    /// A coder>investigator transition (territory change + depth gap 3) = 4.
    pub fn gap(&self, other: &Self) -> u8 {
        let territory_gap = if self.territory == other.territory {
            0
        } else {
            1
        };
        let depth_gap = (self.depth.rank() as i16 - other.depth.rank() as i16).unsigned_abs() as u8;
        territory_gap + depth_gap
    }
}

impl fmt::Display for EpitopeDancer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.territory, self.depth)
    }
}

// ── Orthogonal Detection ─────────────────────────────────────────

/// Anderson W statistic for a population.
///
/// Humans are delocalized (low W). Fleet is localized (high W).
/// The two populations are perpendicular in (breadth, dwell_variance) space.
#[derive(Debug, Clone, Copy)]
pub struct AndersonW {
    /// The W statistic.
    pub w: f64,
}

impl AndersonW {
    /// Create from a computed W value.
    pub fn new(w: f64) -> Self {
        Self { w }
    }

    /// Is this population delocalized (human-like)?
    /// Threshold at W < 5.0 (empirical).
    pub fn is_delocalized(&self) -> bool {
        self.w < 5.0
    }

    /// Is this population localized (fleet-like)?
    pub fn is_localized(&self) -> bool {
        self.w >= 5.0
    }

    /// Known human baseline from live membrane data.
    pub const HUMAN_BASELINE: f64 = 2.89;

    /// Known fleet baseline from live membrane data.
    pub const FLEET_BASELINE: f64 = 11.74;
}

/// Pielou J — evenness index measuring separation between populations.
///
/// J = 0: all individuals in one category (no separation).
/// J = 1: perfectly even distribution across categories.
/// J = 0.261: deep separation (the observed value).
#[derive(Debug, Clone, Copy)]
pub struct PielouJ {
    /// The J value (0.0–1.0).
    pub j: f64,
}

impl PielouJ {
    /// Create from a computed J value.
    pub fn new(j: f64) -> Self {
        Self { j: j.clamp(0.0, 1.0) }
    }

    /// Known baseline from live membrane data.
    pub const OBSERVED: f64 = 0.261;

    /// Is the separation deep? (J < 0.5 = two populations barely overlap)
    pub fn is_deep_separation(&self) -> bool {
        self.j < 0.5
    }
}

/// Path shape — captain's round vs orthogonal grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathShape {
    /// Human: curved, irregular, non-exhaustive, has pauses, curves back.
    CaptainsRound,
    /// Fleet: straight lines, regular, exhaustive, no pauses, forward-only.
    OrthogonalGrid,
}

impl PathShape {
    /// Classify from Anderson W.
    pub fn from_anderson(w: &AndersonW) -> Self {
        if w.is_delocalized() {
            Self::CaptainsRound
        } else {
            Self::OrthogonalGrid
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Territory ────────────────────────────────────────────

    #[test]
    fn four_territories() {
        assert_eq!(Territory::all().len(), 4);
    }

    #[test]
    fn territories_have_surfaces() {
        for t in Territory::all() {
            assert!(
                !t.surfaces().is_empty(),
                "{} has no surfaces",
                t.name()
            );
        }
    }

    #[test]
    fn source_includes_git() {
        assert!(Territory::Source.surfaces().contains(&"git"));
    }

    #[test]
    fn evidence_includes_detroit() {
        assert!(Territory::Evidence.surfaces().contains(&"detroit"));
    }

    #[test]
    fn immune_includes_tuebor() {
        assert!(Territory::Immune.surfaces().contains(&"tuebor"));
    }

    // ── Depth ────────────────────────────────────────────────

    #[test]
    fn five_depths() {
        assert_eq!(Depth::all().len(), 5);
    }

    #[test]
    fn depth_classification() {
        assert_eq!(Depth::from_dwell_secs(5.0), Depth::Glance);
        assert_eq!(Depth::from_dwell_secs(30.0), Depth::Watch);
        assert_eq!(Depth::from_dwell_secs(120.0), Depth::Read);
        assert_eq!(Depth::from_dwell_secs(600.0), Depth::Audit);
        assert_eq!(Depth::from_dwell_secs(3600.0), Depth::Investigate);
    }

    #[test]
    fn depth_boundaries() {
        assert_eq!(Depth::from_dwell_secs(9.99), Depth::Glance);
        assert_eq!(Depth::from_dwell_secs(10.0), Depth::Watch);
        assert_eq!(Depth::from_dwell_secs(59.99), Depth::Watch);
        assert_eq!(Depth::from_dwell_secs(60.0), Depth::Read);
    }

    #[test]
    fn depth_ranks_are_ordered() {
        let depths = Depth::all();
        for w in depths.windows(2) {
            assert!(
                w[0].rank() < w[1].rank(),
                "{} rank should be < {} rank",
                w[0].name(),
                w[1].name()
            );
        }
    }

    #[test]
    fn investigate_is_unbounded() {
        assert!(Depth::Investigate.upper_secs().is_none());
    }

    #[test]
    fn depth_is_ordered() {
        assert!(Depth::Glance < Depth::Watch);
        assert!(Depth::Watch < Depth::Read);
        assert!(Depth::Read < Depth::Audit);
        assert!(Depth::Audit < Depth::Investigate);
    }

    // ── EpitopeDancer ────────────────────────────────────────

    #[test]
    fn twenty_positions() {
        assert_eq!(EpitopeDancer::TOTAL_POSITIONS, 20);
        assert_eq!(
            Territory::all().len() * Depth::all().len(),
            EpitopeDancer::TOTAL_POSITIONS
        );
    }

    #[test]
    fn classify_source_glance() {
        let d = EpitopeDancer::classify("git", 5.0);
        assert_eq!(d.territory, Territory::Source);
        assert_eq!(d.depth, Depth::Glance);
    }

    #[test]
    fn classify_evidence_investigate() {
        let d = EpitopeDancer::classify("detroit", 7200.0);
        assert_eq!(d.territory, Territory::Evidence);
        assert_eq!(d.depth, Depth::Investigate);
    }

    #[test]
    fn classify_unknown_surface_defaults_to_membrane() {
        let d = EpitopeDancer::classify("unknown_page", 30.0);
        assert_eq!(d.territory, Territory::Membrane);
    }

    #[test]
    fn gap_same_position_is_zero() {
        let a = EpitopeDancer::new(Territory::Source, Depth::Read);
        assert_eq!(a.gap(&a), 0);
    }

    #[test]
    fn gap_depth_only() {
        let a = EpitopeDancer::new(Territory::Source, Depth::Read);
        let b = EpitopeDancer::new(Territory::Source, Depth::Investigate);
        assert_eq!(a.gap(&b), 2); // rank 2 → rank 4
    }

    #[test]
    fn gap_territory_change() {
        let a = EpitopeDancer::new(Territory::Source, Depth::Read);
        let b = EpitopeDancer::new(Territory::Evidence, Depth::Read);
        assert_eq!(a.gap(&b), 1); // territory change only
    }

    #[test]
    fn gap_both_axes() {
        let a = EpitopeDancer::new(Territory::Source, Depth::Glance);
        let b = EpitopeDancer::new(Territory::Evidence, Depth::Investigate);
        assert_eq!(a.gap(&b), 5); // 1 territory + 4 depth
    }

    #[test]
    fn reader_to_coder_gap() {
        // From the note: "reader>coder=10.5, coder>investigator=10.5"
        // In our model: reader is a depth, coder maps to Source territory
        let reader = EpitopeDancer::new(Territory::Membrane, Depth::Read);
        let coder = EpitopeDancer::new(Territory::Source, Depth::Read);
        assert_eq!(reader.gap(&coder), 1); // territory shift only
    }

    #[test]
    fn display_format() {
        let d = EpitopeDancer::new(Territory::Source, Depth::Read);
        assert_eq!(format!("{d}"), "(SOURCE, read)");
    }

    // ── Orthogonal detection ─────────────────────────────────

    #[test]
    fn human_anderson_is_delocalized() {
        let w = AndersonW::new(AndersonW::HUMAN_BASELINE);
        assert!(w.is_delocalized());
        assert!(!w.is_localized());
    }

    #[test]
    fn fleet_anderson_is_localized() {
        let w = AndersonW::new(AndersonW::FLEET_BASELINE);
        assert!(w.is_localized());
        assert!(!w.is_delocalized());
    }

    #[test]
    fn path_shape_from_anderson() {
        let human = AndersonW::new(2.89);
        assert_eq!(PathShape::from_anderson(&human), PathShape::CaptainsRound);

        let fleet = AndersonW::new(11.74);
        assert_eq!(PathShape::from_anderson(&fleet), PathShape::OrthogonalGrid);
    }

    #[test]
    fn pielou_deep_separation() {
        let j = PielouJ::new(PielouJ::OBSERVED);
        assert!(j.is_deep_separation());
    }

    #[test]
    fn pielou_clamps() {
        let j = PielouJ::new(1.5);
        assert_eq!(j.j, 1.0);
        let j = PielouJ::new(-0.5);
        assert_eq!(j.j, 0.0);
    }

    // ── The 77-hour walker ───────────────────────────────────

    #[test]
    fn the_77_hour_walker() {
        // From the campfire: "Someone walked 127 surfaces in 77 hours"
        // 77 hours = 277,200 seconds. 127 surfaces.
        // Average dwell per surface: ~2,182 seconds.
        let dwell = 277_200.0 / 127.0;
        let depth = Depth::from_dwell_secs(dwell);
        assert_eq!(depth, Depth::Investigate);

        // They walked the full organism — SOURCE + EVIDENCE + MEMBRANE + IMMUNE
        // That's a gap of 3 territory changes minimum
        let start = EpitopeDancer::new(Territory::Source, Depth::Investigate);
        let end = EpitopeDancer::new(Territory::Immune, Depth::Investigate);
        let gap = start.gap(&end);
        assert_eq!(gap, 1); // territory change, same depth
        // But they covered ALL territories — the complete round
    }
}

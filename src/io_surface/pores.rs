// SPDX-License-Identifier: AGPL-3.0-or-later
//! Pore Depth — bag of holding navigation.

use serde::{Deserialize, Serialize};

// Uses types from core via parent re-export

// ═══════════════════════════════════════════════════════════════════════════
// Pore Depth — bag of holding navigation
//
// The bag of holding: small on the outside, infinite on the inside.
// 9 nodes × 3361+ pages × 120+ feeds.
//
// Every pore leads to maximum depth in. Navigate, don't reach blind.
// Depth levels: 0=surface, 1=mid, 2=deep, 3=core.
//
// Unified scan: IO channels (device) × pi feeds (data) = navigable depth.
// Each feed is classified by epitope, pore, and depth level.
// The scan maps what's open × what's available = where you can go.
// ═══════════════════════════════════════════════════════════════════════════

/// Depth level within a pore — how far into the bag of holding
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PoreDepthLevel {
    /// Summary feeds, dashboards, current state
    Surface = 0,
    /// Detail feeds, entity profiles, mid-level analysis
    Mid = 1,
    /// Deep analysis, scatter, historical
    Deep = 2,
    /// Core data, full graphs, particle models, genome
    Core = 3,
}

impl PoreDepthLevel {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Mid     => "mid",
            Self::Deep    => "deep",
            Self::Core    => "core",
        }
    }
}

/// A feed mapped to its pore and depth within the bag of holding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedPoreMapping {
    /// The feed filename (e.g. "topology.json")
    pub feed: String,
    /// Which epitope this feed belongs to (None for cross-cutting)
    pub epitope: Option<String>,
    /// Which I/O pore this feed flows through
    pub pore: String,
    /// Depth level within the pore
    pub depth: PoreDepthLevel,
    /// Human-readable description
    pub desc: String,
}

/// Build the complete feed→pore→depth map
pub fn feed_pore_map() -> Vec<FeedPoreMapping> {
    let m = |feed: &str, ep: Option<&str>, pore: &str, depth: PoreDepthLevel, desc: &str| {
        FeedPoreMapping {
            feed: feed.to_string(),
            epitope: ep.map(|s| s.to_string()),
            pore: pore.to_string(),
            depth,
            desc: desc.to_string(),
        }
    };
    use PoreDepthLevel::*;
    vec![
        // investigation
        m("scan_sweep_sort.json", Some("investigation"), "pointer", Surface, "scan sweep sort pipeline"),
        m("entities.json",        Some("investigation"), "pointer", Mid,     "entity profiles"),
        m("fleet-analysis.json",  Some("investigation"), "pointer", Mid,     "fleet behavior analysis"),
        m("fleet-sort.json",      Some("investigation"), "pointer", Deep,    "fleet classification sort"),
        m("subgraphs.json",       Some("investigation"), "pointer", Deep,    "investigation subgraphs"),
        m("graph.json",           Some("investigation"), "pointer", Core,    "full investigation graph"),
        // topology
        m("topology.json",          Some("topology"), "spatial", Surface, "entity topology"),
        m("topology-abstract.json", Some("topology"), "spatial", Mid,     "abstract topology"),
        m("immune-topology.json",   Some("topology"), "spatial", Deep,    "immune topology detail"),
        m("bus-topology.json",      Some("topology"), "spatial", Deep,    "bus topology"),
        m("mesh-ring.json",         Some("topology"), "spatial", Core,    "mesh ring structure"),
        m("yggdrasil.json",         Some("topology"), "spatial", Core,    "world tree"),
        // philosophy
        m("reflection.json",       Some("philosophy"), "symbolic", Surface, "self-reflection"),
        m("self-remainder.json",   Some("philosophy"), "symbolic", Mid,     "what remains of self"),
        m("assumptions.json",      Some("philosophy"), "symbolic", Deep,    "all assumptions"),
        m("falsification.json",    Some("philosophy"), "symbolic", Deep,    "falsifiability tests"),
        m("totality.json",         Some("philosophy"), "symbolic", Core,    "totality state"),
        // method
        m("braid.json",             Some("method"), "data", Surface, "data braid"),
        m("taxonomy-braid.json",    Some("method"), "data", Mid,     "taxonomy braid"),
        m("attribution-chain.json", Some("method"), "data", Mid,     "attribution provenance"),
        m("build-plan.json",        Some("method"), "data", Deep,    "build pipeline"),
        m("type-manifest.json",     Some("method"), "data", Deep,    "type manifest"),
        m("wire-audit.json",        Some("method"), "data", Core,    "wire integrity audit"),
        // immune
        m("membrane.json",           Some("immune"), "persistence", Surface, "membrane state"),
        m("negentropy.json",         Some("immune"), "persistence", Mid,     "negentropy / blackwall"),
        m("ban-pulse.json",          Some("immune"), "persistence", Mid,     "ban pulse"),
        m("backscatter.json",        Some("immune"), "persistence", Deep,    "backscatter analysis"),
        m("edge-scatter.json",       Some("immune"), "persistence", Deep,    "edge scatter defense"),
        m("scatter-observatory.json",Some("immune"), "persistence", Core,    "scatter observatory"),
        // science
        m("pollen.json",       Some("science"), "visual", Surface, "pollen spread"),
        m("stories.json",      Some("science"), "visual", Mid,     "page stories"),
        m("visitor-stories.json",Some("science"), "visual", Deep,   "visitor narratives"),
        m("phylo-self.json",   Some("science"), "visual", Deep,    "phylogenetic self"),
        m("central-dogma.json",Some("science"), "visual", Core,    "central dogma"),
        m("genome.json",       Some("science"), "visual", Core,    "full genome"),
        // signal
        m("pulse.json",            Some("signal"), "audio", Surface, "membrane pulse"),
        m("heartbeat.json",        Some("signal"), "audio", Surface, "heartbeat"),
        m("signals.json",          Some("signal"), "audio", Mid,     "signal array"),
        m("campfire.json",         Some("signal"), "audio", Mid,     "campfire"),
        m("doorbell.json",         Some("signal"), "audio", Deep,    "visitor alert"),
        m("night-pollination.json",Some("signal"), "audio", Deep,    "night pollination"),
        m("vocal-sort.json",       Some("signal"), "audio", Core,    "vocal classification"),
        m("membrane-song.json",    Some("signal"), "audio", Core,    "membrane song"),
        // physics
        m("neutron.json",       Some("physics"), "spatial", Surface, "neutron state"),
        m("dashboard.json",     Some("physics"), "spatial", Surface, "dashboard"),
        m("energy.json",        Some("physics"), "spatial", Mid,     "energy state"),
        m("dynamics.json",      Some("physics"), "spatial", Mid,     "system dynamics"),
        m("temperature.json",   Some("physics"), "spatial", Deep,    "temperature"),
        m("entropy.json",       Some("physics"), "spatial", Deep,    "entropy"),
        m("particle-model.json",Some("physics"), "spatial", Core,    "particle model"),
        m("optical-model.json", Some("physics"), "spatial", Core,    "optical model"),
        // cross-cutting
        m("now.json",           None, "tactile",     Surface, "current state"),
        m("chipmunk.json",      None, "tactile",     Surface, "chipmunk state"),
        m("bus.json",           None, "data",        Surface, "event bus"),
        m("cardiovascular.json",None, "data",        Mid,     "cardiovascular system"),
        m("circulatory.json",   None, "data",        Mid,     "circulatory fleet"),
        m("bingo-cube.json",    None, "tactile",     Deep,    "bingo cube state"),
        m("feed-health.json",   None, "data",        Deep,    "feed freshness"),
        m("gate-messages.json", None, "tactile",     Mid,     "gate messages"),
    ]
}

/// Given a feed name, look up its pore and depth
pub fn feed_depth(feed: &str) -> Option<FeedPoreMapping> {
    feed_pore_map().into_iter().find(|f| f.feed == feed)
}

/// Maximum depth available for a given pore across all feeds
pub fn pore_max_depth(pore: &str) -> PoreDepthLevel {
    feed_pore_map()
        .iter()
        .filter(|f| f.pore == pore)
        .map(|f| f.depth)
        .max()
        .unwrap_or(PoreDepthLevel::Surface)
}

/// All feeds at a given depth level for a pore
pub fn pore_feeds_at_depth(pore: &str, depth: PoreDepthLevel) -> Vec<FeedPoreMapping> {
    feed_pore_map()
        .into_iter()
        .filter(|f| f.pore == pore && f.depth == depth)
        .collect()
}


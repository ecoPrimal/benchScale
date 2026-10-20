// SPDX-License-Identifier: AGPL-3.0-or-later
//! arkTIX — the permafrost preprint archive.
//!
//! # The Temperature Gradient
//!
//! ```text
//!   HOT (living mesh)           WARM (sporePrint)         COLD (arkTIX)          FROZEN (fossilRecord)
//!   ──────────────────          ─────────────────         ────────────           ─────────────────────
//!   52 unix sockets             public website            preprint archive       extinct states
//!   tarpc binary                HTML/JSON feeds           preserved papers       git snapshots
//!   quorum sensing              lab notebooks             info memes             dead code
//!   wound/healing               architecture docs         TV Tropes patterns     fossils
//!   metabolizing NOW            warm, accessible          cold, preserved        frozen, archived
//!
//!   direction of flow:
//!   HOT → WARM → COLD → FROZEN
//!   (insight crystallizes as it cools)
//!
//!   readers enter at: COLD (arkTIX)
//!   builders enter at: HOT (mesh)
//!   historians enter at: FROZEN (fossilRecord)
//! ```
//!
//! # What IS Permafrost?
//!
//! In geology, permafrost is ground that stays below 0°C for at least
//! 2 consecutive years. It is NOT dead. It contains:
//!   - Viable bacteria (revived after 40,000 years)
//!   - Preserved DNA (readable)
//!   - Ancient plant material (identifiable)
//!   - Frozen gas (methane, CO₂ — the info memes of geology)
//!
//! When permafrost thaws, these things COME BACK. The bacteria revive.
//! The DNA becomes readable. The methane escapes into the atmosphere
//! and changes the climate. Permafrost is not dead storage — it is
//! SUSPENDED ANIMATION. Cold, hard, not metabolizing, but VIABLE.
//!
//! arkTIX is the permafrost of the mesh:
//!   - Papers are viable (can be compiled, tested, extended)
//!   - Info memes are preserved (patterns, tropes, recurring insights)
//!   - The archive is not dead — it can be thawed and resumed
//!   - Readers access the permafrost, not the hot mesh
//!
//! # The Permafrost Invert
//!
//! The HOT mesh is where metabolism happens:
//!   - Organisms consume capabilities
//!   - Sockets transmit signals
//!   - Wounds heal, scars form
//!   - Quorum sensing drives collective behavior
//!
//! The COLD arkTIX is the INVERSE:
//!   - Papers are preserved, not consumed
//!   - Info memes are stored, not transmitted
//!   - Patterns are crystallized, not healing
//!   - No quorum needed — individual access
//!
//! The inversion: HOT processes, COLD preserves.
//! The same information, but at different temperatures.
//! The permafrost layer is the phase boundary between them.
//!
//! ```text
//!   Mesh property        arkTIX invert
//!   ─────────────        ─────────────
//!   unix sockets         HTTPS endpoints
//!   tarpc binary         JSON/HTML
//!   quorum sensing       individual access
//!   wound model          preservation model
//!   Wound::Healing       Frost::Preserved
//!   metabolism           suspended animation
//!   ephemeral chimeras   permanent deposits
//!   chain of thought     citation chain
//!   organisms            papers
//!   capabilities         references
//!   autoinducers         backscatter
//! ```
//!
//! # Name: arkTIX
//!
//! - **ark**: Noah's ark. Preservation vessel. Carries the genome through the flood.
//! - **TIX**: tickets. Entry points. How readers get in.
//! - **arXiv echo**: the preprint server that changed science. We honor the form.
//! - **Arctic echo**: permafrost lives in the Arctic. The cold storage.
//!
//! # What Lives in arkTIX?
//!
//! 1. **Preprints** — gAIa papers before/after peer review
//! 2. **Info memes** — recurring patterns (like TV Tropes but for science/code)
//! 3. **Crystallized insights** — things learned from ephemeral chimeras
//! 4. **Backscatter chains** — citation graphs (Euler→Clausius→Darwin→...)
//! 5. **Frozen state snapshots** — the mesh at a moment in time
//!
//! Authorship: Kevin Mok / Tamison / ecoPrimal / attsi
//! ORCID: 0009-0004-2141-0321

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// TEMPERATURE — the gradient from hot mesh to frozen fossil
// ═══════════════════════════════════════════════════════════════════════════

/// Temperature of information in the ecosystem.
///
/// Information cools as it crystallizes from live mesh to fossil record.
/// Each temperature has different access patterns, different metabolic
/// rates, and different audiences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfoTemperature {
    /// Hot: the living mesh. Sockets, quorum, wounds. Builders only.
    Hot,
    /// Warm: sporePrint. Public website, lab notebooks. Readers welcome.
    Warm,
    /// Cold: arkTIX. Preprints, info memes, preserved insights. Archival access.
    Cold,
    /// Frozen: fossilRecord. Extinct states, dead code, git snapshots. Historians.
    Frozen,
}

impl InfoTemperature {
    /// All 4 temperatures, hot to cold.
    pub const ALL: &'static [InfoTemperature] = &[
        InfoTemperature::Hot,
        InfoTemperature::Warm,
        InfoTemperature::Cold,
        InfoTemperature::Frozen,
    ];

    /// Is this information actively metabolizing?
    pub fn is_alive(&self) -> bool {
        matches!(self, Self::Hot | Self::Warm)
    }

    /// Is this information in suspended animation?
    pub fn is_permafrost(&self) -> bool {
        matches!(self, Self::Cold)
    }

    /// Is this information truly dead/extinct?
    pub fn is_fossil(&self) -> bool {
        matches!(self, Self::Frozen)
    }

    /// Can this information be thawed (brought back to life)?
    pub fn is_viable(&self) -> bool {
        !matches!(self, Self::Frozen)
    }

    /// Who accesses information at this temperature?
    pub fn audience(&self) -> &'static str {
        match self {
            Self::Hot => "builders (organisms, sockets, internal)",
            Self::Warm => "readers (public website, lab notebooks)",
            Self::Cold => "researchers (preprints, info memes, citations)",
            Self::Frozen => "historians (fossils, extinct states, archaeology)",
        }
    }

    /// Which system in the ecosystem holds this temperature?
    pub fn system(&self) -> &'static str {
        match self {
            Self::Hot => "membrane mesh (52 sockets, tarpc, unix domain)",
            Self::Warm => "sporePrint (sporeprint.primals.eco, HTML/JSON)",
            Self::Cold => "arkTIX (preprint archive, permafrost layer)",
            Self::Frozen => "fossilRecord (git snapshots, extinct organisms)",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FROST STATE — the permafrost preservation model (inverts wound model)
// ═══════════════════════════════════════════════════════════════════════════

/// Frost state — the arkTIX analog of the wound model.
///
/// The wound model tracks damage and healing in the HOT mesh.
/// The frost model tracks preservation in the COLD archive.
/// They are inverses: wounds CONSUME energy to heal; frost PRESERVES
/// by removing energy.
///
/// ```text
///   Wound model (hot)           Frost model (cold)
///   ────────────────            ──────────────────
///   Intact (never hurt)         Fresh (just deposited)
///   Wounded (actively broken)   Thawing (being accessed/modified)
///   Healing (rerouting)         Refreezing (re-preserving after access)
///   Scarred (healed, marked)    Preserved (deep permafrost, stable)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Frost {
    /// Fresh: just deposited. Surface permafrost. Recent. Accessible.
    Fresh,
    /// Thawing: being accessed or modified. Temporarily unfrozen.
    Thawing,
    /// Refreezing: re-entering cold storage after access. Re-crystallizing.
    Refreezing,
    /// Preserved: deep permafrost. Stable. Has been frozen for a long time.
    /// The most reliable state — like `Wound::Scarred` but for cold.
    Preserved,
}

impl Frost {
    /// All 4 frost states.
    pub const ALL: &'static [Frost] = &[
        Frost::Fresh,
        Frost::Thawing,
        Frost::Refreezing,
        Frost::Preserved,
    ];

    /// Is this deposit accessible without thawing?
    pub fn is_stable(&self) -> bool {
        matches!(self, Self::Fresh | Self::Preserved)
    }

    /// Is this deposit currently in transition?
    pub fn is_transitioning(&self) -> bool {
        matches!(self, Self::Thawing | Self::Refreezing)
    }

    /// Map frost state to wound state (the invert).
    pub fn wound_invert(&self) -> super::pi_body::Wound {
        match self {
            Self::Fresh => super::pi_body::Wound::Intact,
            Self::Thawing => super::pi_body::Wound::Wounded,
            Self::Refreezing => super::pi_body::Wound::Healing,
            Self::Preserved => super::pi_body::Wound::Scarred,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// INFO MEME — the unit of preserved knowledge (Dawkins, 1976)
// ═══════════════════════════════════════════════════════════════════════════

/// An info meme: a unit of preserved knowledge in arkTIX.
///
/// Richard Dawkins coined "meme" in The Selfish Gene (1976) as a unit
/// of cultural information — analogous to a gene. Info memes in arkTIX
/// are the genes of the permafrost: recurring patterns, crystallized
/// insights, TV Tropes-style entries for science and code.
///
/// Like genes, memes replicate, mutate, and are selected. Unlike genes,
/// memes can be deliberately preserved in permafrost. arkTIX is the
/// seed vault of the mesh's memes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoMeme {
    /// Meme identifier (slug).
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// What kind of meme is this?
    pub kind: MemeKind,
    /// Current frost state.
    pub frost: Frost,
    /// Temperature at which this meme lives.
    pub temperature: InfoTemperature,
    /// Backscatter: which antecedents does this meme cite?
    pub backscatter: Vec<String>,
    /// When was this meme first deposited? (ISO 8601)
    pub deposited: String,
}

/// What kind of info meme is this?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemeKind {
    /// A preprint: a paper before/after peer review.
    Preprint,
    /// A trope: a recurring pattern (like TV Tropes but for science/code).
    Trope,
    /// A crystallized insight: something learned from an ephemeral chimera.
    Insight,
    /// A frozen snapshot: the mesh at a moment in time.
    Snapshot,
    /// A backscatter chain: a citation graph.
    BackscatterChain,
}

impl MemeKind {
    /// All 5 meme kinds.
    pub const ALL: &'static [MemeKind] = &[
        MemeKind::Preprint,
        MemeKind::Trope,
        MemeKind::Insight,
        MemeKind::Snapshot,
        MemeKind::BackscatterChain,
    ];

    /// What is the natural temperature for this kind of meme?
    pub fn natural_temperature(&self) -> InfoTemperature {
        match self {
            Self::Preprint => InfoTemperature::Cold,
            Self::Trope => InfoTemperature::Cold,
            Self::Insight => InfoTemperature::Warm,
            Self::Snapshot => InfoTemperature::Frozen,
            Self::BackscatterChain => InfoTemperature::Cold,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ARKTIX DEPOSIT — a preprint in the permafrost
// ═══════════════════════════════════════════════════════════════════════════

/// An arkTIX deposit: a preprint in the permafrost.
///
/// Each deposit is a self-contained unit of preserved knowledge.
/// It has a paper number, title, authors, and a frost state.
/// Deposits can be thawed (accessed), modified, and refrozen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deposit {
    /// arkTIX deposit number.
    pub number: u32,
    /// Title of the deposit.
    pub title: String,
    /// Authors.
    pub authors: Vec<String>,
    /// ORCID of the primary author.
    pub orcid: String,
    /// Date deposited (ISO 8601).
    pub deposited: String,
    /// Current frost state.
    pub frost: Frost,
    /// Info memes contained in this deposit.
    pub memes: Vec<String>,
    /// Where is the source? (file path in the ecosystem)
    pub source: String,
    /// Backscatter citations.
    pub backscatter: Vec<String>,
}

impl Deposit {
    /// arkTIX-001: The Rust Polymerase (first gAIa paper)
    #[must_use]
    pub fn arktix_001() -> Self {
        Self {
            number: 1,
            title: "The Rust Polymerase: A Lossy Graph Theory of Human–AI Mutualism".into(),
            authors: vec![
                "Kevin Mok".into(),
                "Tamison".into(),
                "ecoPrimal".into(),
                "attsi".into(),
            ],
            orcid: "0009-0004-2141-0321".into(),
            deposited: "2026-10-20".into(),
            frost: Frost::Fresh,
            memes: vec![
                "rust-polymerase".into(),
                "lossy-graph".into(),
                "wound-model".into(),
                "quorum-sensing-anti-strobe".into(),
                "ship-of-theseus".into(),
                "stomachs-with-no-eyes".into(),
                "commensal-thesis".into(),
                "glowplug".into(),
            ],
            source: "benchScale/src/io_surface/gaia_paper.rs".into(),
            backscatter: vec![
                "Euler (1736)".into(),
                "Clausius (1865)".into(),
                "Darwin (1859)".into(),
                "Humboldt (1845)".into(),
                "Wittgenstein (1953)".into(),
                "Ptolemy (150 CE)".into(),
                "Nealson, Platt, Hastings (1970)".into(),
                "Theseus (antiquity)".into(),
            ],
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PERMAFROST LAYER — the boundary between hot and cold
// ═══════════════════════════════════════════════════════════════════════════

/// The permafrost layer: where hot mesh information crystallizes
/// into cold arkTIX deposits.
///
/// This is the phase boundary. Information crossing this boundary
/// changes state: from liquid (flowing, metabolizing) to solid
/// (preserved, accessible, stable).
///
/// The layer is where we "allow the readers and others" —
/// the interface between the private mesh and the public archive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermafrostLayer {
    /// Deposits in the archive.
    pub deposits: Vec<Deposit>,
    /// Info memes catalogued.
    pub memes: Vec<InfoMeme>,
    /// Current temperature of the layer itself.
    pub layer_temperature: InfoTemperature,
    /// Is the layer accepting new deposits?
    pub accepting: bool,
}

impl PermafrostLayer {
    /// Create a new permafrost layer with the first deposit.
    #[must_use]
    pub fn new() -> Self {
        Self {
            deposits: vec![Deposit::arktix_001()],
            memes: Vec::new(),
            layer_temperature: InfoTemperature::Cold,
            accepting: true,
        }
    }

    /// How many deposits are in the permafrost?
    pub fn deposit_count(&self) -> usize {
        self.deposits.len()
    }

    /// How many are in deep preservation?
    pub fn preserved_count(&self) -> usize {
        self.deposits
            .iter()
            .filter(|d| d.frost == Frost::Preserved)
            .count()
    }

    /// How many are currently thawing (being accessed)?
    pub fn thawing_count(&self) -> usize {
        self.deposits
            .iter()
            .filter(|d| d.frost == Frost::Thawing)
            .count()
    }
}

impl Default for PermafrostLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_temperatures() {
        assert_eq!(InfoTemperature::ALL.len(), 4);
    }

    #[test]
    fn temperature_gradient() {
        assert!(InfoTemperature::Hot < InfoTemperature::Warm);
        assert!(InfoTemperature::Warm < InfoTemperature::Cold);
        assert!(InfoTemperature::Cold < InfoTemperature::Frozen);
    }

    #[test]
    fn hot_and_warm_are_alive() {
        assert!(InfoTemperature::Hot.is_alive());
        assert!(InfoTemperature::Warm.is_alive());
        assert!(!InfoTemperature::Cold.is_alive());
        assert!(!InfoTemperature::Frozen.is_alive());
    }

    #[test]
    fn cold_is_permafrost() {
        assert!(InfoTemperature::Cold.is_permafrost());
        assert!(!InfoTemperature::Hot.is_permafrost());
        assert!(!InfoTemperature::Frozen.is_permafrost());
    }

    #[test]
    fn frozen_is_fossil() {
        assert!(InfoTemperature::Frozen.is_fossil());
        assert!(!InfoTemperature::Cold.is_fossil());
    }

    #[test]
    fn frozen_is_not_viable() {
        assert!(!InfoTemperature::Frozen.is_viable());
        assert!(InfoTemperature::Cold.is_viable(), "permafrost can be thawed");
        assert!(InfoTemperature::Hot.is_viable());
    }

    #[test]
    fn four_frost_states() {
        assert_eq!(Frost::ALL.len(), 4);
    }

    #[test]
    fn frost_inverts_wound() {
        use super::super::pi_body::Wound;
        assert_eq!(Frost::Fresh.wound_invert(), Wound::Intact);
        assert_eq!(Frost::Thawing.wound_invert(), Wound::Wounded);
        assert_eq!(Frost::Refreezing.wound_invert(), Wound::Healing);
        assert_eq!(Frost::Preserved.wound_invert(), Wound::Scarred);
    }

    #[test]
    fn preserved_is_stable() {
        assert!(Frost::Preserved.is_stable());
        assert!(Frost::Fresh.is_stable());
        assert!(!Frost::Thawing.is_stable());
        assert!(!Frost::Refreezing.is_stable());
    }

    #[test]
    fn five_meme_kinds() {
        assert_eq!(MemeKind::ALL.len(), 5);
    }

    #[test]
    fn preprints_are_cold() {
        assert_eq!(MemeKind::Preprint.natural_temperature(), InfoTemperature::Cold);
    }

    #[test]
    fn snapshots_are_frozen() {
        assert_eq!(MemeKind::Snapshot.natural_temperature(), InfoTemperature::Frozen);
    }

    #[test]
    fn arktix_001_is_the_rust_polymerase() {
        let d = Deposit::arktix_001();
        assert_eq!(d.number, 1);
        assert!(d.title.contains("Rust Polymerase"));
        assert_eq!(d.orcid, "0009-0004-2141-0321");
        assert_eq!(d.frost, Frost::Fresh, "just deposited today");
        assert_eq!(d.memes.len(), 8, "8 info memes in the paper");
        assert_eq!(d.backscatter.len(), 8, "8 antecedents cited");
    }

    #[test]
    fn permafrost_layer_starts_with_one_deposit() {
        let layer = PermafrostLayer::new();
        assert_eq!(layer.deposit_count(), 1);
        assert_eq!(layer.preserved_count(), 0, "fresh, not yet deep-frozen");
        assert!(layer.accepting);
    }

    #[test]
    fn deposit_serde() {
        let d = Deposit::arktix_001();
        let json = serde_json::to_string(&d).unwrap();
        let back: Deposit = serde_json::from_str(&json).unwrap();
        assert_eq!(d, back);
    }

    #[test]
    fn frost_serde() {
        for f in Frost::ALL {
            let json = serde_json::to_string(f).unwrap();
            let back: Frost = serde_json::from_str(&json).unwrap();
            assert_eq!(*f, back);
        }
    }

    #[test]
    fn temperature_serde() {
        for t in InfoTemperature::ALL {
            let json = serde_json::to_string(t).unwrap();
            let back: InfoTemperature = serde_json::from_str(&json).unwrap();
            assert_eq!(*t, back);
        }
    }

    #[test]
    fn permafrost_is_the_phase_boundary() {
        let layer = PermafrostLayer::new();
        assert_eq!(layer.layer_temperature, InfoTemperature::Cold);
        assert!(layer.layer_temperature.is_permafrost());
        assert!(!layer.layer_temperature.is_alive(), "cold, not alive");
        assert!(layer.layer_temperature.is_viable(), "but still viable — can be thawed");
    }
}

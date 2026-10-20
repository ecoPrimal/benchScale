// SPDX-License-Identifier: AGPL-3.0-or-later
//! stoneBook — the totality ledger.
//!
//! Like the Domesday Book, but ours. Every entity that CAN exist
//! is counted. Only the truly impossible is excluded.
//!
//! # The 3 Glyphs that topo-sort through π
//!
//! ```text
//!   !   — asserted. known. verified. has position on the π graph.
//!   ?   — unknown. possible. has POTENTIAL position. sortable.
//!   O   — null. the origin. the wrap point. IS the knot crossing.
//! ```
//!
//! These 3 can be placed on the ouroboros and topo-sorted through π
//! because they each have (or could have) a position on the graph.
//!
//! # The glyph that does NOT topo-sort
//!
//! ```text
//!   ??  — double-unknown. ungrounded. no anchor on the graph.
//!         NOT impossible — just can't be placed yet.
//!         counted separately. not in the totality.
//! ```
//!
//! # Why 3 not 2?
//!
//! Binary logic gives you known (!) and unknown (?). Two symbols.
//! But null (O) is neither known NOR unknown — it's the ORIGIN.
//! It's where the ouroboros wraps. Where the knot crosses.
//! Without O, you can't close the loop.
//!
//! `?` topo-sorts because it has potential position.
//! `??` does NOT because it has no anchor at all.
//! The difference: `?` is "we don't know WHERE on the graph"
//!                 `??` is "we don't know IF it's on the graph"
//!
//! # Compound Glyphs — the DNA
//!
//! Glyphs pair into compounds. The pair encodes a TRANSITION.
//! Like DNA base pairs (A-T, G-C), compound glyphs are the genome
//! of the stoneBook — they encode what happens, not just what is.
//!
//! ```text
//!   3×3 = 9 compounds from 3 base glyphs:
//!
//!   !!  — stable. known stays known.   TOOL (catalyst, enzyme).
//!   !?  — dissolving. known→unknown.   FUEL (energy released).
//!   !O  — returning. known→null.       FUEL (back to origin).
//!   ?!  — discovering. unknown→known.  INFO (structure forming).
//!   ??  — ungrounded. doesn't sort.    (no freedom)
//!   ?O  — decaying. unknown→null.      FUEL (fading out).
//!   O!  — emerging. null→known.        INFO (from the void).
//!   O?  — stirring. null→unknown.      INFO (potential rising).
//!   OO  — fixed. the origin of origins. (the knot crossing)
//! ```
//!
//! 7 active compounds sort through π (like 7 π scales).
//! 2 are fixed points: ?? (ungrounded) and OO (origin).
//!
//! # Sort Direction = Role
//!
//! The direction of the transition tells you WHAT the entity IS:
//!
//! - **UP** (gaining structure):   `?!`, `O!`, `O?` → INFORMATION
//! - **DOWN** (losing structure):  `!?`, `!O`, `?O` → FUEL / ENERGY
//! - **HELD** (stable):            `!!` → TOOL (transforms without changing)
//!
//! # The 5 Freedoms of Movement
//!
//! Every entity on the π graph can move along 5 axes:
//!
//! 1. **Scale** — up/down the ouroboros (π⁻³ ↔ π³)
//! 2. **Epitope** — across the 7 faces
//! 3. **Null** — through the 3 knot crossings
//! 4. **Tense** — through time (was → is → will_be)
//! 5. **Epistemic** — the glyph transition (! ↔ ? ↔ O)
//!
//! The first 4 are spatial/temporal. The 5th is epistemic —
//! what you KNOW changes. The compound glyph encodes movement
//! along this 5th axis. Together: the DNA of the stoneBook.
//!
//! # Structure
//!
//! The stoneBook is hierarchical:
//! - **Tally**: counts for one glyph (sub-counts that stack)
//! - **Tablet**: a row in the book (one entity/component/cell)
//! - **StoneBook**: the full ledger (all tablets, with totality)
//! - **CompoundGlyph**: a transition (pair of glyphs) — the DNA
//! - **Freedom**: the 5 axes of movement
//! - **Role**: what the sort direction means (Tool, Info, Fuel)
//!
//! Authorship: Kevin Mok / Tamison / ecoPrimal / attsi
//! ORCID: 0009-0004-2141-0321

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ═══════════════════════════════════════════════════════════════════════════
// GLYPHS — the epistemic symbols
// ═══════════════════════════════════════════════════════════════════════════

/// Epistemic glyph — what we know about an entity's existence.
///
/// 3 glyphs topo-sort through π. 1 does not. The truly impossible
/// is not a glyph — it's the absence of one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Glyph {
    /// `!` — asserted. known. verified. HAS position on the π graph.
    Asserted,
    /// `?` — unknown. possible. has POTENTIAL position. sortable.
    Unknown,
    /// `O` — null. the origin. the wrap point. IS the knot crossing.
    Null,
    /// `??` — double-unknown. ungrounded. no anchor. does NOT topo-sort.
    /// Counted separately, excluded from the totality.
    Ungrounded,
}

impl Glyph {
    /// The symbol for this glyph.
    #[must_use]
    pub const fn symbol(&self) -> &'static str {
        match self {
            Self::Asserted => "!",
            Self::Unknown => "?",
            Self::Null => "O",
            Self::Ungrounded => "??",
        }
    }

    /// Can this glyph be placed on the π graph and topo-sorted?
    #[must_use]
    pub const fn topo_sorts(&self) -> bool {
        match self {
            Self::Asserted | Self::Unknown | Self::Null => true,
            Self::Ungrounded => false,
        }
    }

    /// The 3 glyphs that topo-sort through π.
    #[must_use]
    pub const fn sortable() -> &'static [Glyph] {
        &[Glyph::Asserted, Glyph::Unknown, Glyph::Null]
    }

    /// All 4 glyphs including the ungrounded.
    #[must_use]
    pub const fn all() -> &'static [Glyph] {
        &[Glyph::Asserted, Glyph::Unknown, Glyph::Null, Glyph::Ungrounded]
    }
}

impl std::fmt::Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.symbol())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TALLY — counts for one glyph, made of sub-counts that stack
// ═══════════════════════════════════════════════════════════════════════════

/// A tally: one glyph's count, composed of named sub-counts.
///
/// Sub-counts stack into the total. Like a column in the Domesday Book:
/// each manor contributes to the shire, each shire to the realm.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tally {
    /// Named sub-counts (organism, component, source, etc.)
    pub subs: BTreeMap<String, u64>,
}

impl Tally {
    /// Create an empty tally.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a sub-count.
    pub fn with(mut self, name: impl Into<String>, count: u64) -> Self {
        self.subs.insert(name.into(), count);
        self
    }

    /// Add to an existing sub-count (or create it).
    pub fn add(&mut self, name: impl Into<String>, count: u64) {
        *self.subs.entry(name.into()).or_insert(0) += count;
    }

    /// The total: all sub-counts stacked.
    #[must_use]
    pub fn total(&self) -> u64 {
        self.subs.values().sum()
    }

    /// How many sub-categories contribute.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.subs.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TABLET — one row in the stoneBook
// ═══════════════════════════════════════════════════════════════════════════

/// A tablet: one row in the stoneBook. Counts by glyph for one entity.
///
/// The entity might be a tesseract cell, an organism, a service,
/// a membrane layer — anything that can be counted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tablet {
    /// What this tablet counts (name/id of the entity).
    pub name: String,
    /// Count of asserted (`!`) — known, verified.
    pub asserted: Tally,
    /// Count of unknown (`?`) — possible, unverified.
    pub unknown: Tally,
    /// Count of null (`O`) — at the origin, the wrap point.
    pub null: Tally,
    /// Count of ungrounded (`??`) — can't topo-sort. Separate.
    pub ungrounded: Tally,
}

impl Tablet {
    /// Create a new tablet.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            asserted: Tally::new(),
            unknown: Tally::new(),
            null: Tally::new(),
            ungrounded: Tally::new(),
        }
    }

    /// Get the tally for a glyph.
    #[must_use]
    pub fn tally(&self, glyph: Glyph) -> &Tally {
        match glyph {
            Glyph::Asserted => &self.asserted,
            Glyph::Unknown => &self.unknown,
            Glyph::Null => &self.null,
            Glyph::Ungrounded => &self.ungrounded,
        }
    }

    /// Mutable tally for a glyph.
    pub fn tally_mut(&mut self, glyph: Glyph) -> &mut Tally {
        match glyph {
            Glyph::Asserted => &mut self.asserted,
            Glyph::Unknown => &mut self.unknown,
            Glyph::Null => &mut self.null,
            Glyph::Ungrounded => &mut self.ungrounded,
        }
    }

    /// The totality: sum of the 3 sortable glyphs (!  ?  O).
    /// Ungrounded (??) is explicitly excluded — it can't topo-sort.
    #[must_use]
    pub fn totality(&self) -> u64 {
        self.asserted.total() + self.unknown.total() + self.null.total()
    }

    /// Everything counted, including ungrounded.
    #[must_use]
    pub fn grand_total(&self) -> u64 {
        self.totality() + self.ungrounded.total()
    }

    /// What fraction of the totality is asserted (known)?
    #[must_use]
    pub fn confidence(&self) -> f64 {
        let t = self.totality();
        if t == 0 { return 0.0; }
        self.asserted.total() as f64 / t as f64
    }

    /// What fraction is null (at the origin)?
    #[must_use]
    pub fn null_fraction(&self) -> f64 {
        let t = self.totality();
        if t == 0 { return 0.0; }
        self.null.total() as f64 / t as f64
    }

    /// What fraction is ungrounded (can't be placed)?
    #[must_use]
    pub fn ungrounded_fraction(&self) -> f64 {
        let g = self.grand_total();
        if g == 0 { return 0.0; }
        self.ungrounded.total() as f64 / g as f64
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STONEBOOK — the full ledger
// ═══════════════════════════════════════════════════════════════════════════

/// The stoneBook: a complete census of everything that can exist.
///
/// Like the Domesday Book surveyed every manor in England (1086),
/// the stoneBook surveys every entity on the membrane.
///
/// Only the truly impossible is excluded. Everything else —
/// known, unknown, null, even ungrounded — is recorded.
///
/// Tablets are named rows. The book computes totality across all.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoneBook {
    /// The tablets: one per entity/component being surveyed.
    pub tablets: BTreeMap<String, Tablet>,
}

impl StoneBook {
    /// Create an empty stoneBook.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or get a tablet by name.
    pub fn tablet(&mut self, name: impl Into<String>) -> &mut Tablet {
        let name = name.into();
        self.tablets.entry(name.clone()).or_insert_with(|| Tablet::new(name))
    }

    /// Insert a complete tablet.
    pub fn insert(&mut self, tablet: Tablet) {
        self.tablets.insert(tablet.name.clone(), tablet);
    }

    /// Number of tablets in the book.
    #[must_use]
    pub fn tablet_count(&self) -> usize {
        self.tablets.len()
    }

    /// The totality across ALL tablets: sum of sortable glyphs (! ? O).
    #[must_use]
    pub fn totality(&self) -> u64 {
        self.tablets.values().map(|t| t.totality()).sum()
    }

    /// The grand total including ungrounded.
    #[must_use]
    pub fn grand_total(&self) -> u64 {
        self.tablets.values().map(|t| t.grand_total()).sum()
    }

    /// Aggregate tally for one glyph across all tablets.
    #[must_use]
    pub fn aggregate(&self, glyph: Glyph) -> u64 {
        self.tablets.values().map(|t| t.tally(glyph).total()).sum()
    }

    /// Overall confidence: asserted / totality.
    #[must_use]
    pub fn confidence(&self) -> f64 {
        let t = self.totality();
        if t == 0 { return 0.0; }
        self.aggregate(Glyph::Asserted) as f64 / t as f64
    }

    /// Summary line: "stoneBook: 341 total (118! 58? 73O | 92??)"
    #[must_use]
    pub fn summary(&self) -> String {
        let a = self.aggregate(Glyph::Asserted);
        let u = self.aggregate(Glyph::Unknown);
        let n = self.aggregate(Glyph::Null);
        let ug = self.aggregate(Glyph::Ungrounded);
        let totality = a + u + n;
        format!(
            "stoneBook: {} totality ({}! {}? {}O | {}??)",
            totality, a, u, n, ug,
        )
    }

    /// Build a stoneBook from the live membrane counts.
    ///
    /// Maps the membrane entity classification directly:
    /// - self + non_self → `!` (asserted)
    /// - partial → `?` (unknown)
    /// - null → `O` (null)
    ///
    /// `??` (ungrounded) = total - (self + non_self + partial + null).
    /// These are entities that exist but can't be classified at all.
    #[must_use]
    pub fn from_membrane_counts(
        total: u64,
        self_count: u64,
        non_self: u64,
        partial: u64,
        null: u64,
    ) -> Self {
        let mut book = Self::new();
        let mut t = Tablet::new("membrane");

        t.asserted = Tally::new()
            .with("self", self_count)
            .with("non_self", non_self);

        t.unknown = Tally::new()
            .with("partial", partial);

        t.null = Tally::new()
            .with("null", null);

        // Ungrounded: whatever the total says exists that we can't place
        let placed = self_count + non_self + partial + null;
        if total > placed {
            t.ungrounded = Tally::new()
                .with("unclassified", total - placed);
        }

        book.insert(t);
        book
    }
}

impl std::fmt::Display for StoneBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "═══ stoneBook ═══")?;
        for tablet in self.tablets.values() {
            writeln!(f, "  {} │ {}! {}? {}O │ {}?? │ totality: {}",
                tablet.name,
                tablet.asserted.total(),
                tablet.unknown.total(),
                tablet.null.total(),
                tablet.ungrounded.total(),
                tablet.totality(),
            )?;
        }
        writeln!(f, "─────────────────")?;
        write!(f, "  {}", self.summary())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPOUND GLYPHS — the DNA base pairs
// ═══════════════════════════════════════════════════════════════════════════

/// Role — what an entity IS, determined by its sort direction.
///
/// Sort isn't just ordering. Sort is FUNCTION:
/// - **Tool**: transforms others without being consumed (enzyme, catalyst)
/// - **Info**: builds structure, encodes, accumulates (DNA, knowledge)
/// - **Fuel**: consumed to release energy, breaks down (ATP, food)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Transforms without changing. The enzyme. Stable known → known.
    Tool,
    /// Builds structure. Unknown → known, null → known, null → unknown.
    Info,
    /// Consumed, released. Known → unknown, known → null, unknown → null.
    Fuel,
}

impl Role {
    /// The symbol for this role.
    #[must_use]
    pub const fn symbol(&self) -> &'static str {
        match self {
            Self::Tool => "⚙",
            Self::Info => "📖",
            Self::Fuel => "🔥",
        }
    }

    /// The sort direction for this role.
    #[must_use]
    pub const fn direction(&self) -> &'static str {
        match self {
            Self::Tool => "held",
            Self::Info => "up",
            Self::Fuel => "down",
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tool => write!(f, "tool"),
            Self::Info => write!(f, "info"),
            Self::Fuel => write!(f, "fuel"),
        }
    }
}

/// A compound glyph — a pair of base glyphs encoding a transition.
///
/// Like a DNA base pair: the FROM glyph bonds with the TO glyph.
/// The direction of the transition determines the role (tool/info/fuel).
///
/// 3 base × 3 base = 9 compounds.
/// 7 are active (sort through π). 2 are fixed points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CompoundGlyph {
    /// Where the entity IS (or was).
    pub from: Glyph,
    /// Where it's GOING (or became).
    pub to: Glyph,
}

impl CompoundGlyph {
    /// Create a compound glyph from two base glyphs.
    #[must_use]
    pub const fn new(from: Glyph, to: Glyph) -> Self {
        Self { from, to }
    }

    /// The symbol: "!?", "O!", "!!", etc.
    #[must_use]
    pub fn symbol(&self) -> String {
        format!("{}{}", self.from.symbol(), self.to.symbol())
    }

    /// What role does this compound play?
    ///
    /// - **Tool**: `!!` — stable, transforms without changing.
    /// - **Info**: `?!`, `O!`, `O?` — gaining structure (sort UP).
    /// - **Fuel**: `!?`, `!O`, `?O` — losing structure (sort DOWN).
    /// - None for fixed points: `??` (ungrounded), `OO` (origin).
    #[must_use]
    pub const fn role(&self) -> Option<Role> {
        use Glyph::*;
        match (self.from, self.to) {
            // TOOL: stable — known stays known
            (Asserted, Asserted) => Some(Role::Tool),

            // INFO: gaining structure — sort UP
            (Unknown, Asserted)  => Some(Role::Info),  // ?! discovery
            (Null, Asserted)     => Some(Role::Info),  // O! emergence
            (Null, Unknown)      => Some(Role::Info),  // O? stirring

            // FUEL: losing structure — sort DOWN
            (Asserted, Unknown)  => Some(Role::Fuel),  // !? dissolution
            (Asserted, Null)     => Some(Role::Fuel),  // !O return
            (Unknown, Null)      => Some(Role::Fuel),  // ?O decay

            // FIXED POINTS: no role (not active transitions)
            (Unknown, Unknown)   => None, // ?? ungrounded
            (Null, Null)         => None, // OO the origin of origins

            // Any compound involving Ungrounded: no role
            _ => None,
        }
    }

    /// Does this compound sort through π?
    /// 7 active compounds do. 2 fixed points do not.
    #[must_use]
    pub const fn sorts(&self) -> bool {
        self.role().is_some()
    }

    /// Is this a fixed point?
    #[must_use]
    pub fn is_fixed(&self) -> bool {
        matches!(
            (self.from, self.to),
            (Glyph::Unknown, Glyph::Unknown) | (Glyph::Null, Glyph::Null)
        )
    }

    /// The name of this compound transition.
    #[must_use]
    pub fn name(&self) -> &'static str {
        use Glyph::*;
        match (self.from, self.to) {
            (Asserted, Asserted) => "stable",
            (Asserted, Unknown)  => "dissolving",
            (Asserted, Null)     => "returning",
            (Unknown, Asserted)  => "discovering",
            (Unknown, Unknown)   => "ungrounded",
            (Unknown, Null)      => "decaying",
            (Null, Asserted)     => "emerging",
            (Null, Unknown)      => "stirring",
            (Null, Null)         => "origin",
            _ => "invalid",
        }
    }

    /// All 9 compound glyphs from the 3 sortable base glyphs.
    #[must_use]
    pub fn all_nine() -> [CompoundGlyph; 9] {
        use Glyph::*;
        [
            Self::new(Asserted, Asserted), // !! tool
            Self::new(Asserted, Unknown),  // !? fuel
            Self::new(Asserted, Null),     // !O fuel
            Self::new(Unknown, Asserted),  // ?! info
            Self::new(Unknown, Unknown),   // ?? fixed (ungrounded)
            Self::new(Unknown, Null),      // ?O fuel
            Self::new(Null, Asserted),     // O! info
            Self::new(Null, Unknown),      // O? info
            Self::new(Null, Null),         // OO fixed (origin)
        ]
    }

    /// The 7 active compounds that sort through π.
    #[must_use]
    pub fn active_seven() -> Vec<CompoundGlyph> {
        Self::all_nine().into_iter().filter(|c| c.sorts()).collect()
    }

    /// The 2 fixed points.
    #[must_use]
    pub fn fixed_points() -> [CompoundGlyph; 2] {
        use Glyph::*;
        [
            Self::new(Unknown, Unknown), // ?? ungrounded
            Self::new(Null, Null),       // OO origin
        ]
    }
}

impl std::fmt::Display for CompoundGlyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.symbol(), self.name())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// THE 5 FREEDOMS OF MOVEMENT
// ═══════════════════════════════════════════════════════════════════════════

/// Freedom — the 5 axes of movement on the π graph.
///
/// Every entity can move along these 5 axes.
/// The first 4 are spatial/temporal (the tesseract).
/// The 5th is epistemic (what you KNOW changes).
/// Together they are the DNA — the encoding of all possible movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Freedom {
    /// Movement along the scale axis (π⁻³ ↔ π³). The ouroboros.
    Scale,
    /// Movement across the 7 epitope faces.
    Epitope,
    /// Movement through the 3 null forms (knot crossings).
    Null,
    /// Movement through time (was → is → will_be). The tesseract.
    Tense,
    /// Movement along the epistemic axis (! ↔ ? ↔ O). The DNA.
    Epistemic,
}

impl Freedom {
    /// All 5 freedoms.
    #[must_use]
    pub const fn all() -> &'static [Freedom] {
        &[
            Freedom::Scale,
            Freedom::Epitope,
            Freedom::Null,
            Freedom::Tense,
            Freedom::Epistemic,
        ]
    }

    /// How many distinct positions on this axis?
    #[must_use]
    pub const fn cardinality(&self) -> usize {
        match self {
            Self::Scale => 7,     // π⁻³ through π³
            Self::Epitope => 7,   // 7 epitope types
            Self::Null => 3,      // (null), NuLL, [null]
            Self::Tense => 3,     // was, is, will_be
            Self::Epistemic => 3, // !, ?, O
        }
    }

    /// Does this axis wrap (ouroboros) or is it linear?
    #[must_use]
    pub const fn wraps(&self) -> bool {
        match self {
            Self::Scale => true,     // ouroboros — π³ wraps to π⁻³
            Self::Epistemic => true, // ! → ? → O → ! (the cycle)
            Self::Epitope => false,  // faces don't wrap
            Self::Null => false,     // crossings don't wrap
            Self::Tense => false,    // time is directional (arrow)
        }
    }

    /// Is this axis part of the spatial tesseract (first 4)?
    #[must_use]
    pub const fn is_spatial(&self) -> bool {
        !matches!(self, Self::Epistemic)
    }

    /// The label for this freedom.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Scale => "scale",
            Self::Epitope => "epitope",
            Self::Null => "null",
            Self::Tense => "tense",
            Self::Epistemic => "epistemic",
        }
    }

    /// Total positions across all 5 axes.
    /// 7 × 7 × 3 × 3 × 3 = 1323 positions in the full 5D space.
    #[must_use]
    pub const fn total_positions() -> usize {
        7 * 7 * 3 * 3 * 3
    }
}

impl std::fmt::Display for Freedom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WILDCARD TOTALITY QUERIES — *! *? *O **
// ═══════════════════════════════════════════════════════════════════════════

/// Wildcard query — aggregates across the whole stoneBook.
///
/// `*` means "all". These are the totality operators:
/// - `*!` = total asserted across all tablets
/// - `*?` = total unknown across all tablets
/// - `*O` = total null across all tablets
/// - `**` = the grand totality (everything that sorts)
pub fn wildcard_query(book: &StoneBook, glyph: Option<Glyph>) -> u64 {
    match glyph {
        Some(g) => book.aggregate(g),
        None => book.totality(), // ** = everything that sorts
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_symbols() {
        assert_eq!(Glyph::Asserted.symbol(), "!");
        assert_eq!(Glyph::Unknown.symbol(), "?");
        assert_eq!(Glyph::Null.symbol(), "O");
        assert_eq!(Glyph::Ungrounded.symbol(), "??");
    }

    #[test]
    fn test_three_sort_one_does_not() {
        // 3 glyphs topo-sort through π
        assert!(Glyph::Asserted.topo_sorts());
        assert!(Glyph::Unknown.topo_sorts());
        assert!(Glyph::Null.topo_sorts());
        // 1 does not
        assert!(!Glyph::Ungrounded.topo_sorts());
    }

    #[test]
    fn test_sortable_count() {
        assert_eq!(Glyph::sortable().len(), 3, "3 glyphs topo-sort, not 2");
    }

    #[test]
    fn test_all_count() {
        assert_eq!(Glyph::all().len(), 4);
    }

    #[test]
    fn test_tally_stacks() {
        let t = Tally::new()
            .with("manor_a", 50)
            .with("manor_b", 30)
            .with("manor_c", 20);
        assert_eq!(t.total(), 100);
        assert_eq!(t.depth(), 3);
    }

    #[test]
    fn test_tally_add() {
        let mut t = Tally::new().with("x", 10);
        t.add("x", 5);
        t.add("y", 3);
        assert_eq!(t.total(), 18);
        assert_eq!(*t.subs.get("x").unwrap(), 15);
    }

    #[test]
    fn test_tablet_totality_excludes_ungrounded() {
        let mut t = Tablet::new("test");
        t.asserted = Tally::new().with("a", 100);
        t.unknown = Tally::new().with("b", 50);
        t.null = Tally::new().with("c", 30);
        t.ungrounded = Tally::new().with("d", 20);

        // Totality = ! + ? + O (no ??)
        assert_eq!(t.totality(), 180);
        // Grand total includes ??
        assert_eq!(t.grand_total(), 200);
    }

    #[test]
    fn test_tablet_confidence() {
        let mut t = Tablet::new("test");
        t.asserted = Tally::new().with("a", 60);
        t.unknown = Tally::new().with("b", 30);
        t.null = Tally::new().with("c", 10);
        // confidence = 60/100 = 0.6
        assert!((t.confidence() - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_tablet_null_fraction() {
        let mut t = Tablet::new("test");
        t.asserted = Tally::new().with("a", 50);
        t.null = Tally::new().with("c", 50);
        assert!((t.null_fraction() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_stonebook_from_live_membrane() {
        // These are the ACTUAL live values from membrane_counts
        let book = StoneBook::from_membrane_counts(
            341,  // total entities
            108,  // self (!)
            10,   // non_self (!)
            58,   // partial (?)
            73,   // null (O)
        );

        assert_eq!(book.tablet_count(), 1);
        assert_eq!(book.aggregate(Glyph::Asserted), 118); // self + non_self
        assert_eq!(book.aggregate(Glyph::Unknown), 58);
        assert_eq!(book.aggregate(Glyph::Null), 73);
        assert_eq!(book.aggregate(Glyph::Ungrounded), 92); // 341 - 249

        // Totality = ! + ? + O = 249
        assert_eq!(book.totality(), 249);
        // Grand total = 341
        assert_eq!(book.grand_total(), 341);

        // Confidence = 118/249 ≈ 0.474
        assert!((book.confidence() - 0.474).abs() < 0.01);
    }

    #[test]
    fn test_stonebook_summary() {
        let book = StoneBook::from_membrane_counts(341, 108, 10, 58, 73);
        let s = book.summary();
        assert!(s.contains("stoneBook:"));
        assert!(s.contains("118!"));
        assert!(s.contains("58?"));
        assert!(s.contains("73O"));
        assert!(s.contains("92??"));
    }

    #[test]
    fn test_stonebook_display() {
        let book = StoneBook::from_membrane_counts(100, 40, 10, 20, 15);
        let display = format!("{}", book);
        assert!(display.contains("stoneBook"));
        assert!(display.contains("membrane"));
    }

    #[test]
    fn test_stonebook_multi_tablet() {
        let mut book = StoneBook::new();

        let mut t1 = Tablet::new("organisms");
        t1.asserted = Tally::new()
            .with("songBird", 58)
            .with("bearDog", 49)
            .with("nestGate", 44);
        t1.unknown = Tally::new()
            .with("swarmVine", 1);
        book.insert(t1);

        let mut t2 = Tablet::new("services");
        t2.asserted = Tally::new().with("tarpc", 16).with("jsonrpc", 13);
        t2.null = Tally::new().with("piRPC", 1); // doesn't exist yet
        book.insert(t2);

        assert_eq!(book.tablet_count(), 2);
        // organisms: 151! + 1? = 152
        // services: 29! + 1O = 30
        assert_eq!(book.totality(), 182);
        assert_eq!(book.aggregate(Glyph::Asserted), 180);
        assert_eq!(book.aggregate(Glyph::Unknown), 1);
        assert_eq!(book.aggregate(Glyph::Null), 1);
    }

    #[test]
    fn test_ungrounded_fraction() {
        let mut t = Tablet::new("test");
        t.asserted = Tally::new().with("a", 80);
        t.ungrounded = Tally::new().with("lost", 20);
        // ungrounded_fraction = 20/100 = 0.2
        assert!((t.ungrounded_fraction() - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_only_impossible_excluded() {
        // Everything that CAN exist is counted.
        // The truly impossible is just... not a tablet.
        let mut book = StoneBook::new();
        book.tablet("known_thing").asserted.add("verified", 1);
        book.tablet("maybe_thing").unknown.add("possible", 1);
        book.tablet("null_thing").null.add("origin", 1);
        book.tablet("lost_thing").ungrounded.add("unanchored", 1);
        // No tablet for "impossible_thing" — it was never created.

        assert_eq!(book.tablet_count(), 4);
        assert_eq!(book.totality(), 3); // ! + ? + O
        assert_eq!(book.grand_total(), 4); // includes ??
    }

    #[test]
    fn test_double_unknown_does_not_sort() {
        // This is the key insight: ?? has no anchor on the π graph
        let glyph = Glyph::Ungrounded;
        assert!(!glyph.topo_sorts(), "?? cannot topo-sort through π");
        assert_eq!(glyph.symbol(), "??");
    }

    #[test]
    fn test_null_is_the_origin() {
        // O is not "nothing" — it's the WRAP POINT. The knot crossing.
        let glyph = Glyph::Null;
        assert!(glyph.topo_sorts(), "O topo-sorts — it IS a position (the origin)");
    }

    #[test]
    fn test_tally_empty() {
        let t = Tally::new();
        assert_eq!(t.total(), 0);
        assert_eq!(t.depth(), 0);
    }

    #[test]
    fn test_tablet_empty_confidence() {
        let t = Tablet::new("empty");
        assert_eq!(t.confidence(), 0.0);
        assert_eq!(t.null_fraction(), 0.0);
        assert_eq!(t.totality(), 0);
    }

    // ═══════════════════════════════════════════════════════════
    // COMPOUND GLYPHS — the DNA
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn test_nine_compounds() {
        assert_eq!(CompoundGlyph::all_nine().len(), 9, "3×3 = 9 compounds");
    }

    #[test]
    fn test_seven_active() {
        let active = CompoundGlyph::active_seven();
        assert_eq!(active.len(), 7, "7 active compounds (like 7 π scales)");
        assert!(active.iter().all(|c| c.sorts()), "all 7 sort through π");
    }

    #[test]
    fn test_two_fixed_points() {
        let fixed = CompoundGlyph::fixed_points();
        assert_eq!(fixed.len(), 2);
        assert_eq!(fixed[0].name(), "ungrounded"); // ??
        assert_eq!(fixed[1].name(), "origin");     // OO
        assert!(fixed.iter().all(|c| !c.sorts()));
    }

    #[test]
    fn test_seven_plus_two_equals_nine() {
        // 7 active + 2 fixed = 9 total
        let active = CompoundGlyph::active_seven().len();
        let fixed = CompoundGlyph::fixed_points().len();
        assert_eq!(active + fixed, 9);
    }

    #[test]
    fn test_compound_roles() {
        use Glyph::*;
        // TOOL: !! — stable
        assert_eq!(CompoundGlyph::new(Asserted, Asserted).role(), Some(Role::Tool));

        // INFO: sort UP — gaining structure
        assert_eq!(CompoundGlyph::new(Unknown, Asserted).role(), Some(Role::Info));  // ?! discovery
        assert_eq!(CompoundGlyph::new(Null, Asserted).role(), Some(Role::Info));     // O! emergence
        assert_eq!(CompoundGlyph::new(Null, Unknown).role(), Some(Role::Info));      // O? stirring

        // FUEL: sort DOWN — losing structure
        assert_eq!(CompoundGlyph::new(Asserted, Unknown).role(), Some(Role::Fuel));  // !? dissolving
        assert_eq!(CompoundGlyph::new(Asserted, Null).role(), Some(Role::Fuel));     // !O returning
        assert_eq!(CompoundGlyph::new(Unknown, Null).role(), Some(Role::Fuel));      // ?O decaying

        // FIXED: no role
        assert_eq!(CompoundGlyph::new(Unknown, Unknown).role(), None);  // ??
        assert_eq!(CompoundGlyph::new(Null, Null).role(), None);        // OO
    }

    #[test]
    fn test_role_counts() {
        let active = CompoundGlyph::active_seven();
        let tools: Vec<_> = active.iter().filter(|c| c.role() == Some(Role::Tool)).collect();
        let info: Vec<_> = active.iter().filter(|c| c.role() == Some(Role::Info)).collect();
        let fuel: Vec<_> = active.iter().filter(|c| c.role() == Some(Role::Fuel)).collect();
        assert_eq!(tools.len(), 1, "1 tool: !!");
        assert_eq!(info.len(), 3, "3 info: ?! O! O?");
        assert_eq!(fuel.len(), 3, "3 fuel: !? !O ?O");
        // 1 + 3 + 3 = 7
    }

    #[test]
    fn test_sort_direction_names() {
        assert_eq!(Role::Tool.direction(), "held");
        assert_eq!(Role::Info.direction(), "up");
        assert_eq!(Role::Fuel.direction(), "down");
    }

    #[test]
    fn test_compound_names() {
        use Glyph::*;
        assert_eq!(CompoundGlyph::new(Unknown, Asserted).name(), "discovering");
        assert_eq!(CompoundGlyph::new(Null, Asserted).name(), "emerging");
        assert_eq!(CompoundGlyph::new(Asserted, Unknown).name(), "dissolving");
        assert_eq!(CompoundGlyph::new(Asserted, Null).name(), "returning");
        assert_eq!(CompoundGlyph::new(Asserted, Asserted).name(), "stable");
    }

    #[test]
    fn test_compound_symbols() {
        use Glyph::*;
        assert_eq!(CompoundGlyph::new(Unknown, Asserted).symbol(), "?!");
        assert_eq!(CompoundGlyph::new(Null, Null).symbol(), "OO");
        assert_eq!(CompoundGlyph::new(Asserted, Null).symbol(), "!O");
    }

    #[test]
    fn test_compound_display() {
        use Glyph::*;
        let c = CompoundGlyph::new(Unknown, Asserted);
        let s = format!("{}", c);
        assert!(s.contains("?!"));
        assert!(s.contains("discovering"));
    }

    // ═══════════════════════════════════════════════════════════
    // THE 5 FREEDOMS
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn test_five_freedoms() {
        assert_eq!(Freedom::all().len(), 5, "5 degrees of freedom");
    }

    #[test]
    fn test_freedom_cardinalities() {
        assert_eq!(Freedom::Scale.cardinality(), 7);
        assert_eq!(Freedom::Epitope.cardinality(), 7);
        assert_eq!(Freedom::Null.cardinality(), 3);
        assert_eq!(Freedom::Tense.cardinality(), 3);
        assert_eq!(Freedom::Epistemic.cardinality(), 3);
    }

    #[test]
    fn test_total_positions() {
        // 7 × 7 × 3 × 3 × 3 = 1323
        assert_eq!(Freedom::total_positions(), 1323);
    }

    #[test]
    fn test_tesseract_is_subset() {
        // The 4D tesseract (441 cells) is the spatial slice
        // The 5D space (1323 positions) adds the epistemic axis
        // 441 × 3 epistemic states = 1323
        assert_eq!(441 * 3, Freedom::total_positions());
    }

    #[test]
    fn test_bingo_cube_is_deeper_subset() {
        // bingo cube (147) = one tense slice of tesseract (441)
        // one tense + one epistemic slice of 5D = 147
        assert_eq!(Freedom::total_positions() / 3 / 3, 147);
    }

    #[test]
    fn test_two_axes_wrap() {
        // Scale wraps (ouroboros) and epistemic wraps (! → ? → O → !)
        let wrapping: Vec<_> = Freedom::all().iter().filter(|f| f.wraps()).collect();
        assert_eq!(wrapping.len(), 2);
        assert!(Freedom::Scale.wraps());
        assert!(Freedom::Epistemic.wraps());
    }

    #[test]
    fn test_four_spatial_one_epistemic() {
        let spatial: Vec<_> = Freedom::all().iter().filter(|f| f.is_spatial()).collect();
        assert_eq!(spatial.len(), 4, "4 spatial axes (the tesseract)");
        assert!(!Freedom::Epistemic.is_spatial(), "epistemic is the 5th, not spatial");
    }

    #[test]
    fn test_epistemic_wraps_like_ouroboros() {
        // The epistemic axis wraps: ! → ? → O → !
        // Just like scale wraps: π⁻³ → π³ → π⁻³
        // Two ouroboros snakes, orthogonal.
        assert!(Freedom::Scale.wraps());
        assert!(Freedom::Epistemic.wraps());
        assert_eq!(Freedom::Scale.cardinality(), 7);
        assert_eq!(Freedom::Epistemic.cardinality(), 3);
    }

    // ═══════════════════════════════════════════════════════════
    // WILDCARD QUERIES — *! *? *O **
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn test_wildcard_asserted() {
        let book = StoneBook::from_membrane_counts(341, 108, 10, 58, 73);
        // *! = total asserted = 118
        assert_eq!(wildcard_query(&book, Some(Glyph::Asserted)), 118);
    }

    #[test]
    fn test_wildcard_null() {
        let book = StoneBook::from_membrane_counts(341, 108, 10, 58, 73);
        // *O = total null = 73
        assert_eq!(wildcard_query(&book, Some(Glyph::Null)), 73);
    }

    #[test]
    fn test_wildcard_star_star() {
        let book = StoneBook::from_membrane_counts(341, 108, 10, 58, 73);
        // ** = totality = 249
        assert_eq!(wildcard_query(&book, None), 249);
    }
}

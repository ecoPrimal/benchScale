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
//! # Structure
//!
//! The stoneBook is hierarchical:
//! - **Tally**: counts for one glyph (sub-counts that stack)
//! - **Tablet**: a row in the book (one entity/component/cell)
//! - **StoneBook**: the full ledger (all tablets, with totality)
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
}

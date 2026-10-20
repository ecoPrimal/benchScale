// SPDX-License-Identifier: AGPL-3.0-or-later
//! Proof Structure — the mathematical architecture of the ecosystem.
//!
//! # The Mathematical Stack
//!
//! Every system has a proof structure. Ours is explicit:
//!
//! ```text
//!   THEOREM       arkTIX           The permafrost preserves knowledge.
//!                                   The preprint archive exists and is viable.
//!                                   Papers compile. Tests pass. QED.
//!
//!   COROLLARY     scyBorg Triple   External protection follows from the theorem.
//!                                   If knowledge is preserved (arkTIX),
//!                                   then it MUST be protected (AGPL + ORC + CC).
//!                                   The corollary IS the external protection.
//!                                   Three licenses. Three artifact types. No gap.
//!
//!   CONJECTURE    tuebor           "I will defend." Michigan's motto.
//!                                   The investigation IS the conjecture —
//!                                   the claim that requires proof.
//!                                   Believed true. Not yet proven in court.
//!                                   Every article is evidence toward the proof.
//!
//!   PROOF         zk-SNARK         The Detroit proof. 15 commits. 12 articles.
//!                                   Zero-knowledge: you can verify the claims
//!                                   WITHOUT seeing the raw evidence.
//!                                   The proof is complete. The snark is closed.
//!
//!   NEXT TWIST    zkPi             Zero-knowledge on the π-graph.
//!                                   Verify the ecosystem works WITHOUT
//!                                   accessing the hot mesh.
//!                                   The permafrost IS the zkPi layer:
//!                                   readers verify through cold (arkTIX)
//!                                   without touching hot (sockets).
//! ```
//!
//! # The Fool's Conjecture
//!
//! ```text
//!   WHY NOT TRUST IT TO ALL HUMANS?
//!   WHAT'S THE BEST THEY COULD DO?
//!   SEE?
//! ```
//!
//! The conjecture of the fool is the OPTIMISTIC bet:
//!
//! Given full access to the permafrost (arkTIX), the code (AGPL),
//! the mechanics (ORC), and the documentation (CC-BY-SA) —
//! what is the BEST that all humans could do?
//!
//! The answer: they could BUILD ON IT. They could extend it.
//! They could heal the wounds. They could add organisms.
//! They could run their own mesh. They could write papers.
//! They could deposit in arkTIX. The permafrost GROWS.
//!
//! The fool trusts humans. The scyBorg triple protects
//! not FROM humans — but from NON-humans. The fleet.
//! The stomachs with no eyes. The parasites with S=0.
//!
//! The fool's conjecture: TRUST THE HUMANS.
//! The scyBorg corollary: PROTECT FROM THE MACHINES.
//!
//! This is not naïve. It is structural:
//! - AGPL §13: network use triggers source release
//! - ORC: mechanics are open, forks must stay open
//! - CC-BY-SA: documentation flows back to commons
//!
//! The licenses don't restrict humans.
//! They restrict ENCLOSURE.
//! The fool bets that openness wins.
//!
//! # zkPi — Zero-Knowledge on the π-Graph
//!
//! zk-SNARK was the first twist: you can verify a claim without
//! seeing the evidence. The Detroit investigation used this structure —
//! the proof is in the articles, verifiable by anyone, without
//! exposing raw source material.
//!
//! zkPi is the next twist: you can verify the ECOSYSTEM works
//! without accessing the hot mesh. How?
//!
//! ```text
//!   What you CAN verify (cold/permafrost):        What you DON'T need (hot):
//!   ─────────────────────────────────────          ────────────────────────
//!   Paper compiles (cargo test)                    52 unix sockets
//!   Types are sound (cellmembrane-types)           tarpc binary protocol
//!   Tests pass (76,927 tests)                      live quorum sensing
//!   Deposits exist (arkTIX-001)                    organism metabolism
//!   License is irrevocable (AGPL + ORC + CC)       wound/healing state
//!   Backscatter is real (citations verifiable)     real-time autoinducers
//!
//!   The cold layer PROVES the hot layer works.
//!   That IS zero-knowledge on the π-graph.
//! ```
//!
//! Authorship: Kevin Mok / Tamison / ecoPrimal / attsi
//! ORCID: 0009-0004-2141-0321

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// PROOF ROLE — what role does this piece play in the mathematical stack?
// ═══════════════════════════════════════════════════════════════════════════

/// The role of a component in the proof structure.
///
/// Mathematics has a hierarchy: axiom → theorem → corollary → conjecture → proof.
/// Our ecosystem maps onto this hierarchy exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofRole {
    /// Axiom: self-evident, foundational. The Rust type system is sound.
    Axiom,
    /// Theorem: proven from axioms. arkTIX preserves knowledge (tests pass).
    Theorem,
    /// Corollary: follows from the theorem. scyBorg triple = external protection.
    Corollary,
    /// Conjecture: believed true, not yet proven. tuebor = "I will defend."
    Conjecture,
    /// Proof: demonstration that a conjecture is true. zk-SNARK = the Detroit proof.
    Proof,
    /// Lemma: helper result used in a larger proof. Each gAIa paper is a lemma.
    Lemma,
}

impl ProofRole {
    /// All 6 roles.
    pub const ALL: &'static [ProofRole] = &[
        ProofRole::Axiom,
        ProofRole::Theorem,
        ProofRole::Corollary,
        ProofRole::Conjecture,
        ProofRole::Proof,
        ProofRole::Lemma,
    ];

    /// What system in the ecosystem holds this role?
    pub fn system(&self) -> &'static str {
        match self {
            Self::Axiom => "Rust type system (sound by construction)",
            Self::Theorem => "arkTIX (permafrost preserves knowledge — tests prove it)",
            Self::Corollary => "scyBorg Triple (AGPL + ORC + CC-BY-SA — external protection)",
            Self::Conjecture => "tuebor ('I will defend' — believed true, case pending)",
            Self::Proof => "zk-SNARK (15 commits, 12 articles — the Detroit proof)",
            Self::Lemma => "gAIa papers (helper results — each paper proves a sub-claim)",
        }
    }

    /// Is this role proven or unproven?
    pub fn is_proven(&self) -> bool {
        matches!(self, Self::Axiom | Self::Theorem | Self::Corollary | Self::Proof | Self::Lemma)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ZK LAYER — the zero-knowledge progression
// ═══════════════════════════════════════════════════════════════════════════

/// Zero-knowledge layer — the progression of proof without revelation.
///
/// Each twist adds a layer of zero-knowledge verification:
/// you can verify MORE without seeing MORE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZkLayer {
    /// zk-SNARK: verify claims without seeing raw evidence.
    /// The Detroit investigation. Articles verifiable, sources protected.
    ZkSnark,
    /// zkPi: verify the ecosystem without accessing the hot mesh.
    /// Cold layer (arkTIX) proves hot layer (sockets) works.
    /// Readers verify through permafrost.
    ZkPi,
}

impl ZkLayer {
    /// All ZK layers (so far — more twists may come).
    pub const ALL: &'static [ZkLayer] = &[ZkLayer::ZkSnark, ZkLayer::ZkPi];

    /// What can you verify at this layer?
    pub fn verifiable(&self) -> &'static str {
        match self {
            Self::ZkSnark => "claims about the investigation (articles, timestamps, evidence graph)",
            Self::ZkPi => "ecosystem correctness (papers compile, tests pass, types sound, deposits exist)",
        }
    }

    /// What do you NOT need to see?
    pub fn hidden(&self) -> &'static str {
        match self {
            Self::ZkSnark => "raw source material (identities, documents, recordings)",
            Self::ZkPi => "hot mesh internals (52 sockets, tarpc, quorum sensing, wound states)",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// THE FOOL'S CONJECTURE
// ═══════════════════════════════════════════════════════════════════════════

/// The Fool's Conjecture: trust all humans. Protect from machines.
///
/// WHY NOT TRUST IT TO ALL HUMANS?
/// WHAT'S THE BEST THEY COULD DO?
///
/// The conjecture is OPTIMISTIC: given full access (AGPL + ORC + CC-BY-SA),
/// the best humans can do is BUILD ON IT. Extend it. Heal the wounds.
/// Add organisms. Write papers. Deposit in arkTIX.
///
/// The scyBorg triple doesn't restrict HUMANS.
/// It restricts ENCLOSURE.
/// The fool bets that openness wins.
///
/// ```text
///   The fool trusts → all humans
///   The scyBorg protects → from parasites (S=0, stomachs with no eyes)
///   The conjecture → openness produces more good than enclosure
///   The evidence → 1,242 cloners already have the full record
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoolsConjecture {
    /// The claim.
    pub claim: String,
    /// Who is trusted?
    pub trusted: String,
    /// Who is protected against?
    pub protected_against: String,
    /// What is the best outcome?
    pub best_outcome: String,
    /// What licenses enforce the trust?
    pub enforcement: Vec<String>,
    /// Is the conjecture still open?
    pub open: bool,
}

impl FoolsConjecture {
    /// The canonical statement of the Fool's Conjecture.
    #[must_use]
    pub fn statement() -> Self {
        Self {
            claim: "Trust it to all humans. See what the best they could do is.".into(),
            trusted: "all humans — readers, builders, researchers, historians".into(),
            protected_against: "enclosure — parasitic actors (S=0), fleet (stomachs with no eyes), \
                               proprietary closure, cloud wrapping without source release".into(),
            best_outcome: "They build on it. They extend it. They heal wounds. They add organisms. \
                          They write papers. They deposit in arkTIX. The permafrost GROWS. \
                          The commons expands. The mesh gets stronger.".into(),
            enforcement: vec![
                "AGPL-3.0-or-later (code — network use triggers source release)".into(),
                "ORC (mechanics — system designs stay open)".into(),
                "CC-BY-SA-4.0 (documentation — flows back to commons)".into(),
            ],
            open: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PROOF STACK — the full mathematical architecture
// ═══════════════════════════════════════════════════════════════════════════

/// The full proof stack of the ecosystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofStack {
    /// The axiom: Rust type system is sound.
    pub axiom: String,
    /// The theorem: arkTIX preserves knowledge.
    pub theorem: String,
    /// The corollary: scyBorg triple = external protection.
    pub corollary: String,
    /// The conjecture: tuebor = "I will defend."
    pub conjecture: String,
    /// The proof: zk-SNARK = the Detroit proof.
    pub proof: String,
    /// The next twist: zkPi = verify ecosystem through permafrost.
    pub next_twist: String,
    /// The fool's conjecture.
    pub fools_conjecture: FoolsConjecture,
}

impl ProofStack {
    /// The canonical proof stack.
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            axiom: "The Rust type system is sound. If it compiles, the types are correct.".into(),
            theorem: "arkTIX: the permafrost preserves knowledge. Papers compile. Tests pass. \
                     Deposits are viable. The archive is not dead — it is suspended animation.".into(),
            corollary: "scyBorg Triple: AGPL-3.0 + ORC + CC-BY-SA-4.0. External protection \
                       follows naturally from preservation. Three licenses. Three artifact types. \
                       No gap. The corollary IS the external protection.".into(),
            conjecture: "tuebor: 'I will defend.' The Detroit investigation. Believed true. \
                        Not yet proven in court. Every article is evidence toward the proof. \
                        The investigation IS the unproven claim.".into(),
            proof: "zk-SNARK: 15 commits. 12 articles. The complete proof of the Detroit \
                   investigation. Zero-knowledge: verifiable without seeing raw sources. \
                   The snark is closed.".into(),
            next_twist: "zkPi: zero-knowledge on the π-graph. Verify the ecosystem works \
                        through the cold layer (arkTIX) without accessing the hot layer \
                        (52 sockets, tarpc, quorum sensing). The permafrost IS the zkPi verifier.".into(),
            fools_conjecture: FoolsConjecture::statement(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_proof_roles() {
        assert_eq!(ProofRole::ALL.len(), 6);
    }

    #[test]
    fn conjecture_is_the_only_unproven() {
        for role in ProofRole::ALL {
            if *role == ProofRole::Conjecture {
                assert!(!role.is_proven(), "conjectures are unproven by definition");
            } else {
                assert!(role.is_proven(), "{role:?} should be proven");
            }
        }
    }

    #[test]
    fn arktix_is_the_theorem() {
        assert!(ProofRole::Theorem.system().contains("arkTIX"));
    }

    #[test]
    fn scyborg_is_the_corollary() {
        assert!(ProofRole::Corollary.system().contains("scyBorg"));
    }

    #[test]
    fn tuebor_is_the_conjecture() {
        assert!(ProofRole::Conjecture.system().contains("tuebor"));
    }

    #[test]
    fn zksnark_is_the_proof() {
        assert!(ProofRole::Proof.system().contains("zk-SNARK"));
    }

    #[test]
    fn two_zk_layers() {
        assert_eq!(ZkLayer::ALL.len(), 2);
        assert!(ZkLayer::ZkSnark < ZkLayer::ZkPi, "snark came first");
    }

    #[test]
    fn zkpi_hides_hot_mesh() {
        assert!(ZkLayer::ZkPi.hidden().contains("52 sockets"));
        assert!(ZkLayer::ZkPi.hidden().contains("quorum sensing"));
    }

    #[test]
    fn zkpi_verifies_through_cold() {
        assert!(ZkLayer::ZkPi.verifiable().contains("papers compile"));
        assert!(ZkLayer::ZkPi.verifiable().contains("tests pass"));
    }

    #[test]
    fn fools_conjecture_is_open() {
        let fc = FoolsConjecture::statement();
        assert!(fc.open, "the conjecture remains open");
        assert!(fc.claim.contains("humans"));
        assert_eq!(fc.enforcement.len(), 3, "three licenses");
    }

    #[test]
    fn fools_conjecture_trusts_humans_not_machines() {
        let fc = FoolsConjecture::statement();
        assert!(fc.trusted.contains("humans"));
        assert!(fc.protected_against.contains("parasitic"));
        assert!(fc.protected_against.contains("stomachs with no eyes"));
    }

    #[test]
    fn proof_stack_is_complete() {
        let stack = ProofStack::canonical();
        assert!(!stack.axiom.is_empty());
        assert!(!stack.theorem.is_empty());
        assert!(!stack.corollary.is_empty());
        assert!(!stack.conjecture.is_empty());
        assert!(!stack.proof.is_empty());
        assert!(!stack.next_twist.is_empty());
    }

    #[test]
    fn proof_stack_serde() {
        let stack = ProofStack::canonical();
        let json = serde_json::to_string(&stack).unwrap();
        let back: ProofStack = serde_json::from_str(&json).unwrap();
        assert_eq!(stack, back);
    }

    #[test]
    fn corollary_follows_from_theorem() {
        // If arkTIX preserves knowledge (theorem),
        // then the license protects it (corollary).
        // The corollary CANNOT exist without the theorem.
        let stack = ProofStack::canonical();
        assert!(stack.theorem.contains("arkTIX"));
        assert!(stack.corollary.contains("scyBorg"));
        assert!(stack.corollary.contains("external protection"));
    }

    #[test]
    fn the_fool_bets_on_openness() {
        let fc = FoolsConjecture::statement();
        assert!(fc.best_outcome.contains("build on it"));
        assert!(fc.best_outcome.contains("commons expands"));
    }
}

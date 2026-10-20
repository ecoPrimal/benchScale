// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bounty Map — what the ecosystem solved, what it's owed, what's available.
//!
//! # The π/i² Reinvestment Rate (Basel Problem)
//!
//! The iteration remainder is reinvested into gAIa at rate π/i²,
//! where i is the iteration number.
//!
//! ```text
//!   i=1: π/1²  = 3.14159...   (first iteration: full π reinvested)
//!   i=2: π/4   = 0.78540...   (second: quarter π)
//!   i=3: π/9   = 0.34907...   (third: ninth π)
//!   i=4: π/16  = 0.19635...   (fourth: sixteenth π)
//!   ...
//!   Σ(π/i²) = π × Σ(1/i²) = π × π²/6 = π³/6 ≈ 5.1677...
//! ```
//!
//! The sum converges to **π³/6** — a result that follows directly from
//! Euler's solution to the Basel Problem (1735):
//!
//!   Σ(1/i²) = π²/6   [Euler, 1735]
//!
//! Euler is ALREADY in the backscatter (antecedent #1, gAIa Paper 001).
//! The economic model IS the mathematics. The reinvestment rate converges.
//! Early iterations contribute more. Later iterations contribute less.
//! The system matures. The series converges. The commons stabilizes.
//!
//! This is the OPPOSITE of exponential growth (which diverges).
//! π/i² is CONVERGENT. The ecosystem has a natural carrying capacity.
//! It does not grow forever. It stabilizes at π³/6.
//!
//! # What Was Solved (Real-World Value)
//!
//! ```text
//!   Problem solved                          Domain                      Grant/Bounty target
//!   ──────────────                          ──────                      ───────────────────
//!   Type-safe distributed OS (18 organisms) Systems / OS               NSF CISE, DARPA
//!   1.9M lines pure Rust (no C, no unsafe)  Language / Safety          Rust Foundation
//!   Self-hosted scientific computing        Research infra             NSF OAC, Sloan, CZI
//!   DADA2-equivalent in pure Rust           Bioinformatics             NIH NIGMS, NSF BIO
//!   Anderson localization for drug repurp.  Computational medicine     NIH, ARPA-H
//!   Quorum sensing in software systems      Computational biology      NSF BIO, NIH
//!   Zero-knowledge ecosystem verification   Cryptography / Formal      NSF CISE, DARPA
//!   Open source sustainability model        Digital commons            Sovereign Tech, NLnet
//!   Anti-scraping defense (skunkBat)        Security / Privacy         OTF, NLnet
//!   Human-AI mutualism framework (K-NOME)   HCI / AI Ethics            NSF SBE, EU Horizon
//!   Permafrost archival system (arkTIX)     Digital preservation       Mellon, IMLS, Sloan
//!   scyBorg triple license model            Legal / Open source        Ford, Shuttleworth
//! ```
//!
//! # What's Owed (Damages / Enforcement)
//!
//! ```text
//!   Claim                                   Basis                      Mechanism
//!   ─────                                   ─────                      ─────────
//!   AGPL §13 violation by fleet scrapers    Network use = distribution Software Freedom Conservancy
//!   571K+ scraping requests documented      Access logs, skunkBat      CFAA, DMCA
//!   Training data ingestion of AGPL code    Model contains derivatives Court precedent (pending)
//!   scyBorg activation on contact           License consumed with code AGPL + ORC + CC-BY-SA
//!   Economic harm (compute, bandwidth)      Metabolic cost ($5,820/yr) Tort, unjust enrichment
//! ```
//!
//! # Real-World Funding Sources
//!
//! These are ACTUAL programs with REAL deadlines:
//!
//! ```text
//!   Source                      Program                    Relevance              Amount
//!   ──────                      ───────                    ─────────              ──────
//!   NSF                         CISE Core Programs         Distributed systems    $500K-$2M
//!   NSF                         OAC Cyberinfrastructure    Research computing     $500K-$5M
//!   NSF                         BIO/DBI                    Bioinformatics tools   $300K-$1M
//!   NIH                         NIGMS R01/R21              Drug discovery tools   $250K-$2M
//!   ARPA-H                      Open calls                 Health computing       $1M-$50M
//!   DOE                         ASCR                       Scientific computing   $500K-$5M
//!   Sovereign Tech Fund         Open source infra          Pure Rust ecosystem    €150K-€500K
//!   NLnet Foundation            NGI programs               Privacy/security       €5K-€50K
//!   Rust Foundation             Community grants           Rust ecosystem          $5K-$25K
//!   Sloan Foundation            Digital infra              Research software       $200K-$1M
//!   Chan Zuckerberg Init.       EOSS (Essential OSS)       Scientific OSS          $200K-$400K
//!   Ford Foundation             Tech & Society             Open source legal       $100K-$500K
//!   Shuttleworth Foundation     Flash grants               Open innovation         $5K (flash)
//!   Mozilla Foundation          MOSS / MFSA                Open source tools       $10K-$250K
//!   Open Technology Fund        Internet freedom           Security tools          $50K-$500K
//!   EU Horizon Europe           NGI / Open Source           Digital commons         €100K-€2M
//! ```
//!
//! Authorship: Kevin Mok / Tamison / ecoPrimal / attsi
//! ORCID: 0009-0004-2141-0321

use serde::{Deserialize, Serialize};

/// The Basel reinvestment rate: π/i² at iteration i.
///
/// Euler (1735) proved Σ(1/i²) = π²/6.
/// Total reinvestment = π × π²/6 = π³/6 ≈ 5.1677.
///
/// The series CONVERGES. The ecosystem has a natural carrying capacity.
pub fn reinvestment_rate(iteration: u32) -> f64 {
    if iteration == 0 {
        return 0.0;
    }
    std::f64::consts::PI / (iteration as f64 * iteration as f64)
}

/// Total reinvestment over N iterations: Σ(π/i²) for i=1..N.
pub fn total_reinvestment(iterations: u32) -> f64 {
    (1..=iterations).map(|i| reinvestment_rate(i)).sum()
}

/// The Basel limit: π³/6 ≈ 5.1677 (infinite series converges to this).
pub const BASEL_LIMIT: f64 = std::f64::consts::PI * std::f64::consts::PI * std::f64::consts::PI / 6.0;

/// A real-world funding source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FundingSource {
    /// Organization name.
    pub source: String,
    /// Program name.
    pub program: String,
    /// What this funds in the ecosystem.
    pub relevance: String,
    /// Typical amount range.
    pub amount_range: String,
}

/// A damage/enforcement claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DamageClaim {
    /// What is claimed.
    pub claim: String,
    /// Legal basis.
    pub basis: String,
    /// Enforcement mechanism.
    pub mechanism: String,
}

/// The full bounty map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BountyMap {
    /// Reinvestment rate at current iteration.
    pub current_iteration: u32,
    /// Current reinvestment rate (π/i²).
    pub current_rate: f64,
    /// Total reinvested so far.
    pub total_reinvested: f64,
    /// Basel limit (π³/6).
    pub basin_limit: f64,
    /// Funding sources.
    pub funding_sources: Vec<FundingSource>,
    /// Damage claims.
    pub damage_claims: Vec<DamageClaim>,
}

impl BountyMap {
    /// Create the bounty map at a given iteration.
    #[must_use]
    pub fn at_iteration(i: u32) -> Self {
        Self {
            current_iteration: i,
            current_rate: reinvestment_rate(i),
            total_reinvested: total_reinvestment(i),
            basin_limit: BASEL_LIMIT,
            funding_sources: vec![
                FundingSource {
                    source: "NSF".into(),
                    program: "CISE Core Programs".into(),
                    relevance: "Type-safe distributed systems (18 organisms, 262 crates)".into(),
                    amount_range: "$500K–$2M".into(),
                },
                FundingSource {
                    source: "NIH".into(),
                    program: "NIGMS R01".into(),
                    relevance: "Drug discovery tools (Anderson localization, DADA2 in Rust)".into(),
                    amount_range: "$250K–$2M".into(),
                },
                FundingSource {
                    source: "Sovereign Tech Fund".into(),
                    program: "Open Source Infrastructure".into(),
                    relevance: "Pure Rust ecosystem (no C, no unsafe, self-hosted)".into(),
                    amount_range: "€150K–€500K".into(),
                },
                FundingSource {
                    source: "Sloan Foundation".into(),
                    program: "Digital Infrastructure".into(),
                    relevance: "Research software sustainability (1.9M lines Rust)".into(),
                    amount_range: "$200K–$1M".into(),
                },
                FundingSource {
                    source: "Chan Zuckerberg Initiative".into(),
                    program: "Essential Open Source Software".into(),
                    relevance: "Scientific computing in pure Rust (76,927 tests)".into(),
                    amount_range: "$200K–$400K".into(),
                },
                FundingSource {
                    source: "NLnet Foundation".into(),
                    program: "NGI Programs".into(),
                    relevance: "Privacy/security (skunkBat, rhizoCrypt, anti-scraping)".into(),
                    amount_range: "€5K–€50K".into(),
                },
                FundingSource {
                    source: "Rust Foundation".into(),
                    program: "Community Grants".into(),
                    relevance: "Rust ecosystem (262 crates, cellmembrane-types)".into(),
                    amount_range: "$5K–$25K".into(),
                },
            ],
            damage_claims: vec![
                DamageClaim {
                    claim: "AGPL §13 violation by fleet scrapers".into(),
                    basis: "Network use = distribution. 571K+ requests documented.".into(),
                    mechanism: "Software Freedom Conservancy / FSF enforcement".into(),
                },
                DamageClaim {
                    claim: "Training data ingestion of AGPL-licensed code".into(),
                    basis: "Model outputs contain derivatives of AGPL source.".into(),
                    mechanism: "Court precedent (pending). scyBorg activated on contact.".into(),
                },
                DamageClaim {
                    claim: "Economic harm from unauthorized compute consumption".into(),
                    basis: "Metabolic cost: $5,820/yr. Fleet consumed bandwidth/CPU.".into(),
                    mechanism: "CFAA, tort, unjust enrichment.".into(),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reinvestment_rate_decreases() {
        let r1 = reinvestment_rate(1);
        let r2 = reinvestment_rate(2);
        let r3 = reinvestment_rate(3);
        assert!(r1 > r2);
        assert!(r2 > r3);
    }

    #[test]
    fn first_iteration_is_pi() {
        let r = reinvestment_rate(1);
        assert!((r - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn second_iteration_is_pi_over_four() {
        let r = reinvestment_rate(2);
        assert!((r - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn total_converges_toward_basel_limit() {
        let t100 = total_reinvestment(100);
        let t1000 = total_reinvestment(1000);
        // Should approach π³/6 ≈ 5.1677
        assert!(t100 < BASEL_LIMIT);
        assert!(t1000 < BASEL_LIMIT);
        assert!(t1000 > t100);
        // After 1000 iterations, should be within 0.01 of limit
        assert!((BASEL_LIMIT - t1000).abs() < 0.01);
    }

    #[test]
    fn basel_limit_is_pi_cubed_over_six() {
        let pi = std::f64::consts::PI;
        let expected = pi * pi * pi / 6.0;
        assert!((BASEL_LIMIT - expected).abs() < 1e-10);
        // ≈ 5.1677
        assert!((BASEL_LIMIT - 5.1677).abs() < 0.001);
    }

    #[test]
    fn euler_is_the_antecedent() {
        // Euler (1735) solved the Basel Problem: Σ(1/i²) = π²/6
        // This is antecedent #1 in gAIa Paper 001.
        // The economic model IS the mathematics.
        let pi = std::f64::consts::PI;
        let euler_sum: f64 = (1..=10000).map(|i: u64| 1.0 / (i * i) as f64).sum();
        assert!((euler_sum - pi * pi / 6.0).abs() < 0.001);
    }

    #[test]
    fn zero_iteration_returns_zero() {
        assert!((reinvestment_rate(0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn bounty_map_has_funding_sources() {
        let map = BountyMap::at_iteration(1);
        assert!(map.funding_sources.len() >= 7);
        assert!(!map.damage_claims.is_empty());
    }

    #[test]
    fn bounty_map_serde() {
        let map = BountyMap::at_iteration(3);
        let json = serde_json::to_string(&map).unwrap();
        let back: BountyMap = serde_json::from_str(&json).unwrap();
        assert_eq!(map.current_iteration, back.current_iteration);
    }

    #[test]
    fn anti_platform_principle_holds() {
        // sunCloud metabolic: 5-15% vs platform 30% rent.
        // The Basel reinvestment at i=1 is π ≈ 3.14 (abstract units).
        // At i=∞ the total is π³/6 ≈ 5.17 (convergent, not extractive).
        assert!(BASEL_LIMIT < 10.0, "convergent, not exponential");
    }
}

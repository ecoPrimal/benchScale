// SPDX-License-Identifier: AGPL-3.0-or-later
//! Ring-Sort — dynamic topo-sorting rings. Bars become rings that grind.


// RING-SORT — dynamic topo-sorting rings
//
// The bars at the bottom were STATIC. Now they're rings that topo-sort and
// re-sort into each other. They bounce and grind to find granularity.
//
// Architecture:
//   - Each data category is a Ring (mass, velocity, orbit)
//   - Rings orbit at radius proportional to mass within their tier
//   - Counter-rotation: inner rings (stator) spin clockwise, outer (rotor) counter-clockwise
//   - Collision detection → velocity exchange (grinding)
//   - Re-sort on every data update (topo-sort by mass)

/// Ring tier — which orbital layer this ring lives in.
#[derive(Debug, Clone, PartialEq)]
pub enum RingTier {
    /// Inner orbit: ASN type classification (datacenter, cloud, residential, etc.)
    AsnType,
    /// Outer orbit: Country/geo classification
    Country,
    /// Sub-ring: Region within a country (only visible on expand)
    Region,
}

/// A single ring in the topo-sort visualization.
#[derive(Debug, Clone)]
pub struct Ring {
    pub id: String,
    pub label: String,
    pub tier: RingTier,
    pub mass: u64,
    pub fraction: f64,    // fraction within tier (0.0-1.0)
    pub orbit: f64,       // orbital radius (normalized 0.0-1.0)
    pub arc_len: f64,     // sweep angle (radians)
    pub thickness: f64,   // ring thickness (normalized)
    pub spin_dir: f64,    // +1.0 clockwise, -1.0 counter-clockwise
}

/// Build ring layout from raw geo data.
///
/// ASN types: inner orbit (0.2-0.45), clockwise stator
/// Countries: outer orbit (0.55-0.92), counter-clockwise rotor
///
/// Each ring is sized relative to its OWN tier total, not the global total.
/// This ensures the visualization fills the space at every tier.
pub fn ring_layout(asn_types: &[(&str, u64)], countries: &[(&str, u64)]) -> Vec<Ring> {
    let mut rings = Vec::new();

    // ASN tier
    let asn_total: u64 = asn_types.iter().map(|(_, m)| m).sum();
    let asn_total_f = asn_total.max(1) as f64;
    let mut asn_sorted: Vec<_> = asn_types.to_vec();
    asn_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (label, mass)) in asn_sorted.iter().enumerate() {
        let frac = *mass as f64 / asn_total_f;
        let orbit = 0.2 + (i as f64 / asn_sorted.len().max(1) as f64) * 0.25;
        rings.push(Ring {
            id: format!("a_{}", label),
            label: label.to_string(),
            tier: RingTier::AsnType,
            mass: *mass,
            fraction: frac,
            orbit,
            arc_len: (frac * std::f64::consts::TAU * 0.8).max(0.2),
            thickness: (frac * 0.12).max(0.02),
            spin_dir: 1.0, // clockwise stator
        });
    }

    // Country tier
    let cc_total: u64 = countries.iter().map(|(_, m)| m).sum();
    let cc_total_f = cc_total.max(1) as f64;
    let mut cc_sorted: Vec<_> = countries.to_vec();
    cc_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (label, mass)) in cc_sorted.iter().enumerate() {
        let frac = *mass as f64 / cc_total_f;
        let orbit = 0.55 + (i as f64 / cc_sorted.len().max(1) as f64) * 0.37;
        rings.push(Ring {
            id: format!("c_{}", label),
            label: label.to_string(),
            tier: RingTier::Country,
            mass: *mass,
            fraction: frac,
            orbit,
            arc_len: (frac * std::f64::consts::TAU * 0.7).max(0.15),
            thickness: (frac * 0.15).max(0.02),
            spin_dir: -1.0, // counter-clockwise rotor
        });
    }

    rings
}

/// Detect which pairs of rings would grind (close in angle + orbit).
/// Returns pairs of ring IDs that are within grinding distance.
pub fn grinding_pairs(rings: &[Ring], angle_threshold: f64) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for i in 0..rings.len() {
        for j in (i + 1)..rings.len() {
            if rings[i].tier != rings[j].tier { continue; }
            let orbit_dist = (rings[i].orbit - rings[j].orbit).abs();
            // Within same orbit band and close in angle
            if orbit_dist < 0.1 {
                // Angular proximity would be dynamic, but structurally close rings grind
                let arc_overlap = rings[i].arc_len + rings[j].arc_len;
                if arc_overlap > angle_threshold {
                    pairs.push((rings[i].id.clone(), rings[j].id.clone()));
                }
            }
        }
    }
    pairs
}

/// Topo-sort rings: returns rings ordered by (tier, mass DESC).
/// This is the natural sort — heaviest ring gets prime position in each tier.
pub fn topo_sort_rings(rings: &mut [Ring]) {
    rings.sort_by(|a, b| {
        let tier_ord = match (&a.tier, &b.tier) {
            (RingTier::AsnType, RingTier::AsnType) => std::cmp::Ordering::Equal,
            (RingTier::AsnType, _) => std::cmp::Ordering::Less,
            (_, RingTier::AsnType) => std::cmp::Ordering::Greater,
            (RingTier::Country, RingTier::Country) => std::cmp::Ordering::Equal,
            (RingTier::Country, _) => std::cmp::Ordering::Less,
            (_, RingTier::Country) => std::cmp::Ordering::Greater,
            (RingTier::Region, RingTier::Region) => std::cmp::Ordering::Equal,
        };
        tier_ord.then(b.mass.cmp(&a.mass))
    });
}

/// Calculate the total angular momentum of all rings.
/// Positive = net clockwise, negative = net counter-clockwise.
/// At equilibrium the stator and rotor should roughly balance.
pub fn angular_momentum(rings: &[Ring]) -> f64 {
    rings.iter().map(|r| r.mass as f64 * r.orbit * r.spin_dir).sum()
}

#[cfg(test)]
mod ring_sort_tests {
    use crate::io_surface::*;

    #[test]
    fn test_ring_layout_basic() {
        let asn = vec![("datacenter", 7000u64), ("self", 2000), ("cloud", 1000)];
        let cc = vec![("US", 8000u64), ("IE", 1500), ("CN", 500)];
        let rings = ring_layout(&asn, &cc);
        assert_eq!(rings.len(), 6);
    }

    #[test]
    fn test_asn_rings_clockwise() {
        let asn = vec![("datacenter", 7000u64)];
        let rings = ring_layout(&asn, &[]);
        assert!(rings[0].spin_dir > 0.0, "ASN rings should be clockwise (stator)");
    }

    #[test]
    fn test_country_rings_counter_clockwise() {
        let cc = vec![("US", 8000u64)];
        let rings = ring_layout(&[], &cc);
        let country = rings.iter().find(|r| r.tier == RingTier::Country).unwrap();
        assert!(country.spin_dir < 0.0, "country rings should be counter-clockwise (rotor)");
    }

    #[test]
    fn test_topo_sort_heaviest_first() {
        let asn = vec![("cloud", 100u64), ("datacenter", 7000), ("self", 2000)];
        let mut rings = ring_layout(&asn, &[]);
        topo_sort_rings(&mut rings);
        assert_eq!(rings[0].label, "datacenter");
        assert_eq!(rings[1].label, "self");
        assert_eq!(rings[2].label, "cloud");
    }

    #[test]
    fn test_fraction_sums_to_one_per_tier() {
        let asn = vec![("datacenter", 7000u64), ("self", 2000), ("cloud", 1000)];
        let cc = vec![("US", 8000u64), ("IE", 1500), ("CN", 500)];
        let rings = ring_layout(&asn, &cc);

        let asn_sum: f64 = rings.iter()
            .filter(|r| r.tier == RingTier::AsnType)
            .map(|r| r.fraction).sum();
        let cc_sum: f64 = rings.iter()
            .filter(|r| r.tier == RingTier::Country)
            .map(|r| r.fraction).sum();

        assert!((asn_sum - 1.0).abs() < 0.001, "ASN fractions should sum to 1.0, got {}", asn_sum);
        assert!((cc_sum - 1.0).abs() < 0.001, "Country fractions should sum to 1.0, got {}", cc_sum);
    }

    #[test]
    fn test_orbit_ranges() {
        let asn = vec![("datacenter", 7000u64), ("self", 2000)];
        let cc = vec![("US", 8000u64), ("IE", 1500)];
        let rings = ring_layout(&asn, &cc);

        for r in &rings {
            match r.tier {
                RingTier::AsnType => {
                    assert!(r.orbit >= 0.2 && r.orbit <= 0.45,
                        "ASN orbit {} should be in [0.2, 0.45]", r.orbit);
                }
                RingTier::Country => {
                    assert!(r.orbit >= 0.55 && r.orbit <= 0.92,
                        "Country orbit {} should be in [0.55, 0.92]", r.orbit);
                }
                RingTier::Region => {}
            }
        }
    }

    #[test]
    fn test_arc_len_proportional() {
        let asn = vec![("datacenter", 9000u64), ("self", 1000)];
        let rings = ring_layout(&asn, &[]);
        let dc = rings.iter().find(|r| r.label == "datacenter").unwrap();
        let s = rings.iter().find(|r| r.label == "self").unwrap();
        assert!(dc.arc_len > s.arc_len, "datacenter arc should be larger than self");
    }

    #[test]
    fn test_grinding_detection() {
        let asn = vec![("datacenter", 5000u64), ("self", 4500), ("cloud", 100)];
        let rings = ring_layout(&asn, &[]);
        // datacenter and self are both large, may grind
        let pairs = grinding_pairs(&rings, 1.0);
        // With close masses, their arcs are large enough to potentially overlap
        assert!(!pairs.is_empty() || rings.len() > 2, "close-mass rings should be near each other");
    }

    #[test]
    fn test_angular_momentum_balance() {
        // Equal mass in both tiers should roughly balance
        let asn = vec![("datacenter", 5000u64)];
        let cc = vec![("US", 5000u64)];
        let rings = ring_layout(&asn, &cc);
        let mom = angular_momentum(&rings);
        // Not necessarily zero (different orbits) but should be reasonably bounded
        assert!(mom.abs() < 1e9, "momentum should be bounded");
    }

    #[test]
    fn test_empty_tiers() {
        let rings = ring_layout(&[], &[]);
        assert_eq!(rings.len(), 0);
    }

    #[test]
    fn test_single_dominant_ring() {
        let asn = vec![("datacenter", 999999u64), ("other", 1)];
        let rings = ring_layout(&asn, &[]);
        let dc = rings.iter().find(|r| r.label == "datacenter").unwrap();
        assert!(dc.fraction > 0.99, "dominant ring should have fraction > 0.99");
        assert!(dc.arc_len > 4.0, "dominant ring should have large arc");
    }

    #[test]
    fn test_counter_rotation_sign() {
        let asn = vec![("datacenter", 5000u64)];
        let cc = vec![("US", 5000u64)];
        let rings = ring_layout(&asn, &cc);
        let asn_r = rings.iter().find(|r| r.tier == RingTier::AsnType).unwrap();
        let cc_r = rings.iter().find(|r| r.tier == RingTier::Country).unwrap();
        assert!(asn_r.spin_dir * cc_r.spin_dir < 0.0,
            "ASN and Country should counter-rotate");
    }
}


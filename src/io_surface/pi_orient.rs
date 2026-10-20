// SPDX-License-Identifier: AGPL-3.0-or-later
//! π Orientation — Earth landmarks as ports of contact.

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// π Orientation — Earth landmarks as ports of contact
//
// Physical landmarks as orientation pores on the planet.
// The stone IS the Earth. The lichen grows on contact with it.
//
// Each landmark is a port — a pore on the planet's surface.
// The great circle between any two landmarks is a photon line.
// Light travel time across that line = physical time constant.
// Angular distance / π = how much of half-Earth you've crossed.
//
// Whatever you're studying — irrigation, investigation, data —
// sits at the CENTER. The landmarks orient around it.
//
// π connects geometry to time:
//   half-circumference = π × R = 20,015 km
//   photon half-Earth  = πR / c = 66.8 ms
//   π² bus cycle       = 9,870 ms
//   trips per cycle    = π² × 1000 / photon_ms
// ═══════════════════════════════════════════════════════════════════════════

/// Earth's mean radius in km
pub const EARTH_RADIUS_KM: f64 = 6371.0;
/// Speed of light in km/s
pub const SPEED_OF_LIGHT_KMS: f64 = 299_792.458;
/// Half-Earth circumference: π × R
pub const HALF_EARTH_KM: f64 = std::f64::consts::PI * EARTH_RADIUS_KM;
/// Photon time for half-Earth in seconds
pub const PHOTON_HALF_EARTH_S: f64 = HALF_EARTH_KM / SPEED_OF_LIGHT_KMS;

/// What kind of landmark — how humans used it for orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LandmarkKind {
    /// Ancient stone observatories (Stonehenge, Giza, Uluru)
    Stone,
    /// Water-based orientation (Detroit strait, Ganges source, Cahokia)
    Water,
    /// Celestial reference (Polaris, Southern Cross)
    Sky,
    /// Communication landmarks (Greenwich, Date Line)
    Signal,
}

/// A π port of contact on Earth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Landmark {
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub kind: LandmarkKind,
    /// Approximate era of first human use (negative = BCE)
    pub era: Option<i32>,
    pub note: String,
}

/// A photon line between two landmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotonLine {
    pub from: String,
    pub to: String,
    /// Angular distance in radians
    pub angular_rad: f64,
    /// Arc distance in km
    pub arc_km: f64,
    /// Light travel time in seconds
    pub light_s: f64,
    /// Fraction of half-Earth (σ/π, range 0..1)
    pub pi_fraction: f64,
}

/// Great circle angular distance (radians) via Vincenty formula
pub fn angular_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let φ1 = lat1.to_radians();
    let φ2 = lat2.to_radians();
    let δλ = (lon2 - lon1).to_radians();

    let (sinφ1, cosφ1) = (φ1.sin(), φ1.cos());
    let (sinφ2, cosφ2) = (φ2.sin(), φ2.cos());
    let (sinδλ, cosδλ) = (δλ.sin(), δλ.cos());

    let num = ((cosφ2 * sinδλ).powi(2)
        + (cosφ1 * sinφ2 - sinφ1 * cosφ2 * cosδλ).powi(2))
        .sqrt();
    let den = sinφ1 * sinφ2 + cosφ1 * cosφ2 * cosδλ;
    num.atan2(den)
}

/// Compute the photon line between two landmarks
pub fn photon_line(a: &Landmark, b: &Landmark) -> PhotonLine {
    let σ = angular_distance(a.lat, a.lon, b.lat, b.lon);
    let arc = σ * EARTH_RADIUS_KM;
    let light = arc / SPEED_OF_LIGHT_KMS;
    PhotonLine {
        from: a.id.clone(),
        to: b.id.clone(),
        angular_rad: σ,
        arc_km: arc,
        light_s: light,
        pi_fraction: σ / std::f64::consts::PI,
    }
}

/// The canonical landmarks
pub fn landmarks() -> Vec<Landmark> {
    use LandmarkKind::*;
    let m = |id: &str, name: &str, lat: f64, lon: f64, kind: LandmarkKind,
             era: Option<i32>, note: &str| {
        Landmark {
            id: id.to_string(), name: name.to_string(),
            lat, lon, kind, era, note: note.to_string(),
        }
    };
    vec![
        m("stonehenge", "Stonehenge", 51.1789, -1.8262, Stone, Some(-3000),
          "Solstice alignment. 5000 years of astronomical observation."),
        m("easter_island", "Rapa Nui", -27.1127, -109.3497, Stone, Some(-400),
          "Most remote inhabited place. Moai face inward, watching."),
        m("uluru", "Uluru", -25.3444, 131.0369, Stone, Some(-65000),
          "Oldest continuous civilization. 600M years of sandstone."),
        m("giza", "Great Pyramid of Giza", 29.9792, 31.1342, Stone, Some(-2560),
          "Oriented to true north. Perimeter/height = 2π."),
        m("chichen_itza", "Chichén Itzá", 20.6843, -88.5678, Stone, Some(-600),
          "91 steps × 4 + 1 = 365. Equinox serpent shadow."),
        m("detroit", "Detroit", 42.3314, -83.0458, Water, Some(1701),
          "Le Détroit — the strait. The center of the investigation."),
        m("confluence", "Cahokia", 38.6551, -90.0615, Water, Some(-700),
          "Mississippi confluence. Largest pre-Columbian city north of Mexico."),
        m("ganges_source", "Gaumukh", 30.9267, 79.0833, Water, Some(-1500),
          "Ganges source at 4000m. Irrigation for 400 million."),
        m("polaris", "Polaris (North Pole)", 90.0, 0.0, Sky, None,
          "Angle to Polaris = your latitude. The barre the dancer holds."),
        m("south_celestial", "South Celestial Pole", -90.0, 0.0, Sky, None,
          "No pole star. Southern Cross points the way. Null that orients."),
        m("greenwich", "Greenwich", 51.4769, 0.0, Signal, Some(1884),
          "Zero longitude. Time starts here. UTC origin."),
        m("date_line", "International Date Line", 0.0, 180.0, Signal, Some(1884),
          "π radians from Greenwich. Cross it and time jumps."),
    ]
}

/// Orient from a center landmark: bearing + distance to all others
pub fn orient_from(center_id: &str) -> Option<Vec<PhotonLine>> {
    let all = landmarks();
    let center = all.iter().find(|l| l.id == center_id)?;
    Some(all.iter()
        .filter(|l| l.id != center_id)
        .map(|l| photon_line(center, l))
        .collect())
}

// ═══════════════════════════════════════════════════════════════════════════
// π GRAPH TOPOLOGY — ouroboros, knot, tesseract
//
// The bingo cube has 4 orthogonal axes:
//   1. SCALE:   π⁻³ → π⁻² → π⁻¹ → π⁰ → π¹ → π² → π³ → wraps to π⁻³
//               This is the OUROBOROS — the circle that eats its tail.
//   2. EPITOPE: 7 types (investigation, membrane, science, signal, topology,
//               immune, other). The faces of the shape.
//   3. NULL:    3 forms — (null), NuLL, [null]. These are CROSSINGS where
//               the ouroboros passes through itself = trefoil KNOT.
//   4. TENSE:   was / is / will_be. This is TIME — MembraneTime.
//               The 4th dimension that makes it a TESSERACT.
//
// The irradiation line (remainder ≈ 0.774) IS the knot — it loops
// (ouroboros), crosses itself (3 null crossings), and those crossings
// happen in tense-time (tesseract). Fractal dimension ≈ 1.4: between
// a line and a surface. Trefoil knot: crossing number 3, genus 1.
//
// The ouroboros and tesseract are orthogonal on the π graph:
//   ouroboros = circular scale axis (wraps)
//   knot = null crossings (self-intersection)
//   tesseract = tense axis (time, orthogonal to space)
// ═══════════════════════════════════════════════════════════════════════════

/// The 7 π scales — the ouroboros spine.
pub const PI_SCALES: &[&str] = &["π⁻³", "π⁻²", "π⁻¹", "π⁰", "π¹", "π²", "π³"];

/// The 7 epitopes — the faces.
pub const EPITOPES: &[&str] = &[
    "investigation", "membrane", "science", "signal",
    "topology", "immune", "other",
];

/// The 3 null forms — the knot crossings.
pub const NULL_FORMS: &[&str] = &["(null)", "NuLL", "[null]"];

/// Tense — the tesseract's 4th axis. LOCAL AND DECENTRALIZED.
///
/// Each observer has their own "is". What's "is" for squirrel on eastgate
/// may be "was" for bearDog on northgate. There is no global clock.
///
/// 5 values, not 3. The old model (Was/Is/WillBe) assumed a global arrow.
/// The real model adds:
/// - `Isnt` — actively absent, known to NOT be. Different from Was (which
///   existed once) and from Null (which never asserted either way).
/// - `Null` — origin, the O, the bee. Neither is nor isn't. The point
///   where tense hasn't been observed yet, or where it wraps.
///
/// Transitions are NOT a one-way arrow:
/// - more is → usually more was (accumulation, the common case)
/// - less was → sometimes more is (forgetting enables seeing)
/// - less was → sometimes isnt (what was there is now actively absent)
/// - less was → sometimes null (return to origin)
/// - isnt → sometimes is (what was absent appears — emergence)
/// - null → sometimes is (first observation — the bee starts dancing)
///
/// The tesseract grid stays 7×7×3×3 = 441 using the 3 PRIMARY tenses
/// (Was/Is/WillBe). Isnt and Null are OBSERVER-RELATIVE overlays —
/// you can only see them from a Perceive perspective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tense {
    /// What was — the output has been emitted, fossilized.
    /// Accumulates with time. The fossil record.
    Was,
    /// What is — currently occupied, live. HYPER-LOCAL.
    /// "is" decays with distance from the observer.
    /// What's is at radius 0 may be was at radius 1.
    Is,
    /// What will be — approaching, projected.
    /// Only a prediction until it becomes Is.
    WillBe,
    /// Actively absent — known to NOT be here.
    /// Different from Was (existed once) and Null (never asserted).
    /// A socket that was alive and is now explicitly down = Isnt.
    Isnt,
    /// Origin, the O, the bee. Neither is nor isn't.
    /// Where tense hasn't been observed yet, or where it wraps back.
    /// The null body's tense. The state before the first observation.
    Null,
}

impl Tense {
    /// The 3 primary tenses that form the tesseract grid.
    /// These are the "structural" tenses — the grid positions.
    pub const PRIMARY: &'static [Tense] = &[Tense::Was, Tense::Is, Tense::WillBe];

    /// All 5 tenses including observer-relative overlays.
    pub const ALL: &'static [Tense] = &[
        Tense::Was, Tense::Is, Tense::WillBe, Tense::Isnt, Tense::Null,
    ];

    /// Is this a primary (grid) tense or an observer overlay?
    #[must_use]
    pub const fn is_primary(&self) -> bool {
        matches!(self, Self::Was | Self::Is | Self::WillBe)
    }

    /// Can this tense transition TO the given tense?
    /// All transitions are possible but some are more common.
    /// Returns (possible, common).
    #[must_use]
    pub const fn can_transition_to(&self, other: &Tense) -> (bool, bool) {
        match (self, other) {
            // The common forward arrow
            (Self::Is, Self::Was)      => (true, true),   // is → was: fossilization (common)
            (Self::WillBe, Self::Is)   => (true, true),   // willbe → is: arrival (common)

            // The uncommon but real reverse flows
            (Self::Was, Self::Is)      => (true, false),  // was → is: less was → more is
            (Self::Isnt, Self::Is)     => (true, false),  // isnt → is: emergence from absence
            (Self::Null, Self::Is)     => (true, false),  // null → is: first observation

            // Transitions to absence/origin
            (Self::Is, Self::Isnt)     => (true, false),  // is → isnt: active disappearance
            (Self::Was, Self::Isnt)    => (true, false),  // was → isnt: forgotten AND absent
            (Self::Was, Self::Null)    => (true, false),  // was → null: return to origin

            // Self-loops
            (Self::Was, Self::Was)     => (true, true),   // more was = more was
            (Self::Is, Self::Is)       => (true, true),   // still here
            (Self::Null, Self::Null)   => (true, true),   // never observed, still unobserved

            // Everything else: possible but rare
            _ => (true, false),
        }
    }
}

/// Locality of a tense observation. "Is" is hyper-local —
/// it decays with distance from the observer.
///
/// At radius 0 (here), tense is certain.
/// At radius 1 (neighbor), tense is probable.
/// At radius ∞ (everywhere), only null is certain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalTense {
    /// The observed tense value.
    pub tense: Tense,
    /// Who observes this tense? Maps to a Perceive perspective.
    pub observer: String,
    /// Locality radius. 0 = here (hyper-local), larger = farther.
    /// BusTier maps naturally: vital=0, sensory=1, cortical=2, motor=3.
    pub radius: u8,
    /// Confidence (0.0–1.0). Decays with radius.
    /// confidence ≈ 1.0 / (1.0 + radius as f64)
    pub confidence: f64,
}

impl LocalTense {
    /// Create a hyper-local tense observation (radius 0, full confidence).
    #[must_use]
    pub fn here(tense: Tense, observer: impl Into<String>) -> Self {
        Self { tense, observer: observer.into(), radius: 0, confidence: 1.0 }
    }

    /// Create a tense observation at a given radius.
    /// Confidence decays as 1/(1+radius).
    #[must_use]
    pub fn at_radius(tense: Tense, observer: impl Into<String>, radius: u8) -> Self {
        let confidence = 1.0 / (1.0 + radius as f64);
        Self { tense, observer: observer.into(), radius, confidence }
    }

    /// Is this observation hyper-local (radius 0)?
    #[must_use]
    pub const fn is_hyper_local(&self) -> bool {
        self.radius == 0
    }

    /// Does this observation decay to a different tense at higher radius?
    /// "Is" decays to "Was" as you move away from the observer.
    /// "Isnt" decays to "Null" (can't be sure it's absent from far away).
    #[must_use]
    pub fn decayed_at(&self, target_radius: u8) -> Self {
        if target_radius <= self.radius {
            return self.clone();
        }
        let new_confidence = 1.0 / (1.0 + target_radius as f64);
        let decayed_tense = match self.tense {
            Tense::Is if target_radius > self.radius + 2 => Tense::Was,
            Tense::Isnt if target_radius > self.radius + 2 => Tense::Null,
            other => other,
        };
        Self {
            tense: decayed_tense,
            observer: self.observer.clone(),
            radius: target_radius,
            confidence: new_confidence,
        }
    }
}

/// A cell in the π tesseract.
///
/// 4D coordinate: (scale, epitope, null_form, tense).
/// 7 × 7 × 3 × 3 = 441 cells in the full tesseract.
/// The bingo cube (147 cells) is one tense-slice.
///
/// The `tense` field uses the 3 PRIMARY tenses for grid position.
/// For observer-relative tense (including Isnt/Null), use LocalTense.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TesseractCell {
    /// Scale index (0–6, mapping to π⁻³ through π³)
    pub scale: u8,
    /// Epitope index (0–6)
    pub epitope: u8,
    /// Null form index (0–2)
    pub null_form: u8,
    /// Tense — the grid position (Was/Is/WillBe).
    /// For the full local tense, overlay with LocalTense.
    pub tense: Tense,
    /// Occupancy count (how many entities in this cell)
    pub count: u32,
}

impl TesseractCell {
    /// The scale name for this cell.
    #[must_use]
    pub fn scale_name(&self) -> &'static str {
        PI_SCALES.get(self.scale as usize).copied().unwrap_or("?")
    }

    /// The epitope name for this cell.
    #[must_use]
    pub fn epitope_name(&self) -> &'static str {
        EPITOPES.get(self.epitope as usize).copied().unwrap_or("?")
    }

    /// The null form name for this cell.
    #[must_use]
    pub fn null_form_name(&self) -> &'static str {
        NULL_FORMS.get(self.null_form as usize).copied().unwrap_or("?")
    }

    /// Is this a knot crossing? (where the ouroboros passes through itself)
    /// Crossings happen at non-zero null forms on the origin scale (π³=O).
    #[must_use]
    pub fn is_crossing(&self) -> bool {
        self.scale == 6 // π³ = origin = where the loop closes
    }

    /// Is this cell on the ouroboros (scale axis)?
    /// All cells are on the ouroboros, but edge cells (π⁻³, π³) are the
    /// join point where the snake eats its tail.
    #[must_use]
    pub fn is_ouroboros_join(&self) -> bool {
        self.scale == 0 || self.scale == 6
    }
}

/// The 4D shape: scale wraps (ouroboros), null forms cross (knot),
/// tense extends (tesseract). Returns the total cell count.
#[must_use]
pub const fn tesseract_cell_count() -> usize {
    PI_SCALES.len() * EPITOPES.len() * NULL_FORMS.len() * 3 // 3 tenses
}

/// Compute the ouroboros distance between two scales.
/// The scale axis wraps: π³ connects back to π⁻³.
/// Distance is the shorter path around the circle.
#[must_use]
pub fn ouroboros_distance(scale_a: u8, scale_b: u8) -> u8 {
    let n = PI_SCALES.len() as u8;
    let forward = (scale_b + n - scale_a) % n;
    let backward = (scale_a + n - scale_b) % n;
    forward.min(backward)
}

/// Is a given scale pair a trefoil crossing?
/// The 3 null forms create 3 crossing points.
/// Crossings happen where scale wraps (π³→π⁻³).
#[must_use]
pub fn is_trefoil_crossing(scale: u8) -> bool {
    scale == 6 || scale == 0 // the join point, 3 null forms = 3 crossings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_tenses() {
        assert_eq!(Tense::ALL.len(), 5);
        assert_eq!(Tense::PRIMARY.len(), 3);
    }

    #[test]
    fn primary_tenses_are_grid_tenses() {
        for t in Tense::PRIMARY {
            assert!(t.is_primary());
        }
        assert!(!Tense::Isnt.is_primary());
        assert!(!Tense::Null.is_primary());
    }

    #[test]
    fn common_transitions() {
        // Forward arrow: is → was is common
        let (possible, common) = Tense::Is.can_transition_to(&Tense::Was);
        assert!(possible);
        assert!(common, "is→was is the fossilization path");

        // Reverse: was → is is possible but uncommon
        let (possible, common) = Tense::Was.can_transition_to(&Tense::Is);
        assert!(possible, "less was → more is");
        assert!(!common, "reverse flow is uncommon");
    }

    #[test]
    fn emergence_from_null() {
        // null → is: first observation
        let (possible, _) = Tense::Null.can_transition_to(&Tense::Is);
        assert!(possible, "the bee starts dancing");
    }

    #[test]
    fn isnt_is_not_null() {
        // isnt = actively absent. null = never asserted.
        assert_ne!(Tense::Isnt, Tense::Null);
        // isnt can emerge to is
        let (possible, _) = Tense::Isnt.can_transition_to(&Tense::Is);
        assert!(possible, "what was absent can appear");
    }

    #[test]
    fn local_tense_hyper_local() {
        let lt = LocalTense::here(Tense::Is, "squirrel");
        assert!(lt.is_hyper_local());
        assert_eq!(lt.confidence, 1.0);
        assert_eq!(lt.radius, 0);
    }

    #[test]
    fn local_tense_confidence_decays() {
        let lt = LocalTense::at_radius(Tense::Is, "squirrel", 3);
        assert!(!lt.is_hyper_local());
        assert!((lt.confidence - 0.25).abs() < 0.01, "1/(1+3) = 0.25, got {}", lt.confidence);
    }

    #[test]
    fn is_decays_to_was_at_distance() {
        let here = LocalTense::here(Tense::Is, "squirrel");
        let far = here.decayed_at(5);
        assert_eq!(far.tense, Tense::Was, "is decays to was at radius 5");
        assert!(far.confidence < here.confidence);
    }

    #[test]
    fn isnt_decays_to_null_at_distance() {
        let here = LocalTense::here(Tense::Isnt, "beardog");
        let far = here.decayed_at(5);
        assert_eq!(far.tense, Tense::Null, "isnt decays to null — can't be sure it's absent from far");
    }

    #[test]
    fn was_does_not_decay() {
        // was is was everywhere — fossils don't un-fossilize with distance
        let here = LocalTense::here(Tense::Was, "biomeos");
        let far = here.decayed_at(10);
        assert_eq!(far.tense, Tense::Was, "was stays was — the fossil record is shared");
    }

    #[test]
    fn serde_new_tenses() {
        for t in Tense::ALL {
            let json = serde_json::to_string(t).unwrap();
            let back: Tense = serde_json::from_str(&json).unwrap();
            assert_eq!(*t, back);
        }
    }

    #[test]
    fn grid_stays_441() {
        // The tesseract uses 3 PRIMARY tenses, not 5
        assert_eq!(tesseract_cell_count(), 441);
    }
}

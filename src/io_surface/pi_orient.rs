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

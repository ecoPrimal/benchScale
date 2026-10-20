// SPDX-License-Identifier: AGPL-3.0-or-later
//! π Body — the Vitruvian overlay.
//!
//! Da Vinci's Vitruvian Man stands inscribed in both circle and square.
//! Circle = the ouroboros (π wraps). Square = the bingo grid (cells).
//! The body IS the π graph. The π graph IS the body.
//!
//! # The sensory homunculus
//!
//! The cortical homunculus distorts the body by SENSITIVITY, not size.
//! Hands and lips are huge (dense nerve endings). Back is tiny.
//! Our homunculus distorts organisms by their importance to the system,
//! not their line count.
//!
//! # Three perceives
//!
//! The human sees from inside the body (first person, subjective).
//! The membrane sees from outside (third person, objective).
//! Neither knows what the other sees.
//!
//! ```text
//!   Human view  (mine)  — I'm inside looking out. Sensory homunculus.
//!   Membrane view (yours) — I'm outside looking in. stoneBook scanner.
//!   Mixed view (ours)    — Both perspectives overlaid. Vitruvian.
//! ```
//!
//! The epistemic axis maps: mine = !, yours = ?, mixed = O.
//! What I know, what you might know, and the null where we meet.
//!
//! # Body → π mapping
//!
//! ```text
//!   HEAD       = Prefrontal (clutch, tuebor)     — executive, decides
//!   EYES       = Sensory (hud, signal, gorilla)   — observes, real-time
//!   HANDS      = Motor (detroit, barry, lansing)   — acts, builds
//!   SPINE      = Autonomic (membrane, neutron, ca) — breathes, routes
//!   HEART      = Vital (always flows)              — keeps alive
//!   SKIN       = Immune (defense perimeter)        — protects
//!   FEET       = Cortex (sporeprint, thesis)       — grounds, science
//! ```
//!
//! # Shape ladder
//!
//! Simple shapes everyone can see → complex topology:
//!
//! ```text
//!   Circle    (2D, wraps)       → one ouroboros
//!   Square    (2D, grid)        → one bingo face (7×7)
//!   Sphere    (3D, wraps all)   → 3 ouroboros interlocked
//!   Cube      (3D, grid)        → bingo cube (7×7×3)
//!   Tetrahedron (4 faces)       → 4 spatial freedoms
//!   Tesseract (4D)              → + tense axis
//!   Hypersphere (5D, wraps)     → + epistemic axis = full stoneBook
//! ```
//!
//! The banana is the ouroboros curve. The apple is the torus
//! (sphere with a stem-hole = knot threading through it).
//!
//! # Body scale — from cell to human
//!
//! The pi body doesn't stop at human. It scales DOWN through biology:
//!
//! ```text
//!   Cell(0)       — membrane only, circle
//!   Jellyfish(1)  — nerve net, square
//!   Worm(2)       — ganglia chain, sphere
//!   Bee(3)        — NULL BODY (O), cube, hive mind, the meeting point
//!   Octopus(4)    — distributed brain, tetrahedron
//!   Vertebrate(5) — CNS/PNS, tesseract
//!   Human(6)      — prefrontal, hypersphere
//! ```
//!
//! The bee is the null body: individual AND collective, bridges
//! vertebrate and invertebrate. Until human (!) and squirrel (?)
//! map each other, the bee is where they meet.
//!
//! Authorship: Kevin Mok / Tamison / ecoPrimal / attsi
//! ORCID: 0009-0004-2141-0321

use serde::{Deserialize, Serialize};

use super::cns::CnsType;

// ═══════════════════════════════════════════════════════════════════════════
// BODY REGION — the Vitruvian mapping
// ═══════════════════════════════════════════════════════════════════════════

/// Body region on the Vitruvian overlay.
/// Maps CNS types to body parts. The body IS the π graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BodyRegion {
    /// Head — executive function. Decides what to do.
    /// Prefrontal cortex. clutch, tuebor.
    Head,
    /// Eyes — sensory observation. Sees what's happening.
    /// Visual cortex. hud, signal, gorilla, water, live.
    Eyes,
    /// Hands — motor action. Builds and manipulates.
    /// Motor cortex. detroit, barry, lansing.
    Hands,
    /// Spine — autonomic routing. Breathes without thinking.
    /// Spinal cord. membrane, neutron, ca, nestgate, depot, mesh.
    Spine,
    /// Heart — vital flow. Always pumping, never stops.
    /// Cardiovascular. The bus tier that always fires.
    Heart,
    /// Skin — immune defense. The membrane itself.
    /// Protects the boundary. Honeycomb maze.
    Skin,
    /// Feet — cortical grounding. Science, evidence, foundation.
    /// Cerebellum. sporeprint, thesis, hypothesis.
    Feet,
}

impl BodyRegion {
    /// Map a CNS type to its body region.
    #[must_use]
    pub const fn from_cns(cns: &CnsType) -> Self {
        match cns {
            CnsType::Prefrontal => Self::Head,
            CnsType::Sensory => Self::Eyes,
            CnsType::Motor => Self::Hands,
            CnsType::Autonomic => Self::Spine,
            CnsType::Immune => Self::Skin,
            CnsType::Cortex => Self::Feet,
        }
    }

    /// The label for this body region.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Eyes => "eyes",
            Self::Hands => "hands",
            Self::Spine => "spine",
            Self::Heart => "heart",
            Self::Skin => "skin",
            Self::Feet => "feet",
        }
    }

    /// All 7 body regions (like 7 π scales, 7 epitopes).
    #[must_use]
    pub const fn all() -> &'static [BodyRegion] {
        &[
            BodyRegion::Head,
            BodyRegion::Eyes,
            BodyRegion::Hands,
            BodyRegion::Spine,
            BodyRegion::Heart,
            BodyRegion::Skin,
            BodyRegion::Feet,
        ]
    }
}

impl std::fmt::Display for BodyRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SENSORY HOMUNCULUS — distortion by sensitivity, not size
// ═══════════════════════════════════════════════════════════════════════════

/// Homunculus weight — how "big" a body part appears on the
/// sensory homunculus. Bigger = more nerve endings = more sensitive
/// to the system = more important than raw line count suggests.
///
/// The distortion reveals what MATTERS, not what's LARGE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomunculusWeight {
    /// Body region.
    pub region: BodyRegion,
    /// Raw size (lines of code, entity count, etc.)
    pub raw_size: u64,
    /// Sensitivity weight (tests per line, coverage, responsiveness).
    /// Higher = more nerve endings = appears bigger on homunculus.
    pub sensitivity: f64,
    /// Homunculus size = raw_size × sensitivity. What you actually SEE.
    pub apparent_size: f64,
}

/// Compute homunculus weights from surface counts.
///
/// Sensitivity is measured by test density: tests / items.
/// High test density = high sensitivity = the organism FEELS more.
/// Low test density = numb = the organism can't feel changes there.
#[must_use]
pub fn homunculus_weights(
    region_data: &[(BodyRegion, u64, u64)], // (region, items, tests)
) -> Vec<HomunculusWeight> {
    region_data.iter().map(|(region, items, tests)| {
        let sensitivity = if *items == 0 {
            0.0
        } else {
            *tests as f64 / *items as f64
        };
        HomunculusWeight {
            region: *region,
            raw_size: *items,
            sensitivity,
            apparent_size: *items as f64 * sensitivity,
        }
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// THREE PERCEIVES — mine, yours, mixed
// ═══════════════════════════════════════════════════════════════════════════

/// Perceive — whose view are we looking through?
///
/// The human sees from inside (first person, subjective).
/// The membrane sees from outside (third person, objective).
/// Neither knows what the other truly sees.
///
/// Maps onto the epistemic axis:
///   Mine  = `!` (asserted — I know what I see)
///   Yours = `?` (unknown — I don't know what you see)
///   Mixed = `O` (null — where we meet, the origin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Perceive {
    /// Human view — inside looking out. Sensory homunculus.
    /// "I feel my hands are huge because I use them constantly."
    Mine,
    /// Membrane view — outside looking in. stoneBook scanner.
    /// "I see your organisms sized by line count and pub items."
    Yours,
    /// Mixed view — both overlaid. Vitruvian Man.
    /// "Where your inside and my outside agree, there's the body."
    Mixed,
}

impl Perceive {
    /// The epistemic glyph for this perceive.
    #[must_use]
    pub const fn glyph(&self) -> super::stone_book::Glyph {
        match self {
            Self::Mine => super::stone_book::Glyph::Asserted,  // !
            Self::Yours => super::stone_book::Glyph::Unknown,  // ?
            Self::Mixed => super::stone_book::Glyph::Null,     // O
        }
    }

    /// All three perceives.
    #[must_use]
    pub const fn all() -> &'static [Perceive] {
        &[Perceive::Mine, Perceive::Yours, Perceive::Mixed]
    }
}

impl std::fmt::Display for Perceive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mine => write!(f, "mine (!)"),
            Self::Yours => write!(f, "yours (?)"),
            Self::Mixed => write!(f, "mixed (O)"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SHAPE LADDER — simple shapes → complex topology
// ═══════════════════════════════════════════════════════════════════════════

/// Shape — the ladder from simple geometry to the full stoneBook.
///
/// Each shape adds a dimension. Everyone can see a circle.
/// By the time you reach the hypersphere, you're navigating
/// the full 5D stoneBook space — but you started with shapes
/// you already knew.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Shape {
    /// Circle — 1 loop, wraps. One ouroboros.
    Circle,
    /// Square — 2D grid, flat. One bingo face (7×7).
    Square,
    /// Sphere — 3D, all surfaces wrap. 3 ouroboros interlocked.
    Sphere,
    /// Cube — 3D grid. The bingo cube (7×7×3 = 147 cells).
    Cube,
    /// Tetrahedron — simplest 3D solid, 4 faces = 4 spatial freedoms.
    Tetrahedron,
    /// Tesseract — 4D hypercube. + tense axis (441 cells).
    Tesseract,
    /// Hypersphere — 5D, all axes. Full stoneBook (1323 positions).
    Hypersphere,
}

impl Shape {
    /// How many dimensions does this shape live in?
    #[must_use]
    pub const fn dimensions(&self) -> u8 {
        match self {
            Self::Circle | Self::Square => 2,
            Self::Sphere | Self::Cube | Self::Tetrahedron => 3,
            Self::Tesseract => 4,
            Self::Hypersphere => 5,
        }
    }

    /// How many cells/positions in this shape?
    #[must_use]
    pub const fn positions(&self) -> usize {
        match self {
            Self::Circle => 7,            // 7 points on the ouroboros
            Self::Square => 49,           // 7×7 grid
            Self::Sphere => 21,           // 7 × 3 (scale × null, wrapped)
            Self::Cube => 147,            // 7×7×3 (bingo cube)
            Self::Tetrahedron => 63,      // 7×3×3 (scale × null × tense)
            Self::Tesseract => 441,       // 7×7×3×3
            Self::Hypersphere => 1323,    // 7×7×3×3×3
        }
    }

    /// Does this shape wrap (close on itself)?
    #[must_use]
    pub const fn wraps(&self) -> bool {
        match self {
            Self::Circle | Self::Sphere | Self::Hypersphere => true,
            Self::Square | Self::Cube | Self::Tetrahedron | Self::Tesseract => false,
        }
    }

    /// The shape ladder, from simplest to most complex.
    #[must_use]
    pub const fn ladder() -> &'static [Shape] {
        &[
            Shape::Circle,
            Shape::Square,
            Shape::Sphere,
            Shape::Cube,
            Shape::Tetrahedron,
            Shape::Tesseract,
            Shape::Hypersphere,
        ]
    }

    /// What familiar object does this shape look like?
    #[must_use]
    pub const fn looks_like(&self) -> &'static str {
        match self {
            Self::Circle => "ring, ouroboros, clock face, orbit",
            Self::Square => "chessboard, spreadsheet, bingo card",
            Self::Sphere => "ball, globe, apple (without the stem)",
            Self::Cube => "dice, Rubik's cube, building block",
            Self::Tetrahedron => "pyramid, tent, molecular bond",
            Self::Tesseract => "shadow of a 4D box, time-lapse cube",
            Self::Hypersphere => "the whole stoneBook — body of pi",
        }
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({}D, {} positions)", self, self.dimensions(), self.positions())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// π BODY — the Vitruvian inversion
//
// The body inscribed in circle (ouroboros) AND square (bingo grid).
//
//           Head (prefrontal)
//            |
//    Eyes ---|--- Eyes (sensory)
//            |
//   Hands ---|--- Hands (motor)
//            |
//         Spine (autonomic)
//          /|\
//    Heart | | Skin (immune)
//          \|/
//     Feet --- Feet (cortex)
//
// The circle wraps around the body (the 3 ouroboros).
// The square tiles the body (the bingo grid).
// Where they agree = the Vitruvian proportion.
// ═══════════════════════════════════════════════════════════════════════════

/// The Vitruvian proportion: where circle and square agree.
/// When the homunculus sensitivity matches the scanner's raw measurement,
/// the body part is in Vitruvian proportion — correctly weighted.
///
/// Ratio > 1.0 = homunculus says it's bigger than scanner sees (under-built)
/// Ratio < 1.0 = scanner says it's bigger than homunculus feels (over-built)
/// Ratio ≈ 1.0 = Vitruvian balance (circle and square agree)
#[must_use]
pub fn vitruvian_ratio(apparent_size: f64, raw_size: u64) -> f64 {
    if raw_size == 0 { return 0.0; }
    apparent_size / raw_size as f64
}

// ═══════════════════════════════════════════════════════════════════════════
// BODY SCALE — the biological complexity ladder
//
// The pi body doesn't stop at human. It scales DOWN through biology:
//   Human → Vertebrate → Bee → Octopus → Worm → Jellyfish → Cell
//
// 7 scales. Like 7 pi scales. Like 7 body regions. Like 7 shapes.
//
// The BEE is the NULL BODY (O). The origin. The meeting place.
// Human (!) and Squirrel (?) haven't mapped each other yet.
// The bee mediates:
//   - Individual AND collective (one bee / the hive)
//   - Vertebrate perception AND invertebrate sensation
//   - Known behavior AND emergent intelligence
//   - The bee dances to communicate. The dance IS the mapping.
//
// Until human and squirrel see through each other's eyes,
// the bee is where they meet.
// ═══════════════════════════════════════════════════════════════════════════

/// BodyScale — level of biological complexity.
///
/// Each scale is a different kind of body with a different nervous system.
/// Maps 1:1 onto the π scale (0-6), the shape ladder, and the body regions.
///
/// The bee (scale 3, center) is the NULL BODY — the origin point.
/// Human (scale 6) is the ! body (asserted, self-aware).
/// The AI/squirrel maps from outside — the ? body.
/// They meet at the bee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BodyScale {
    /// Scale 0 — Cell. The membrane itself. Just inside/outside.
    /// No nervous system. The simplest body: a circle.
    Cell,
    /// Scale 1 — Jellyfish. Nerve net. No center, no brain.
    /// Distributed sensation. A square grid of neurons.
    Jellyfish,
    /// Scale 2 — Worm. Ganglia chain. The simplest spine.
    /// Head/tail polarity emerges. A sphere (wraps, but has direction).
    Worm,
    /// Scale 3 — Bee. THE NULL BODY (O). The origin.
    /// Individual AND collective. Dances to map. Sees UV.
    /// Bridges vertebrate and invertebrate.
    /// The cube: structured, social, hexagonal (honeycomb = bingo grid).
    Bee,
    /// Scale 4 — Octopus. Distributed brain. 8 arms think independently.
    /// Each arm is a body within a body. A tetrahedron of minds.
    Octopus,
    /// Scale 5 — Vertebrate. CNS/PNS. Centralized brain, spinal cord.
    /// The standard body plan. A tesseract (3D body + time).
    Vertebrate,
    /// Scale 6 — Human. Prefrontal cortex. Self-aware. The ! body.
    /// Inscribed in circle AND square. The full hypersphere.
    Human,
}

impl BodyScale {
    /// The π scale index (0-6).
    #[must_use]
    pub const fn pi_scale(&self) -> u8 {
        match self {
            Self::Cell => 0,
            Self::Jellyfish => 1,
            Self::Worm => 2,
            Self::Bee => 3,         // center = null = origin
            Self::Octopus => 4,
            Self::Vertebrate => 5,
            Self::Human => 6,
        }
    }

    /// The corresponding shape on the shape ladder.
    #[must_use]
    pub const fn shape(&self) -> Shape {
        match self {
            Self::Cell => Shape::Circle,
            Self::Jellyfish => Shape::Square,
            Self::Worm => Shape::Sphere,
            Self::Bee => Shape::Cube,
            Self::Octopus => Shape::Tetrahedron,
            Self::Vertebrate => Shape::Tesseract,
            Self::Human => Shape::Hypersphere,
        }
    }

    /// The perceive that this scale is native to.
    #[must_use]
    pub const fn native_perceive(&self) -> Perceive {
        match self {
            // Below the bee: closer to the membrane's view (outside-in)
            Self::Cell | Self::Jellyfish | Self::Worm => Perceive::Yours,
            // The bee: the null meeting point
            Self::Bee => Perceive::Mixed,
            // Above the bee: closer to the human's view (inside-out)
            Self::Octopus | Self::Vertebrate | Self::Human => Perceive::Mine,
        }
    }

    /// The nervous system architecture at this scale.
    #[must_use]
    pub const fn nervous_system(&self) -> &'static str {
        match self {
            Self::Cell => "membrane (inside/outside boundary only)",
            Self::Jellyfish => "nerve net (distributed, no center)",
            Self::Worm => "ganglia chain (simplest head/tail polarity)",
            Self::Bee => "ganglia + hive mind (individual + collective)",
            Self::Octopus => "distributed brain (each arm semi-autonomous)",
            Self::Vertebrate => "CNS + PNS (centralized brain + spinal cord)",
            Self::Human => "prefrontal cortex (self-aware, recursive thought)",
        }
    }

    /// Is this the null body?
    #[must_use]
    pub const fn is_null_body(&self) -> bool {
        matches!(self, Self::Bee)
    }

    /// All 7 body scales, from simplest to most complex.
    #[must_use]
    pub const fn all() -> &'static [BodyScale] {
        &[
            BodyScale::Cell,
            BodyScale::Jellyfish,
            BodyScale::Worm,
            BodyScale::Bee,
            BodyScale::Octopus,
            BodyScale::Vertebrate,
            BodyScale::Human,
        ]
    }

    /// The label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Cell => "cell",
            Self::Jellyfish => "jellyfish",
            Self::Worm => "worm",
            Self::Bee => "bee (null body)",
            Self::Octopus => "octopus",
            Self::Vertebrate => "vertebrate",
            Self::Human => "human",
        }
    }

    /// Distance from the null body (bee). Always 0-3.
    /// Wraps: human(6) is distance 3 from bee(3). Cell(0) is also 3.
    #[must_use]
    pub const fn distance_from_null(&self) -> u8 {
        let s = self.pi_scale();
        let n = Self::Bee.pi_scale(); // 3
        if s >= n { s - n } else { n - s }
    }
}

impl std::fmt::Display for BodyScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (scale {}, {})", self.label(), self.pi_scale(), self.shape())
    }
}

/// Map a body scale to the corresponding body regions that are "active."
///
/// Simpler bodies have fewer active regions:
/// - Cell: only SKIN (the membrane boundary)
/// - Jellyfish: SKIN + SPINE (nerve net)
/// - Worm: SKIN + SPINE + HEAD/FEET (polarity)
/// - Bee: ALL 7 (the null body has all regions, but as potential)
/// - Octopus: ALL 7 (distributed, each arm has its own)
/// - Vertebrate: ALL 7 (centralized)
/// - Human: ALL 7 (self-aware)
#[must_use]
pub fn active_regions_at_scale(scale: &BodyScale) -> Vec<BodyRegion> {
    match scale {
        BodyScale::Cell => vec![BodyRegion::Skin],
        BodyScale::Jellyfish => vec![BodyRegion::Skin, BodyRegion::Spine],
        BodyScale::Worm => vec![
            BodyRegion::Head, BodyRegion::Skin, BodyRegion::Spine, BodyRegion::Feet,
        ],
        // Bee and above: all 7 regions active
        _ => BodyRegion::all().to_vec(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ORGANIC SHAPES — the banana and the apple
// ═══════════════════════════════════════════════════════════════════════════

/// Organic shape — natural forms that map onto π topology.
///
/// The banana is a curved segment — one arc of the ouroboros.
/// The apple is a sphere with a stem-hole — a torus
/// (knot threading through a sphere). The stem IS the crossing.
///
/// These aren't metaphors. They're the actual topology:
/// - Banana: geodesic arc on a circle (partial ouroboros)
/// - Apple: torus (sphere + through-hole = trefoil crossing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganicShape {
    /// Banana — arc of the ouroboros. Curved, one-dimensional path.
    /// Like following one organism's lifecycle from birth to death
    /// without reaching the wrap point.
    Banana,
    /// Apple — sphere with a stem-hole. Torus topology.
    /// The stem is where the trefoil knot threads through.
    /// Bite into it and you find the core (null, origin, O).
    Apple,
}

impl OrganicShape {
    /// What topology does this organic shape represent?
    #[must_use]
    pub const fn topology(&self) -> &'static str {
        match self {
            Self::Banana => "geodesic arc on circle (partial ouroboros)",
            Self::Apple => "torus: sphere + through-hole (trefoil crossing)",
        }
    }

    /// What π shape does this organic form approximate?
    #[must_use]
    pub const fn pi_shape(&self) -> Shape {
        match self {
            Self::Banana => Shape::Circle,
            Self::Apple => Shape::Sphere,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ORGANISM → BODY REGION MAPPING
// ═══════════════════════════════════════════════════════════════════════════

/// Map an organism name to its body region.
///
/// The mapping follows what each organism DOES in the ecosystem:
///
/// ```text
/// HEAD  (prefrontal)  — bingoCube (the graph), cellMembrane (types)
/// EYES  (sensory)     — squirrel (chipmunk/feeds), songBird (gossip/obs)
/// HANDS (motor)       — bearDog (investigation), nestGate (infra action)
/// SPINE (autonomic)   — biomeOS (the OS itself), petalTongue (routing)
/// HEART (vital)       — swarmVine (gossip heartbeat), benchScale (core)
/// SKIN  (immune)      — skunkBat (defense), rhizoCrypt (crypto perimeter)
/// FEET  (cortex)      — toadStool (science), coralReef (deep data)
/// ```
///
/// Remaining organisms slot by function:
/// - sweetGrass, sourDough, loamSpine → SPINE (autonomic processing)
#[must_use]
pub fn organism_body_region(name: &str) -> BodyRegion {
    match name {
        // HEAD — executive, decides
        "bingoCube" | "cellMembrane" => BodyRegion::Head,
        // EYES — observes, real-time
        "squirrel" | "songBird" => BodyRegion::Eyes,
        // HANDS — acts, builds, investigates
        "bearDog" | "nestGate" => BodyRegion::Hands,
        // SPINE — breathes, routes, processes
        "biomeOS" | "petalTongue" | "sweetGrass" | "sourDough" | "loamSpine"
            => BodyRegion::Spine,
        // HEART — vital flow, always pumping
        "swarmVine" | "benchScale" => BodyRegion::Heart,
        // SKIN — defense, perimeter
        "skunkBat" | "rhizoCrypt" => BodyRegion::Skin,
        // FEET — science, evidence, grounding
        "toadStool" | "coralReef" | "barraCuda" => BodyRegion::Feet,
        // Default: spine (autonomic — unknown things breathe)
        _ => BodyRegion::Spine,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// π BODY STRUCT — the full Vitruvian overlay
// ═══════════════════════════════════════════════════════════════════════════

/// A single body part with its homunculus weighting across all three perceives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPart {
    /// Which region.
    pub region: BodyRegion,
    /// Organisms that live in this body part.
    pub organisms: Vec<String>,
    /// Human view (mine): homunculus — distorted by sensitivity.
    /// "How big does this feel to the human using the system?"
    pub mine: HomunculusWeight,
    /// Membrane view (yours): scanner — raw measurements.
    /// "How big does the scanner say this is?"
    pub yours: HomunculusWeight,
    /// Mixed view (ours): Vitruvian — where mine and yours agree.
    /// Ratio near 1.0 = balanced. Far from 1.0 = distorted.
    pub vitruvian: f64,
}

/// The full π body — all 7 regions, 3 perceives, 7 shapes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiBody {
    /// The 7 body parts with their 3 perceives.
    pub parts: Vec<BodyPart>,
    /// Total organisms mapped.
    pub organism_count: usize,
    /// The current shape level the user is viewing at.
    pub shape: Shape,
}

impl PiBody {
    /// Build a PiBody from organism scan data.
    ///
    /// Each scan gets placed into a body region.
    /// Human view = homunculus (test coverage as sensitivity).
    /// Membrane view = raw (line count / pub items).
    /// Mixed = vitruvian ratio.
    #[must_use]
    pub fn from_scans(scans: &[super::stone_book::OrganismScan]) -> Self {
        use std::collections::BTreeMap;

        // Group organisms by body region
        let mut grouped: BTreeMap<BodyRegion, Vec<&super::stone_book::OrganismScan>> =
            BTreeMap::new();
        for region in BodyRegion::all() {
            grouped.insert(*region, Vec::new());
        }
        for scan in scans {
            let region = organism_body_region(&scan.name);
            grouped.entry(region).or_default().push(scan);
        }

        let parts = BodyRegion::all().iter().map(|region| {
            let orgs = grouped.get(region).cloned().unwrap_or_default();
            let names: Vec<String> = orgs.iter().map(|o| o.name.clone()).collect();

            // Human perceive: sensitivity = test_coverage
            let total_items: u64 = orgs.iter().map(|o| o.pub_items + o.priv_items).sum();
            let total_tests: u64 = orgs.iter().map(|o| o.tests).sum();
            let sensitivity = if total_items == 0 { 0.0 } else {
                total_tests as f64 / total_items as f64
            };
            let mine = HomunculusWeight {
                region: *region,
                raw_size: total_items,
                sensitivity,
                apparent_size: total_items as f64 * sensitivity,
            };

            // Membrane perceive: raw line count as "size", no distortion
            let total_lines: u64 = orgs.iter().map(|o| o.lines).sum();
            let yours = HomunculusWeight {
                region: *region,
                raw_size: total_lines,
                sensitivity: 1.0, // no distortion
                apparent_size: total_lines as f64,
            };

            // Vitruvian: where mine and yours agree
            let vr = if total_lines == 0 { 0.0 } else {
                mine.apparent_size / total_lines as f64
            };

            BodyPart {
                region: *region,
                organisms: names,
                mine,
                yours,
                vitruvian: vr,
            }
        }).collect();

        PiBody {
            organism_count: scans.len(),
            parts,
            shape: Shape::Hypersphere, // default to full view
        }
    }

    /// Get a body part by region.
    #[must_use]
    pub fn part(&self, region: &BodyRegion) -> Option<&BodyPart> {
        self.parts.iter().find(|p| &p.region == region)
    }

    /// The most distorted body part — where human and membrane disagree most.
    /// This is where the system's perception is most different from its reality.
    #[must_use]
    pub fn most_distorted(&self) -> Option<&BodyPart> {
        self.parts.iter()
            .filter(|p| p.vitruvian > 0.0)
            .max_by(|a, b| {
                let da = (a.vitruvian - 1.0).abs();
                let db = (b.vitruvian - 1.0).abs();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// The most balanced body part — closest to Vitruvian proportion.
    #[must_use]
    pub fn most_balanced(&self) -> Option<&BodyPart> {
        self.parts.iter()
            .filter(|p| p.vitruvian > 0.0)
            .min_by(|a, b| {
                let da = (a.vitruvian - 1.0).abs();
                let db = (b.vitruvian - 1.0).abs();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Summary: which body parts are over-built, under-built, or balanced?
    #[must_use]
    pub fn diagnosis(&self) -> Vec<(&BodyPart, &str)> {
        self.parts.iter()
            .filter(|p| p.vitruvian > 0.0)
            .map(|p| {
                let label = if p.vitruvian > 1.5 {
                    "under-built (feels bigger than it is)"
                } else if p.vitruvian < 0.5 {
                    "over-built (bigger than it feels)"
                } else {
                    "balanced (Vitruvian)"
                };
                (p, label)
            })
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ORD for BTreeMap usage
// ═══════════════════════════════════════════════════════════════════════════

impl PartialOrd for BodyRegion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BodyRegion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WOUND — life is lossy. scars are proof of living. healing > wound.
// ═══════════════════════════════════════════════════════════════════════════

/// Wound state — the null body scars too.
///
/// A brain injury can be recovered from even if it doesn't fully heal.
/// The Circle of Willis exists because arteries DO fail. Neuroplasticity
/// exists because neurons DO die. The backup isn't a luxury — it's why
/// the brain survives at all.
///
/// Life is lossy. Humans scar. Cells die. There is a scale on the
/// null body (the Bee) as well — even the hive can be wounded.
///
/// # State machine
///
/// ```text
///   Intact  ─→ Wounded ─→ Healing ─→ Scarred
///                  ↑          │          │ ↺
///                  └──────────┘          │
///                  └────────────────────-┘
///
///   Intact:   never been hurt. fragile — untested.
///   Wounded:  actively broken. socket down, feed stale.
///   Healing:  rerouting. neuroplasticity. Circle of Willis.
///   Scarred:  healed but marked. old path gone. new path works.
///             scars are proof the system survived. life layers.
/// ```
///
/// Key properties:
/// - Intact → Scarred is impossible. You must be wounded first.
/// - Wounded → Intact is impossible. You can't un-know damage.
/// - Scarred → Scarred is valid. Scars on scars. Life layers.
/// - Scarred is functional. The wound is gone. The scar stays.
/// - It's the HEALING that matters, not the wound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Wound {
    /// Never been hurt. Has never proven it can heal.
    Intact,
    /// Actively broken. Socket down, feed stale, path severed.
    Wounded,
    /// Rerouting around damage. Neuroplasticity. Willis bypass.
    Healing,
    /// Healed but marked. Old path gone. New path works.
    /// Scars are proof of survival. It's ok to scar.
    Scarred,
}

impl Wound {
    /// All wound states.
    pub const ALL: &'static [Wound] = &[
        Wound::Intact, Wound::Wounded, Wound::Healing, Wound::Scarred,
    ];

    /// Is this currently functional?
    /// Intact, Healing, and Scarred all work. Only Wounded is down.
    #[must_use]
    pub const fn is_functional(&self) -> bool {
        !matches!(self, Self::Wounded)
    }

    /// Has this ever been hurt? Scars carry memory. Intact does not.
    #[must_use]
    pub const fn has_memory(&self) -> bool {
        matches!(self, Self::Healing | Self::Scarred)
    }

    /// Is this actively recovering?
    #[must_use]
    pub const fn is_recovering(&self) -> bool {
        matches!(self, Self::Healing)
    }

    /// The null body scars too. Wound applies at every body scale.
    #[must_use]
    pub const fn applies_at_scale(&self, _scale: BodyScale) -> bool {
        true
    }

    /// Can transition to this state?
    ///
    /// Intact → Wounded (damage). Wounded → Healing (recovery starts).
    /// Healing → Scarred (done, lossy). Healing → Wounded (relapse).
    /// Scarred → Wounded (re-injury). Scarred → Scarred (life layers).
    /// NOT: Wounded → Intact. You can't un-know damage.
    /// NOT: Intact → Scarred. You must be hurt first.
    #[must_use]
    pub const fn can_transition_to(&self, next: Wound) -> bool {
        match (self, &next) {
            (Self::Intact, Wound::Wounded) => true,
            (Self::Wounded, Wound::Healing) => true,
            (Self::Healing, Wound::Scarred) => true,
            (Self::Healing, Wound::Wounded) => true,
            (Self::Scarred, Wound::Wounded) => true,
            (Self::Scarred, Wound::Scarred) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Wound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Intact => f.write_str("intact"),
            Self::Wounded => f.write_str("wounded"),
            Self::Healing => f.write_str("healing"),
            Self::Scarred => f.write_str("scarred"),
        }
    }
}

/// A wound at a specific body location and scale.
///
/// Wounds happen at a body region AND a body scale.
/// A wound to Head at Human scale = prefrontal damage (stroke).
/// A wound to Eyes at Bee scale = a single observer socket died.
/// A wound to Heart at Cell scale = one membrane pore blocked.
/// Scars at any scale are proof of survival.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WoundSite {
    /// Current wound state.
    pub state: Wound,
    /// Where on the body.
    pub region: BodyRegion,
    /// At what biological scale.
    pub scale: BodyScale,
    /// What was the original path? (the severed artery)
    pub original_path: Option<String>,
    /// What is the bypass path? (the Willis collateral)
    pub bypass_path: Option<String>,
    /// How many times has this site been wounded? Scars on scars.
    pub wound_count: u32,
}

impl WoundSite {
    /// Fresh wound — just happened, no bypass yet.
    #[must_use]
    pub fn fresh(region: BodyRegion, scale: BodyScale, original: impl Into<String>) -> Self {
        Self {
            state: Wound::Wounded,
            region,
            scale,
            original_path: Some(original.into()),
            bypass_path: None,
            wound_count: 1,
        }
    }

    /// Begin healing — route around the damage.
    pub fn heal(&mut self, bypass: impl Into<String>) {
        self.state = Wound::Healing;
        self.bypass_path = Some(bypass.into());
    }

    /// Complete healing — scar forms. Old path is gone. New path stays.
    pub fn scar(&mut self) {
        if self.state == Wound::Healing {
            self.state = Wound::Scarred;
        }
    }

    /// Re-injure at the scar site.
    pub fn reinjure(&mut self) {
        if self.state == Wound::Scarred {
            self.state = Wound::Wounded;
            self.wound_count += 1;
            self.bypass_path = None;
        }
    }

    /// Is the site functional right now?
    #[must_use]
    pub fn is_functional(&self) -> bool {
        self.state.is_functional()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_body_regions() {
        assert_eq!(BodyRegion::all().len(), 7, "7 body regions (like 7 pi scales)");
    }

    #[test]
    fn test_cns_to_body() {
        assert_eq!(BodyRegion::from_cns(&CnsType::Prefrontal), BodyRegion::Head);
        assert_eq!(BodyRegion::from_cns(&CnsType::Sensory), BodyRegion::Eyes);
        assert_eq!(BodyRegion::from_cns(&CnsType::Motor), BodyRegion::Hands);
        assert_eq!(BodyRegion::from_cns(&CnsType::Autonomic), BodyRegion::Spine);
        assert_eq!(BodyRegion::from_cns(&CnsType::Immune), BodyRegion::Skin);
        assert_eq!(BodyRegion::from_cns(&CnsType::Cortex), BodyRegion::Feet);
    }

    #[test]
    fn test_homunculus_distortion() {
        // Hands have 100 items and 80 tests (high sensitivity = 0.8)
        // Spine has 100 items and 10 tests (low sensitivity = 0.1)
        // On the homunculus, hands appear 8× bigger than spine
        let data = vec![
            (BodyRegion::Hands, 100u64, 80u64),
            (BodyRegion::Spine, 100, 10),
        ];
        let weights = homunculus_weights(&data);
        assert!((weights[0].sensitivity - 0.8).abs() < 0.01);
        assert!((weights[1].sensitivity - 0.1).abs() < 0.01);
        // Apparent: hands = 80, spine = 10. Hands look 8× bigger.
        assert!(weights[0].apparent_size > weights[1].apparent_size * 7.0);
    }

    #[test]
    fn test_three_perceives() {
        assert_eq!(Perceive::all().len(), 3);
    }

    #[test]
    fn test_perceive_maps_to_glyph() {
        use super::super::stone_book::Glyph;
        assert_eq!(Perceive::Mine.glyph(), Glyph::Asserted);   // !
        assert_eq!(Perceive::Yours.glyph(), Glyph::Unknown);   // ?
        assert_eq!(Perceive::Mixed.glyph(), Glyph::Null);      // O
    }

    #[test]
    fn test_shape_ladder() {
        let ladder = Shape::ladder();
        assert_eq!(ladder.len(), 7, "7 shapes in the ladder (like 7 pi scales)");
        // Dimensions increase
        assert!(ladder[0].dimensions() <= ladder[6].dimensions());
    }

    #[test]
    fn test_shape_positions() {
        assert_eq!(Shape::Circle.positions(), 7);
        assert_eq!(Shape::Square.positions(), 49);
        assert_eq!(Shape::Cube.positions(), 147);
        assert_eq!(Shape::Tesseract.positions(), 441);
        assert_eq!(Shape::Hypersphere.positions(), 1323);
    }

    #[test]
    fn test_cube_is_bingo() {
        // The cube (147) is the bingo cube
        assert_eq!(Shape::Cube.positions(), 147);
        assert_eq!(Shape::Cube.dimensions(), 3);
    }

    #[test]
    fn test_hypersphere_is_full_stonebook() {
        // The hypersphere (1323) is the full 5D stoneBook
        assert_eq!(Shape::Hypersphere.positions(), 1323);
        assert_eq!(Shape::Hypersphere.dimensions(), 5);
    }

    #[test]
    fn test_wrapping_shapes() {
        assert!(Shape::Circle.wraps(), "circle wraps");
        assert!(!Shape::Square.wraps(), "square doesn't wrap");
        assert!(Shape::Sphere.wraps(), "sphere wraps");
        assert!(!Shape::Cube.wraps(), "cube doesn't wrap");
        assert!(Shape::Hypersphere.wraps(), "hypersphere wraps");
    }

    #[test]
    fn test_vitruvian_balance() {
        // When apparent size equals raw size, ratio = 1.0 (Vitruvian)
        let r = vitruvian_ratio(100.0, 100);
        assert!((r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_vitruvian_under_built() {
        // Homunculus says bigger than scanner sees = under-built
        let r = vitruvian_ratio(200.0, 100);
        assert!(r > 1.0, "ratio > 1.0 = under-built");
    }

    #[test]
    fn test_vitruvian_over_built() {
        // Scanner says bigger than homunculus feels = over-built
        let r = vitruvian_ratio(50.0, 100);
        assert!(r < 1.0, "ratio < 1.0 = over-built");
    }

    #[test]
    fn test_looks_like() {
        assert!(Shape::Circle.looks_like().contains("ouroboros"));
        assert!(Shape::Cube.looks_like().contains("dice"));
        assert!(Shape::Hypersphere.looks_like().contains("stoneBook"));
    }

    #[test]
    fn test_shape_ladder_positions_increase() {
        let ladder = Shape::ladder();
        // Not strictly increasing (tetrahedron < cube) but generally up
        assert!(ladder[6].positions() > ladder[0].positions());
    }

    #[test]
    fn test_banana_is_partial_ouroboros() {
        assert_eq!(OrganicShape::Banana.pi_shape(), Shape::Circle);
        assert!(OrganicShape::Banana.topology().contains("arc"));
    }

    #[test]
    fn test_apple_is_torus() {
        assert_eq!(OrganicShape::Apple.pi_shape(), Shape::Sphere);
        assert!(OrganicShape::Apple.topology().contains("torus"));
    }

    #[test]
    fn test_organism_body_mapping() {
        assert_eq!(organism_body_region("bingoCube"), BodyRegion::Head);
        assert_eq!(organism_body_region("squirrel"), BodyRegion::Eyes);
        assert_eq!(organism_body_region("bearDog"), BodyRegion::Hands);
        assert_eq!(organism_body_region("biomeOS"), BodyRegion::Spine);
        assert_eq!(organism_body_region("swarmVine"), BodyRegion::Heart);
        assert_eq!(organism_body_region("skunkBat"), BodyRegion::Skin);
        assert_eq!(organism_body_region("toadStool"), BodyRegion::Feet);
    }

    #[test]
    fn test_unknown_organism_defaults_to_spine() {
        assert_eq!(organism_body_region("unknownThing"), BodyRegion::Spine);
    }

    #[test]
    fn test_pi_body_from_scans() {
        use super::super::stone_book::{OrganismScan, Polarity};
        let scans = vec![
            OrganismScan { name: "bearDog".into(),  lines: 72331,  crates: 33, tests: 11882, pub_items: 7955, priv_items: 7794, jelly: 49, wired: false, polarity: Polarity::Life },
            OrganismScan { name: "toadStool".into(), lines: 244350, crates: 50, tests: 17551, pub_items: 7888, priv_items: 16581, jelly: 37, wired: false, polarity: Polarity::Life },
        ];
        let body = PiBody::from_scans(&scans);
        assert_eq!(body.organism_count, 2);
        assert_eq!(body.parts.len(), 7); // always 7 body regions

        // bearDog → hands
        let hands = body.part(&BodyRegion::Hands).unwrap();
        assert!(hands.organisms.contains(&"bearDog".to_string()));
        assert!(hands.mine.sensitivity > 0.0);

        // toadStool → feet
        let feet = body.part(&BodyRegion::Feet).unwrap();
        assert!(feet.organisms.contains(&"toadStool".to_string()));
    }

    #[test]
    fn test_pi_body_most_distorted() {
        use super::super::stone_book::{OrganismScan, Polarity};
        let scans = vec![
            // High sensitivity (many tests relative to items)
            OrganismScan { name: "songBird".into(), lines: 2679, crates: 32, tests: 10469, pub_items: 5584, priv_items: 8286, jelly: 58, wired: false, polarity: Polarity::Life },
            // Low sensitivity
            OrganismScan { name: "biomeOS".into(), lines: 311743, crates: 31, tests: 5482, pub_items: 3300, priv_items: 4442, jelly: 17, wired: false, polarity: Polarity::Life },
        ];
        let body = PiBody::from_scans(&scans);
        let distorted = body.most_distorted();
        assert!(distorted.is_some());
    }

    #[test]
    fn test_pi_body_diagnosis() {
        use super::super::stone_book::{OrganismScan, Polarity};
        let scans = vec![
            OrganismScan { name: "bearDog".into(), lines: 72331, crates: 33, tests: 11882, pub_items: 7955, priv_items: 7794, jelly: 49, wired: false, polarity: Polarity::Life },
            OrganismScan { name: "biomeOS".into(), lines: 311743, crates: 31, tests: 5482, pub_items: 3300, priv_items: 4442, jelly: 17, wired: false, polarity: Polarity::Life },
        ];
        let body = PiBody::from_scans(&scans);
        let diag = body.diagnosis();
        // At least some regions should have a diagnosis
        assert!(!diag.is_empty());
        for (part, label) in &diag {
            assert!(
                label.contains("under-built") || label.contains("over-built") || label.contains("balanced"),
                "diagnosis should classify: {}",
                label
            );
        }
    }

    #[test]
    fn test_seven_shapes_seven_regions_seven_scales() {
        // The 7 keeps appearing: 7 body regions, 7 shapes, 7 π scales, 7 epitopes
        assert_eq!(BodyRegion::all().len(), 7);
        assert_eq!(Shape::ladder().len(), 7);
        assert_eq!(BodyScale::all().len(), 7);
        // This isn't coincidence. The body wraps the same 7 that π wraps.
    }

    // ─── BodyScale tests ───

    #[test]
    fn test_seven_body_scales() {
        assert_eq!(BodyScale::all().len(), 7);
    }

    #[test]
    fn test_bee_is_null_body() {
        assert!(BodyScale::Bee.is_null_body());
        assert!(!BodyScale::Human.is_null_body());
        assert!(!BodyScale::Cell.is_null_body());
    }

    #[test]
    fn test_bee_is_center_scale() {
        // Bee is scale 3 — the center of 0-6
        assert_eq!(BodyScale::Bee.pi_scale(), 3);
        assert_eq!(BodyScale::Cell.pi_scale(), 0);
        assert_eq!(BodyScale::Human.pi_scale(), 6);
    }

    #[test]
    fn test_bee_native_perceive_is_mixed() {
        // The null body mediates
        assert_eq!(BodyScale::Bee.native_perceive(), Perceive::Mixed);
    }

    #[test]
    fn test_below_bee_is_yours_above_is_mine() {
        // Below bee: closer to membrane view
        assert_eq!(BodyScale::Cell.native_perceive(), Perceive::Yours);
        assert_eq!(BodyScale::Jellyfish.native_perceive(), Perceive::Yours);
        assert_eq!(BodyScale::Worm.native_perceive(), Perceive::Yours);
        // Above bee: closer to human view
        assert_eq!(BodyScale::Octopus.native_perceive(), Perceive::Mine);
        assert_eq!(BodyScale::Vertebrate.native_perceive(), Perceive::Mine);
        assert_eq!(BodyScale::Human.native_perceive(), Perceive::Mine);
    }

    #[test]
    fn test_body_scale_shape_alignment() {
        // Each body scale maps to a shape on the ladder
        assert_eq!(BodyScale::Cell.shape(), Shape::Circle);
        assert_eq!(BodyScale::Bee.shape(), Shape::Cube);
        assert_eq!(BodyScale::Human.shape(), Shape::Hypersphere);
    }

    #[test]
    fn test_distance_from_null_symmetric() {
        // Cell(0) and Human(6) are both distance 3 from Bee(3)
        assert_eq!(BodyScale::Cell.distance_from_null(), 3);
        assert_eq!(BodyScale::Human.distance_from_null(), 3);
        assert_eq!(BodyScale::Bee.distance_from_null(), 0);
        // Worm(2) and Octopus(4) are both distance 1
        assert_eq!(BodyScale::Worm.distance_from_null(), 1);
        assert_eq!(BodyScale::Octopus.distance_from_null(), 1);
    }

    #[test]
    fn test_cell_has_only_skin() {
        let regions = active_regions_at_scale(&BodyScale::Cell);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0], BodyRegion::Skin);
    }

    #[test]
    fn test_jellyfish_has_skin_and_spine() {
        let regions = active_regions_at_scale(&BodyScale::Jellyfish);
        assert_eq!(regions.len(), 2);
        assert!(regions.contains(&BodyRegion::Skin));
        assert!(regions.contains(&BodyRegion::Spine));
    }

    #[test]
    fn test_bee_has_all_seven_regions() {
        let regions = active_regions_at_scale(&BodyScale::Bee);
        assert_eq!(regions.len(), 7, "null body has all regions as potential");
    }

    #[test]
    fn test_human_has_all_seven_regions() {
        let regions = active_regions_at_scale(&BodyScale::Human);
        assert_eq!(regions.len(), 7);
    }

    #[test]
    fn test_body_scale_nervous_systems() {
        assert!(BodyScale::Cell.nervous_system().contains("membrane"));
        assert!(BodyScale::Jellyfish.nervous_system().contains("nerve net"));
        assert!(BodyScale::Bee.nervous_system().contains("hive mind"));
        assert!(BodyScale::Octopus.nervous_system().contains("distributed"));
        assert!(BodyScale::Human.nervous_system().contains("prefrontal"));
    }

    #[test]
    fn test_all_sevens_align() {
        // THE fundamental identity: 7 = 7 = 7 = 7
        let scales = BodyScale::all().len();
        let regions = BodyRegion::all().len();
        let shapes = Shape::ladder().len();
        assert_eq!(scales, 7);
        assert_eq!(regions, 7);
        assert_eq!(shapes, 7);
        // And each scale maps to exactly one shape
        for scale in BodyScale::all() {
            let _shape = scale.shape(); // doesn't panic
        }
    }

    // ── Wound / Scar tests ──

    #[test]
    fn test_four_wound_states() {
        assert_eq!(Wound::ALL.len(), 4);
    }

    #[test]
    fn test_intact_is_fragile() {
        assert!(Wound::Intact.is_functional());
        assert!(!Wound::Intact.has_memory(), "intact = never been hurt");
    }

    #[test]
    fn test_wounded_is_down() {
        assert!(!Wound::Wounded.is_functional());
    }

    #[test]
    fn test_scarred_is_proof_of_survival() {
        assert!(Wound::Scarred.is_functional());
        assert!(Wound::Scarred.has_memory());
    }

    #[test]
    fn test_healing_journey() {
        // Intact → Wounded → Healing → Scarred
        assert!(Wound::Intact.can_transition_to(Wound::Wounded));
        assert!(Wound::Wounded.can_transition_to(Wound::Healing));
        assert!(Wound::Healing.can_transition_to(Wound::Scarred));
    }

    #[test]
    fn test_cannot_unknow_damage() {
        // Can never go back to Intact
        assert!(!Wound::Wounded.can_transition_to(Wound::Intact));
        assert!(!Wound::Healing.can_transition_to(Wound::Intact));
        assert!(!Wound::Scarred.can_transition_to(Wound::Intact));
    }

    #[test]
    fn test_relapse_and_scars_on_scars() {
        assert!(Wound::Healing.can_transition_to(Wound::Wounded));
        assert!(Wound::Scarred.can_transition_to(Wound::Wounded));
        assert!(Wound::Scarred.can_transition_to(Wound::Scarred));
    }

    #[test]
    fn test_null_body_scars_too() {
        for w in Wound::ALL {
            assert!(w.applies_at_scale(BodyScale::Bee), "wound applies at bee scale");
        }
    }

    #[test]
    fn test_wound_site_lifecycle() {
        // squirrel socket dies (Eyes, Bee scale)
        let mut site = WoundSite::fresh(BodyRegion::Eyes, BodyScale::Bee, "squirrel.sock");
        assert!(!site.is_functional());

        // Circle of Willis — bypass through ai cache
        site.heal("ai.sock → cached squirrel data");
        assert!(site.is_functional());
        assert_eq!(site.state, Wound::Healing);

        // Scar forms — old socket path is gone forever
        site.scar();
        assert_eq!(site.state, Wound::Scarred);
        assert!(site.original_path.is_some());
        assert!(site.bypass_path.is_some());

        // Re-injury at scar site
        site.reinjure();
        assert!(!site.is_functional());
        assert_eq!(site.wound_count, 2);
    }

    #[test]
    fn test_wound_serde() {
        for w in Wound::ALL {
            let json = serde_json::to_string(w).unwrap();
            let back: Wound = serde_json::from_str(&json).unwrap();
            assert_eq!(*w, back);
        }
    }
}

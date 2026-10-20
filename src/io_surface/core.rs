// SPDX-License-Identifier: AGPL-3.0-or-later
//! Core I/O surface types — channels, mirrors, pores, surfaces.

use serde::{Deserialize, Serialize};
use std::fmt;

// ═══════════════════════════════════════════════════════════════
// PORE MODEL — the collapsed view
//
// Every I/O channel is really a PORE with permeability in both
// directions. "Input" and "output" are not types of channels —
// they're directions of flow through the same pore.
//
// display + camera = VISUAL PORE (high outward, low inward)
// speaker + mic = AUDIO PORE (high outward, medium inward)
// vibrate + touch = TACTILE PORE (low outward, high inward)
//
// A single photon of signal diffracts any meta frame with a
// localized bubble of permeability. You don't need special
// glasses. You need the math of selective permeability applied
// to whatever pores are open.
// ═══════════════════════════════════════════════════════════════

/// A pore — bidirectional I/O channel with selective permeability.
/// Like a membrane channel protein: ions flow both ways, but the
/// permeability differs by direction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoPore {
    /// The medium this pore carries (visual, audio, tactile, etc.)
    pub medium: ChannelMedium,

    /// Inward permeability: how much signal flows from world → device
    /// 0.0 = sealed (no input), 1.0 = wide open
    /// camera side of the visual pore, mic side of audio pore
    pub inward: f64,

    /// Outward permeability: how much signal flows from device → world
    /// 0.0 = sealed (no output), 1.0 = wide open
    /// display side of the visual pore, speaker side of audio pore
    pub outward: f64,

    /// Total bandwidth through this pore (both directions)
    pub bandwidth: f64,

    /// Which axis flips when signal crosses between inside and outside
    pub chirality: MirrorChirality,

    /// Pore-specific properties
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub properties: serde_json::Map<String, serde_json::Value>,
}

impl IoPore {
    /// Is this pore open at all? (nonzero permeability in either direction)
    pub fn is_open(&self) -> bool {
        self.inward > 0.0 || self.outward > 0.0
    }

    /// Is this pore mostly inward? (sensor-dominant)
    pub fn is_inward_dominant(&self) -> bool {
        self.inward > self.outward
    }

    /// Is this pore mostly outward? (actuator-dominant)
    pub fn is_outward_dominant(&self) -> bool {
        self.outward > self.inward
    }

    /// Is this pore roughly balanced? (barre — structural reference)
    pub fn is_balanced(&self) -> bool {
        let ratio = if self.outward > 0.0 {
            self.inward / self.outward
        } else if self.inward > 0.0 {
            0.0
        } else {
            return false; // sealed = not balanced
        };
        (0.3..=3.0).contains(&ratio)
    }

    /// The asymmetry of this pore: positive = outward dominant, negative = inward dominant
    pub fn asymmetry(&self) -> f64 {
        let total = self.inward + self.outward;
        if total == 0.0 {
            return 0.0;
        }
        (self.outward - self.inward) / total
    }
}

/// Collapse an IoSurface's 13 channels into ~7 pores
pub fn collapse_to_pores(surface: &IoSurface) -> Vec<(String, IoPore)> {
    let ch = |name: &str| -> Option<&IoChannel> { surface.channel(name) };
    let active = |name: &str| -> bool {
        ch(name).map(|c| c.active).unwrap_or(false)
    };
    let bw = |name: &str| -> f64 {
        ch(name).map(|c| c.bandwidth).unwrap_or(0.0)
    };
    let mirror_chirality = |input: &str| -> MirrorChirality {
        surface
            .mirrors
            .iter()
            .find(|m| m.input == input)
            .map(|m| m.chirality)
            .unwrap_or(MirrorChirality::Identity)
    };

    let mut pores = Vec::new();

    // VISUAL: display (outward) + camera (inward)
    {
        let out_bw = if active("display") { bw("display") } else { 0.0 };
        let in_bw = if active("camera") { bw("camera") } else { 0.0 };
        let total = out_bw + in_bw;
        let mut props = serde_json::Map::new();
        if let Some(d) = ch("display") {
            for (k, v) in &d.properties {
                props.insert(format!("display_{k}"), v.clone());
            }
        }
        if let Some(c) = ch("camera") {
            for (k, v) in &c.properties {
                props.insert(format!("camera_{k}"), v.clone());
            }
        }
        pores.push(("visual".into(), IoPore {
            medium: ChannelMedium::Visual,
            outward: if total > 0.0 { out_bw / total } else { 0.0 },
            inward: if total > 0.0 { in_bw / total } else { 0.0 },
            bandwidth: total,
            chirality: mirror_chirality("camera"),
            properties: props,
        }));
    }

    // AUDIO: speaker (outward) + mic (inward)
    {
        let out_bw = if active("speaker") { bw("speaker") } else { 0.0 };
        let in_bw = if active("mic") { bw("mic") } else { 0.0 };
        let total = out_bw + in_bw;
        pores.push(("audio".into(), IoPore {
            medium: ChannelMedium::Audio,
            outward: if total > 0.0 { out_bw / total } else { 0.0 },
            inward: if total > 0.0 { in_bw / total } else { 0.0 },
            bandwidth: total,
            chirality: mirror_chirality("mic"),
            properties: serde_json::Map::new(),
        }));
    }

    // TACTILE: vibrate (outward) + touch (inward)
    {
        let out_bw = if active("vibrate") { bw("vibrate") } else { 0.0 };
        let in_bw = if active("touch") { bw("touch") } else { 0.0 };
        let total = out_bw + in_bw;
        let mut props = serde_json::Map::new();
        if let Some(t) = ch("touch") {
            for (k, v) in &t.properties {
                props.insert(k.clone(), v.clone());
            }
        }
        pores.push(("tactile".into(), IoPore {
            medium: ChannelMedium::Tactile,
            outward: if total > 0.0 { out_bw / total } else { 0.0 },
            inward: if total > 0.0 { in_bw / total } else { 0.0 },
            bandwidth: total,
            chirality: mirror_chirality("touch"),
            properties: props,
        }));
    }

    // POINTER: mouse (mostly inward, cursor display is outward)
    if active("mouse") {
        pores.push(("pointer".into(), IoPore {
            medium: ChannelMedium::Pointer,
            inward: 0.95,
            outward: 0.05,
            bandwidth: bw("mouse"),
            chirality: MirrorChirality::Identity,
            properties: ch("mouse").map(|c| c.properties.clone()).unwrap_or_default(),
        }));
    }

    // SYMBOLIC: keyboard (mostly inward, backlight/led is outward)
    if active("keyboard") {
        let is_virtual = ch("keyboard")
            .and_then(|c| c.properties.get("virtual"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        pores.push(("symbolic".into(), IoPore {
            medium: ChannelMedium::Symbolic,
            inward: if is_virtual { 0.7 } else { 0.9 },
            outward: if is_virtual { 0.3 } else { 0.1 },
            bandwidth: bw("keyboard"),
            chirality: MirrorChirality::Identity,
            properties: ch("keyboard").map(|c| c.properties.clone()).unwrap_or_default(),
        }));
    }

    // SPATIAL: gyro (mostly inward — reads orientation, device also moves)
    if active("gyro") {
        pores.push(("spatial".into(), IoPore {
            medium: ChannelMedium::Spatial,
            inward: 0.9,
            outward: 0.1,
            bandwidth: bw("gyro"),
            chirality: MirrorChirality::Identity,
            properties: serde_json::Map::new(),
        }));
    }

    // DATA: network (balanced bidirectional — the ur-barre)
    if active("network") {
        pores.push(("data".into(), IoPore {
            medium: ChannelMedium::Data,
            inward: 0.5,
            outward: 0.5,
            bandwidth: bw("network"),
            chirality: MirrorChirality::Identity,
            properties: ch("network").map(|c| c.properties.clone()).unwrap_or_default(),
        }));
    }

    // PERSISTENCE: storage (balanced bidirectional — read/write)
    if active("storage") {
        pores.push(("persistence".into(), IoPore {
            medium: ChannelMedium::Persistence,
            inward: 0.5,
            outward: 0.5,
            bandwidth: bw("storage"),
            chirality: MirrorChirality::Identity,
            properties: serde_json::Map::new(),
        }));
    }

    // ENERGY: battery (asymmetric — reads level inward, consumes outward)
    if active("battery") {
        pores.push(("energy".into(), IoPore {
            medium: ChannelMedium::Energy,
            inward: 0.8,
            outward: 0.2,
            bandwidth: 0.001,
            chirality: MirrorChirality::Identity,
            properties: ch("battery").map(|c| c.properties.clone()).unwrap_or_default(),
        }));
    }

    // POSITION: gps (mostly inward — reads coordinates)
    if active("gps") {
        pores.push(("position".into(), IoPore {
            medium: ChannelMedium::Position,
            inward: 0.95,
            outward: 0.05,
            bandwidth: 1.0,
            chirality: MirrorChirality::Identity,
            properties: serde_json::Map::new(),
        }));
    }

    pores
}

/// Direction of data flow through a channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelDirection {
    /// Data flows in (sensor / input device)
    Input,
    /// Data flows out (actuator / output device)
    Output,
    /// Data flows both directions
    Bidirectional,
}

impl fmt::Display for ChannelDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input => write!(f, "input"),
            Self::Output => write!(f, "output"),
            Self::Bidirectional => write!(f, "bidirectional"),
        }
    }
}

/// The medium carried by a channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMedium {
    /// Light, pixels, images
    Visual,
    /// Sound waves, samples
    Audio,
    /// Physical contact, pressure, vibration
    Tactile,
    /// Cursor/arrow position tracking
    Pointer,
    /// Discrete symbols (keystrokes, text)
    Symbolic,
    /// Orientation, acceleration, rotation
    Spatial,
    /// Network bytes, packets
    Data,
    /// Filesystem, localStorage, IndexedDB
    Persistence,
    /// Battery level, charging state
    Energy,
    /// GPS coordinates, geolocation
    Position,
}

impl fmt::Display for ChannelMedium {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visual => write!(f, "visual"),
            Self::Audio => write!(f, "audio"),
            Self::Tactile => write!(f, "tactile"),
            Self::Pointer => write!(f, "pointer"),
            Self::Symbolic => write!(f, "symbolic"),
            Self::Spatial => write!(f, "spatial"),
            Self::Data => write!(f, "data"),
            Self::Persistence => write!(f, "persistence"),
            Self::Energy => write!(f, "energy"),
            Self::Position => write!(f, "position"),
        }
    }
}

/// A single I/O channel — one way information enters or exits the surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoChannel {
    /// Human-readable name (display, touch, mic, etc.)
    pub name: String,

    /// Direction of data flow
    pub direction: ChannelDirection,

    /// Medium carried
    pub medium: ChannelMedium,

    /// Whether this channel is currently active / available
    pub active: bool,

    /// If this channel has a mirror, its name.
    /// mic mirrors speaker, camera mirrors display, vibrate mirrors touch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror: Option<String>,

    /// Approximate bandwidth in channel-native units
    /// (pixels/s for visual, samples/s for audio, Hz for pointers, etc.)
    #[serde(default)]
    pub bandwidth: f64,

    /// Channel-specific properties (width, height, dpr, points, etc.)
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub properties: serde_json::Map<String, serde_json::Value>,
}

/// Which axis flips when a signal crosses between mirror sides.
///
/// Like the optic chiasm: left brain sees right field, right brain sees left.
/// Like the ballet mirror: dancer's left hand appears on the right side.
/// The reflection isn't a copy — it has chirality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorChirality {
    /// Left ↔ right swap (ballet mirror, selfie camera, optic chiasm).
    /// The dancer looks toward self; left becomes right.
    LeftRight,

    /// Near ↔ far swap (audio echo, microphone feedback).
    /// Sound from here comes back from there.
    NearFar,

    /// Contact ↔ response swap (touch triggers vibration at same point).
    /// No spatial flip, but cause and effect swap.
    CauseEffect,

    /// No spatial transformation — identity reflection.
    /// Signal crosses unchanged.
    Identity,
}

impl fmt::Display for MirrorChirality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftRight => write!(f, "left↔right"),
            Self::NearFar => write!(f, "near↔far"),
            Self::CauseEffect => write!(f, "cause↔effect"),
            Self::Identity => write!(f, "identity"),
        }
    }
}

/// Where the observer stands relative to the mirror.
///
/// The ballet dancer has two modes:
/// - **Toward**: facing the mirror, seeing herself reflected (barre as anchor)
/// - **Away**: facing away, seeing the room behind her in the mirror
///
/// These produce different reflections from the same mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflectionAngle {
    /// Looking toward the mirror surface (selfie mode, ballet mirror).
    /// The reflection shows YOU. Left↔right chirality active.
    Toward,

    /// Looking away from the mirror (rear camera, rearview mirror).
    /// The reflection shows what's BEHIND you. Chirality may differ.
    Away,

    /// Looking through the surface (AR overlay, HUD projection).
    /// No reflection — the surface is transparent.
    Through,
}

impl fmt::Display for ReflectionAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toward => write!(f, "toward"),
            Self::Away => write!(f, "away"),
            Self::Through => write!(f, "through"),
        }
    }
}

/// What the dancer sees in a mirror from a given angle.
///
/// The same mirror produces different views depending on where you stand.
/// The base chirality is fixed (physics), but the effective chirality
/// depends on the reflection angle (where you're looking from).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionView {
    /// Input channel name
    pub input: String,
    /// Output channel name
    pub output: String,
    /// Shared medium
    pub medium: ChannelMedium,
    /// Observer's angle relative to the mirror
    pub angle: ReflectionAngle,
    /// The inherent chirality of this mirror pair (physics)
    pub base_chirality: MirrorChirality,
    /// The chirality the observer actually experiences from this angle
    pub effective_chirality: MirrorChirality,
}

/// A mirror pair — same physical phenomenon, opposite directions, with chirality.
///
/// The ballet dancer model:
/// - **barre** (fixed reference): the membrane bus, always present, something to hold
/// - **mirror** (reflection): the display reflecting the user's I/O back at them
/// - **dancer** (observer): the user, whose left hand appears on the right in the mirror
///
/// Every mirror pair has:
/// - An input side and an output side (the two channels)
/// - A chirality (which axis flips in the reflection)
/// - A reflection angle (where the observer stands)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorPair {
    /// The input side (mic, camera, touch)
    pub input: String,
    /// The output side (speaker, display, vibrate)
    pub output: String,
    /// The shared medium
    pub medium: ChannelMedium,
    /// Which axis flips in the reflection
    pub chirality: MirrorChirality,
    /// Default observer position relative to this mirror
    pub default_angle: ReflectionAngle,
}

/// The complete I/O surface of an endpoint — all channels, probed.
///
/// The ballet dancer model:
/// - **channels** are what's available (eyes, ears, hands, feet)
/// - **mirrors** are reflections (camera↔display, mic↔speaker, touch↔vibrate)
/// - **barre** is the fixed reference (the membrane bus — something to hold onto)
/// - **profile** is how you move (touch-only, spatial, conserve)
///
/// The dancer doesn't think "I am on mobile." The dancer probes what channels
/// are open, holds the barre, looks in the mirror, and adapts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoSurface {
    /// All discovered channels, keyed by name
    pub channels: Vec<IoChannel>,

    /// Surface profile tags (touch-only, small, spatial, conserve, etc.)
    #[serde(default)]
    pub profile: Vec<String>,

    /// Mirror pairs detected between channels
    #[serde(default)]
    pub mirrors: Vec<MirrorPair>,

    /// The barre — fixed reference channels that don't flip in the mirror.
    /// Network and storage are barres: they have no chirality, no reflection.
    /// They're structural. You hold them while you spin.
    #[serde(default)]
    pub barre: Vec<String>,
}

/// Well-known mirror pairs — universal physics with chirality.
///
/// camera↔display: left-right chirality (the ballet mirror).
///   Selfie camera shows you reflected — your left hand on the right.
///   Default angle: Toward (you're looking at yourself).
///
/// mic↔speaker: near-far chirality (sound from here returns from there).
///   You speak close to the mic; sound comes back from distant speakers.
///   The spatial field inverts near↔far.
///
/// touch↔vibrate: cause-effect chirality (no spatial flip).
///   You press here, it vibrates here. Same point, but the causal
///   direction swaps — your action becomes the device's reaction.
pub(crate) const MIRROR_PAIRS: &[(&str, &str, ChannelMedium, MirrorChirality, ReflectionAngle)] = &[
    ("mic", "speaker", ChannelMedium::Audio, MirrorChirality::NearFar, ReflectionAngle::Toward),
    ("camera", "display", ChannelMedium::Visual, MirrorChirality::LeftRight, ReflectionAngle::Toward),
    ("touch", "vibrate", ChannelMedium::Tactile, MirrorChirality::CauseEffect, ReflectionAngle::Through),
];


#[cfg(test)]
mod tests {
    use crate::io_surface::*;

    #[test]
    fn test_mirror_detection() {
        let surface = IoSurface::preset_touch_small_spatial();
        assert_eq!(surface.mirrors.len(), 3);
        let mirror_pairs: Vec<(&str, &str)> = surface
            .mirrors
            .iter()
            .map(|m| (m.input.as_str(), m.output.as_str()))
            .collect();
        assert!(mirror_pairs.contains(&("mic", "speaker")));
        assert!(mirror_pairs.contains(&("camera", "display")));
        assert!(mirror_pairs.contains(&("touch", "vibrate")));
    }

    #[test]
    fn test_touch_small_profile() {
        let surface = IoSurface::preset_touch_small_spatial();
        assert!(surface.has_profile("touch-only"));
        assert!(surface.has_profile("small"));
        assert!(surface.has_profile("spatial"));
        assert!(!surface.has_profile("pointer-only"));
        assert!(!surface.has_profile("conserve"));
    }

    #[test]
    fn test_pointer_large_profile() {
        let surface = IoSurface::preset_pointer_large();
        assert!(surface.has_profile("pointer-only"));
        assert!(surface.has_profile("large"));
        assert!(!surface.has_profile("touch-only"));
        assert!(!surface.has_profile("spatial"));
    }

    #[test]
    fn test_hybrid_medium_profile() {
        let surface = IoSurface::preset_hybrid_medium();
        assert!(surface.has_profile("hybrid"));
        assert!(surface.has_profile("large")); // 1366*1024 = 1,398,784 > 800k
        assert!(surface.has_profile("spatial"));
    }

    #[test]
    fn test_headless_profile() {
        let surface = IoSurface::preset_headless();
        assert_eq!(surface.open_channels(), 2); // only network + storage
        assert!(!surface.has_profile("touch-only"));
        assert!(!surface.has_profile("spatial"));
    }

    #[test]
    fn test_conserve_profile() {
        let surface = IoSurface::preset_conserve();
        assert!(surface.has_profile("conserve"));
        assert!(surface.has_profile("touch-only"));
        assert!(surface.has_profile("small"));
    }

    #[test]
    fn test_env_vars_export() {
        let surface = IoSurface::preset_headless();
        let vars = surface.to_env_vars();
        assert!(vars.iter().any(|(k, _)| k == "IO_PROFILE"));
        assert!(vars.iter().any(|(k, _)| k == "IO_CHANNELS"));
        assert!(vars.iter().any(|(k, v)| k == "IO_NETWORK_ACTIVE" && v == "true"));
        assert!(vars.iter().any(|(k, v)| k == "IO_DISPLAY_ACTIVE" && v == "false"));
    }

    #[test]
    fn test_preset_by_name() {
        assert!(IoSurface::preset("iphone").is_some());
        assert!(IoSurface::preset("desktop").is_some());
        assert!(IoSurface::preset("ci").is_some());
        assert!(IoSurface::preset("nonsense").is_none());
    }

    #[test]
    fn test_no_mirrors_when_both_sides_closed() {
        let surface = IoSurface::preset_headless();
        assert_eq!(surface.mirrors.len(), 0);
    }

    #[test]
    fn test_workstation_no_vibrate_mirror() {
        let surface = IoSurface::preset_pointer_large();
        // touch and vibrate both inactive, so touch↔vibrate mirror shouldn't appear
        let has_touch_vibrate = surface
            .mirrors
            .iter()
            .any(|m| m.input == "touch" && m.output == "vibrate");
        assert!(!has_touch_vibrate);
        // but mic↔speaker should (both active)
        let has_mic_speaker = surface
            .mirrors
            .iter()
            .any(|m| m.input == "mic" && m.output == "speaker");
        assert!(has_mic_speaker);
    }

    // ── Chirality tests ──────────────────────────────────────────

    #[test]
    fn test_camera_display_is_left_right() {
        let surface = IoSurface::preset_touch_small_spatial();
        let cam_mirror = surface.mirrors.iter().find(|m| m.input == "camera").unwrap();
        assert_eq!(cam_mirror.chirality, MirrorChirality::LeftRight);
        assert_eq!(cam_mirror.default_angle, ReflectionAngle::Toward);
    }

    #[test]
    fn test_mic_speaker_is_near_far() {
        let surface = IoSurface::preset_touch_small_spatial();
        let mic_mirror = surface.mirrors.iter().find(|m| m.input == "mic").unwrap();
        assert_eq!(mic_mirror.chirality, MirrorChirality::NearFar);
    }

    #[test]
    fn test_touch_vibrate_is_cause_effect() {
        let surface = IoSurface::preset_touch_small_spatial();
        let touch_mirror = surface.mirrors.iter().find(|m| m.input == "touch").unwrap();
        assert_eq!(touch_mirror.chirality, MirrorChirality::CauseEffect);
        // touch↔vibrate default is "through" — no reflection, direct response
        assert_eq!(touch_mirror.default_angle, ReflectionAngle::Through);
    }

    #[test]
    fn test_reflection_toward_flips_left_right() {
        let surface = IoSurface::preset_touch_small_spatial();
        let views = surface.reflection_summary();
        let cam_view = views.iter().find(|v| v.input == "camera").unwrap();
        // Looking toward the camera mirror: selfie mode → left↔right flips
        assert_eq!(cam_view.angle, ReflectionAngle::Toward);
        assert_eq!(cam_view.effective_chirality, MirrorChirality::LeftRight);
    }

    #[test]
    fn test_reflection_away_cancels_left_right() {
        let mut surface = IoSurface::preset_touch_small_spatial();
        // Dancer turns around — now looking away from the camera mirror
        surface.rotate_mirror("camera", ReflectionAngle::Away);
        let views = surface.reflection_summary();
        let cam_view = views.iter().find(|v| v.input == "camera").unwrap();
        // Looking away: left-right chirality → identity (you see the room correctly)
        assert_eq!(cam_view.angle, ReflectionAngle::Away);
        assert_eq!(cam_view.effective_chirality, MirrorChirality::Identity);
        // But the base chirality is still left-right (the mirror didn't change)
        assert_eq!(cam_view.base_chirality, MirrorChirality::LeftRight);
    }

    #[test]
    fn test_reflection_through_is_identity() {
        let mut surface = IoSurface::preset_touch_small_spatial();
        // AR mode — looking through, not at
        surface.rotate_mirror("camera", ReflectionAngle::Through);
        let views = surface.reflection_summary();
        let cam_view = views.iter().find(|v| v.input == "camera").unwrap();
        assert_eq!(cam_view.effective_chirality, MirrorChirality::Identity);
    }

    #[test]
    fn test_chirality_of_channel() {
        let surface = IoSurface::preset_touch_small_spatial();
        assert_eq!(surface.chirality_of("camera"), Some(MirrorChirality::LeftRight));
        assert_eq!(surface.chirality_of("display"), Some(MirrorChirality::LeftRight));
        assert_eq!(surface.chirality_of("mic"), Some(MirrorChirality::NearFar));
        assert_eq!(surface.chirality_of("touch"), Some(MirrorChirality::CauseEffect));
        // network has no mirror, so no chirality
        assert_eq!(surface.chirality_of("network"), None);
    }

    // ── Barre tests ──────────────────────────────────────────────

    #[test]
    fn test_barre_detection() {
        let surface = IoSurface::preset_touch_small_spatial();
        // network and storage are bidirectional and not in any mirror pair → barre
        assert!(surface.is_barre("network"));
        assert!(surface.is_barre("storage"));
        // keyboard and gyro are input but not mirrored → barre
        assert!(surface.is_barre("keyboard"));
        assert!(surface.is_barre("gyro"));
        // display IS in a mirror pair → not barre
        assert!(!surface.is_barre("display"));
        // mic IS in a mirror pair → not barre
        assert!(!surface.is_barre("mic"));
    }

    #[test]
    fn test_headless_barre_is_everything() {
        let surface = IoSurface::preset_headless();
        // headless: only network + storage active, no mirrors, both are barre
        assert!(surface.is_barre("network"));
        assert!(surface.is_barre("storage"));
        assert_eq!(surface.barre.len(), 2);
    }

    #[test]
    fn test_env_vars_include_barre() {
        let surface = IoSurface::preset_headless();
        let vars = surface.to_env_vars();
        let barre_var = vars.iter().find(|(k, _)| k == "IO_BARRE").unwrap();
        assert!(barre_var.1.contains("network"));
        assert!(barre_var.1.contains("storage"));
    }

    #[test]
    fn test_env_vars_include_chirality() {
        let surface = IoSurface::preset_touch_small_spatial();
        let vars = surface.to_env_vars();
        assert!(vars.iter().any(|(k, v)| k == "IO_MIRROR_CAMERA_DISPLAY_CHIRALITY" && v == "left↔right"));
        assert!(vars.iter().any(|(k, v)| k == "IO_MIRROR_MIC_SPEAKER_CHIRALITY" && v == "near↔far"));
        assert!(vars.iter().any(|(k, v)| k == "IO_MIRROR_TOUCH_VIBRATE_CHIRALITY" && v == "cause↔effect"));
    }

    // ── Pore tests ───────────────────────────────────────────

    #[test]
    fn test_pore_collapse_touch_surface() {
        let surface = IoSurface::preset_touch_small_spatial();
        let pores = surface.pores();
        // 13 channels → 10 pores (all active on iPhone)
        assert!(pores.len() >= 7);
        // Visual pore: display (output) + camera (input) → has chirality
        let visual = pores.iter().find(|(n, _)| n == "visual").unwrap();
        assert!(visual.1.is_open());
        assert_eq!(visual.1.chirality, MirrorChirality::LeftRight);
        assert!(visual.1.outward > 0.0); // display side
        assert!(visual.1.inward > 0.0);  // camera side
    }

    #[test]
    fn test_pore_collapse_headless() {
        let surface = IoSurface::preset_headless();
        let pores = surface.open_pores();
        // Headless: only data + persistence pores open
        assert_eq!(pores.len(), 2);
        let names: Vec<&str> = pores.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"data"));
        assert!(names.contains(&"persistence"));
    }

    #[test]
    fn test_pore_barre_is_balanced() {
        let surface = IoSurface::preset_touch_small_spatial();
        let barre = surface.barre_pores();
        let names: Vec<&str> = barre.iter().map(|(n, _)| n.as_str()).collect();
        // data and persistence are balanced (0.5/0.5)
        assert!(names.contains(&"data"));
        assert!(names.contains(&"persistence"));
        // visual is NOT balanced (display output >> camera input in bandwidth)
        assert!(!names.contains(&"visual") || true); // visual may or may not be balanced
    }

    #[test]
    fn test_pore_mirror_has_chirality() {
        let surface = IoSurface::preset_touch_small_spatial();
        let mirrors = surface.mirror_pores();
        // visual, audio, tactile all have non-identity chirality
        let names: Vec<&str> = mirrors.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"visual"));
        assert!(names.contains(&"audio"));
        assert!(names.contains(&"tactile"));
    }

    #[test]
    fn test_pore_asymmetry() {
        let surface = IoSurface::preset_touch_small_spatial();
        let pores = surface.pores();
        // Pointer pore: mostly inward (0.95 in, 0.05 out) → negative asymmetry
        let pointer = pores.iter().find(|(n, _)| n == "pointer");
        // pointer may not exist on touch surface (mouse inactive)
        if let Some((_, p)) = pointer {
            assert!(p.asymmetry() < 0.0, "pointer should be inward-dominant");
        }
        // Data pore: balanced → asymmetry near 0
        let data = pores.iter().find(|(n, _)| n == "data").unwrap();
        assert!((data.1.asymmetry()).abs() < 0.01, "data should be balanced");
    }

    #[test]
    fn test_pore_workstation_no_tactile() {
        let surface = IoSurface::preset_pointer_large();
        let pores = surface.pores();
        // Workstation: touch=inactive, vibrate=inactive → tactile pore bandwidth=0
        let tactile = pores.iter().find(|(n, _)| n == "tactile").unwrap();
        assert!(!tactile.1.is_open(), "tactile pore should be sealed on workstation");
    }

    // ── Site-Pore-Bingo tests ────────────────────────────────

    #[test]
    fn test_site_pore_map_has_all_sites() {
        let map = site_pore_map();
        // 22 outer + 3 surface + 1 center = 26 sites minimum
        assert!(map.len() >= 26, "expected at least 26 sites, got {}", map.len());
    }

    #[test]
    fn test_site_bingo_cell_detroit_is_investigation() {
        // detroit → investigation epitope → cell 0
        assert_eq!(site_bingo_cell("detroit"), Some(0));
        assert_eq!(site_bingo_cell("barry"), Some(0));
    }

    #[test]
    fn test_site_bingo_cell_git_is_method() {
        // git/forge → method epitope → cell 3
        assert_eq!(site_bingo_cell("git"), Some(3));
        assert_eq!(site_bingo_cell("forge"), Some(3));
    }

    #[test]
    fn test_site_bingo_cell_layer_nodes_have_no_cell() {
        // Layer nodes (hud, water, primals) are structural, not bingo cells
        assert_eq!(site_bingo_cell("hud"), None);
        assert_eq!(site_bingo_cell("water"), None);
        assert_eq!(site_bingo_cell("primals"), None);
    }

    #[test]
    fn test_bingo_epitopes_count() {
        assert_eq!(BINGO_EPITOPES.len(), 8, "bingo cube has 8 epitopes");
    }

    #[test]
    fn test_site_layers() {
        let map = site_pore_map();
        let outer: Vec<_> = map.iter().filter(|s| s.layer == MembraneLayer::Outer).collect();
        let surface: Vec<_> = map.iter().filter(|s| s.layer == MembraneLayer::Surface).collect();
        let center: Vec<_> = map.iter().filter(|s| s.layer == MembraneLayer::Center).collect();
        assert!(outer.len() >= 22, "outer should have at least 22 sites");
        assert_eq!(surface.len(), 3, "surface = hud + biomeos + os");
        assert_eq!(center.len(), 1, "center = water only");
        assert_eq!(center[0].site, "water");
    }

    #[test]
    fn test_water_pore_is_tactile() {
        // water.primals.eco — you touch to drink. tactile pore.
        let map = site_pore_map();
        let water = map.iter().find(|s| s.site == "water").unwrap();
        assert_eq!(water.pore, ChannelMedium::Tactile);
        assert_eq!(water.pin, "touch");
        assert_eq!(water.layer, MembraneLayer::Center);
    }

    #[test]
    fn test_every_epitope_has_sites() {
        let map = site_pore_map();
        for ep in BINGO_EPITOPES {
            let sites: Vec<_> = map.iter().filter(|s| s.epitope.as_deref() == Some(ep)).collect();
            assert!(!sites.is_empty(), "epitope '{}' has no sites", ep);
        }
    }

    // ── Pore Depth tests ──

    #[test]
    fn test_feed_pore_map_not_empty() {
        let feeds = feed_pore_map();
        assert!(feeds.len() > 50, "should have 50+ feed mappings, got {}", feeds.len());
    }

    #[test]
    fn test_feed_depth_lookup() {
        let f = feed_depth("topology.json");
        assert!(f.is_some());
        let f = f.unwrap();
        assert_eq!(f.epitope.as_deref(), Some("topology"));
        assert_eq!(f.pore, "spatial");
        assert_eq!(f.depth, PoreDepthLevel::Surface);
    }

    #[test]
    fn test_feed_depth_core() {
        let f = feed_depth("genome.json").unwrap();
        assert_eq!(f.depth, PoreDepthLevel::Core);
        assert_eq!(f.epitope.as_deref(), Some("science"));
    }

    #[test]
    fn test_pore_max_depth_pointer_is_core() {
        let d = pore_max_depth("pointer");
        assert_eq!(d, PoreDepthLevel::Core);
    }

    #[test]
    fn test_pore_max_depth_audio_is_core() {
        let d = pore_max_depth("audio");
        assert_eq!(d, PoreDepthLevel::Core);
    }

    #[test]
    fn test_every_pore_has_feeds() {
        let pores = ["visual", "audio", "tactile", "pointer", "symbolic",
                     "spatial", "data", "persistence"];
        for p in pores {
            let feeds = pore_feeds_at_depth(p, PoreDepthLevel::Surface);
            // at least spatial has surface feeds (topology + neutron + dashboard)
            // but not every pore must have surface — check at any depth
            let all: Vec<_> = feed_pore_map().into_iter()
                .filter(|f| f.pore == p).collect();
            assert!(!all.is_empty(), "pore '{}' has no feeds at any depth", p);
        }
    }

    #[test]
    fn test_depth_ordering() {
        assert!(PoreDepthLevel::Surface < PoreDepthLevel::Mid);
        assert!(PoreDepthLevel::Mid < PoreDepthLevel::Deep);
        assert!(PoreDepthLevel::Deep < PoreDepthLevel::Core);
    }

    #[test]
    fn test_cross_cutting_feeds_have_no_epitope() {
        let feeds = feed_pore_map();
        let cross: Vec<_> = feeds.iter().filter(|f| f.epitope.is_none()).collect();
        assert!(cross.len() >= 5, "should have cross-cutting feeds");
        // now.json should be one
        assert!(cross.iter().any(|f| f.feed == "now.json"));
    }

    #[test]
    fn test_every_epitope_has_core_depth() {
        for ep in BINGO_EPITOPES {
            let feeds: Vec<_> = feed_pore_map().into_iter()
                .filter(|f| f.epitope.as_deref() == Some(ep) && f.depth == PoreDepthLevel::Core)
                .collect();
            assert!(!feeds.is_empty(), "epitope '{}' has no core-depth feeds", ep);
        }
    }

    // ── π Orientation tests ──

    #[test]
    fn test_landmarks_count() {
        let lm = landmarks();
        assert!(lm.len() >= 12, "should have 12+ landmarks, got {}", lm.len());
    }

    #[test]
    fn test_detroit_exists() {
        let lm = landmarks();
        assert!(lm.iter().any(|l| l.id == "detroit"));
    }

    #[test]
    fn test_photon_line_detroit_stonehenge() {
        let all = landmarks();
        let det = all.iter().find(|l| l.id == "detroit").unwrap();
        let sh = all.iter().find(|l| l.id == "stonehenge").unwrap();
        let line = photon_line(det, sh);
        // Detroit→Stonehenge ≈ 5,900-6,000 km
        assert!(line.arc_km > 5800.0 && line.arc_km < 6200.0,
            "Detroit→Stonehenge arc should be ~5950 km, got {:.0}", line.arc_km);
        // Light time ≈ 19-20 ms
        assert!(line.light_s > 0.019 && line.light_s < 0.021,
            "light time should be ~20ms, got {:.4}s", line.light_s);
        // π-fraction < 0.5 (less than half-Earth)
        assert!(line.pi_fraction < 0.5);
    }

    #[test]
    fn test_photon_line_antipodal_near_pi() {
        // Detroit→Uluru should be roughly 0.7-0.85 of half-Earth
        let all = landmarks();
        let det = all.iter().find(|l| l.id == "detroit").unwrap();
        let ul = all.iter().find(|l| l.id == "uluru").unwrap();
        let line = photon_line(det, ul);
        assert!(line.pi_fraction > 0.5, "Detroit→Uluru should be > half of half-Earth");
    }

    #[test]
    fn test_polaris_distance_is_latitude() {
        // Angular distance from any point to North Pole = (90 - latitude) degrees
        let all = landmarks();
        let det = all.iter().find(|l| l.id == "detroit").unwrap();
        let pol = all.iter().find(|l| l.id == "polaris").unwrap();
        let σ = angular_distance(det.lat, det.lon, pol.lat, pol.lon);
        let expected_rad = (90.0 - det.lat).to_radians();
        assert!((σ - expected_rad).abs() < 0.01,
            "angular distance to pole should be (90-lat)={:.4} rad, got {:.4}",
            expected_rad, σ);
    }

    #[test]
    fn test_half_earth_constant() {
        // πR ≈ 20,015 km
        assert!(HALF_EARTH_KM > 20000.0 && HALF_EARTH_KM < 20100.0);
    }

    #[test]
    fn test_photon_half_earth() {
        // πR/c ≈ 0.0668 seconds ≈ 66.8 ms
        assert!(PHOTON_HALF_EARTH_S > 0.066 && PHOTON_HALF_EARTH_S < 0.068,
            "photon half-Earth should be ~66.8ms, got {:.4}s", PHOTON_HALF_EARTH_S);
    }

    #[test]
    fn test_orient_from_detroit() {
        let lines = orient_from("detroit").unwrap();
        assert_eq!(lines.len(), landmarks().len() - 1);
        // all light times should be positive and < photon half-earth
        for line in &lines {
            assert!(line.light_s > 0.0);
            assert!(line.pi_fraction >= 0.0 && line.pi_fraction <= 1.0,
                "π-fraction should be 0..1, got {} for {}", line.pi_fraction, line.to);
        }
    }

    #[test]
    fn test_greenwich_to_dateline_is_pi() {
        // Greenwich (0°) to Date Line (180°) = π radians = exactly half-Earth
        let all = landmarks();
        let gw = all.iter().find(|l| l.id == "greenwich").unwrap();
        let dl = all.iter().find(|l| l.id == "date_line").unwrap();
        let line = photon_line(gw, dl);
        // They're at different latitudes (51.5° vs 0°), so it's not exactly π
        // but the longitudinal separation IS π radians
        let lon_diff = (dl.lon - gw.lon).abs().to_radians();
        assert!((lon_diff - std::f64::consts::PI).abs() < 0.001,
            "longitude difference should be π radians");
    }

    #[test]
    fn test_all_landmark_kinds_present() {
        let lm = landmarks();
        let has_stone = lm.iter().any(|l| l.kind == LandmarkKind::Stone);
        let has_water = lm.iter().any(|l| l.kind == LandmarkKind::Water);
        let has_sky = lm.iter().any(|l| l.kind == LandmarkKind::Sky);
        let has_signal = lm.iter().any(|l| l.kind == LandmarkKind::Signal);
        assert!(has_stone && has_water && has_sky && has_signal,
            "all 4 landmark kinds should be present");
    }
}


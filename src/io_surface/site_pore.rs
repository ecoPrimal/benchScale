// SPDX-License-Identifier: AGPL-3.0-or-later
//! Site-Pore-Bingo Map — wildcards are touch pins on the membrane.

use serde::{Deserialize, Serialize};

use super::core::*;

// ═══════════════════════════════════════════════════════════════════════════
// Site-Pore-Bingo Map — wildcards are touch pins on the membrane
//
// Three layers:
//   OUTER: primals.eco, *.primals.eco (public face — touch pins)
//   SURFACE: hud.primals.eco (the window you always look through)
//   CENTER: water.primals.eco (the watering hole — vocal transit converges)
//
// Each wildcard subdomain IS a bingo cell. Touching one transits
// through the telephone wire's vocal system.
// ═══════════════════════════════════════════════════════════════════════════

/// Which layer a site belongs to in the membrane architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MembraneLayer {
    /// Public face: primals.eco, detroit, tuebor, sporePrint...
    Outer,
    /// The window: hud.primals.eco — you always look through it
    Surface,
    /// The watering hole: water.primals.eco — onboard here, middle-out
    Center,
}

/// A wildcard subdomain mapped to its epitope, pore, and interaction mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitePore {
    /// The subdomain (e.g. "detroit", "signal", "water")
    pub site: String,
    /// Which bingo epitope this site belongs to (None for layer nodes)
    pub epitope: Option<String>,
    /// Which I/O pore modality governs interaction
    pub pore: ChannelMedium,
    /// Touch pin or pointer pin — how you physically activate it
    pub pin: String,
    /// Which membrane layer
    pub layer: MembraneLayer,
}

/// The 8 bingo epitopes, indexed to match the bingo cube grid
pub const BINGO_EPITOPES: &[&str] = &[
    "investigation", // 0 — detroit, barry
    "topology",      // 1 — topology, mesh
    "philosophy",    // 2 — hypothesis, thesis
    "method",        // 3 — git, forge, lab, depot
    "immune",        // 4 — membrane, identity, ca
    "science",       // 5 — sporeprint, clutch, gorilla
    "signal",        // 6 — signal, notify, pollination, pollen
    "physics",       // 7 — neutron, live
];

/// Build the complete site-pore map for all known *.primals.eco subdomains
pub fn site_pore_map() -> Vec<SitePore> {
    vec![
        // INVESTIGATION epitope: pointer pore (click to investigate)
        SitePore { site: "detroit".into(), epitope: Some("investigation".into()), pore: ChannelMedium::Pointer, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "barry".into(), epitope: Some("investigation".into()), pore: ChannelMedium::Pointer, pin: "touch".into(), layer: MembraneLayer::Outer },
        // TOPOLOGY epitope: spatial pore (navigate structure)
        SitePore { site: "topology".into(), epitope: Some("topology".into()), pore: ChannelMedium::Spatial, pin: "pointer".into(), layer: MembraneLayer::Outer },
        SitePore { site: "mesh".into(), epitope: Some("topology".into()), pore: ChannelMedium::Spatial, pin: "pointer".into(), layer: MembraneLayer::Outer },
        // PHILOSOPHY epitope: symbolic pore (read, think, type)
        SitePore { site: "hypothesis".into(), epitope: Some("philosophy".into()), pore: ChannelMedium::Symbolic, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "thesis".into(), epitope: Some("philosophy".into()), pore: ChannelMedium::Symbolic, pin: "touch".into(), layer: MembraneLayer::Outer },
        // METHOD epitope: data pore (forge, code, build)
        SitePore { site: "git".into(), epitope: Some("method".into()), pore: ChannelMedium::Data, pin: "pointer".into(), layer: MembraneLayer::Outer },
        SitePore { site: "forge".into(), epitope: Some("method".into()), pore: ChannelMedium::Data, pin: "pointer".into(), layer: MembraneLayer::Outer },
        SitePore { site: "lab".into(), epitope: Some("method".into()), pore: ChannelMedium::Data, pin: "pointer".into(), layer: MembraneLayer::Outer },
        SitePore { site: "depot".into(), epitope: Some("method".into()), pore: ChannelMedium::Data, pin: "pointer".into(), layer: MembraneLayer::Outer },
        // IMMUNE epitope: persistence pore (boundary, identity)
        SitePore { site: "membrane".into(), epitope: Some("immune".into()), pore: ChannelMedium::Persistence, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "identity".into(), epitope: Some("immune".into()), pore: ChannelMedium::Persistence, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "ca".into(), epitope: Some("immune".into()), pore: ChannelMedium::Persistence, pin: "pointer".into(), layer: MembraneLayer::Outer },
        // SCIENCE epitope: visual pore (observe, read, data viz)
        SitePore { site: "sporeprint".into(), epitope: Some("science".into()), pore: ChannelMedium::Visual, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "clutch".into(), epitope: Some("science".into()), pore: ChannelMedium::Visual, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "gorilla".into(), epitope: Some("science".into()), pore: ChannelMedium::Visual, pin: "touch".into(), layer: MembraneLayer::Outer },
        // SIGNAL epitope: audio pore (listen, relay, beacon)
        SitePore { site: "signal".into(), epitope: Some("signal".into()), pore: ChannelMedium::Audio, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "notify".into(), epitope: Some("signal".into()), pore: ChannelMedium::Audio, pin: "pointer".into(), layer: MembraneLayer::Outer },
        SitePore { site: "pollination".into(), epitope: Some("signal".into()), pore: ChannelMedium::Audio, pin: "pointer".into(), layer: MembraneLayer::Outer },
        SitePore { site: "pollen".into(), epitope: Some("signal".into()), pore: ChannelMedium::Audio, pin: "pointer".into(), layer: MembraneLayer::Outer },
        // PHYSICS epitope: spatial pore (live state, neutron flow)
        SitePore { site: "neutron".into(), epitope: Some("physics".into()), pore: ChannelMedium::Spatial, pin: "pointer".into(), layer: MembraneLayer::Outer },
        SitePore { site: "live".into(), epitope: Some("physics".into()), pore: ChannelMedium::Spatial, pin: "pointer".into(), layer: MembraneLayer::Outer },
        // LAYER NODES — structural, not epitopes
        SitePore { site: "primals".into(), epitope: None, pore: ChannelMedium::Visual, pin: "touch".into(), layer: MembraneLayer::Outer },
        SitePore { site: "hud".into(), epitope: None, pore: ChannelMedium::Visual, pin: "pointer".into(), layer: MembraneLayer::Surface },
        SitePore { site: "biomeos".into(), epitope: None, pore: ChannelMedium::Visual, pin: "pointer".into(), layer: MembraneLayer::Surface },
        SitePore { site: "os".into(), epitope: None, pore: ChannelMedium::Visual, pin: "pointer".into(), layer: MembraneLayer::Surface },
        SitePore { site: "water".into(), epitope: None, pore: ChannelMedium::Tactile, pin: "touch".into(), layer: MembraneLayer::Center },
    ]
}

/// Lookup a site's bingo cell index (0-7, or None for layer nodes)
pub fn site_bingo_cell(site: &str) -> Option<usize> {
    let map = site_pore_map();
    let sp = map.iter().find(|s| s.site == site)?;
    let epitope = sp.epitope.as_ref()?;
    BINGO_EPITOPES.iter().position(|e| e == epitope)
}

impl IoSurface {
    /// Create an empty surface (no channels probed yet)
    pub fn empty() -> Self {
        Self {
            channels: Vec::new(),
            profile: Vec::new(),
            mirrors: Vec::new(),
            barre: Vec::new(),
        }
    }

    /// Build a surface from a set of channels, auto-detecting mirrors and barre
    pub fn from_channels(channels: Vec<IoChannel>) -> Self {
        let mirrors = Self::detect_mirrors(&channels);
        let profile = Self::classify(&channels);
        let barre = Self::detect_barre(&channels);
        Self {
            channels,
            profile,
            mirrors,
            barre,
        }
    }

    /// Detect barre channels — structural references that don't flip.
    /// These are bidirectional channels without mirrors. They're the pole
    /// the dancer holds: network, storage. Always there, no chirality.
    fn detect_barre(channels: &[IoChannel]) -> Vec<String> {
        let mirror_names: std::collections::HashSet<&str> = MIRROR_PAIRS
            .iter()
            .flat_map(|(i, o, _, _, _)| [*i, *o])
            .collect();

        channels
            .iter()
            .filter(|ch| ch.active)
            .filter(|ch| !mirror_names.contains(ch.name.as_str()))
            .filter(|ch| {
                matches!(
                    ch.direction,
                    ChannelDirection::Bidirectional | ChannelDirection::Input
                )
            })
            .map(|ch| ch.name.clone())
            .collect()
    }

    /// Detect mirror pairs among active channels, assigning chirality
    fn detect_mirrors(channels: &[IoChannel]) -> Vec<MirrorPair> {
        let names: std::collections::HashSet<&str> = channels
            .iter()
            .filter(|ch| ch.active)
            .map(|ch| ch.name.as_str())
            .collect();

        MIRROR_PAIRS
            .iter()
            .filter(|(input, output, _, _, _)| names.contains(input) && names.contains(output))
            .map(|(input, output, medium, chirality, angle)| MirrorPair {
                input: (*input).to_string(),
                output: (*output).to_string(),
                medium: *medium,
                chirality: *chirality,
                default_angle: *angle,
            })
            .collect()
    }

    /// Classify the surface profile from its channels
    fn classify(channels: &[IoChannel]) -> Vec<String> {
        let active = |name: &str| -> bool {
            channels.iter().any(|ch| ch.name == name && ch.active)
        };
        let prop_f64 = |name: &str, key: &str| -> Option<f64> {
            channels
                .iter()
                .find(|ch| ch.name == name)
                .and_then(|ch| ch.properties.get(key))
                .and_then(|v| v.as_f64())
        };
        let prop_bool = |name: &str, key: &str| -> bool {
            channels
                .iter()
                .find(|ch| ch.name == name)
                .and_then(|ch| ch.properties.get(key))
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        };

        let has_touch = active("touch");
        let has_pointer = active("mouse");
        let has_gyro = active("gyro");

        let width = prop_f64("display", "width").unwrap_or(1920.0);
        let height = prop_f64("display", "height").unwrap_or(1080.0);
        let viewport = width * height;

        let low_battery = prop_f64("battery", "level")
            .map(|l| l < 0.15)
            .unwrap_or(false)
            && !prop_bool("battery", "charging");
        let save_data = prop_bool("network", "save_data");

        let mut profile = Vec::new();

        // Input classification
        if has_touch && !has_pointer {
            profile.push("touch-only".to_string());
        }
        if has_touch && has_pointer {
            profile.push("hybrid".to_string());
        }
        if !has_touch && has_pointer {
            profile.push("pointer-only".to_string());
        }

        // Viewport classification (CSS pixels, not physical)
        // small: phones in portrait (<400k: up to ~430×930)
        // medium: tablets, small laptops (<1.2M: up to ~1200×1000)
        // large: desktops, laptops, tablets landscape (>=1.2M)
        if viewport < 400_000.0 {
            profile.push("small".to_string());
        } else if viewport < 1_200_000.0 {
            profile.push("medium".to_string());
        } else {
            profile.push("large".to_string());
        }

        // Spatial
        if has_gyro {
            profile.push("spatial".to_string());
        }

        // Energy conservation
        if low_battery || save_data {
            profile.push("conserve".to_string());
        }

        profile
    }

    /// Get a channel by name
    pub fn channel(&self, name: &str) -> Option<&IoChannel> {
        self.channels.iter().find(|ch| ch.name == name)
    }

    /// How many channels are open (active)
    pub fn open_channels(&self) -> usize {
        self.channels.iter().filter(|ch| ch.active).count()
    }

    /// Whether this surface matches a profile tag
    pub fn has_profile(&self, tag: &str) -> bool {
        self.profile.iter().any(|p| p == tag)
    }

    /// Whether a channel is a barre (structural reference, no chirality)
    pub fn is_barre(&self, name: &str) -> bool {
        self.barre.iter().any(|b| b == name)
    }

    /// Collapse this surface into pores — the bidirectional view.
    /// 13 channels become ~7-10 pores, each with inward/outward permeability.
    pub fn pores(&self) -> Vec<(String, IoPore)> {
        collapse_to_pores(self)
    }

    /// Get just the open pores (nonzero permeability in either direction)
    pub fn open_pores(&self) -> Vec<(String, IoPore)> {
        self.pores().into_iter().filter(|(_, p)| p.is_open()).collect()
    }

    /// The barre pores — balanced permeability, structural references.
    /// These are the pole the dancer holds. Network, storage.
    pub fn barre_pores(&self) -> Vec<(String, IoPore)> {
        self.pores().into_iter().filter(|(_, p)| p.is_balanced()).collect()
    }

    /// The mirror pores — dominant permeability in one direction with chirality.
    /// These are the reflective surfaces. Visual, audio, tactile.
    pub fn mirror_pores(&self) -> Vec<(String, IoPore)> {
        self.pores()
            .into_iter()
            .filter(|(_, p)| p.chirality != MirrorChirality::Identity && p.is_open())
            .collect()
    }

    /// Get the chirality of a mirror pair by the input channel name.
    /// Returns None if the channel isn't part of a mirror.
    pub fn chirality_of(&self, channel_name: &str) -> Option<MirrorChirality> {
        self.mirrors
            .iter()
            .find(|m| m.input == channel_name || m.output == channel_name)
            .map(|m| m.chirality)
    }

    /// Rotate a mirror pair to a different reflection angle.
    ///
    /// Like the dancer turning: same mirror, same chirality, different view.
    /// - **Toward**: selfie mode — you see yourself, left↔right flipped
    /// - **Away**: rear camera — you see behind you, different chirality
    /// - **Through**: AR mode — transparent, no flip at all
    pub fn rotate_mirror(&mut self, input_name: &str, angle: ReflectionAngle) {
        if let Some(mirror) = self.mirrors.iter_mut().find(|m| m.input == input_name) {
            mirror.default_angle = angle;
        }
    }

    /// What the dancer sees in each mirror from the current angle.
    /// Returns a description of the reflection state.
    pub fn reflection_summary(&self) -> Vec<ReflectionView> {
        self.mirrors
            .iter()
            .map(|m| {
                let effective_chirality = match m.default_angle {
                    ReflectionAngle::Toward => m.chirality,
                    ReflectionAngle::Away => match m.chirality {
                        // Looking away: left-right no longer flips (you see the room correctly)
                        MirrorChirality::LeftRight => MirrorChirality::Identity,
                        // Looking away through audio: near-far inverts differently
                        MirrorChirality::NearFar => MirrorChirality::NearFar,
                        other => other,
                    },
                    // Looking through: no reflection, no chirality
                    ReflectionAngle::Through => MirrorChirality::Identity,
                };

                ReflectionView {
                    input: m.input.clone(),
                    output: m.output.clone(),
                    medium: m.medium,
                    angle: m.default_angle,
                    base_chirality: m.chirality,
                    effective_chirality,
                }
            })
            .collect()
    }

    /// Export as environment variables for a benchScale node
    /// (so io-probe.js on the HUD and io_surface.rs in Rust agree)
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        let mut env = Vec::new();

        env.push((
            "IO_PROFILE".to_string(),
            self.profile.join(","),
        ));
        env.push((
            "IO_BARRE".to_string(),
            self.barre.join(","),
        ));
        env.push((
            "IO_CHANNELS".to_string(),
            self.channels.len().to_string(),
        ));
        env.push((
            "IO_OPEN".to_string(),
            self.open_channels().to_string(),
        ));

        for ch in &self.channels {
            let prefix = format!("IO_{}", ch.name.to_uppercase());
            env.push((format!("{prefix}_ACTIVE"), ch.active.to_string()));
            env.push((format!("{prefix}_DIRECTION"), ch.direction.to_string()));
            env.push((format!("{prefix}_MEDIUM"), ch.medium.to_string()));

            for (key, value) in &ch.properties {
                let val_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    other => other.to_string(),
                };
                env.push((format!("{prefix}_{}", key.to_uppercase()), val_str));
            }
        }

        for mirror in &self.mirrors {
            let key = format!("IO_MIRROR_{}_{}", mirror.input.to_uppercase(), mirror.output.to_uppercase());
            env.push((key.clone(), mirror.medium.to_string()));
            env.push((format!("{key}_CHIRALITY"), mirror.chirality.to_string()));
            env.push((format!("{key}_ANGLE"), mirror.default_angle.to_string()));
        }

        env
    }
}

// ─── PRESETS ────────────────────────────────────────────────────────────
// Named I/O surfaces for common test scenarios.
// Not "mobile" and "desktop" — these are channel configurations.

impl IoSurface {
    /// iPhone 14 Pro — touch-only, small, spatial, all sensors
    pub fn preset_touch_small_spatial() -> Self {
        Self::from_channels(vec![
            IoChannel {
                name: "display".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Visual,
                active: true,
                mirror: None,
                bandwidth: 390.0 * 844.0 * 3.0,
                properties: serde_json::Map::from_iter([
                    ("width".into(), 390.0.into()),
                    ("height".into(), 844.0.into()),
                    ("dpr".into(), 3.0.into()),
                    ("color_depth".into(), 30.into()),
                ]),
            },
            IoChannel {
                name: "touch".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Tactile,
                active: true,
                mirror: None,
                bandwidth: 5.0 * 120.0,
                properties: serde_json::Map::from_iter([
                    ("points".into(), 5.into()),
                    ("coarse".into(), true.into()),
                ]),
            },
            IoChannel {
                name: "mouse".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Pointer,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::from_iter([
                    ("hover".into(), false.into()),
                    ("fine".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "keyboard".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Symbolic,
                active: true,
                mirror: None,
                bandwidth: 15.0,
                properties: serde_json::Map::from_iter([
                    ("virtual".into(), true.into()),
                ]),
            },
            IoChannel {
                name: "speaker".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Audio,
                active: true,
                mirror: None,
                bandwidth: 48000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "mic".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Audio,
                active: true,
                mirror: Some("speaker".into()),
                bandwidth: 48000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "camera".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Visual,
                active: true,
                mirror: Some("display".into()),
                bandwidth: 12_000_000.0 * 30.0,
                properties: serde_json::Map::from_iter([
                    ("megapixels".into(), 48.into()),
                ]),
            },
            IoChannel {
                name: "gyro".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Spatial,
                active: true,
                mirror: None,
                bandwidth: 100.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "vibrate".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Tactile,
                active: true,
                mirror: Some("touch".into()),
                bandwidth: 1.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "network".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Data,
                active: true,
                mirror: None,
                bandwidth: 30_000_000.0,
                properties: serde_json::Map::from_iter([
                    ("effective_type".into(), "4g".into()),
                    ("downlink_mbps".into(), 30.into()),
                    ("rtt_ms".into(), 50.into()),
                    ("save_data".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "storage".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Persistence,
                active: true,
                mirror: None,
                bandwidth: 10_000_000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "battery".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Energy,
                active: true,
                mirror: None,
                bandwidth: 0.001,
                properties: serde_json::Map::from_iter([
                    ("level".into(), 0.85.into()),
                    ("charging".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "gps".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Position,
                active: true,
                mirror: None,
                bandwidth: 1.0,
                properties: serde_json::Map::new(),
            },
        ])
    }

    /// Workstation — pointer-only, large, full I/O
    pub fn preset_pointer_large() -> Self {
        Self::from_channels(vec![
            IoChannel {
                name: "display".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Visual,
                active: true,
                mirror: None,
                bandwidth: 2560.0 * 1440.0 * 1.0,
                properties: serde_json::Map::from_iter([
                    ("width".into(), 2560.0.into()),
                    ("height".into(), 1440.0.into()),
                    ("dpr".into(), 1.0.into()),
                    ("color_depth".into(), 24.into()),
                ]),
            },
            IoChannel {
                name: "touch".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Tactile,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::from_iter([
                    ("points".into(), 0.into()),
                    ("coarse".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "mouse".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Pointer,
                active: true,
                mirror: None,
                bandwidth: 1000.0,
                properties: serde_json::Map::from_iter([
                    ("hover".into(), true.into()),
                    ("fine".into(), true.into()),
                ]),
            },
            IoChannel {
                name: "keyboard".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Symbolic,
                active: true,
                mirror: None,
                bandwidth: 15.0,
                properties: serde_json::Map::from_iter([
                    ("virtual".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "speaker".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Audio,
                active: true,
                mirror: None,
                bandwidth: 96000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "mic".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Audio,
                active: true,
                mirror: Some("speaker".into()),
                bandwidth: 48000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "camera".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Visual,
                active: true,
                mirror: Some("display".into()),
                bandwidth: 1920.0 * 1080.0 * 30.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "gyro".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Spatial,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "vibrate".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Tactile,
                active: false,
                mirror: Some("touch".into()),
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "network".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Data,
                active: true,
                mirror: None,
                bandwidth: 1_000_000_000.0,
                properties: serde_json::Map::from_iter([
                    ("effective_type".into(), "4g".into()),
                    ("downlink_mbps".into(), 1000.into()),
                    ("rtt_ms".into(), 2.into()),
                    ("save_data".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "storage".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Persistence,
                active: true,
                mirror: None,
                bandwidth: 500_000_000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "battery".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Energy,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "gps".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Position,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
        ])
    }

    /// Hybrid surface — laptop with touchscreen (Surface Pro, iPad + keyboard, etc.)
    pub fn preset_hybrid_medium() -> Self {
        Self::from_channels(vec![
            IoChannel {
                name: "display".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Visual,
                active: true,
                mirror: None,
                bandwidth: 1366.0 * 1024.0 * 2.0,
                properties: serde_json::Map::from_iter([
                    ("width".into(), 1366.0.into()),
                    ("height".into(), 1024.0.into()),
                    ("dpr".into(), 2.0.into()),
                ]),
            },
            IoChannel {
                name: "touch".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Tactile,
                active: true,
                mirror: None,
                bandwidth: 10.0 * 60.0,
                properties: serde_json::Map::from_iter([
                    ("points".into(), 10.into()),
                    ("coarse".into(), true.into()),
                ]),
            },
            IoChannel {
                name: "mouse".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Pointer,
                active: true,
                mirror: None,
                bandwidth: 1000.0,
                properties: serde_json::Map::from_iter([
                    ("hover".into(), true.into()),
                    ("fine".into(), true.into()),
                ]),
            },
            IoChannel {
                name: "keyboard".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Symbolic,
                active: true,
                mirror: None,
                bandwidth: 15.0,
                properties: serde_json::Map::from_iter([
                    ("virtual".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "speaker".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Audio,
                active: true,
                mirror: None,
                bandwidth: 48000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "mic".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Audio,
                active: true,
                mirror: Some("speaker".into()),
                bandwidth: 48000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "camera".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Visual,
                active: true,
                mirror: Some("display".into()),
                bandwidth: 1920.0 * 1080.0 * 30.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "gyro".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Spatial,
                active: true,
                mirror: None,
                bandwidth: 60.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "vibrate".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Tactile,
                active: false,
                mirror: Some("touch".into()),
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "network".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Data,
                active: true,
                mirror: None,
                bandwidth: 100_000_000.0,
                properties: serde_json::Map::from_iter([
                    ("effective_type".into(), "4g".into()),
                    ("downlink_mbps".into(), 100.into()),
                    ("rtt_ms".into(), 10.into()),
                    ("save_data".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "storage".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Persistence,
                active: true,
                mirror: None,
                bandwidth: 100_000_000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "battery".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Energy,
                active: true,
                mirror: None,
                bandwidth: 0.001,
                properties: serde_json::Map::from_iter([
                    ("level".into(), 0.72.into()),
                    ("charging".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "gps".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Position,
                active: true,
                mirror: None,
                bandwidth: 1.0,
                properties: serde_json::Map::new(),
            },
        ])
    }

    /// Headless / CI — no display, no touch, no audio. Just network + storage.
    pub fn preset_headless() -> Self {
        Self::from_channels(vec![
            IoChannel {
                name: "display".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Visual,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "touch".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Tactile,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "mouse".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Pointer,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "keyboard".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Symbolic,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "speaker".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Audio,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "mic".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Audio,
                active: false,
                mirror: Some("speaker".into()),
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "camera".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Visual,
                active: false,
                mirror: Some("display".into()),
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "gyro".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Spatial,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "vibrate".into(),
                direction: ChannelDirection::Output,
                medium: ChannelMedium::Tactile,
                active: false,
                mirror: Some("touch".into()),
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "network".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Data,
                active: true,
                mirror: None,
                bandwidth: 10_000_000_000.0,
                properties: serde_json::Map::from_iter([
                    ("effective_type".into(), "4g".into()),
                    ("downlink_mbps".into(), 10000.into()),
                    ("rtt_ms".into(), 1.into()),
                    ("save_data".into(), false.into()),
                ]),
            },
            IoChannel {
                name: "storage".into(),
                direction: ChannelDirection::Bidirectional,
                medium: ChannelMedium::Persistence,
                active: true,
                mirror: None,
                bandwidth: 3_000_000_000.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "battery".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Energy,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
            IoChannel {
                name: "gps".into(),
                direction: ChannelDirection::Input,
                medium: ChannelMedium::Position,
                active: false,
                mirror: None,
                bandwidth: 0.0,
                properties: serde_json::Map::new(),
            },
        ])
    }

    /// Low-power / conserve — touch, small screen, save-data, low battery
    pub fn preset_conserve() -> Self {
        let mut surface = Self::preset_touch_small_spatial();
        if let Some(batt) = surface.channels.iter_mut().find(|ch| ch.name == "battery") {
            batt.properties.insert("level".into(), 0.08.into());
            batt.properties.insert("charging".into(), false.into());
        }
        if let Some(net) = surface.channels.iter_mut().find(|ch| ch.name == "network") {
            net.properties.insert("save_data".into(), true.into());
            net.properties.insert("effective_type".into(), "2g".into());
            net.properties.insert("downlink_mbps".into(), (0.3).into());
            net.properties.insert("rtt_ms".into(), 600.into());
            net.bandwidth = 300_000.0;
        }
        surface.profile = Self::classify(&surface.channels);
        surface
    }

    /// Get a preset by name
    pub fn preset(name: &str) -> Option<Self> {
        match name {
            "touch-small-spatial" | "iphone" | "mobile" => Some(Self::preset_touch_small_spatial()),
            "pointer-large" | "workstation" | "desktop" => Some(Self::preset_pointer_large()),
            "hybrid-medium" | "laptop" | "surface" => Some(Self::preset_hybrid_medium()),
            "headless" | "ci" | "server" => Some(Self::preset_headless()),
            "conserve" | "low-power" | "2g" => Some(Self::preset_conserve()),
            _ => None,
        }
    }

    /// List all available preset names
    pub fn preset_names() -> &'static [&'static str] {
        &[
            "touch-small-spatial",
            "pointer-large",
            "hybrid-medium",
            "headless",
            "conserve",
        ]
    }
}


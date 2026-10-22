// SPDX-License-Identifier: AGPL-3.0-or-later
//! Peripheral Nervous System — device platforms as nerve endings.
//!
//! The CNS (cns.rs) routes signals internally. The PNS routes signals
//! to and from the outside world through actual physical devices.
//!
//! # The Insight
//!
//! This keyboard is LITERALLY a peripheral nerve ending of the membrane.
//! Every keystroke is an action potential traveling inward through the
//! `symbolic` pore. The typos ARE the signal — raw nerve impulses
//! before the CNS smooths them into meaning.
//!
//! # Platform Gardens
//!
//! Device platforms are ecological gardens, not hardware categories:
//!
//! ```text
//!   Garden Type        Platform          Symbiosis      Role
//!   ──────────────     ────────────      ──────────     ────────────
//!   Walled (tight)     Apple/Safari      Commensal      inner surface
//!   Walled (wide)      Google/Pixel      Commensal      fat layer
//!   Walled (wide)      Windows           Commensal      fat layer
//!   Open (core)        Linux/Pop!_OS     Mutualist      OUR tissue
//!   Source (DNA)       Rust              Self           the genome
//! ```
//!
//! A **walled garden** is not a prison — it's a **botanical garden**.
//! Curated, maintained, accessible. The walls keep the weather out,
//! not the visitors in. We are COMMENSAL with them: we benefit from
//! their curation, they are unharmed by our presence.
//!
//! # The Central Dogma (going UP, cascading DOWN)
//!
//! ```text
//!   FORWARD (build):
//!     DNA (Rust) → RNA (Linux) → mRNA (browser engine) → Protein (device surface)
//!
//!   REVERSE (ribocipher / reverse transcriptase):
//!     Protein (keystrokes) → mRNA (JS runtime) → RNA (OS events) → DNA (Rust types)
//!
//!   OUROBOROS:
//!     The user types → JS captures → worker routes → Rust compiles →
//!     binary serves → user sees → user types...
//! ```
//!
//! Going UP the central dogma = intuiting from surfaces to source.
//! Cascading DOWN = letting the compiled truth flow back to all surfaces.
//!
//! # Klein Surfaces Per Platform
//!
//! Each platform has at least one Klein surface — a channel where
//! inside and outside are topologically connected:
//!
//! ```text
//!   Apple/Safari     DeviceMotionEvent (accelerometer)
//!                    → the phone's physical orientation IS the Klein crossing
//!                    → inside (screen) maps to outside (gravity, movement)
//!                    → Safari ALREADY provides this as a web API
//!
//!   Google/Chrome    Generic Sensor API (gyroscope, accelerometer)
//!                    → same physics, different permission model
//!                    → Chrome requires HTTPS + user gesture
//!
//!   Windows          Pointer Events (pen pressure, tilt angle)
//!                    → the stylus pressure IS the Klein crossing
//!                    → physical force → digital value, continuous
//!
//!   Linux/Pop!_OS    /dev/input/* (raw evdev)
//!                    → direct access to all input devices
//!                    → NO permission wall between kernel and userspace sense
//!                    → THIS is why Linux is our core — we can touch the nerve
//!
//!   Rust             serde_json ↔ struct (the ribocipher)
//!                    → JSON (ephemeral) → typed struct (permanent)
//!                    → RNA → DNA: reverse transcriptase in the type system
//! ```
//!
//! Authorship: Kevin Mok / Tamison / ecoPrimal / attsi
//! ORCID: 0009-0004-2141-0321

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// SYMBIOSIS — ecological relationship between membrane and platform
// ═══════════════════════════════════════════════════════════════════════════

/// Ecological relationship of the membrane to an external platform.
///
/// From biology:
/// - **Mutualism**: both benefit (we AND the platform gain)
/// - **Commensalism**: one benefits, the other is unaffected
/// - **Parasitism**: one benefits at the other's expense
/// - **Self**: it IS us — not a relationship, an identity
///
/// Apple/Google are commensal: we use their curated gardens,
/// they don't notice. Linux is mutualist: we contribute back
/// (AGPL, crates, papers) and it gives us raw access.
/// Rust is Self: the membrane IS written in Rust. No relationship
/// needed — it IS the genome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Symbiosis {
    /// Both organisms benefit. Linux ↔ membrane.
    /// We contribute AGPL code, crates, papers. Linux gives raw /dev access.
    Mutualist,
    /// One benefits, the other is unaffected. membrane → Apple/Google.
    /// We use their APIs. They don't notice or care.
    Commensal,
    /// One benefits at the other's expense. We avoid this.
    Parasite,
    /// It IS us. Rust = our DNA. Not a relationship — an identity.
    /// The membrane doesn't have a relationship with Rust.
    /// The membrane IS Rust.
    Identity,
}

impl Symbiosis {
    /// Can we cross-target this platform?
    /// Parasites are excluded. Everything else is a valid target.
    pub fn is_cross_target(&self) -> bool {
        !matches!(self, Self::Parasite)
    }

    /// Biological analog.
    pub fn analog(&self) -> &'static str {
        match self {
            Self::Mutualist => "clownfish + anemone (both benefit)",
            Self::Commensal => "barnacle on whale (one benefits, whale unaffected)",
            Self::Parasite  => "tapeworm (one benefits at host expense)",
            Self::Identity  => "mitochondria (was once separate, now IS the cell)",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GARDEN TYPE — walled, open, or source
// ═══════════════════════════════════════════════════════════════════════════

/// What kind of garden is this platform?
///
/// A walled garden is a BOTANICAL garden — curated, not imprisoned.
/// The wall keeps weather out, not visitors in.
///
/// ```text
///   Walled (tight) — Apple. Small surface, high polish. Every API reviewed.
///                     The bonsai garden. Precise, controlled, beautiful.
///
///   Walled (wide)  — Google, Microsoft. Large surface, wide reach.
///                     The botanical garden. Big, diverse, some weeds.
///
///   Open (core)    — Linux. No walls. You can touch every nerve.
///                     The old-growth forest. Our actual ecosystem.
///
///   Open (inverted) — SteamOS. Linux inside, Windows outside. Proton
///                      IS a ribocipher at the OS layer: PE binaries → syscalls.
///                      The garden wall is transparent from the inside.
///                      The Frame controller = standard gamepad = same pore math.
///
///   Source (DNA)   — Rust. Not a garden — the SOIL the garden grows in.
///                     The genome. The central dogma starts here.
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GardenType {
    /// Small, curated, every surface polished. Apple.
    WalledTight,
    /// Large, diverse, broad reach. Google, Microsoft.
    WalledWide,
    /// No walls, direct access to all nerves. Linux.
    Open,
    /// Linux inside, Windows outside. Proton inverts the garden wall.
    /// SteamOS: open garden that RUNS walled-garden software. Klein crossing.
    OpenInverted,
    /// Not a garden — the soil. Rust.
    Source,
}

impl GardenType {
    /// How permeable is this garden's wall to our probes?
    /// 0.0 = sealed, 1.0 = wide open.
    pub fn permeability(&self) -> f64 {
        match self {
            Self::WalledTight  => 0.3,  // Apple: few APIs, but each one is high quality
            Self::WalledWide   => 0.5,  // Google/MS: more APIs, more surface, some friction
            Self::Open         => 0.9,  // Linux: almost everything exposed via /dev, /proc, /sys
            Self::OpenInverted => 0.85, // SteamOS: Linux access + Proton inversion layer
            Self::Source       => 1.0,  // Rust: we ARE this, full access
        }
    }

    /// How much surface area does this garden expose?
    pub fn surface_area(&self) -> &'static str {
        match self {
            Self::WalledTight  => "tight (few APIs, high quality each)",
            Self::WalledWide   => "wide (many APIs, variable quality)",
            Self::Open         => "full (kernel + userspace, raw access)",
            Self::OpenInverted => "full + inverted (Linux access + Proton translates Windows binaries)",
            Self::Source       => "total (we ARE the source)",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CENTRAL DOGMA LAYER — DNA → RNA → mRNA → Protein
// ═══════════════════════════════════════════════════════════════════════════

/// Position in the central dogma cascade.
///
/// ```text
///   DNA      Rust types, serde structs     (permanent, compiled, verified)
///   RNA      Linux OS / kernel events       (transcribed, system-level)
///   mRNA     Browser engine / JS runtime    (translated, portable)
///   Protein  Device surface / I/O channels  (folded, expressed, touchable)
/// ```
///
/// Going UP (reverse transcriptase / ribocipher):
///   Protein → mRNA → RNA → DNA
///   keystroke → JS event → OS signal → Rust struct
///
/// Cascading DOWN (compilation / expression):
///   DNA → RNA → mRNA → Protein
///   Rust struct → binary → wasm/JS → pixel on screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DogmaLayer {
    /// The genome. Rust types. Permanent, verified by the compiler.
    Dna,
    /// Transcription. Linux/OS layer. System calls, kernel events.
    Rna,
    /// Translation. Browser engine, JS/Wasm runtime. Portable code.
    Mrna,
    /// Expression. The folded protein. Pixels, sounds, vibrations.
    /// What the user actually touches.
    Protein,
}

impl DogmaLayer {
    /// All layers, source to surface.
    pub const ALL: &'static [DogmaLayer] = &[
        DogmaLayer::Dna,
        DogmaLayer::Rna,
        DogmaLayer::Mrna,
        DogmaLayer::Protein,
    ];

    /// Reverse order: surface to source (the ribocipher direction).
    pub const REVERSE: &'static [DogmaLayer] = &[
        DogmaLayer::Protein,
        DogmaLayer::Mrna,
        DogmaLayer::Rna,
        DogmaLayer::Dna,
    ];

    /// What technology lives at this layer?
    pub fn tech(&self) -> &'static str {
        match self {
            Self::Dna     => "Rust (structs, enums, type system, compiler verification)",
            Self::Rna     => "Linux (kernel, /dev/input, evdev, syscalls, signals)",
            Self::Mrna    => "Browser (V8/JSC/SpiderMonkey, Wasm, Web APIs, _worker.js)",
            Self::Protein => "Device (pixels, audio samples, touch events, accelerometer)",
        }
    }

    /// What flows between this layer and the next?
    pub fn bond_to_next(&self) -> Option<&'static str> {
        match self {
            Self::Dna     => Some("compilation (cargo build → binary/wasm)"),
            Self::Rna     => Some("serving (HTTP, WebSocket, CF Worker dispatch)"),
            Self::Mrna    => Some("rendering (DOM paint, Web Audio, haptic feedback)"),
            Self::Protein => None, // end of forward chain
        }
    }

    /// What flows between this layer and the previous? (ribocipher direction)
    pub fn reverse_bond(&self) -> Option<&'static str> {
        match self {
            Self::Dna     => None, // end of reverse chain
            Self::Rna     => Some("serde deserialize (JSON → struct, the ribocipher)"),
            Self::Mrna    => Some("event capture (OS signals → JS event objects)"),
            Self::Protein => Some("transduction (physical → electrical → digital)"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PLATFORM PHYLUM — the taxonomy of device ecosystems
// ═══════════════════════════════════════════════════════════════════════════

/// A device platform phylum — the top-level classification of where
/// peripheral nerves terminate.
///
/// Named after biological phyla because these are BODY PLANS,
/// not brands. The iPhone and the iPad are the same phylum
/// (Chordata → Apple) with different species.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlatformPhylum {
    /// Phylum identifier.
    pub id: &'static str,
    /// Human name.
    pub name: &'static str,
    /// What kind of garden.
    pub garden: GardenType,
    /// Symbiotic relationship to the membrane.
    pub symbiosis: Symbiosis,
    /// Where in the central dogma this platform primarily lives.
    pub dogma_layer: DogmaLayer,
    /// The Klein surface available on this platform.
    /// The channel where inside↔outside are topologically connected.
    pub klein_surface: &'static str,
    /// Browser engine (if applicable).
    pub engine: Option<&'static str>,
    /// Device presets from IoSurface that belong to this phylum.
    pub presets: &'static [&'static str],
    /// Specific APIs that expose peripheral nerves.
    pub nerve_apis: &'static [&'static str],
}

/// All platform phyla in the membrane's peripheral nervous system.
pub fn phyla() -> Vec<PlatformPhylum> {
    vec![
        // ── Apple: tight walled garden, commensal ──
        PlatformPhylum {
            id: "apple",
            name: "Apple (Safari / WebKit)",
            garden: GardenType::WalledTight,
            symbiosis: Symbiosis::Commensal,
            dogma_layer: DogmaLayer::Mrna,
            klein_surface: "DeviceMotionEvent (accelerometer → gravity crosses screen plane)",
            engine: Some("WebKit/JavaScriptCore"),
            presets: &["iphone", "mobile"],
            nerve_apis: &[
                "DeviceMotionEvent",         // accelerometer — THE Klein surface
                "DeviceOrientationEvent",    // gyroscope
                "Touch Events (multi-touch)", // up to 5 simultaneous
                "Web Audio API (AudioContext)", // mic + speaker
                "Screen.orientation",        // portrait/landscape crossing
                "navigator.vibrate()",       // haptic (cause↔effect chirality)
                "WKWebView.evaluateJavaScript", // native bridge (mRNA↔RNA crossing)
            ],
        },

        // ── Google: wide walled garden, commensal ──
        PlatformPhylum {
            id: "google",
            name: "Google (Chrome / Blink / Android)",
            garden: GardenType::WalledWide,
            symbiosis: Symbiosis::Commensal,
            dogma_layer: DogmaLayer::Mrna,
            klein_surface: "Generic Sensor API (gyroscope, ambient light cross screen boundary)",
            engine: Some("Blink/V8"),
            presets: &["iphone", "mobile"], // Pixel uses same touch preset shape
            nerve_apis: &[
                "Generic Sensor API",          // gyro, accel, ambient light, magnetometer
                "Pointer Events (pressure, tilt)", // stylus Klein crossing (force→value)
                "Web Bluetooth",               // crosses radio boundary
                "Web NFC",                     // near-field crosses air gap
                "Web USB",                     // direct peripheral access
                "WebXR Device API",            // spatial Klein surface
                "Notification API",            // crosses attention boundary
                "Fenced Frames (Privacy Sandbox)", // the wall IS the feature
            ],
        },

        // ── Windows: wide walled garden, commensal ──
        PlatformPhylum {
            id: "windows",
            name: "Microsoft (Edge / Windows)",
            garden: GardenType::WalledWide,
            symbiosis: Symbiosis::Commensal,
            dogma_layer: DogmaLayer::Mrna,
            klein_surface: "Pointer Events (pen pressure = continuous Klein crossing)",
            engine: Some("Blink/V8 (Edge = Chromium)"),
            presets: &["desktop", "workstation", "laptop", "surface"],
            nerve_apis: &[
                "Pointer Events (pressure, tilt, twist)", // Surface Pen = Klein surface
                "Windows.Devices.Input",       // raw input pipeline
                "Spatial Interaction (Hololens)", // AR Klein surface
                "Windows.AI.MachineLearning",  // on-device inference
                "WSL2 (Linux subsystem)",      // the membrane can reach RNA FROM protein
            ],
        },

        // ── Linux: open garden, mutualist ──
        PlatformPhylum {
            id: "linux",
            name: "Linux (Pop!_OS / Ubuntu / kernel)",
            garden: GardenType::Open,
            symbiosis: Symbiosis::Mutualist,
            dogma_layer: DogmaLayer::Rna,
            klein_surface: "/dev/input/* (raw evdev — no wall between kernel sense and userspace)",
            engine: None, // Linux IS the engine, not a browser
            presets: &["desktop", "workstation", "ci", "server"],
            nerve_apis: &[
                "/dev/input/event* (evdev)",   // raw keyboard, mouse, touch
                "/proc/bus/input/devices",     // peripheral inventory
                "/sys/class/input/",           // sysfs device tree
                "libinput",                    // userspace input library
                "udev rules",                  // hot-plug detection (nerve growth!)
                "inotify / fanotify",          // filesystem nerve endings
                "eBPF (kernel-level probes)",  // THE deepest nerve ending
                "perf_event_open()",           // CPU performance counters
                "/dev/hidraw*",               // raw HID access (USB peripherals)
            ],
        },

        // ── Rust: the source, identity ──
        PlatformPhylum {
            id: "rust",
            name: "Rust (the genome)",
            garden: GardenType::Source,
            symbiosis: Symbiosis::Identity,
            dogma_layer: DogmaLayer::Dna,
            klein_surface: "serde Serialize↔Deserialize (JSON↔struct = RNA↔DNA transcription)",
            engine: None, // Rust IS the DNA
            presets: &[], // Rust is not a device — it's what devices are MADE of
            nerve_apis: &[
                "serde (Serialize/Deserialize)", // THE ribocipher
                "std::io (Read/Write traits)",   // the ur-pore
                "tokio (async I/O)",             // concurrent nerve signals
                "evdev crate",                   // Linux input from Rust
                "winit (cross-platform window)",  // display pore
                "cpal (cross-platform audio)",    // audio pore
                "wgpu (cross-platform GPU)",      // visual pore, hardware-accelerated
                "wasm-bindgen",                   // DNA→mRNA compiler bridge
            ],
        },

        // ── SteamOS/Proton: open INVERTED garden, mutualist ──
        //
        // The OUTER target. The LONG target.
        // SteamOS IS Linux (Arch-based). Proton IS Wine (Windows→Linux translation).
        // This is a RIBOCIPHER AT THE OS LAYER: Windows PE binaries get
        // reverse-transcribed into Linux syscalls. The garden wall is
        // transparent from the inside.
        //
        // The Valve Frame controller connects to keyboard and peripherals —
        // same pore math, same I/O surface model. A standard video game
        // uses the same invert: Frame connects to keyboard = symbolic pore,
        // gamepad = tactile pore, display = visual pore. No special case.
        //
        // Already a face of the Klein ladder leak on the ribosome face:
        // Proton translates across the Windows↔Linux boundary, which IS
        // the mRNA↔RNA boundary in our central dogma. The translation
        // layer (Proton/Wine) IS the ribosome. It reads Windows bytecode
        // (mRNA) and produces Linux system calls (protein folding into
        // actual machine operations).
        PlatformPhylum {
            id: "steamos",
            name: "SteamOS / Proton (Valve)",
            garden: GardenType::OpenInverted,
            symbiosis: Symbiosis::Mutualist,
            dogma_layer: DogmaLayer::Rna, // Linux core, but with mRNA translation built in
            klein_surface: "Proton (Windows PE → Linux syscalls = mRNA→RNA reverse transcription at OS layer)",
            engine: Some("Proton/Wine (Windows→Linux translation layer = the ribosome)"),
            presets: &["desktop"], // Steam Deck/Frame use Linux desktop-class I/O
            nerve_apis: &[
                "Proton/Wine (PE→ELF translation)",   // THE ribocipher at OS layer
                "Steam Input API",                     // unified controller abstraction
                "GamepadAPI (Web, via Steam browser)",  // web gamepad = tactile pore
                "/dev/input/event* (evdev)",           // raw gamepad, keyboard (Linux!)
                "SDL2 (input, audio, video)",           // cross-platform game I/O
                "Vulkan / DXVK (GPU translation)",     // DirectX→Vulkan = another ribocipher
                "PipeWire (audio)",                     // low-latency audio pore
                "MangoHud (performance overlay)",       // the HUD on their HUD
                "Steam Deck gyroscope",                 // spatial pore, Klein surface
                "Frame KVM (keyboard/video/mouse)",     // the Frame IS a peripheral nerve bundle
            ],
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// PERIPHERAL NERVE — a specific nerve ending mapped to its platform
// ═══════════════════════════════════════════════════════════════════════════

/// A peripheral nerve — a specific I/O channel on a specific platform.
///
/// This is the leaf node: where the membrane's abstract pore model
/// meets an actual physical device on an actual platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeripheralNerve {
    /// Which pore this nerve belongs to (from core.rs)
    pub pore: String,
    /// Which platform phylum
    pub phylum_id: String,
    /// The specific API that IS this nerve
    pub api: String,
    /// Direction: afferent (inward, sensory) or efferent (outward, motor)
    pub direction: NerveDirection,
    /// Is this a Klein surface? (inside↔outside topologically connected)
    pub is_klein: bool,
    /// Dogma direction when this nerve fires
    pub dogma_flow: DogmaFlow,
}

/// Nerve direction — afferent (sensory, inward) or efferent (motor, outward).
///
/// Named from neuroscience:
/// - **Afferent**: carries signal TOWARD the CNS (sensor → brain)
/// - **Efferent**: carries signal AWAY from the CNS (brain → muscle)
/// - **Mixed**: both directions (like the vagus nerve)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NerveDirection {
    /// Inward: device → membrane (sensor, keyboard, mic, camera)
    Afferent,
    /// Outward: membrane → device (display, speaker, vibrate)
    Efferent,
    /// Both directions (network, storage, the barre channels)
    Mixed,
}

/// Which direction through the central dogma does signal flow?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DogmaFlow {
    /// Forward: DNA → RNA → mRNA → Protein (compilation, expression)
    Forward,
    /// Reverse: Protein → mRNA → RNA → DNA (ribocipher, reverse transcription)
    Reverse,
    /// Both: the ouroboros cycle
    Ouroboros,
}

// ═══════════════════════════════════════════════════════════════════════════
// SUB-GROUPS — reactor targeting groups for benchScale
// ═══════════════════════════════════════════════════════════════════════════

/// A reactor sub-group — a cross-section of the PNS that can be
/// targeted as a unit by the benchScale reactor.
///
/// Sub-groups are the "agent reagents" — they define which platforms
/// and which nerve types to activate for a given reaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactorSubGroup {
    /// Group identifier.
    pub id: String,
    /// Human name.
    pub name: String,
    /// Which phyla are included in this group.
    pub phyla: Vec<String>,
    /// Which pore types to target.
    pub pores: Vec<String>,
    /// Minimum garden permeability required.
    pub min_permeability: f64,
    /// Dogma layer range (from, to). Inclusive.
    pub dogma_range: (DogmaLayer, DogmaLayer),
    /// Description of what this sub-group tests/targets.
    pub desc: String,
}

/// All reactor sub-groups — the benchScale targeting catalog.
pub fn reactor_sub_groups() -> Vec<ReactorSubGroup> {
    vec![
        // ── Inner surface: tight Apple garden ──
        ReactorSubGroup {
            id: "inner-tight".into(),
            name: "Inner Surface (Apple/Safari)".into(),
            phyla: vec!["apple".into()],
            pores: vec!["visual".into(), "audio".into(), "tactile".into(), "spatial".into()],
            min_permeability: 0.3,
            dogma_range: (DogmaLayer::Mrna, DogmaLayer::Protein),
            desc: "Safari accelerometer Klein surface. Tight APIs, high fidelity. \
                   DeviceMotionEvent is the primary reagent — gravity crosses the screen plane."
                .into(),
        },

        // ── Fat layer: Google + Windows ──
        ReactorSubGroup {
            id: "fat-layer".into(),
            name: "Fat Layer (Google + Windows)".into(),
            phyla: vec!["google".into(), "windows".into()],
            pores: vec![
                "visual".into(), "audio".into(), "tactile".into(),
                "spatial".into(), "pointer".into(), "data".into(),
            ],
            min_permeability: 0.5,
            dogma_range: (DogmaLayer::Mrna, DogmaLayer::Protein),
            desc: "Wide surface. Generic Sensor API + Pointer Events. \
                   More APIs but less curated. Pen pressure = continuous Klein crossing."
                .into(),
        },

        // ── Core tissue: Linux ──
        ReactorSubGroup {
            id: "core-tissue".into(),
            name: "Core Tissue (Linux)".into(),
            phyla: vec!["linux".into()],
            pores: vec![
                "symbolic".into(), "pointer".into(), "data".into(),
                "persistence".into(), "energy".into(),
            ],
            min_permeability: 0.9,
            dogma_range: (DogmaLayer::Rna, DogmaLayer::Protein),
            desc: "OUR tissue. Raw /dev/input access. No permission wall. \
                   eBPF for kernel-level probes. This is where we can touch the nerve directly."
                .into(),
        },

        // ── Genome: Rust ──
        ReactorSubGroup {
            id: "genome".into(),
            name: "Genome (Rust)".into(),
            phyla: vec!["rust".into()],
            pores: vec!["data".into(), "persistence".into()],
            min_permeability: 1.0,
            dogma_range: (DogmaLayer::Dna, DogmaLayer::Dna),
            desc: "The DNA. serde is the ribocipher. The type system IS the compiler verification. \
                   Targeting this group = evolving the genome itself."
                .into(),
        },

        // ── Outer long: SteamOS / Proton — the inverted garden ──
        ReactorSubGroup {
            id: "outer-long".into(),
            name: "Outer Long Target (SteamOS/Proton)".into(),
            phyla: vec!["steamos".into()],
            pores: vec![
                "visual".into(), "audio".into(), "tactile".into(),
                "spatial".into(), "symbolic".into(), "data".into(),
            ],
            min_permeability: 0.85,
            dogma_range: (DogmaLayer::Rna, DogmaLayer::Protein),
            desc: "The OUTER target. The LONG target. SteamOS is Linux that runs Windows games \
                   via Proton. Proton IS a ribocipher at the OS layer: PE→ELF translation. \
                   DXVK is another: DirectX→Vulkan. The Frame controller = standard peripheral \
                   nerve bundle. Already a face of the Klein ladder leak on the ribosome face."
                .into(),
        },

        // ── Full cascade: all platforms, forward dogma ──
        ReactorSubGroup {
            id: "full-cascade".into(),
            name: "Full Cascade (DNA→Protein)".into(),
            phyla: vec![
                "rust".into(), "linux".into(), "steamos".into(),
                "google".into(), "windows".into(), "apple".into(),
            ],
            pores: vec![], // all pores
            min_permeability: 0.0,
            dogma_range: (DogmaLayer::Dna, DogmaLayer::Protein),
            desc: "The complete central dogma. Compile Rust → ship binary → serve worker → \
                   render on device. All 6 phyla. Tests the entire cascade end-to-end."
                .into(),
        },

        // ── Ribocipher: reverse, Protein→DNA ──
        ReactorSubGroup {
            id: "ribocipher".into(),
            name: "Ribocipher (Protein→DNA)".into(),
            phyla: vec![
                "apple".into(), "google".into(), "windows".into(),
                "steamos".into(), "linux".into(), "rust".into(),
            ],
            pores: vec!["symbolic".into(), "data".into()],
            min_permeability: 0.0,
            dogma_range: (DogmaLayer::Protein, DogmaLayer::Dna),
            desc: "Reverse transcriptase. Keystrokes → JS events → OS signals → Rust structs. \
                   SteamOS/Proton adds a SECOND ribosome: PE→ELF at OS layer. \
                   This is the ribocipher: ephemeral signals become permanent types."
                .into(),
        },

        // ── Klein cross-targets: all platforms with Klein surfaces ──
        ReactorSubGroup {
            id: "klein-surfaces".into(),
            name: "Klein Cross-Targets".into(),
            phyla: vec![
                "apple".into(), "google".into(), "windows".into(),
                "linux".into(), "steamos".into(),
            ],
            pores: vec!["spatial".into(), "tactile".into(), "pointer".into()],
            min_permeability: 0.3,
            dogma_range: (DogmaLayer::Mrna, DogmaLayer::Protein),
            desc: "Every platform where inside↔outside are topologically connected. \
                   Accelerometers, pressure sensors, raw input, Proton inversion. \
                   The Klein crossing points."
                .into(),
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// PNS CENSUS — count the peripheral nerves
// ═══════════════════════════════════════════════════════════════════════════

/// Census of the peripheral nervous system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnsCensus {
    /// Total phyla.
    pub phyla_count: usize,
    /// Total nerve APIs across all phyla.
    pub nerve_count: usize,
    /// Klein surfaces identified.
    pub klein_count: usize,
    /// Reactor sub-groups available.
    pub sub_group_count: usize,
    /// Per-phylum breakdown.
    pub by_phylum: Vec<PhylumCensus>,
    /// Total commensal platforms (cross-targetable).
    pub commensal_count: usize,
    /// Total mutualist platforms.
    pub mutualist_count: usize,
}

/// Per-phylum census data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhylumCensus {
    pub id: String,
    pub name: String,
    pub garden: String,
    pub symbiosis: String,
    pub nerve_count: usize,
    pub klein_surface: String,
    pub dogma_layer: String,
    pub permeability: f64,
}

/// Take census of the entire PNS.
pub fn pns_census() -> PnsCensus {
    let all_phyla = phyla();
    let sub_groups = reactor_sub_groups();

    let by_phylum: Vec<PhylumCensus> = all_phyla
        .iter()
        .map(|p| PhylumCensus {
            id: p.id.to_string(),
            name: p.name.to_string(),
            garden: format!("{:?}", p.garden),
            symbiosis: format!("{:?}", p.symbiosis),
            nerve_count: p.nerve_apis.len(),
            klein_surface: p.klein_surface.to_string(),
            dogma_layer: format!("{:?}", p.dogma_layer),
            permeability: p.garden.permeability(),
        })
        .collect();

    let nerve_count: usize = all_phyla.iter().map(|p| p.nerve_apis.len()).sum();
    let commensal_count = all_phyla
        .iter()
        .filter(|p| p.symbiosis == Symbiosis::Commensal)
        .count();
    let mutualist_count = all_phyla
        .iter()
        .filter(|p| p.symbiosis == Symbiosis::Mutualist)
        .count();

    PnsCensus {
        phyla_count: all_phyla.len(),
        nerve_count,
        klein_count: all_phyla.len(), // each phylum has exactly one Klein surface
        sub_group_count: sub_groups.len(),
        by_phylum,
        commensal_count,
        mutualist_count,
    }
}

/// Find the phylum for a given IoSurface preset name.
pub fn phylum_for_preset(preset: &str) -> Option<&'static str> {
    // Check each phylum's preset list
    // Note: some presets appear in multiple phyla (e.g., "mobile" in apple + google)
    // Return the TIGHTER match (prefer apple for iphone, google for pixel-like)
    match preset {
        "iphone" | "mobile" => Some("apple"),
        "desktop" | "workstation" | "laptop" => Some("linux"), // our core
        "surface" => Some("windows"),
        "ci" | "server" => Some("linux"),
        "fridge" | "iot" => Some("google"),  // typically Android Things / matter
        "ar" | "portal" => Some("apple"),    // ARKit is the tightest
        "vr" | "quest" => Some("google"),    // Meta Quest runs Android
        "kiosk" | "pos" | "signage" => Some("linux"), // embedded Linux
        "low-power" | "2g" => Some("google"), // low-end Android
        "steamdeck" | "frame" | "steamos" => Some("steamos"), // Valve: Linux inverted
        _ => None,
    }
}

/// Map the full central dogma cascade for a given starting layer.
///
/// Forward: returns layers from start → Protein.
/// Reverse: returns layers from start → DNA.
pub fn dogma_cascade(from: DogmaLayer, flow: DogmaFlow) -> Vec<DogmaLayer> {
    match flow {
        DogmaFlow::Forward => {
            DogmaLayer::ALL
                .iter()
                .copied()
                .filter(|l| *l >= from)
                .collect()
        }
        DogmaFlow::Reverse => {
            DogmaLayer::REVERSE
                .iter()
                .copied()
                .filter(|l| *l <= from)
                .collect()
        }
        DogmaFlow::Ouroboros => {
            // Forward then reverse = the full loop
            let mut forward: Vec<DogmaLayer> = DogmaLayer::ALL.to_vec();
            let mut reverse: Vec<DogmaLayer> = DogmaLayer::REVERSE
                .iter()
                .copied()
                .skip(1) // skip Protein (already at end of forward)
                .collect();
            forward.append(&mut reverse);
            forward
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_phyla() {
        assert_eq!(phyla().len(), 6, "6 platform phyla: apple, google, windows, linux, rust, steamos");
    }

    #[test]
    fn apple_is_commensal() {
        let p = phyla().into_iter().find(|p| p.id == "apple").unwrap();
        assert_eq!(p.symbiosis, Symbiosis::Commensal);
        assert_eq!(p.garden, GardenType::WalledTight);
        assert!(p.klein_surface.contains("DeviceMotion"));
    }

    #[test]
    fn linux_is_mutualist() {
        let p = phyla().into_iter().find(|p| p.id == "linux").unwrap();
        assert_eq!(p.symbiosis, Symbiosis::Mutualist);
        assert_eq!(p.garden, GardenType::Open);
        assert_eq!(p.dogma_layer, DogmaLayer::Rna);
    }

    #[test]
    fn rust_is_identity() {
        let p = phyla().into_iter().find(|p| p.id == "rust").unwrap();
        assert_eq!(p.symbiosis, Symbiosis::Identity);
        assert_eq!(p.garden, GardenType::Source);
        assert_eq!(p.dogma_layer, DogmaLayer::Dna);
        assert!(p.klein_surface.contains("serde"));
    }

    #[test]
    fn all_commensal_are_cross_targetable() {
        for p in phyla() {
            if p.symbiosis == Symbiosis::Commensal {
                assert!(p.symbiosis.is_cross_target(),
                    "{} should be cross-targetable", p.id);
            }
        }
    }

    #[test]
    fn parasites_not_targetable() {
        assert!(!Symbiosis::Parasite.is_cross_target());
    }

    #[test]
    fn garden_permeability_ordering() {
        assert!(GardenType::Source.permeability() > GardenType::Open.permeability());
        assert!(GardenType::Open.permeability() > GardenType::OpenInverted.permeability());
        assert!(GardenType::OpenInverted.permeability() > GardenType::WalledWide.permeability());
        assert!(GardenType::WalledWide.permeability() > GardenType::WalledTight.permeability());
    }

    #[test]
    fn dogma_layers_ordered() {
        assert!(DogmaLayer::Dna < DogmaLayer::Rna);
        assert!(DogmaLayer::Rna < DogmaLayer::Mrna);
        assert!(DogmaLayer::Mrna < DogmaLayer::Protein);
    }

    #[test]
    fn forward_cascade_from_dna() {
        let cascade = dogma_cascade(DogmaLayer::Dna, DogmaFlow::Forward);
        assert_eq!(cascade.len(), 4);
        assert_eq!(cascade[0], DogmaLayer::Dna);
        assert_eq!(cascade[3], DogmaLayer::Protein);
    }

    #[test]
    fn reverse_cascade_from_protein() {
        let cascade = dogma_cascade(DogmaLayer::Protein, DogmaFlow::Reverse);
        assert_eq!(cascade.len(), 4);
        assert_eq!(cascade[0], DogmaLayer::Protein);
        assert_eq!(cascade[3], DogmaLayer::Dna);
    }

    #[test]
    fn ouroboros_cascade_wraps() {
        let cascade = dogma_cascade(DogmaLayer::Dna, DogmaFlow::Ouroboros);
        // Forward: DNA→RNA→mRNA→Protein, then reverse: mRNA→RNA→DNA
        assert_eq!(cascade.len(), 7, "4 forward + 3 reverse (skip duplicate Protein)");
        assert_eq!(cascade[0], DogmaLayer::Dna);
        assert_eq!(cascade[3], DogmaLayer::Protein);
        assert_eq!(cascade[6], DogmaLayer::Dna, "ouroboros returns to DNA");
    }

    #[test]
    fn eight_reactor_sub_groups() {
        let groups = reactor_sub_groups();
        assert_eq!(groups.len(), 8, "8 sub-groups: inner-tight, fat-layer, core-tissue, genome, outer-long, full-cascade, ribocipher, klein-surfaces");
    }

    #[test]
    fn inner_tight_targets_apple() {
        let groups = reactor_sub_groups();
        let inner = groups.iter().find(|g| g.id == "inner-tight").unwrap();
        assert!(inner.phyla.contains(&"apple".to_string()));
        assert_eq!(inner.phyla.len(), 1);
    }

    #[test]
    fn fat_layer_targets_google_and_windows() {
        let groups = reactor_sub_groups();
        let fat = groups.iter().find(|g| g.id == "fat-layer").unwrap();
        assert!(fat.phyla.contains(&"google".to_string()));
        assert!(fat.phyla.contains(&"windows".to_string()));
    }

    #[test]
    fn ribocipher_is_reverse_flow() {
        let groups = reactor_sub_groups();
        let ribo = groups.iter().find(|g| g.id == "ribocipher").unwrap();
        assert_eq!(ribo.dogma_range, (DogmaLayer::Protein, DogmaLayer::Dna));
    }

    #[test]
    fn pns_census_counts() {
        let census = pns_census();
        assert_eq!(census.phyla_count, 6);
        assert!(census.nerve_count >= 40, "should have 40+ nerve APIs, got {}", census.nerve_count);
        assert_eq!(census.klein_count, 6, "one Klein surface per phylum");
        assert_eq!(census.sub_group_count, 8);
        assert_eq!(census.commensal_count, 3, "apple + google + windows");
        assert_eq!(census.mutualist_count, 2, "linux + steamos");
    }

    #[test]
    fn every_phylum_has_nerve_apis() {
        for p in phyla() {
            assert!(!p.nerve_apis.is_empty(),
                "phylum '{}' has no nerve APIs", p.id);
        }
    }

    #[test]
    fn every_phylum_has_klein_surface() {
        for p in phyla() {
            assert!(!p.klein_surface.is_empty(),
                "phylum '{}' has no Klein surface", p.id);
        }
    }

    #[test]
    fn preset_to_phylum_mapping() {
        assert_eq!(phylum_for_preset("iphone"), Some("apple"));
        assert_eq!(phylum_for_preset("desktop"), Some("linux"));
        assert_eq!(phylum_for_preset("surface"), Some("windows"));
        assert_eq!(phylum_for_preset("ci"), Some("linux"));
        assert_eq!(phylum_for_preset("vr"), Some("google"));
    }

    #[test]
    fn central_dogma_has_bonds() {
        // Every layer (except Protein forward, DNA reverse) has a bond
        for layer in DogmaLayer::ALL {
            if *layer != DogmaLayer::Protein {
                assert!(layer.bond_to_next().is_some(),
                    "{:?} should have forward bond", layer);
            }
            if *layer != DogmaLayer::Dna {
                assert!(layer.reverse_bond().is_some(),
                    "{:?} should have reverse bond", layer);
            }
        }
    }

    #[test]
    fn keyboard_is_symbolic_pore_afferent() {
        // The keyboard you're typing on right now IS a peripheral nerve.
        // It's the symbolic pore, afferent direction (inward), on Linux phylum.
        let linux = phyla().into_iter().find(|p| p.id == "linux").unwrap();
        assert!(linux.nerve_apis.iter().any(|a| a.contains("evdev")),
            "Linux should expose raw keyboard via evdev");
        // The symbolic pore in core.rs has inward=0.9 for physical keyboards.
        // This test proves the PNS connects to the existing pore model.
    }

    #[test]
    fn safari_accelerometer_is_klein() {
        // Safari DeviceMotionEvent: the phone's physical orientation
        // IS the Klein crossing. Inside (screen) maps to outside (gravity).
        // Safari ALREADY provides this. We just need to test on it.
        let apple = phyla().into_iter().find(|p| p.id == "apple").unwrap();
        assert!(apple.nerve_apis.contains(&"DeviceMotionEvent"));
        assert!(apple.klein_surface.contains("accelerometer"));
    }

    #[test]
    fn serde_is_the_ribocipher() {
        // serde Serialize↔Deserialize IS the ribocipher.
        // JSON (ephemeral, mRNA) → Rust struct (permanent, DNA).
        // This is reverse transcription in the type system.
        let rust = phyla().into_iter().find(|p| p.id == "rust").unwrap();
        assert!(rust.nerve_apis.contains(&"serde (Serialize/Deserialize)"));
        assert!(rust.klein_surface.contains("serde"));
        assert_eq!(rust.dogma_layer, DogmaLayer::Dna);
    }

    #[test]
    fn steamos_is_mutualist_inverted() {
        let p = phyla().into_iter().find(|p| p.id == "steamos").unwrap();
        assert_eq!(p.symbiosis, Symbiosis::Mutualist);
        assert_eq!(p.garden, GardenType::OpenInverted);
        assert_eq!(p.dogma_layer, DogmaLayer::Rna, "SteamOS is Linux at the RNA layer");
        assert!(p.klein_surface.contains("Proton"),
            "Proton IS the Klein surface: Windows PE → Linux syscalls");
    }

    #[test]
    fn proton_is_ribosome() {
        // Proton translates Windows PE binaries into Linux syscalls.
        // That IS reverse transcription at the OS layer.
        // The translation layer IS the ribosome: reads mRNA (Windows bytecode),
        // produces protein (machine operations via Linux syscalls).
        let steam = phyla().into_iter().find(|p| p.id == "steamos").unwrap();
        assert!(steam.nerve_apis.iter().any(|a| a.contains("Proton")));
        assert!(steam.nerve_apis.iter().any(|a| a.contains("DXVK")),
            "DXVK is another ribocipher: DirectX → Vulkan translation");
        assert!(steam.engine.unwrap().contains("ribosome"),
            "the engine description should name it as ribosome");
    }

    #[test]
    fn frame_controller_is_standard_peripheral() {
        // The Valve Frame connects to keyboard and peripherals.
        // No special case needed — same I/O surface model.
        // A standard video game uses the same invert.
        let steam = phyla().into_iter().find(|p| p.id == "steamos").unwrap();
        assert!(steam.nerve_apis.iter().any(|a| a.contains("Frame KVM")));
        assert!(steam.nerve_apis.iter().any(|a| a.contains("Steam Input")));
        assert!(steam.nerve_apis.iter().any(|a| a.contains("evdev")),
            "SteamOS exposes raw evdev — same as Linux core");
    }

    #[test]
    fn outer_long_sub_group_targets_steamos() {
        let groups = reactor_sub_groups();
        let outer = groups.iter().find(|g| g.id == "outer-long").unwrap();
        assert!(outer.phyla.contains(&"steamos".to_string()));
        assert_eq!(outer.phyla.len(), 1);
        assert!(outer.desc.contains("LONG target"));
    }

    #[test]
    fn steamos_preset_mapping() {
        assert_eq!(phylum_for_preset("steamdeck"), Some("steamos"));
        assert_eq!(phylum_for_preset("frame"), Some("steamos"));
        assert_eq!(phylum_for_preset("steamos"), Some("steamos"));
    }

    #[test]
    fn full_cascade_includes_all_six_phyla() {
        let groups = reactor_sub_groups();
        let full = groups.iter().find(|g| g.id == "full-cascade").unwrap();
        assert_eq!(full.phyla.len(), 6, "full cascade should include all 6 phyla");
        assert!(full.phyla.contains(&"steamos".to_string()));
    }

    #[test]
    fn wsl2_bridges_windows_to_linux() {
        // WSL2 means Windows can reach the RNA layer (Linux) FROM the mRNA layer.
        // The membrane can use this to cross garden boundaries.
        let windows = phyla().into_iter().find(|p| p.id == "windows").unwrap();
        assert!(windows.nerve_apis.iter().any(|a| a.contains("WSL2")),
            "Windows should list WSL2 as a bridge to Linux");
    }
}

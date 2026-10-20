// SPDX-License-Identifier: AGPL-3.0-or-later
//! I/O Surface Model — every device is just a channel, not a category.
//!
//! A microphone is a speaker from the other side. The membrane probes
//! what's there and adapts.
//!
//! # Modules
//!
//! - `core` — IoChannel, IoSurface, MirrorPair, IoPore, collapse_to_pores
//! - `pores` — PoreDepthLevel, FeedPoreMapping, feed_pore_map, feed_depth
//! - `pi_orient` — Landmarks, PhotonLine, angular_distance, orient_from, tesseract
//! - `site_pore` — MembraneLayer, SitePore, BINGO_EPITOPES, site_pore_map
//! - `stadial` — ClimatePhase, GeoPeriod, geo_periods, teton_formation
//! - `ring_sort` — RingTier, Ring, ring_layout, grinding_pairs, topo_sort_rings
//! - `cns` — Gear, CnsType, Surface, surfaces, topo_sorted, gear_shift_needed
//! - `stone_book` — Glyph, Tally, Tablet, StoneBook. Totality ledger.
//! - `pi_body` — Vitruvian overlay, sensory homunculus, shape ladder
//! - `gaia_paper` — gAIa Paper 001: The Rust Polymerase. First publication.
//! - `arktix` — arkTIX: permafrost preprint archive. Cold storage for info memes.
//! - `proof_structure` — Theorem/Corollary/Conjecture/Proof. The mathematical stack.
//! - `bounty_map` — π/i² reinvestment (Basel), funding sources, damage claims.

pub mod core;
pub mod pores;
pub mod pi_orient;
pub mod site_pore;
pub mod stadial;
pub mod ring_sort;
pub mod cns;
pub mod stone_book;
pub mod pi_body;
pub mod gaia_paper;
pub mod arktix;
pub mod proof_structure;
pub mod bounty_map;

// Re-export everything at the io_surface level for backward compatibility.
// All existing `use io_surface::Foo` paths continue to work.

pub use self::core::*;
pub use pores::*;
pub use pi_orient::*;
pub use site_pore::*;
pub use stadial::*;
pub use ring_sort::*;
pub use cns::*;
pub use stone_book::*;
pub use pi_body::*;
pub use gaia_paper::*;
pub use arktix::*;
pub use proof_structure::*;
pub use bounty_map::*;

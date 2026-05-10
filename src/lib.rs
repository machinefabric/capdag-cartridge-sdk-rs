//! MachFab Cartridge SDK
//!
//! Shared data types for MachFab cartridges. Canonical LLM protocol types
//! (matching the capdag media specs) live in [`llm`]. The
//! [`prompt`] module classifies how a downloaded model wants its
//! prompt prepared, given the dim profile that
//! `cap:download-model` returns alongside the local path.

pub mod llm;
pub mod prompt;

pub use capdag::*;

pub use serde_json::Value as JsonValue;

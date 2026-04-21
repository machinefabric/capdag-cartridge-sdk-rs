//! MachFab Cartridge SDK
//!
//! Shared data types for MachFab cartridges. Canonical LLM protocol types
//! (matching the capdag media specs) live in [`llm`].

pub mod llm;

pub use capdag::*;

pub use serde_json::Value as JsonValue;

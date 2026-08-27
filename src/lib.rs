//! CapDAG Cartridge SDK
//!
//! Shared data types for CapDAG cartridges. Canonical LLM protocol types
//! (matching the capdag media defs) live in [`llm`]. The
//! [`prompt`] module classifies how a downloaded model wants its
//! prompt prepared, given the dim profile that
//! `cap:download-model` returns alongside the local path. The [`net_retry`]
//! module is the single shared policy for retrying transient HTTP failures —
//! every cartridge that makes network calls routes its requests through it.

pub mod llm;
pub mod net_retry;
pub mod pages;
pub mod prompt;
pub mod structured_queries;

pub use capdag::*;
pub use structured_queries::{
    StructuredQuery, StructuredQueryBuilder, StructuredQueryRegistry, MakeDecisionResult,
    MakeMultipleDecisionsResult, DecisionItem,
};

pub use serde_json::Value as JsonValue;

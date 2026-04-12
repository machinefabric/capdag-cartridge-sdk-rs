//! MACINA Cartridge SDK
//!
//! This library provides common data structures and traits for MACINA document processing cartridges.
//! It ensures consistency across different file handlers (PDF, EPUB, etc.) and enables
//! a unified cartridge architecture with standardized cap-based calling.
//!
//! Also provides canonical LLM protocol types matching capdag media specs for LLM cartridges.

pub mod document;
pub mod document_cap;
pub mod handler;
pub mod llm;
pub mod metadata;
pub mod output;
pub mod validation;

// Re-export cap SDK types
pub use capdag::*;

pub use document::*;
pub use document_cap::*;
pub use handler::*;
pub use metadata::*;
pub use output::*;
pub use validation::*;

// Re-export common dependencies
pub use anyhow::{Result, Context};
pub use serde::{Deserialize, Serialize};
pub use serde_json::Value as JsonValue;

/// Cartridge SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Standard error type for cartridge operations
pub type CartridgeError = anyhow::Error;

/// Cartridge result type
pub type CartridgeResult<T> = Result<T, CartridgeError>;

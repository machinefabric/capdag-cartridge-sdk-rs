//! FGND Plugin SDK
//! 
//! This library provides common data structures and traits for FGND document processing plugins.
//! It ensures consistency across different file handlers (PDF, EPUB, etc.) and enables
//! a unified plugin architecture with standardized cap-based calling.

pub mod document;
pub mod document_cap;
pub mod handler;
pub mod metadata;
pub mod output;
pub mod validation;

// Re-export cap SDK types
pub use capns::*;

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

/// Plugin SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Standard error type for plugin operations
pub type PluginError = anyhow::Error;

/// Plugin result type
pub type PluginResult<T> = Result<T, PluginError>;
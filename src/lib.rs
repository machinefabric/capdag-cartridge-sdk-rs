//! LBVR Plugin SDK
//! 
//! This library provides common data structures and traits for LBVR document processing plugins.
//! It ensures consistency across different file handlers (PDF, EPUB, etc.) and enables
//! a unified plugin architecture.

pub mod document;
pub mod handler;
pub mod metadata;
pub mod output;

pub use document::*;
pub use handler::*;
pub use metadata::*;
pub use output::*;

// Re-export common dependencies
pub use anyhow::{Result, Context};
pub use serde::{Deserialize, Serialize};

/// Plugin SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Standard error type for plugin operations
pub type PluginError = anyhow::Error;

/// Plugin result type
pub type PluginResult<T> = Result<T, PluginError>;
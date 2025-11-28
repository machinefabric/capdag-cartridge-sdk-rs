//! Unified cap-based plugin interface
//! 
//! This module defines the unified plugin interfaces with standardized cap-based calling.

// Re-export the unified manifest from capns
pub use capns::CapManifest;
pub use capns::ComponentMetadata;

/// Trait for plugins to provide metadata about themselves
pub trait PluginMetadata {
    /// Get plugin manifest
    fn plugin_manifest(&self) -> CapManifest;
    
    /// Get plugin caps
    fn caps(&self) -> Vec<capns::Cap> {
        self.plugin_manifest().caps
    }
}

/// Basic file information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    /// File path
    #[serde(serialize_with = "serialize_path", deserialize_with = "deserialize_path")]
    pub path: std::path::PathBuf,
    
    /// File size in bytes
    pub size: u64,
    
    /// Document type detected
    pub document_type: String,
    
    /// Whether the file appears to be valid
    pub is_valid: bool,
    
    /// Quick metadata (title, author if easily accessible)
    pub quick_metadata: Option<QuickMetadata>,
}

/// Quick metadata that can be extracted without full processing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickMetadata {
    /// Document title
    pub title: Option<String>,
    
    /// Primary author
    pub author: Option<String>,
    
    /// Page/section count
    pub page_count: Option<usize>,
}

/// Serialize PathBuf as string for JSON compatibility
fn serialize_path<S>(path: &std::path::PathBuf, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&path.to_string_lossy())
}

/// Deserialize string to PathBuf for JSON compatibility  
fn deserialize_path<'de, D>(deserializer: D) -> Result<std::path::PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let s = String::deserialize(deserializer)?;
    Ok(std::path::PathBuf::from(s))
}
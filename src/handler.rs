//! Unified capability-based plugin interface
//! 
//! This module defines the unified plugin interfaces with standardized capability-based calling.

/// Plugin priority levels
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginPriority {
    Optional,
    Recommended,
    Critical,
}

/// Plugin capabilities
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PluginCapabilities {
    pub capabilities: Vec<String>,
}

impl PluginCapabilities {
    /// Check if the plugin has a specific capability
    pub fn can(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

/// Plugin information for --plugin-info output
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin description
    pub description: String,

    /// Plugin priority level
    pub priority: PluginPriority,
    
    /// Plugin capabilities with file type specificity
    pub capabilities: PluginCapabilities,
    
    /// Plugin author/maintainer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

impl PluginInfo {
    /// Create a new plugin info
    pub fn new(
        name: String,
        version: String,
        description: String,
        capabilities: PluginCapabilities,
        priority: PluginPriority,
    ) -> Self {
        Self {
            name,
            version,
            description,
            priority,
            capabilities,
            author: None,
        }
    }
    
    /// Set the author of the plugin
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }
}

/// Trait for plugins to provide metadata about themselves
pub trait PluginMetadata {
    /// Get plugin information
    fn plugin_info(&self) -> PluginInfo;
    
    /// Get plugin capabilities
    fn capabilities(&self) -> PluginCapabilities {
        self.plugin_info().capabilities
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
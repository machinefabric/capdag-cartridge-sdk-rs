//! File handler trait and plugin interfaces
//! 
//! This module defines the core trait that document processing plugins must implement.

use async_trait::async_trait;
use std::path::Path;
use crate::{DocumentOutline, FileMetadata, PluginResult};

/// Core trait that all document file handlers must implement
#[async_trait]
pub trait DocumentHandler: Send + Sync {
    /// Get the name of this handler
    fn name(&self) -> &str;
    
    /// Get the version of this handler
    fn version(&self) -> &str;
    
    /// Get handler capabilities with file type specificity
    fn get_capabilities(&self) -> PluginCapabilities;
    
    /// Check if this handler can process the given file
    fn can_handle(&self, file_path: &Path) -> bool {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let file_type = ext_str.to_lowercase();
                let capabilities = self.get_capabilities();
                
                // Check for specific file type capabilities first, then wildcards
                capabilities.can_handle_file_type(&file_type)
            } else {
                false
            }
        } else {
            false
        }
    }
    
    /// Extract document metadata
    async fn extract_metadata(&self, file_path: &Path) -> PluginResult<FileMetadata>;
    
    /// Extract document outline/table of contents
    async fn extract_outline(&self, file_path: &Path) -> PluginResult<DocumentOutline>;
    
    /// Extract document pages with text content organized by pages and paragraphs
    async fn extract_pages(&self, file_path: &Path) -> PluginResult<crate::DocumentPages>;
    
    /// Validate that the file is not corrupted and can be processed
    async fn validate_file(&self, file_path: &Path) -> PluginResult<bool>;
    
    /// Get basic file information without full processing
    async fn get_file_info(&self, file_path: &Path) -> PluginResult<FileInfo>;
    
    /// Generate thumbnail image for the document
    /// Returns PNG image data
    async fn generate_thumbnail(&self, file_path: &Path, width: u32, height: u32) -> PluginResult<Vec<u8>>;
    
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

/// Registry for document handlers
pub struct HandlerRegistry {
    handlers: Vec<Box<dyn DocumentHandler>>,
}

impl std::fmt::Debug for HandlerRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HandlerRegistry")
            .field("handler_count", &self.handlers.len())
            .field("handlers", &self.handlers.iter().map(|h| h.name()).collect::<Vec<_>>())
            .finish()
    }
}

impl HandlerRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
    
    /// Register a document handler
    pub fn register<H: DocumentHandler + 'static>(&mut self, handler: H) {
        self.handlers.push(Box::new(handler));
    }
    
    /// Find a handler for the given file
    pub fn find_handler(&self, file_path: &Path) -> Option<&dyn DocumentHandler> {
        self.handlers.iter()
            .find(|handler| handler.can_handle(file_path))
            .map(|boxed| boxed.as_ref())
    }
    
    /// Get all registered handlers
    pub fn handlers(&self) -> &[Box<dyn DocumentHandler>] {
        &self.handlers
    }
    
    /// Get handlers that support a specific file type
    pub fn handlers_for_file_type(&self, file_type: &str) -> Vec<&dyn DocumentHandler> {
        self.handlers.iter()
            .filter(|handler| {
                handler.get_capabilities().can_handle_file_type(file_type)
            })
            .map(|boxed| boxed.as_ref())
            .collect()
    }

    /// Get the number of registered handlers
    pub fn get_handler_count(&self) -> usize {
        self.handlers.len()
    }

    /// Get all supported file types
    pub fn get_supported_file_types(&self) -> Vec<String> {
        let mut file_types = std::collections::HashSet::new();
        for handler in &self.handlers {
            for capability in &handler.get_capabilities().capabilities {
                if let Some(colon_pos) = capability.find(':') {
                    let filetype = &capability[colon_pos + 1..];
                    if filetype != "*" {
                        file_types.insert(filetype.to_lowercase());
                    }
                }
            }
        }
        file_types.into_iter().collect()
    }
    
    /// Find the best handler for a specific operation and file type
    pub fn find_best_handler(&self, operation: &str, file_type: &str) -> Option<&dyn DocumentHandler> {
        let mut best_handler: Option<&dyn DocumentHandler> = None;
        let mut best_specificity = 0;
        
        for handler in &self.handlers {
            let capabilities = handler.get_capabilities();
            if let Some(capability) = capabilities.get_most_specific_capability(operation, file_type) {
                let specificity = if capability.contains(&format!(":{}", file_type)) {
                    2 // Exact file type match
                } else if capability.contains(":*") {
                    1 // Wildcard match
                } else {
                    0 // Legacy operation-only match
                };
                
                if specificity > best_specificity {
                    best_handler = Some(handler.as_ref());
                    best_specificity = specificity;
                }
            }
        }
        
        best_handler
    }

    /// Check if a file is supported by any handler
    pub fn is_supported(&self, file_path: &Path) -> bool {
        self.handlers.iter().any(|handler| handler.can_handle(file_path))
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}


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
    
    /// Check if the plugin can handle a specific file type
    pub fn can_handle_file_type(&self, file_type: &str) -> bool {
        // Check for exact match with file type (e.g., "extract_metadata:pdf")
        let specific_match = self.capabilities.iter().any(|capability| {
            if let Some(colon_pos) = capability.find(':') {
                let (operation, filetype) = capability.split_at(colon_pos);
                let filetype = &filetype[1..]; // Remove the ':'
                filetype == file_type && self.has_extract_operations(operation)
            } else {
                false
            }
        });
        
        if specific_match {
            return true;
        }
        
        // Check for wildcard match (e.g., "extract_metadata:*")
        self.capabilities.iter().any(|capability| {
            if let Some(colon_pos) = capability.find(':') {
                let (operation, filetype) = capability.split_at(colon_pos);
                let filetype = &filetype[1..]; // Remove the ':'
                filetype == "*" && self.has_extract_operations(operation)
            } else {
                false
            }
        })
    }
    
    /// Check if an operation is a document processing operation
    fn has_extract_operations(&self, operation: &str) -> bool {
        matches!(operation, 
            "extract_metadata" | "extract_outline" | "extract_pages" | 
            "extract_text" | "validate_file" | "generate_thumbnail"
        )
    }
    
    /// Get the most specific capability for a given operation and file type
    pub fn get_most_specific_capability(&self, operation: &str, file_type: &str) -> Option<String> {
        // First look for exact file type match
        let specific = format!("{}:{}", operation, file_type);
        if self.capabilities.contains(&specific) {
            return Some(specific);
        }
        
        // Then look for wildcard match
        let wildcard = format!("{}:*", operation);
        if self.capabilities.contains(&wildcard) {
            return Some(wildcard);
        }
        
        // Finally check for operation without file type specifier (legacy support)
        if self.capabilities.contains(&operation.to_string()) {
            return Some(operation.to_string());
        }
        
        None
    }
    
    /// Check if the plugin can perform an operation on a specific file type
    pub fn can_perform_operation(&self, operation: &str, file_type: &str) -> bool {
        self.get_most_specific_capability(operation, file_type).is_some()
    }
}

/// Plugin information
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

/// Result of a document processing operation
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    /// Whether the operation was successful
    pub success: bool,
    
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    
    /// Any warnings generated during processing
    pub warnings: Vec<String>,
    
    /// Error message if operation failed
    pub error: Option<String>,
}

impl ProcessingResult {
    /// Create a successful result
    pub fn success(processing_time_ms: u64) -> Self {
        Self {
            success: true,
            processing_time_ms,
            warnings: Vec::new(),
            error: None,
        }
    }
    
    /// Create a failed result
    pub fn failure(error: impl Into<String>, processing_time_ms: u64) -> Self {
        Self {
            success: false,
            processing_time_ms,
            warnings: Vec::new(),
            error: Some(error.into()),
        }
    }
    
    /// Add a warning
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }
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
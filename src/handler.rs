//! File handler trait and plugin interfaces
//! 
//! This module defines the core trait that document processing plugins must implement.

use async_trait::async_trait;
use std::path::Path;
use crate::{DocumentOutline, DocumentMetadata, PluginResult};

/// Core trait that all document file handlers must implement
#[async_trait]
pub trait DocumentHandler: Send + Sync {
    /// Get the name of this handler
    fn name(&self) -> &str;
    
    /// Get the version of this handler
    fn version(&self) -> &str;
    
    /// Get the file extensions this handler supports (e.g., ["pdf"])
    fn supported_extensions(&self) -> Vec<String>;
    
    /// Check if this handler can process the given file
    fn can_handle(&self, file_path: &Path) -> bool {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.supported_extensions().iter()
                    .any(|ext| ext.eq_ignore_ascii_case(ext_str));
            }
        }
        false
    }
    
    /// Extract document metadata
    async fn extract_metadata(&self, file_path: &Path) -> PluginResult<DocumentMetadata>;
    
    /// Extract document outline/table of contents
    async fn extract_outline(&self, file_path: &Path) -> PluginResult<DocumentOutline>;
    
    /// Extract text content from the document
    async fn extract_text(&self, file_path: &Path) -> PluginResult<String>;
    
    /// Validate that the file is not corrupted and can be processed
    async fn validate_file(&self, file_path: &Path) -> PluginResult<bool>;
    
    /// Get basic file information without full processing
    async fn get_file_info(&self, file_path: &Path) -> PluginResult<FileInfo>;
    
    /// Generate thumbnail image for the document
    /// Returns PNG image data
    async fn generate_thumbnail(&self, file_path: &Path, width: u32, height: u32) -> PluginResult<Vec<u8>>;
    
    /// Get handler capabilities
    fn get_capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::default()
    }
}

/// Basic file information
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// File path
    pub path: std::path::PathBuf,
    
    /// File size in bytes
    pub size: u64,
    
    /// Document type detected
    pub document_type: crate::DocumentType,
    
    /// Whether the file appears to be valid
    pub is_valid: bool,
    
    /// Quick metadata (title, author if easily accessible)
    pub quick_metadata: Option<QuickMetadata>,
}

/// Quick metadata that can be extracted without full processing
#[derive(Debug, Clone)]
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
    
    /// Get handlers that support a specific extension
    pub fn handlers_for_extension(&self, extension: &str) -> Vec<&dyn DocumentHandler> {
        self.handlers.iter()
            .filter(|handler| {
                handler.supported_extensions().iter()
                    .any(|ext| ext.eq_ignore_ascii_case(extension))
            })
            .map(|boxed| boxed.as_ref())
            .collect()
    }

    /// Get the number of registered handlers
    pub fn get_handler_count(&self) -> usize {
        self.handlers.len()
    }

    /// Get all supported file extensions
    pub fn get_supported_extensions(&self) -> Vec<String> {
        let mut extensions = std::collections::HashSet::new();
        for handler in &self.handlers {
            for ext in handler.supported_extensions() {
                extensions.insert(ext.to_lowercase());
            }
        }
        extensions.into_iter().collect()
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

/// Plugin capabilities
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PluginCapabilities {
    pub capabilities: Vec<String>,
}

impl PluginCapabilities {
	pub fn can(&self, capability: &str) -> bool {
		self.capabilities.iter().any(|c| c == capability)
	}
}

/// Plugin information
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin description
    pub description: String,
    
    /// Supported file extensions
    pub extensions: Vec<String>,
    
    /// Plugin capabilities
    pub capabilities: PluginCapabilities,
    
    /// Plugin author/maintainer
    pub author: Option<String>,
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
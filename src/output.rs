//! Standardized output formats for plugin results
//! 
//! This module provides common output formatting functions and structures
//! to ensure consistent output across all plugins.

use crate::FileMetadata;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    /// Human-readable text format
    Text,
    /// JSON format
    Json,
    /// Structured data for programmatic use
    Structured,
}

/// Formatter for document metadata
pub struct MetadataFormatter;

impl MetadataFormatter {
    /// Format metadata as human-readable text
    pub fn format_text(metadata: &FileMetadata) -> String {
        let mut output = String::new();
        
        output.push_str("=== Document Metadata ===\n\n");
        
        if let Some(title) = &metadata.title {
            output.push_str(&format!("Title: {}\n", title));
        }
        
        if !metadata.authors.is_empty() {
            output.push_str(&format!("Author(s): {}\n", metadata.authors.join(", ")));
        }
        
        if let Some(subject) = &metadata.subject {
            output.push_str(&format!("Subject: {}\n", subject));
        }
        
        if let Some(creator) = &metadata.creator {
            output.push_str(&format!("Creator: {}\n", creator));
        }
        
        if let Some(producer) = &metadata.producer {
            output.push_str(&format!("Producer: {}\n", producer));
        }
        
        if let Some(language) = &metadata.language {
            output.push_str(&format!("Language: {}\n", language));
        }
        
        if let Some(creation_date) = &metadata.creation_date {
            output.push_str(&format!("Creation Date: {}\n", creation_date));
        }
        
        if let Some(modification_date) = &metadata.modification_date {
            output.push_str(&format!("Modification Date: {}\n", modification_date));
        }
        
        if let Some(identifier) = &metadata.identifier {
            output.push_str(&format!("Identifier: {}\n", identifier));
        }
        
        if !metadata.keywords.is_empty() {
            output.push_str(&format!("Keywords: {}\n", metadata.keywords.join(", ")));
        }
        
        output.push_str(&format!("File Size: {} bytes ({:.2} MB)\n", 
            metadata.file_size_bytes, 
            metadata.file_size_bytes as f64 / 1_048_576.0
        ));
        
        output.push_str(&format!("Document Type: {}\n", metadata.document_type));
        
        if let Some(version) = &metadata.format_version {
            output.push_str(&format!("Format Version: {}\n", version));
        }
        
        if !metadata.extended_metadata.is_empty() {
            output.push_str("\nExtended Metadata:\n");
            for (key, value) in &metadata.extended_metadata {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
        }
        
        output
    }
}

/// Combined output containing multiple types of extracted data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedData {
    /// Document metadata
    pub metadata: Option<FileMetadata>,
    
    /// Extracted text content
    pub text_content: Option<String>,
    
    /// Thumbnail information
    pub thumbnail: Option<ThumbnailInfo>,
    
    /// Extraction summary
    pub extraction_summary: ExtractionSummary,
}

/// Information about an extracted thumbnail
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThumbnailInfo {
    /// Image format (png, jpg, etc.)
    pub format: String,
    
    /// Image dimensions if known
    pub dimensions: Option<(u32, u32)>,
    
    /// File size of the image data
    pub size_bytes: usize,
    
    /// Suggested filename
    pub filename: String,
}

/// Summary of what was extracted
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractionSummary {
    /// Source file path
    pub source_file: String,
    
    /// Handler that performed the extraction
    pub handler_name: String,
    
    /// Extraction timestamp
    pub extracted_at: String,
    
    /// What was successfully extracted
    pub extracted_components: Vec<String>,
    
    /// Any errors or warnings
    pub warnings: Vec<String>,
    
    /// Total processing time in milliseconds
    pub processing_time_ms: u64,
}

impl ExtractedData {
    /// Create new extracted data
    pub fn new(source_file: impl Into<String>, handler_name: impl Into<String>) -> Self {
        Self {
            metadata: None,
            text_content: None,
            thumbnail: None,
            extraction_summary: ExtractionSummary {
                source_file: source_file.into(),
                handler_name: handler_name.into(),
                extracted_at: chrono::Utc::now().to_rfc3339(),
                extracted_components: Vec::new(),
                warnings: Vec::new(),
                processing_time_ms: 0,
            },
        }
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, metadata: FileMetadata) -> Self {
        self.metadata = Some(metadata);
        self.extraction_summary.extracted_components.push("metadata".to_string());
        self
    }
    
    /// Add text content
    pub fn with_text(mut self, text: String) -> Self {
        self.text_content = Some(text);
        self.extraction_summary.extracted_components.push("text".to_string());
        self
    }
    
    /// Add thumbnail
    pub fn with_thumbnail(mut self, thumbnail: ThumbnailInfo) -> Self {
        self.thumbnail = Some(thumbnail);
        self.extraction_summary.extracted_components.push("thumbnail".to_string());
        self
    }
    
    /// Add warning
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.extraction_summary.warnings.push(warning.into());
    }
    
    /// Set processing time
    pub fn set_processing_time(&mut self, time_ms: u64) {
        self.extraction_summary.processing_time_ms = time_ms;
    }
}

impl fmt::Display for ExtractedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExtractedData from {} ({})", 
            self.extraction_summary.source_file,
            self.extraction_summary.extracted_components.join(", ")
        )
    }
}

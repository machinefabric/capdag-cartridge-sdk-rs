//! Standardized output formats for plugin results
//! 
//! This module provides common output formatting functions and structures
//! to ensure consistent output across all plugins.

use serde::{Deserialize, Serialize};
use std::fmt;
use crate::{DocumentOutline, DocumentMetadata, TocEntry};

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

/// Formatter for document outlines
pub struct OutlineFormatter;

impl OutlineFormatter {
    /// Format outline as human-readable text
    pub fn format_text(outline: &DocumentOutline) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("=== Document Outline for {} ===\n\n", outline.source_file));
        
        if let Some(title) = &outline.document_title {
            output.push_str(&format!("Document Title: {}\n", title));
        }
        
        output.push_str(&format!("Document Type: {}\n", outline.document_type));
        output.push_str(&format!("Total Pages/Sections: {}\n\n", outline.total_pages));
        
        if outline.has_outline && !outline.entries.is_empty() {
            output.push_str("Table of Contents:\n");
            Self::format_entries_text(&outline.entries, &mut output, 0, &mut 0);
        } else {
            output.push_str("No table of contents found in this document.\n");
        }
        
        if !outline.extraction_info.warnings.is_empty() {
            output.push_str("\nWarnings:\n");
            for warning in &outline.extraction_info.warnings {
                output.push_str(&format!("  - {}\n", warning));
            }
        }
        
        output
    }
    
    /// Format outline as JSON
    pub fn format_json(outline: &DocumentOutline) -> serde_json::Result<String> {
        serde_json::to_string_pretty(outline)
    }
    
    /// Format TOC entries as text (recursive helper)
    fn format_entries_text(
        entries: &[TocEntry], 
        output: &mut String, 
        base_level: usize, 
        counter: &mut usize
    ) {
        for entry in entries {
            let indent = "  ".repeat(entry.level + base_level);
            *counter += 1;
            
            let page_info = match entry.page {
                Some(page) => format!(" (page {})", page),
                None => String::new(),
            };
            
            let source_info = match &entry.source_ref {
                Some(source) => format!(" [{}]", source),
                None => String::new(),
            };
            
            output.push_str(&format!(
                "{}{}.{} {}{}{}\n",
                indent,
                entry.level + 1,
                *counter,
                entry.title,
                page_info,
                source_info
            ));
            
            if !entry.children.is_empty() {
                Self::format_entries_text(&entry.children, output, base_level, counter);
            }
        }
    }
}

/// Formatter for document metadata
pub struct MetadataFormatter;

impl MetadataFormatter {
    /// Format metadata as human-readable text
    pub fn format_text(metadata: &DocumentMetadata) -> String {
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
            metadata.file_size, 
            metadata.file_size as f64 / 1_048_576.0
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
    
    /// Format metadata as JSON
    pub fn format_json(metadata: &DocumentMetadata) -> serde_json::Result<String> {
        serde_json::to_string_pretty(metadata)
    }
}

/// Combined output containing multiple types of extracted data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedData {
    /// Document metadata
    pub metadata: Option<DocumentMetadata>,
    
    /// Document outline/TOC
    pub outline: Option<DocumentOutline>,
    
    /// Extracted text content
    pub text_content: Option<String>,
    
    /// Cover image information
    pub cover_image: Option<CoverImageInfo>,
    
    /// Extraction summary
    pub extraction_summary: ExtractionSummary,
}

/// Information about an extracted cover image
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoverImageInfo {
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
            outline: None,
            text_content: None,
            cover_image: None,
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
    pub fn with_metadata(mut self, metadata: DocumentMetadata) -> Self {
        self.metadata = Some(metadata);
        self.extraction_summary.extracted_components.push("metadata".to_string());
        self
    }
    
    /// Add outline
    pub fn with_outline(mut self, outline: DocumentOutline) -> Self {
        self.outline = Some(outline);
        self.extraction_summary.extracted_components.push("outline".to_string());
        self
    }
    
    /// Add text content
    pub fn with_text(mut self, text: String) -> Self {
        self.text_content = Some(text);
        self.extraction_summary.extracted_components.push("text".to_string());
        self
    }
    
    /// Add cover image
    pub fn with_cover(mut self, cover: CoverImageInfo) -> Self {
        self.cover_image = Some(cover);
        self.extraction_summary.extracted_components.push("cover".to_string());
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
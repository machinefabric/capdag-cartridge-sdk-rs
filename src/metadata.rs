//! Document metadata structures
//! 
//! This module defines common metadata structures that can be extracted
//! from various document formats.

use serde::{Deserialize, Serialize};
use crate::DocumentType;

/// Common document metadata structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentMetadata {
    /// Document title
    pub title: Option<String>,
    
    /// Author(s)
    pub authors: Vec<String>,
    
    /// Subject/description
    pub subject: Option<String>,
    
    /// Publisher/creator application
    pub creator: Option<String>,
    
    /// PDF producer or EPUB generator
    pub producer: Option<String>,
    
    /// Document language
    pub language: Option<String>,
    
    /// Publication date
    pub creation_date: Option<String>,
    
    /// Last modification date
    pub modification_date: Option<String>,
    
    /// Document identifier (ISBN for books, etc.)
    pub identifier: Option<String>,
    
    /// Keywords/tags
    pub keywords: Vec<String>,
    
    /// File size in bytes
    pub file_size: u64,
    
    /// Document type
    pub document_type: DocumentType,
    
    /// Format-specific version (PDF version, EPUB version, etc.)
    pub format_version: Option<String>,
    
    /// Additional format-specific metadata
    pub extended_metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl DocumentMetadata {
    /// Create new metadata for a document
    pub fn new(document_type: impl Into<String>, file_size: u64) -> Self {
        Self {
            title: None,
            authors: Vec::new(),
            subject: None,
            creator: None,
            producer: None,
            language: None,
            creation_date: None,
            modification_date: None,
            identifier: None,
            keywords: Vec::new(),
            file_size,
            document_type: document_type.into(),
            format_version: None,
            extended_metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    /// Add an author
    pub fn add_author(&mut self, author: impl Into<String>) {
        self.authors.push(author.into());
    }
    
    /// Set a single author (convenience method)
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.authors.push(author.into());
        self
    }
    
    /// Set the subject
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }
    
    /// Set the language
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
    
    /// Add a keyword
    pub fn add_keyword(&mut self, keyword: impl Into<String>) {
        self.keywords.push(keyword.into());
    }
    
    /// Set extended metadata value
    pub fn set_extended(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.extended_metadata.insert(key.into(), value);
    }
    
    /// Get extended metadata value
    pub fn get_extended(&self, key: &str) -> Option<&serde_json::Value> {
        self.extended_metadata.get(key)
    }
    
    /// Get primary author (first in list)
    pub fn primary_author(&self) -> Option<&str> {
        self.authors.first().map(|s| s.as_str())
    }
    
    /// Check if metadata is empty (has no meaningful content)
    pub fn is_empty(&self) -> bool {
        self.title.is_none() 
            && self.authors.is_empty() 
            && self.subject.is_none() 
            && self.creator.is_none()
    }
}

/// PDF-specific metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdfMetadata {
    /// Base metadata
    #[serde(flatten)]
    pub base: DocumentMetadata,
    
    /// PDF version
    pub pdf_version: Option<String>,
    
    /// Number of pages
    pub page_count: usize,
    
    /// Whether PDF contains forms
    pub has_forms: bool,
    
    /// Whether PDF is encrypted
    pub is_encrypted: bool,
    
    /// Number of attachments
    pub attachment_count: usize,
    
    /// Whether PDF is linearized (fast web view)
    pub is_linearized: bool,
}

impl PdfMetadata {
    /// Create new PDF metadata
    pub fn new(file_size: u64, page_count: usize) -> Self {
        Self {
            base: DocumentMetadata::new("pdf", file_size),
            pdf_version: None,
            page_count,
            has_forms: false,
            is_encrypted: false,
            attachment_count: 0,
            is_linearized: false,
        }
    }
}

/// EPUB-specific metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpubMetadata {
    /// Base metadata
    #[serde(flatten)]
    pub base: DocumentMetadata,
    
    /// EPUB version (2.0, 3.0, etc.)
    pub epub_version: Option<String>,
    
    /// Publisher
    pub publisher: Option<String>,
    
    /// Publication date
    pub publication_date: Option<String>,
    
    /// Rights information
    pub rights: Option<String>,
    
    /// Number of chapters/sections
    pub chapter_count: usize,
    
    /// Whether EPUB has DRM
    pub has_drm: bool,
    
    /// Cover image path (if any)
    pub cover_image_path: Option<String>,
}

impl EpubMetadata {
    /// Create new EPUB metadata
    pub fn new(file_size: u64, chapter_count: usize) -> Self {
        Self {
            base: DocumentMetadata::new("epub", file_size),
            epub_version: None,
            publisher: None,
            publication_date: None,
            rights: None,
            chapter_count,
            has_drm: false,
            cover_image_path: None,
        }
    }
}
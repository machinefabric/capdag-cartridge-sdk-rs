//! Document outline and table of contents data structures
//! 
//! This module defines the unified data model for document outlines/TOCs
//! that can be used across different document formats (PDF, EPUB, etc.)

use serde::{Deserialize, Serialize};

/// A single entry in a document's table of contents
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OutlineEntry {
    /// The title/label of this Outline entry
    pub title: String,
    
    /// Hierarchical level (0 = top level, 1 = first sublevel, etc.)
    pub level: usize,
    
    /// Page/section number this entry points to (1-indexed, None if no destination)
    /// For PDFs this is typically a page number, for EPUBs it might be a section number
    pub page: Option<usize>,
    
    /// Optional source reference (filename, anchor, etc.)
    pub source_ref: Option<String>,
    
    /// Child entries under this one
    pub children: Vec<OutlineEntry>,
}

impl OutlineEntry {
    /// Create a new Outline entry
    pub fn new(title: impl Into<String>, level: usize) -> Self {
        Self {
            title: title.into(),
            level,
            page: None,
            source_ref: None,
            children: Vec::new(),
        }
    }
    
    /// Set the page/section number
    pub fn with_page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }
    
    /// Set the source reference
    pub fn with_source_ref(mut self, source_ref: impl Into<String>) -> Self {
        self.source_ref = Some(source_ref.into());
        self
    }
    
    /// Add a child entry
    pub fn add_child(&mut self, child: OutlineEntry) {
        self.children.push(child);
    }
    
    /// Get total number of entries (including children)
    pub fn count_all_entries(&self) -> usize {
        1 + self.children.iter().map(|child| child.count_all_entries()).sum::<usize>()
    }
    
    /// Find entry by title (depth-first search)
    pub fn find_by_title(&self, title: &str) -> Option<&OutlineEntry> {
        if self.title == title {
            return Some(self);
        }
        
        for child in &self.children {
            if let Some(found) = child.find_by_title(title) {
                return Some(found);
            }
        }
        
        None
    }
}

/// Complete document outline structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentOutline {
    /// Source file path
    pub source_file: String,
    
    /// Document title (from metadata if available)
    pub document_title: Option<String>,
    
    /// Document format/type (PDF, EPUB, etc.)
    pub document_type: String,
    
    /// Total number of pages/sections in document
    pub total_pages: usize,
    
    /// Root-level Outline entries
    pub entries: Vec<OutlineEntry>,
    
    /// Whether this document has any outline/bookmarks
    pub has_outline: bool,
    
    /// Metadata about the extraction process
    pub extraction_info: ExtractionInfo,
}

impl DocumentOutline {
    /// Create a new document outline
    pub fn new(
        source_file: impl Into<String>,
        document_type: impl Into<String>,
        total_pages: usize,
    ) -> Self {
        Self {
            source_file: source_file.into(),
            document_title: None,
            document_type: document_type.into(),
            total_pages,
            entries: Vec::new(),
            has_outline: false,
            extraction_info: ExtractionInfo::default(),
        }
    }
    
    /// Set the document title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.document_title = Some(title.into());
        self
    }
    
    /// Add a root-level Outline entry
    pub fn add_entry(&mut self, entry: OutlineEntry) {
        self.entries.push(entry);
        self.has_outline = true;
    }
    
    /// Get total number of Outline entries (including all children)
    pub fn total_entries(&self) -> usize {
        self.entries.iter().map(|entry| entry.count_all_entries()).sum()
    }
    
    /// Check if outline is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}


/// A single page within a document
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChip {
    /// Page number (1-indexed)
    pub order_index: usize,
    
    /// Text content of this page as a single string
    pub text_content: String,
    
    /// Optional source reference (filename, section, etc.)
    pub source_ref: Option<String>,
    
    /// Word count for this page
    pub word_count: Option<usize>,
    
    /// Character count for this page
    pub character_count: Option<usize>,
}

impl FileChip {
    /// Create a new file chip
    pub fn new(order_index: usize) -> Self {
        Self {
            order_index,
            text_content: String::new(),
            source_ref: None,
            word_count: None,
            character_count: None,
        }
    }
    
    /// Create a new file chip with text content
    pub fn new_with_text(order_index: usize, text_content: impl Into<String>) -> Self {
        let content = text_content.into();
        let trimmed_content = content.trim().to_string();
        
        // Calculate word and character counts
        let word_count = if trimmed_content.is_empty() {
            0
        } else {
            trimmed_content.split_whitespace().count()
        };
        let character_count = trimmed_content.chars().count();
        
        Self {
            order_index,
            text_content: trimmed_content,
            source_ref: None,
            word_count: Some(word_count),
            character_count: Some(character_count),
        }
    }
    
    /// Set the source reference
    pub fn with_source_ref(mut self, source_ref: impl Into<String>) -> Self {
        self.source_ref = Some(source_ref.into());
        self
    }
    
    /// Get text content for this page
    pub fn get_text_content(&self) -> String {
        self.text_content.clone()
    }
    
    /// Get word count for this page
    pub fn word_count(&self) -> usize {
        self.word_count.unwrap_or_else(|| {
            if self.text_content.trim().is_empty() {
                0
            } else {
                self.text_content.split_whitespace().count()
            }
        })
    }
    
    /// Get character count for this page
    pub fn character_count(&self) -> usize {
        self.character_count.unwrap_or_else(|| self.text_content.chars().count())
    }
    
    /// Check if page is empty
    pub fn is_empty(&self) -> bool {
        self.text_content.trim().is_empty()
    }
}

/// Complete document with pages
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisboundPages {
    /// Source file path
    pub source_file: String,
    
    /// Document title (from metadata if available)
    pub document_title: Option<String>,
    
    /// Document format/type (PDF, EPUB, etc.)
    pub document_type: String,
    
    /// Total number of pages in document
    pub total_pages: usize,
    
    /// All pages in the document
    pub pages: Vec<FileChip>,
    
    /// Metadata about the extraction process
    pub extraction_info: ExtractionInfo,
}

impl DisboundPages {
    /// Create a new file chips structure
    pub fn new(
        source_file: impl Into<String>,
        document_type: impl Into<String>,
    ) -> Self {
        Self {
            source_file: source_file.into(),
            document_title: None,
            document_type: document_type.into(),
            total_pages: 0,
            pages: Vec::new(),
            extraction_info: ExtractionInfo::default(),
        }
    }
    
    /// Set the document title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.document_title = Some(title.into());
        self
    }
    
    /// Add a page to the document
    pub fn add_chip(&mut self, page: FileChip) {
        self.pages.push(page);
        self.total_pages = self.pages.len();
    }
    
    /// Get a specific page by number (1-indexed)
    pub fn get_chip(&self, order_index: usize) -> Option<&FileChip> {
        self.pages.iter().find(|p| p.order_index == order_index)
    }
    
    /// Get all text content concatenated
    pub fn get_all_text(&self) -> String {
        self.pages.iter()
            .map(|page| page.get_text_content())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
    
    /// Get total word count across all pages
    pub fn total_word_count(&self) -> usize {
        self.pages.iter()
            .map(|page| page.word_count())
            .sum()
    }
    
    /// Get total character count across all pages
    pub fn total_character_count(&self) -> usize {
        self.pages.iter()
            .map(|page| page.character_count())
            .sum()
    }
    
    
    /// Check if document is empty
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty() || self.pages.iter().all(|p| p.is_empty())
    }
}

/// Information about the extraction process
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractionInfo {
    /// Tool that performed the extraction
    pub extractor_name: String,
    
    /// Version of the extraction tool
    pub extractor_version: String,
    
    /// Timestamp of extraction
    pub extracted_at: Option<String>,
    
    /// Any warnings or notes about the extraction
    pub warnings: Vec<String>,
}

impl Default for ExtractionInfo {
    fn default() -> Self {
        Self {
            extractor_name: "unknown".to_string(),
            extractor_version: "unknown".to_string(),
            extracted_at: None,
            warnings: Vec::new(),
        }
    }
}

impl ExtractionInfo {
    /// Create extraction info for a specific tool
    pub fn new(extractor_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            extractor_name: extractor_name.into(),
            extractor_version: version.into(),
            extracted_at: Some(chrono::Utc::now().to_rfc3339()),
            warnings: Vec::new(),
        }
    }
    
    /// Add a warning message
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_outline_entry_creation() {
        let entry = OutlineEntry::new("Chapter 1", 0)
            .with_page(5)
            .with_source_ref("chapter1.html");
        
        assert_eq!(entry.title, "Chapter 1");
        assert_eq!(entry.level, 0);
        assert_eq!(entry.page, Some(5));
        assert_eq!(entry.source_ref, Some("chapter1.html".to_string()));
    }
    
    #[test]
    fn test_document_outline_creation() {
        let mut outline = DocumentOutline::new("test.pdf", "pdf".to_string(), 10)
            .with_title("Test Document");
        
        let entry = OutlineEntry::new("Introduction", 0).with_page(1);
        outline.add_entry(entry);
        
        assert_eq!(outline.document_title, Some("Test Document".to_string()));
        assert_eq!(outline.total_pages, 10);
        assert_eq!(outline.total_entries(), 1);
        assert!(outline.has_outline);
    }
    
    #[test]
    fn test_nested_outline_entries() {
        let mut parent = OutlineEntry::new("Chapter 1", 0);
        let child1 = OutlineEntry::new("Section 1.1", 1);
        let child2 = OutlineEntry::new("Section 1.2", 1);
        
        parent.add_child(child1);
        parent.add_child(child2);
        
        assert_eq!(parent.count_all_entries(), 3); // parent + 2 children
        assert_eq!(parent.children.len(), 2);
    }
}
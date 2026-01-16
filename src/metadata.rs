//! Consolidated file metadata structure
//! 
//! This module consolidates previous DocumentMetadata, PdfMetadata, EpubMetadata
//! and the original FileMetadata into a single FileMetadata type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;
use std::path::Path;

/// Consolidated metadata for any file/document type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
	// Basic file info
	pub file_path: String,
	pub file_size_bytes: u64,
	pub content_length: usize,
	pub media_urn: Option<String>,
	pub encoding: Option<String>,

	// Common document metadata
	pub title: Option<String>,
	pub subject: Option<String>,
	pub identifier: Option<String>,
	pub language: Option<String>,
	pub creator: Option<String>,
	pub producer: Option<String>,

	// People and keywords
	pub authors: Vec<String>,
	pub contributors: Vec<String>,
	pub keywords: Vec<String>,

	// Dates (stored as strings; ISO-like formats recommended)
	pub creation_date: Option<String>,
	pub modification_date: Option<String>,

	// Counts
	pub page_count: Option<usize>,
	pub chapter_count: Option<usize>,
	pub word_count: Option<usize>,
	pub character_count: Option<usize>,

	// Document classification and versions
	pub document_type: String,
	pub format_version: Option<String>,

	// Extended/format-specific metadata (arbitrary JSON values)
	pub extended_metadata: HashMap<String, Value>,

	// PDF-specific
	pub pdf_version: Option<String>,
	pub has_forms: bool,
	pub is_encrypted: bool,
	pub attachment_count: usize,
	pub is_linearized: bool,

	// EPUB-specific
	pub epub_version: Option<String>,
	pub publisher: Option<String>,
	pub publication_date: Option<String>,
	pub rights: Option<String>,
	pub has_drm: bool,
	pub thumbnail_path: Option<String>,
}

impl FileMetadata {
	/// Create a new consolidated FileMetadata
	pub fn new(file_path: impl Into<String>, document_type: String, file_size_bytes: u64) -> Self {
		Self {
			file_path: file_path.into(),
			file_size_bytes,
			content_length: 0,
			media_urn: None,
			encoding: None,

			title: None,
			subject: None,
			identifier: None,
			language: None,
			creator: None,
			producer: None,

			authors: Vec::new(),
			contributors: Vec::new(),
			keywords: Vec::new(),

			creation_date: None,
			modification_date: None,

			page_count: None,
			chapter_count: None,
			word_count: None,
			character_count: None,

			document_type,
			format_version: None,

			extended_metadata: HashMap::new(),

			pdf_version: None,
			has_forms: false,
			is_encrypted: false,
			attachment_count: 0,
			is_linearized: false,

			epub_version: None,
			publisher: None,
			publication_date: None,
			rights: None,
			has_drm: false,
			thumbnail_path: None,
		}
	}

	/// Minimal metadata with just file path, size and optional mime type
	pub fn minimal(file_path: impl Into<String>, file_size_bytes: u64, media_urn: Option<String>, document_type: String) -> Self {
		let mut s = Self::new(file_path, document_type, file_size_bytes);
		s.media_urn = media_urn;
		s
	}

	/// Add an author
	pub fn add_author(&mut self, author: impl Into<String>) {
		self.authors.push(author.into());
	}

	/// Add a contributor
	pub fn add_contributor(&mut self, contributor: impl Into<String>) {
		self.contributors.push(contributor.into());
	}

	/// Add a keyword
	pub fn add_keyword(&mut self, keyword: impl Into<String>) {
		self.keywords.push(keyword.into());
	}

	/// Set extended metadata value
	pub fn set_extended(&mut self, key: impl Into<String>, value: Value) {
		self.extended_metadata.insert(key.into(), value);
	}

	/// Get extended metadata value
	pub fn get_extended(&self, key: &str) -> Option<&Value> {
		self.extended_metadata.get(key)
	}

	/// Primary author, if any
	pub fn primary_author(&self) -> Option<&str> {
		self.authors.first().map(|s| s.as_str())
	}

	/// Check if metadata is essentially empty (no meaningful fields)
	pub fn is_empty(&self) -> bool {
		self.title.is_none()
			&& self.authors.is_empty()
			&& self.subject.is_none()
			&& self.creator.is_none()
			&& self.producer.is_none()
	}

	/// Create a short prompt-style summary of key metadata fields.
	/// Useful for embedding or quick display. Truncates long strings.
	pub fn to_prompt(&self) -> String {
		let mut parts = vec![];

		// Filename only
		if let Some(filename) = Path::new(&self.file_path).file_name() {
			parts.push(format!("File: {}", filename.to_string_lossy()));
		}

		if let Some(title) = &self.title {
			let t = if title.len() > 100 { format!("{}...", &title[..97]) } else { title.clone() };
			parts.push(format!("Title: {}", t));
		}

		if !self.authors.is_empty() {
			let authors_str = self.authors.join(", ");
			let a = if authors_str.len() > 150 { format!("{}...", &authors_str[..147]) } else { authors_str };
			parts.push(format!("Authors: {}", a));
		}

		if !self.contributors.is_empty() {
			let contrib = self.contributors.join(", ");
			let c = if contrib.len() > 150 { format!("{}...", &contrib[..147]) } else { contrib };
			parts.push(format!("Contributors: {}", c));
		}

		if !self.keywords.is_empty() {
			let keywords_limited: Vec<_> = self.keywords.iter().take(5).cloned().collect();
			parts.push(format!("Keywords: {}", keywords_limited.join(", ")));
		}

		if let Some(cd) = &self.creation_date {
			// Try to display just YYYY-MM-DD if possible
			let date = if cd.len() >= 10 { cd[..10].to_string() } else { cd.clone() };
			parts.push(format!("Created: {}", date));
		}

		if parts.is_empty() {
			"".to_string()
		} else {
			let metadata_str = parts.join("\n");
			if metadata_str.len() > 300 {
				format!("{}...", &metadata_str[..297])
			} else {
				metadata_str
			}
		}
	}
}

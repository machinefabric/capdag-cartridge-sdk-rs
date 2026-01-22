//! Document processing specific cap extensions
//!
//! This module provides extensions to the general cap system for document processing,
//! adding file type support and document-specific functionality.

use capns::{Cap, CapUrn};
use std::collections::HashMap;

/// Extension trait for document processing caps
pub trait DocumentCapExt {
    /// Check if this cap supports a given file type
    fn supports_file_type(&self, file_type: &str) -> bool;
    
    /// Get supported file types for this cap
    fn get_supported_file_types(&self) -> Vec<String>;
    
    /// Check if this cap supports all file types
    fn supports_all_file_types(&self) -> bool;
}

impl DocumentCapExt for Cap {
    fn supports_file_type(&self, file_type: &str) -> bool {
        if let Some(file_types_str) = self.get_metadata("file_types") {
            let file_types: Vec<&str> = file_types_str.split(',').map(|s| s.trim()).collect();
            file_types.contains(&"*") || file_types.contains(&file_type.to_lowercase().as_str())
        } else {
            false
        }
    }
    
    fn get_supported_file_types(&self) -> Vec<String> {
        if let Some(file_types_str) = self.get_metadata("file_types") {
            file_types_str.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            Vec::new()
        }
    }
    
    fn supports_all_file_types(&self) -> bool {
        self.get_supported_file_types().contains(&"*".to_string())
    }
}

/// Extension trait for document processing cap collections
pub trait DocumentCapsExt {
    /// Get caps that support a specific file type
    fn caps_for_file_type(&self, file_type: &str) -> Vec<&Cap>;
    
    /// Get all supported file types across all caps
    fn get_all_supported_file_types(&self) -> Vec<String>;
    
    /// Check if any cap supports all file types
    fn supports_all_file_types(&self) -> bool;
}

impl DocumentCapsExt for Vec<Cap> {
    fn caps_for_file_type(&self, file_type: &str) -> Vec<&Cap> {
        self.iter()
            .filter(|c| c.supports_file_type(file_type))
            .collect()
    }
    
    fn get_all_supported_file_types(&self) -> Vec<String> {
        let mut file_types = Vec::new();
        for cap in self {
            for file_type in cap.get_supported_file_types() {
                if !file_types.contains(&file_type) {
                    file_types.push(file_type);
                }
            }
        }
        file_types.sort();
        file_types
    }
    
    fn supports_all_file_types(&self) -> bool {
        self.iter().any(|c| c.supports_all_file_types())
    }
}

/// Helper functions for creating document processing caps
pub struct DocumentCapBuilder;

impl DocumentCapBuilder {
    /// Create a document processing cap with file type support
    pub fn new_document_cap(
        id_str: &str, 
        _version: &str, 
        file_types: Vec<&str>, 
        description: Option<&str>,
        metadata_json: Option<serde_json::Value>
    ) -> Result<Cap, capns::CapUrnError> {
        let id = CapUrn::from_string(id_str)?;
        let mut metadata = HashMap::new();
        metadata.insert("file_types".to_string(), file_types.join(","));
        
        let desc_option = description.map(|d| d.to_string());

        Ok(Cap::with_full_definition(
            id,
            "Document Cap".to_string(),
            desc_option,
            metadata,
            "document-cap".to_string(),
            HashMap::new(),
            vec![],
            None,
            metadata_json,
        ))
    }
    
    /// Create a cap that supports all file types
    pub fn new_universal_cap(
        id_str: &str, 
        version: &str, 
        description: Option<&str>,
        metadata_json: Option<serde_json::Value>
    ) -> Result<Cap, capns::CapUrnError> {
        Self::new_document_cap(id_str, version, vec!["*"], description, metadata_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_cap_file_types() {
        let cap = DocumentCapBuilder::new_document_cap(
            "cap:op=extract;target=metadata;",
            "1.0.0",
            vec!["pdf", "txt", "md"],
            Some("Extract document metadata"),
            None
        ).unwrap();
        
        assert!(cap.supports_file_type("pdf"));
        assert!(cap.supports_file_type("txt"));
        assert!(cap.supports_file_type("md"));
        assert!(!cap.supports_file_type("jpg"));
        
        let file_types = cap.get_supported_file_types();
        assert_eq!(file_types, vec!["pdf", "txt", "md"]);
    }
    
    #[test]
    fn test_universal_cap() {
        let cap = DocumentCapBuilder::new_universal_cap(
            "cap:op=extract;target=pages",
            "1.0.0",
            Some("grind any document"),
            None
        ).unwrap();
        
        assert!(cap.supports_all_file_types());
        assert!(cap.supports_file_type("pdf"));
        assert!(cap.supports_file_type("anything"));
    }
    
    #[test]
    fn test_document_caps_collection() {
        let mut caps = Vec::new();
        
        let cap1 = DocumentCapBuilder::new_document_cap(
            "cap:op=extract;target=metadata;",
            "1.0.0",
            vec!["pdf"],
            None,
            None
        ).unwrap();
        
        let cap2 = DocumentCapBuilder::new_document_cap(
            "cap:op=extract;target=text;",
            "1.0.0",
            vec!["txt", "md"],
            None,
            None
        ).unwrap();
        
        caps.push(cap1);
        caps.push(cap2);
        
        let pdf_caps = caps.caps_for_file_type("pdf");
        assert_eq!(pdf_caps.len(), 1);
        
        let txt_caps = caps.caps_for_file_type("txt");
        assert_eq!(txt_caps.len(), 1);
        
        let all_types = caps.get_all_supported_file_types();
        assert!(all_types.contains(&"pdf".to_string()));
        assert!(all_types.contains(&"txt".to_string()));
        assert!(all_types.contains(&"md".to_string()));
    }
}
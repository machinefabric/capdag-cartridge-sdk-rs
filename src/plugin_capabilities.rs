//! Plugin capabilities collection
//!
//! This module defines the PluginCapabilities type that collects capabilities
//! that plugins can provide.

use capdef::{Capability, CapabilityKey, CapabilityMatcher};
use serde::{Deserialize, Serialize};

/// Plugin capabilities collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapabilities {
    pub capabilities: Vec<Capability>,
}

impl PluginCapabilities {
    /// Create a new empty capabilities collection
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    /// Create capabilities collection from a list of capabilities
    pub fn from_capabilities(capabilities: Vec<Capability>) -> Self {
        Self { capabilities }
    }

    /// Add a capability to the collection
    pub fn add_capability(&mut self, capability: Capability) {
        self.capabilities.push(capability);
    }

    /// Check if the plugin has a specific capability
    pub fn can(&self, capability_request: &str) -> bool {
        self.capabilities.iter().any(|c| c.matches_request(capability_request))
    }
    
    /// Get all capability identifiers as strings
    pub fn get_capability_keys(&self) -> Vec<String> {
        self.capabilities.iter().map(|c| c.id.to_string()).collect()
    }
    
    /// Find a capability by identifier
    pub fn find_capability(&self, id: &str) -> Option<&Capability> {
        let search_id = CapabilityKey::from_string(id).ok()?;
        self.capabilities.iter().find(|c| c.id == search_id)
    }
    
    /// Find the most specific capability that can handle a request
    pub fn find_best_capability(&self, request: &str) -> Option<&Capability> {
        let request_id = CapabilityKey::from_string(request).ok()?;
        let capability_keys: Vec<CapabilityKey> = self.capabilities.iter().map(|c| c.id.clone()).collect();
        let best_id = CapabilityMatcher::find_best_match(&capability_keys, &request_id)?;
        self.capabilities.iter().find(|c| &c.id == best_id)
    }

    /// Get capabilities that have specific metadata
    pub fn capabilities_with_metadata(&self, key: &str, value: Option<&str>) -> Vec<&Capability> {
        self.capabilities
            .iter()
            .filter(|c| {
                if let Some(expected_value) = value {
                    c.get_metadata(key) == Some(&expected_value.to_string())
                } else {
                    c.has_metadata(key)
                }
            })
            .collect()
    }

    /// Get all unique metadata keys across all capabilities
    pub fn get_all_metadata_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        for capability in &self.capabilities {
            for key in capability.metadata.keys() {
                if !keys.contains(key) {
                    keys.push(key.clone());
                }
            }
        }
        keys.sort();
        keys
    }

    /// Get capabilities by version
    pub fn capabilities_by_version(&self, version: &str) -> Vec<&Capability> {
        self.capabilities
            .iter()
            .filter(|c| c.version == version)
            .collect()
    }
}

impl Default for PluginCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use capdef::{CapabilityKey, Capability};
    use std::collections::HashMap;

    #[test]
    fn test_plugin_capabilities() {
        let mut capabilities = PluginCapabilities::new();
        
        let id1 = CapabilityKey::from_string("action=transform;type=data;format=json").unwrap();
        let cap1 = Capability::new(id1, "1.0.0".to_string(), "transform-json".to_string());
        
        let id2 = CapabilityKey::from_string("action=validate;type=data;format=*").unwrap();
        let mut metadata = HashMap::new();
        metadata.insert("formats".to_string(), "json,xml,yaml".to_string());
        let cap2 = Capability::with_metadata(id2, "1.0.0".to_string(), "validate-data".to_string(), metadata);
        
        capabilities.add_capability(cap1);
        capabilities.add_capability(cap2);
        
        assert!(capabilities.can("action=transform;type=data;format=json"));
        assert!(capabilities.can("action=validate;type=data;format=xml"));
        assert!(!capabilities.can("action=compute;type=math"));
        
        let metadata_caps = capabilities.capabilities_with_metadata("formats", None);
        assert_eq!(metadata_caps.len(), 1);
        
        let version_caps = capabilities.capabilities_by_version("1.0.0");
        assert_eq!(version_caps.len(), 2);
    }
}
//! Unified plugin registry with capability-based access

use anyhow::{anyhow, Result};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use crate::ResponseWrapper;

/// Unified plugin registry that provides capability-based access to plugins
#[derive(Debug)]
pub struct PluginRegistry {
    plugins: HashMap<String, PluginEntry>,
    capability_index: HashMap<String, Vec<String>>, // capability -> plugin names
}

/// Individual plugin entry in the registry
#[derive(Debug, Clone)]
struct PluginEntry {
    binary_path: String,
    capabilities: Vec<String>,
    priority: u8, // 0=optional, 1=recommended, 2=critical
}

/// Capability caller that provides the unified interface
pub struct CapabilityCaller {
    plugin_name: String,
    capability: String,
    binary_path: String,
}

impl PluginRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            capability_index: HashMap::new(),
        }
    }
    
    /// Register a plugin with its capabilities
    pub fn register_plugin(
        &mut self,
        name: String,
        binary_path: String,
        capabilities: Vec<String>,
        priority: u8,
    ) {
        let entry = PluginEntry {
            binary_path,
            capabilities: capabilities.clone(),
            priority,
        };
        
        // Update capability index
        for capability in &capabilities {
            self.capability_index
                .entry(capability.clone())
                .or_insert_with(Vec::new)
                .push(name.clone());
        }
        
        self.plugins.insert(name, entry);
    }
    
    /// Check if a capability is available and return a caller
    pub fn can(&self, capability: &str) -> Result<CapabilityCaller> {
        // Find the best plugin for this capability
        let plugin_name = self.find_best_plugin_for_capability(capability)
            .ok_or_else(|| anyhow!("Capability '{}' is not available in any registered plugin", capability))?;
        
        let plugin = self.plugins.get(&plugin_name)
            .ok_or_else(|| anyhow!("Plugin '{}' not found in registry", plugin_name))?;
        
        Ok(CapabilityCaller {
            plugin_name: plugin_name.clone(),
            capability: capability.to_string(),
            binary_path: plugin.binary_path.clone(),
        })
    }
    
    /// Find the best plugin for a capability (most specific match with highest priority)
    fn find_best_plugin_for_capability(&self, capability: &str) -> Option<String> {
        // Get all plugins that support this capability
        let candidates = self.get_capability_candidates(capability)?;
        
        // Sort by specificity and priority
        let mut scored_candidates: Vec<(String, i32)> = candidates
            .into_iter()
            .filter_map(|plugin_name| {
                let plugin = self.plugins.get(&plugin_name)?;
                let score = self.calculate_capability_score(&plugin, capability);
                Some((plugin_name, score))
            })
            .collect();
        
        // Sort by score (descending) - higher score = better match
        scored_candidates.sort_by(|a, b| b.1.cmp(&a.1));
        
        scored_candidates.first().map(|(name, _)| name.clone())
    }
    
    /// Get candidate plugins for a capability (exact match and wildcard)
    fn get_capability_candidates(&self, capability: &str) -> Option<Vec<String>> {
        let mut candidates = Vec::new();
        
        // Check for exact match
        if let Some(exact_plugins) = self.capability_index.get(capability) {
            candidates.extend(exact_plugins.clone());
        }
        
        // Check for wildcard matches
        if let Some(colon_pos) = capability.rfind(':') {
            let operation = &capability[..colon_pos];
            let wildcard_capability = format!("{}:*", operation);
            
            if let Some(wildcard_plugins) = self.capability_index.get(&wildcard_capability) {
                candidates.extend(wildcard_plugins.clone());
            }
        }
        
        if candidates.is_empty() {
            None
        } else {
            candidates.sort();
            candidates.dedup();
            Some(candidates)
        }
    }
    
    /// Calculate capability match score (higher = better)
    fn calculate_capability_score(&self, plugin: &PluginEntry, capability: &str) -> i32 {
        let mut score = 0;
        
        // Priority bonus (critical=200, recommended=100, optional=0)
        score += match plugin.priority {
            2 => 200, // Critical
            1 => 100, // Recommended  
            _ => 0,   // Optional
        };
        
        // Specificity bonus
        if plugin.capabilities.contains(&capability.to_string()) {
            score += 50; // Exact match
        } else if let Some(colon_pos) = capability.rfind(':') {
            let operation = &capability[..colon_pos];
            let wildcard = format!("{}:*", operation);
            if plugin.capabilities.contains(&wildcard) {
                score += 25; // Wildcard match
            }
        }
        
        score
    }
    
    /// Get all available capabilities
    pub fn list_capabilities(&self) -> Vec<String> {
        let mut capabilities: Vec<String> = self.capability_index.keys().cloned().collect();
        capabilities.sort();
        capabilities
    }
    
    /// Get plugin statistics
    pub fn stats(&self) -> PluginRegistryStats {
        PluginRegistryStats {
            plugin_count: self.plugins.len(),
            capability_count: self.capability_index.len(),
            plugin_names: self.plugins.keys().cloned().collect(),
        }
    }
}

impl CapabilityCaller {
    /// Call the capability with JSON arguments
    pub async fn call(&self, args: Vec<JsonValue>) -> Result<ResponseWrapper> {
        // Convert capability to command line flag
        let flag = self.capability_to_flag(&self.capability);
        
        // Build command arguments
        let mut cmd_args = Vec::new();
        
        // Add the main flag
        cmd_args.push(flag);
        
        // Convert JSON args to command line arguments
        for arg in args {
            match arg {
                JsonValue::String(s) => cmd_args.push(s),
                JsonValue::Number(n) => cmd_args.push(n.to_string()),
                JsonValue::Bool(b) => cmd_args.push(b.to_string()),
                JsonValue::Array(_) | JsonValue::Object(_) => {
                    // For complex JSON, pass as JSON string
                    cmd_args.push(serde_json::to_string(&arg)?);
                }
                JsonValue::Null => {
                    // Skip null values
                    continue;
                }
            }
        }
        
        // Always request JSON output when available
        cmd_args.push("--json".to_string());
        
        // Execute the plugin command
        let output = self.execute_plugin_command(&cmd_args).await?;
        
        // Determine response type based on capability
        let response = if self.is_binary_capability() {
            ResponseWrapper::from_binary(output)
        } else if self.is_json_capability() {
            ResponseWrapper::from_json(output)
        } else {
            ResponseWrapper::from_text(output)
        };
        
        Ok(response)
    }
    
    /// Convert capability name to command line flag
    fn capability_to_flag(&self, capability: &str) -> String {
        // Extract operation part (everything before the last colon)
        let operation = if let Some(colon_pos) = capability.rfind(':') {
            &capability[..colon_pos]
        } else {
            capability
        };
        
        // Convert underscores to hyphens and add double dash prefix
        format!("--{}", operation.replace('_', "-"))
    }
    
    /// Check if this capability produces binary output
    fn is_binary_capability(&self) -> bool {
        self.capability.starts_with("generate-thumbnail")
    }
    
    /// Check if this capability should produce JSON output
    fn is_json_capability(&self) -> bool {
        matches!(
            self.capability.split(':').next().unwrap_or(""),
            "extract-metadata" | "extract-outline" | "extract-pages" | "list-models" | "get-model-status"
        )
    }
    
    /// Execute the plugin command with timeout and error handling
    async fn execute_plugin_command(&self, args: &[String]) -> Result<Vec<u8>> {
        use std::process::Stdio;
        use tokio::process::Command;
        use tokio::time::{timeout, Duration};
        
        let timeout_duration = Duration::from_secs(30);
        
        let output = timeout(
            timeout_duration,
            Command::new(&self.binary_path)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        ).await
        .map_err(|_| anyhow!("Plugin execution timed out after 30 seconds"))?
        .map_err(|e| anyhow!("Failed to execute plugin '{}': {}", self.plugin_name, e))?;
        
        if output.status.success() {
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let exit_code = output.status.code().unwrap_or(-1);
            Err(anyhow!(
                "Plugin '{}' failed with exit code {}: {}",
                self.plugin_name,
                exit_code,
                stderr
            ))
        }
    }
}

/// Plugin registry statistics
#[derive(Debug, Clone)]
pub struct PluginRegistryStats {
    pub plugin_count: usize,
    pub capability_count: usize,
    pub plugin_names: Vec<String>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_capability_to_flag() {
        let caller = CapabilityCaller {
            plugin_name: "test".to_string(),
            capability: "generate-thumbnail:pdf".to_string(),
            binary_path: "/test".to_string(),
        };
        
        assert_eq!(caller.capability_to_flag("generate-thumbnail:pdf"), "--generate-thumbnail");
        assert_eq!(caller.capability_to_flag("extract-metadata:epub"), "--extract-metadata");
        assert_eq!(caller.capability_to_flag("list-models"), "--list-models");
    }
    
    #[test]
    fn test_registry_capability_matching() {
        let mut registry = PluginRegistry::new();
        
        // Register plugins with different capabilities
        registry.register_plugin(
            "pdfczar".to_string(),
            "/plugins/pdfczar".to_string(),
            vec!["generate-thumbnail:pdf".to_string(), "extract-metadata:pdf".to_string()],
            2, // Critical
        );
        
        registry.register_plugin(
            "universal".to_string(),
            "/plugins/universal".to_string(),
            vec!["generate-thumbnail:*".to_string(), "extract-metadata:*".to_string()],
            1, // Recommended
        );
        
        // Test exact match preference
        let caller = registry.can("generate-thumbnail:pdf").unwrap();
        assert_eq!(caller.plugin_name, "pdfczar");
        
        // Test wildcard fallback
        let caller = registry.can("generate-thumbnail:epub").unwrap();
        assert_eq!(caller.plugin_name, "universal");
    }
}
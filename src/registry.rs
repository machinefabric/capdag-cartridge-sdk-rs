//! Pure capability-based plugin execution via XPC

use anyhow::Result;
use serde_json::Value as JsonValue;
use crate::ResponseWrapper;

/// Capability caller that executes via XPC service
pub struct CapabilityCaller {
    capability: String,
    xpc_client: Box<dyn XPCClient>,
}

/// Trait for XPC client communication
pub trait XPCClient: Send + Sync {
    fn execute_capability(
        &self,
        capability: &str,
        args: &[&str]
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send + '_>>;
}

impl CapabilityCaller {
    /// Create a new capability caller
    pub fn new(
        capability: String,
        xpc_client: Box<dyn XPCClient>,
    ) -> Self {
        Self {
            capability,
            xpc_client,
        }
    }
    
    /// Call the capability with JSON arguments via XPC service
    pub async fn call(&self, args: Vec<JsonValue>) -> Result<ResponseWrapper> {
        // Convert capability to command
        let command = self.capability_to_command(&self.capability);
        
        // Build command arguments
        let mut cmd_args = Vec::new();
        
        // Add the main command
        cmd_args.push(command);
        
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
        
        // Plugins output JSON by default
        
        // Convert to &str slice for XPC client
        let str_args: Vec<&str> = cmd_args.iter().map(|s| s.as_str()).collect();
        
        // Execute via XPC service
        let output = self.xpc_client.execute_capability(&self.capability, &str_args).await?;
        
        // Determine response type based on capability
        let response = if self.is_binary_capability() {
            ResponseWrapper::from_binary(output.as_bytes().to_vec())
        } else if self.is_json_capability() {
            ResponseWrapper::from_json(output.into_bytes())
        } else {
            ResponseWrapper::from_text(output.into_bytes())
        };
        
        Ok(response)
    }
    
    /// Convert capability name to command
    fn capability_to_command(&self, capability: &str) -> String {
        // Extract operation part (everything before the last colon)
        let operation = if let Some(colon_pos) = capability.rfind(':') {
            &capability[..colon_pos]
        } else {
            capability
        };
        
        // Convert underscores to hyphens for command name
        operation.replace('_', "-")
    }
    
    /// Check if this capability produces binary output
    fn is_binary_capability(&self) -> bool {
        self.capability.starts_with("generate-thumbnail")
    }
    
    /// Check if this capability should produce JSON output
    fn is_json_capability(&self) -> bool {
        // All structured data capabilities now return JSON
        !matches!(
            self.capability.split(':').next().unwrap_or(""),
            "generate-thumbnail"  // Only binary capabilities return non-JSON
        )
    }
}

/// Plugin registry statistics
#[derive(Debug, Clone)]
pub struct PluginRegistryStats {
    pub plugin_count: usize,
    pub capability_count: usize,
    pub plugin_names: Vec<String>,
}
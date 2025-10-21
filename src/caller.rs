//! Pure capability-based plugin execution via XPC

use anyhow::Result;
use serde_json::Value as JsonValue;
use crate::ResponseWrapper;

/// Capability caller that executes via XPC service
pub struct CapabilityCaller {
    capability: String,
    plugin_host: Box<dyn PluginHost>,
}

/// Trait for Plugin Host communication
pub trait PluginHost: Send + Sync {
    fn execute_capability(
        &self,
        capability: &str,
        args: &[&str]
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send + '_>>;
    
    fn execute_capability_structured(
        &self,
        capability: &str,
        positional_args: &[String],
        named_args: &[(String, String)]
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send + '_>>;
}

impl CapabilityCaller {
    /// Create a new capability caller
    pub fn new(
        capability: String,
        plugin_host: Box<dyn PluginHost>,
    ) -> Self {
        Self {
            capability,
            plugin_host,
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
        
        // Convert to &str slice for Plugin Host
        let str_args: Vec<&str> = cmd_args.iter().map(|s| s.as_str()).collect();
        
        // Execute via XPC service
        let output = self.plugin_host.execute_capability(&self.capability, &str_args).await?;
        
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
    
    /// Call the capability with structured arguments (positional and named)
    pub async fn call_structured(
        &self,
        positional_args: Vec<JsonValue>,
        named_args: Vec<JsonValue>
    ) -> Result<ResponseWrapper> {
        // Convert JsonValue positional args to strings
        let string_positional_args: Vec<String> = positional_args
            .into_iter()
            .map(|arg| match arg {
                JsonValue::String(s) => s,
                JsonValue::Number(n) => n.to_string(),
                JsonValue::Bool(b) => b.to_string(),
                JsonValue::Array(_) | JsonValue::Object(_) => {
                    serde_json::to_string(&arg).unwrap_or_default()
                }
                JsonValue::Null => String::new(),
            })
            .collect();

        // Convert JsonValue named args to (String, String) tuples
        let string_named_args: Vec<(String, String)> = named_args
            .into_iter()
            .filter_map(|arg| {
                if let JsonValue::Object(map) = arg {
                    if let (Some(JsonValue::String(name)), Some(value)) = 
                        (map.get("name"), map.get("value")) {
                        let value_str = match value {
                            JsonValue::String(s) => s.clone(),
                            JsonValue::Number(n) => n.to_string(),
                            JsonValue::Bool(b) => b.to_string(),
                            _ => serde_json::to_string(value).unwrap_or_default(),
                        };
                        return Some((name.clone(), value_str));
                    }
                }
                None
            })
            .collect();

        // Execute via structured plugin host method
        let output = self.plugin_host.execute_capability_structured(
            &self.capability, 
            &string_positional_args,
            &string_named_args
        ).await?;
        
        // Determine response type based on capability (same logic as call method)
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
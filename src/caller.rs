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
        positional_args: &[String],
        named_args: &[(String, String)]
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(Option<Vec<u8>>, Option<String>)>> + Send + '_>>;
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
    
    /// Call the capability with structured arguments (positional and named)
    pub async fn call(
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

        // Execute via plugin host method
        let (binary_output, text_output) = self.plugin_host.execute_capability(
            &self.capability, 
            &string_positional_args,
            &string_named_args
        ).await?;
        
        // Determine response type based on what was returned
        let response = if let Some(binary_data) = binary_output {
            ResponseWrapper::from_binary(binary_data)
        } else if let Some(text_data) = text_output {
            if self.is_json_capability() {
                ResponseWrapper::from_json(text_data.into_bytes())
            } else {
                ResponseWrapper::from_text(text_data.into_bytes())
            }
        } else {
            return Err(anyhow::anyhow!("Plugin returned no output"));
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
        // Use the formal capability identifier system to detect binary capabilities
        let capability_key = capdef::CapabilityKey::from_string(&self.capability)
            .expect("Invalid capability identifier");
        capability_key.is_binary()
    }
    
    /// Check if this capability should produce JSON output
    fn is_json_capability(&self) -> bool {
        let capability_key = capdef::CapabilityKey::from_string(&self.capability)
            .expect("Invalid capability identifier");
        !capability_key.is_binary()
    }
}
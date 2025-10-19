//! Plugin interface schema validation framework
//! 
//! This module provides type-safe validation of plugin implementations against formal schemas.
//! It validates capability definitions, plugin interfaces, and runtime plugin behavior.

use crate::PluginResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, bail};

/// Represents a formal capability definition with typed arguments and validation rules
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapabilitySchema {
    pub schema_version: String,
    pub capability: CapabilityInfo,
    pub command_interface: CommandInterface,
    pub arguments: ArgumentsSpec,
    pub response: ResponseSpec,
    #[serde(default)]
    pub validation: ValidationRules,
    #[serde(default)]
    pub error_handling: ErrorHandling,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapabilityInfo {
    pub name: String,
    pub description: String,
    pub file_types: Vec<String>,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandInterface {
    pub cli_flag: String,
    pub usage_pattern: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArgumentsSpec {
    pub required: Vec<ArgumentDef>,
    pub optional: Vec<ArgumentDef>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArgumentDef {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: ArgumentType,
    pub description: String,
    pub cli_flag: Option<String>,
    pub position: Option<usize>,
    #[serde(default)]
    pub validation: ArgumentValidation,
    pub default: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgumentType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgumentValidation {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub allowed_values: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseSpec {
    #[serde(rename = "type")]
    pub response_type: ResponseType,
    pub schema_ref: Option<String>,
    pub content_type: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Json,
    Binary,
    Text,
    Boolean,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationRules {
    #[serde(default = "default_true")]
    pub file_existence: bool,
    #[serde(default = "default_true")]
    pub file_type_check: bool,
    #[serde(default)]
    pub custom_validators: Vec<String>,
}

fn default_true() -> bool { true }

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorHandling {
    #[serde(default)]
    pub error_codes: HashMap<String, ErrorCode>,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: f64,
}

fn default_timeout() -> f64 { 30.0 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorCode {
    pub code: i32,
    pub message: String,
    pub description: String,
}

/// Represents a complete plugin interface with multiple capabilities
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginInterfaceSchema {
    pub schema_version: String,
    pub interface: InterfaceInfo,
    pub capabilities: Vec<CapabilityReference>,
    #[serde(default)]
    pub global_requirements: GlobalRequirements,
    #[serde(default)]
    pub validation_rules: InterfaceValidationRules,
    #[serde(default)]
    pub testing: TestingSpec,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InterfaceInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub compatibility: CompatibilitySpec,
    #[serde(default)]
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompatibilitySpec {
    pub min_sdk_version: String,
    pub max_sdk_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CapabilityReference {
    Inline(CapabilitySchema),
    Reference { capability_ref: String },
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalRequirements {
    #[serde(default)]
    pub plugin_info_command: PluginInfoRequirement,
    #[serde(default)]
    pub json_output_support: JsonOutputSupport,
    #[serde(default)]
    pub error_handling: GlobalErrorHandling,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginInfoRequirement {
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default = "default_json_format")]
    pub format: String,
    #[serde(default = "default_plugin_info_schema")]
    pub schema_ref: String,
}

fn default_json_format() -> String { "json".to_string() }
fn default_plugin_info_schema() -> String { "plugin-info.json".to_string() }

impl Default for PluginInfoRequirement {
    fn default() -> Self {
        Self {
            required: true,
            format: default_json_format(),
            schema_ref: default_plugin_info_schema(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonOutputSupport {
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default = "default_json_flag")]
    pub flag: String,
}

fn default_json_flag() -> String { "--json".to_string() }

impl Default for JsonOutputSupport {
    fn default() -> Self {
        Self {
            required: true,
            flag: default_json_flag(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalErrorHandling {
    #[serde(default)]
    pub standard_exit_codes: HashMap<String, i32>,
    #[serde(default = "default_stderr")]
    pub error_output_format: String,
}

fn default_stderr() -> String { "stderr".to_string() }

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InterfaceValidationRules {
    #[serde(default)]
    pub capability_uniqueness: UniquenessRule,
    #[serde(default)]
    pub file_type_consistency: FileTypeConsistencyRule,
    #[serde(default)]
    pub version_compatibility: VersionCompatibilityRule,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UniquenessRule {
    #[serde(default = "default_true")]
    pub enforce: bool,
    #[serde(default = "default_interface_scope")]
    pub scope: String,
}

fn default_interface_scope() -> String { "interface".to_string() }

impl Default for UniquenessRule {
    fn default() -> Self {
        Self {
            enforce: true,
            scope: default_interface_scope(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileTypeConsistencyRule {
    #[serde(default = "default_true")]
    pub enforce: bool,
    #[serde(default)]
    pub related_capabilities: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionCompatibilityRule {
    #[serde(default = "default_true")]
    pub enforce: bool,
    #[serde(default = "default_true")]
    pub breaking_change_detection: bool,
}

impl Default for VersionCompatibilityRule {
    fn default() -> Self {
        Self {
            enforce: true,
            breaking_change_detection: true,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestingSpec {
    #[serde(default)]
    pub test_files: HashMap<String, Vec<TestFile>>,
    #[serde(default)]
    pub test_scenarios: Vec<TestScenario>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestFile {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub expected_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestScenario {
    pub name: String,
    pub description: Option<String>,
    pub capability: String,
    pub arguments: HashMap<String, Value>,
    pub expected_result: ExpectedResult,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExpectedResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub output_validation: Option<HashMap<String, Value>>,
}

/// Plugin schema validator for type-safe validation of plugin implementations
pub struct PluginValidator {
    schema_dir: PathBuf,
    capability_schemas: HashMap<String, CapabilitySchema>,
    interface_schemas: HashMap<String, PluginInterfaceSchema>,
}

impl PluginValidator {
    /// Create a new plugin validator with the given schema directory
    pub fn new<P: AsRef<Path>>(schema_dir: P) -> PluginResult<Self> {
        let schema_dir = schema_dir.as_ref().to_path_buf();
        
        if !schema_dir.exists() {
            bail!("Schema directory not found: {}", schema_dir.display());
        }

        Ok(Self {
            schema_dir,
            capability_schemas: HashMap::new(),
            interface_schemas: HashMap::new(),
        })
    }

    /// Load and validate a capability schema from file
    pub fn load_capability_schema<P: AsRef<Path>>(&mut self, path: P) -> PluginResult<&CapabilitySchema> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read capability schema: {}", path.display()))?;
        
        let schema: CapabilitySchema = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse capability schema: {}", path.display()))?;

        self.validate_capability_schema(&schema)?;
        
        let name = schema.capability.name.clone();
        self.capability_schemas.insert(name.clone(), schema);
        
        Ok(&self.capability_schemas[&name])
    }

    /// Load and validate a plugin interface schema from file
    pub fn load_interface_schema<P: AsRef<Path>>(&mut self, path: P) -> PluginResult<&PluginInterfaceSchema> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read interface schema: {}", path.display()))?;
        
        let schema: PluginInterfaceSchema = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse interface schema: {}", path.display()))?;

        self.validate_interface_schema(&schema)?;
        
        let name = schema.interface.name.clone();
        self.interface_schemas.insert(name.clone(), schema);
        
        Ok(&self.interface_schemas[&name])
    }

    /// Validate a plugin binary against an interface schema
    pub fn validate_plugin_implementation<P: AsRef<Path>>(
        &self, 
        plugin_binary: P,
        interface_name: &str
    ) -> PluginResult<ValidationReport> {
        let plugin_binary = plugin_binary.as_ref();
        
        if !plugin_binary.exists() {
            bail!("Plugin binary not found: {}", plugin_binary.display());
        }

        let interface = self.interface_schemas.get(interface_name)
            .ok_or_else(|| anyhow::anyhow!("Interface schema not loaded: {}", interface_name))?;

        let mut report = ValidationReport::new(plugin_binary, interface_name);

        // Test plugin-info command
        self.validate_plugin_info_command(plugin_binary, interface, &mut report)?;

        // Test each capability
        for cap_ref in &interface.capabilities {
            match cap_ref {
                CapabilityReference::Inline(cap_schema) => {
                    self.validate_capability_implementation(plugin_binary, cap_schema, &mut report)?;
                }
                CapabilityReference::Reference { capability_ref } => {
                    // Load referenced capability and validate
                    let cap_path = self.schema_dir.join(capability_ref);
                    if cap_path.exists() {
                        let content = std::fs::read_to_string(&cap_path)?;
                        let cap_schema: CapabilitySchema = serde_json::from_str(&content)?;
                        self.validate_capability_implementation(plugin_binary, &cap_schema, &mut report)?;
                    } else {
                        report.add_error(format!("Referenced capability schema not found: {}", capability_ref));
                    }
                }
            }
        }

        Ok(report)
    }

    /// Validate that a capability schema is well-formed
    fn validate_capability_schema(&self, schema: &CapabilitySchema) -> PluginResult<()> {
        // Validate capability name format
        if !schema.capability.name.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
            bail!("Invalid capability name format: {}", schema.capability.name);
        }

        // Validate CLI flag format
        if !schema.command_interface.cli_flag.starts_with("--") {
            bail!("CLI flag must start with '--': {}", schema.command_interface.cli_flag);
        }

        // Validate argument definitions
        for arg in &schema.arguments.required {
            self.validate_argument_def(arg)?;
        }
        for arg in &schema.arguments.optional {
            self.validate_argument_def(arg)?;
        }

        Ok(())
    }

    /// Validate an argument definition
    fn validate_argument_def(&self, arg: &ArgumentDef) -> PluginResult<()> {
        // Check that either position or cli_flag is specified, but not both
        match (arg.position, &arg.cli_flag) {
            (Some(_), Some(_)) => bail!("Argument cannot have both position and cli_flag: {}", arg.name),
            (None, None) => bail!("Argument must have either position or cli_flag: {}", arg.name),
            _ => {}
        }

        // Validate CLI flag format if present
        if let Some(flag) = &arg.cli_flag {
            if !flag.starts_with("--") {
                bail!("CLI flag must start with '--': {}", flag);
            }
        }

        Ok(())
    }

    /// Validate that an interface schema is well-formed
    fn validate_interface_schema(&self, schema: &PluginInterfaceSchema) -> PluginResult<()> {
        // Validate interface name format
        if !schema.interface.name.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
            bail!("Invalid interface name format: {}", schema.interface.name);
        }

        // Check for capability uniqueness if enforced
        if schema.validation_rules.capability_uniqueness.enforce {
            let mut seen_capabilities = std::collections::HashSet::new();
            for cap_ref in &schema.capabilities {
                let cap_name = match cap_ref {
                    CapabilityReference::Inline(cap) => &cap.capability.name,
                    CapabilityReference::Reference { capability_ref } => {
                        // Extract capability name from reference (basic heuristic)
                        capability_ref.split('/').last().unwrap_or(capability_ref).trim_end_matches(".json")
                    }
                };
                
                if !seen_capabilities.insert(cap_name) {
                    bail!("Duplicate capability in interface: {}", cap_name);
                }
            }
        }

        Ok(())
    }

    /// Validate plugin-info command implementation
    fn validate_plugin_info_command(
        &self,
        plugin_binary: &Path,
        interface: &PluginInterfaceSchema,
        report: &mut ValidationReport,
    ) -> PluginResult<()> {
        if !interface.global_requirements.plugin_info_command.required {
            return Ok(());
        }

        let output = Command::new(plugin_binary)
            .args(&["plugin-info", "--json"])
            .output()
            .context("Failed to execute plugin-info command")?;

        if !output.status.success() {
            report.add_error("plugin-info command failed".to_string());
            return Ok(());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let plugin_info: Value = serde_json::from_str(&stdout)
            .map_err(|e| {
                report.add_error(format!("plugin-info output is not valid JSON: {}", e));
                e
            })?;

        // Validate required fields
        let required_fields = ["name", "version", "capabilities"];
        for field in &required_fields {
            if !plugin_info.get(field).is_some() {
                report.add_error(format!("plugin-info missing required field: {}", field));
            }
        }

        report.add_success("plugin-info command validation passed".to_string());
        Ok(())
    }

    /// Validate a specific capability implementation
    fn validate_capability_implementation(
        &self,
        plugin_binary: &Path,
        capability: &CapabilitySchema,
        report: &mut ValidationReport,
    ) -> PluginResult<()> {
        let cap_name = &capability.capability.name;
        
        // Test that the capability flag is recognized
        let output = Command::new(plugin_binary)
            .args(&[&capability.command_interface.cli_flag, "--help"])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() || result.status.code() == Some(2) {
                    // Exit code 2 is common for --help in many CLI tools
                    report.add_success(format!("Capability {} CLI flag recognized", cap_name));
                } else {
                    report.add_error(format!("Capability {} CLI flag not recognized", cap_name));
                }
            }
            Err(e) => {
                report.add_error(format!("Failed to test capability {}: {}", cap_name, e));
            }
        }

        Ok(())
    }
}

/// Report of plugin validation results
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub plugin_path: PathBuf,
    pub interface_name: String,
    pub successes: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    fn new<P: AsRef<Path>>(plugin_path: P, interface_name: &str) -> Self {
        Self {
            plugin_path: plugin_path.as_ref().to_path_buf(),
            interface_name: interface_name.to_string(),
            successes: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn add_success(&mut self, message: String) {
        self.successes.push(message);
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }


    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get a summary of validation results
    pub fn summary(&self) -> String {
        format!(
            "Plugin: {}\nInterface: {}\nSuccesses: {}\nErrors: {}\nWarnings: {}",
            self.plugin_path.display(),
            self.interface_name,
            self.successes.len(),
            self.errors.len(),
            self.warnings.len()
        )
    }
}
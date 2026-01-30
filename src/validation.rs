//! Plugin interface schema validation framework
//! 
//! This module provides type-safe validation of plugin implementations against formal schemas.
//! It validates cap definitions, plugin interfaces, and runtime plugin behavior.

use crate::PluginResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, bail};

/// Represents a formal cap definition with typed arguments and validation rules
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapSchema {
    pub schema_version: String,
    pub cap: CapInfo,
    pub command: String,
    /// New unified args array (no required/optional split)
    pub args: Vec<CapArg>,
    pub response: ResponseSpec,
    #[serde(default)]
    pub validation: ValidationRules,
    #[serde(default)]
    pub error_handling: ErrorHandling,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapInfo {
    pub name: String,
    pub description: String,
    pub file_types: Vec<String>,
    pub version: String,
}


/// New argument source variants
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ArgSource {
    Stdin { stdin: String },
    Position { position: usize },
    CliFlag { cli_flag: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapArg {
    /// Unique semantic media URN (e.g., media:string, media:object)
    pub media_urn: String,
    /// Whether this arg is required
    pub required: bool,
    /// How this arg can be provided (non-empty)
    pub sources: Vec<ArgSource>,
    /// Human description
    #[serde(default)]
    pub arg_description: String,
    /// Validation rules
    #[serde(default)]
    pub validation: MediaValidation,
    /// Default value if optional
    pub default_value: Option<Value>,
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
pub struct MediaValidation {
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

/// Represents a complete plugin interface with multiple caps
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginInterfaceSchema {
    pub schema_version: String,
    pub interface: InterfaceInfo,
    pub caps: Vec<CapReference>,
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
pub enum CapReference {
    Inline(CapSchema),
    Reference { cap_ref: String },
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalRequirements {
    #[serde(default)]
    pub plugin_manifest_command: PluginManifestRequirement,
    #[serde(default)]
    pub error_handling: GlobalErrorHandling,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginManifestRequirement {
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default = "default_plugin_manifest_schema")]
    pub schema_ref: String,
}

fn default_plugin_manifest_schema() -> String { "manifest.json".to_string() }

impl Default for PluginManifestRequirement {
    fn default() -> Self {
        Self {
            required: true,
            schema_ref: default_plugin_manifest_schema(),
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
    pub cap_uniqueness: UniquenessRule,
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
    pub related_caps: Vec<Vec<String>>,
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
    pub expected_caps: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestScenario {
    pub name: String,
    pub description: Option<String>,
    pub cap: String,
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
    cap_schemas: HashMap<String, CapSchema>,
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
            cap_schemas: HashMap::new(),
            interface_schemas: HashMap::new(),
        })
    }

    /// Load and validate a cap schema from file
    pub fn load_cap_schema<P: AsRef<Path>>(&mut self, path: P) -> PluginResult<&CapSchema> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read cap schema: {}", path.display()))?;
        
        let schema: CapSchema = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse cap schema: {}", path.display()))?;

        self.validate_cap_schema(&schema)?;
        
        let name = schema.cap.name.clone();
        self.cap_schemas.insert(name.clone(), schema);
        
        Ok(&self.cap_schemas[&name])
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

        // Test manifest command
        self.validate_plugin_manifest_command(plugin_binary, interface, &mut report)?;

        // Test each cap with comprehensive validation
        for cap_ref in &interface.caps {
            match cap_ref {
                CapReference::Inline(cap_schema) => {
                    self.validate_cap_implementation(plugin_binary, cap_schema, &mut report)?;
                    self.validate_cap_output_format(plugin_binary, cap_schema, &mut report)?;
                    self.validate_cap_error_handling(plugin_binary, cap_schema, &mut report)?;
                }
                CapReference::Reference { cap_ref } => {
                    // Load referenced cap and validate
                    let cap_path = self.schema_dir.join(cap_ref);
                    if cap_path.exists() {
                        let content = std::fs::read_to_string(&cap_path)?;
                        let cap_schema: CapSchema = serde_json::from_str(&content)?;
                        self.validate_cap_implementation(plugin_binary, &cap_schema, &mut report)?;
                        self.validate_cap_output_format(plugin_binary, &cap_schema, &mut report)?;
                        self.validate_cap_error_handling(plugin_binary, &cap_schema, &mut report)?;
                    } else {
                        report.add_error(format!("Referenced cap schema not found: {}", cap_ref));
                    }
                }
            }
        }

        // Test with real files if available
        self.validate_with_test_files(plugin_binary, interface, &mut report)?;

        Ok(report)
    }

    /// Validate that a cap schema is well-formed
    fn validate_cap_schema(&self, schema: &CapSchema) -> PluginResult<()> {
        // Validate cap name format
        if !schema.cap.name.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
            bail!("Invalid cap name format: {}", schema.cap.name);
        }

        // Validate CLI flag format
        if !schema.command.starts_with("--") {
            bail!("CLI flag must start with '--': {}", schema.command);
        }

        // Validate argument definitions (new args model)
        if schema.args.is_empty() {
            bail!("cap.args must not be empty");
        }
        self.validate_args(&schema.args)?;

        Ok(())
    }

    /// Validate new args+sources rules
    fn validate_args(&self, args: &Vec<CapArg>) -> PluginResult<()> {
        use std::collections::{HashSet, HashMap};
        // RULE1: No duplicate media_urns
        let mut seen_urns: HashSet<&str> = HashSet::new();
        for a in args { if !seen_urns.insert(a.media_urn.as_str()) { bail!("Duplicate media_urn: {}", a.media_urn); } }
        // RULE2: sources non-empty, RULE4: no duplicate source types per arg
        for a in args {
            if a.sources.is_empty() { bail!("Arg {} must have at least one source", a.media_urn); }
            let mut types: HashSet<&'static str> = HashSet::new();
            for s in &a.sources {
                let t = match s { ArgSource::Stdin{..}=>"stdin", ArgSource::Position{..}=>"position", ArgSource::CliFlag{..}=>"cli_flag" };
                if !types.insert(t) { bail!("Arg {} has duplicate source type: {}", a.media_urn, t); }
            }
        }
        // RULE5/6: unique, sequential positions
        let mut positions: Vec<usize> = args.iter().filter_map(|a| a.sources.iter().find_map(|s| if let ArgSource::Position{position} = s { Some(*position)} else {None})).collect();
        positions.sort_unstable();
        for (i,pos) in positions.iter().enumerate() { if *pos != i { bail!("Positions must be 0-based sequential with no gaps; got {} at index {}", pos, i); } }
        // RULE7: no arg may have both position and cli_flag (per-arg types set already enforces this implicitly if both present)
        for a in args {
            let has_pos = a.sources.iter().any(|s| matches!(s, ArgSource::Position{..}));
            let has_flag = a.sources.iter().any(|s| matches!(s, ArgSource::CliFlag{..}));
            if has_pos && has_flag { bail!("Arg {} cannot have both position and cli_flag", a.media_urn); }
        }
        // RULE9/10: cli_flag uniqueness and reserved flags
        let mut flags: HashSet<&str> = HashSet::new();
        let reserved = ["manifest","--help","--version","-v","-h"];
        for a in args {
            if let Some(flag) = a.sources.iter().find_map(|s| if let ArgSource::CliFlag{cli_flag} = s { Some(cli_flag.as_str()) } else { None }) {
                if reserved.contains(&flag) { bail!("cli_flag {} is reserved", flag); }
                if !flag.starts_with("-") { bail!("cli_flag must start with '-' or '--': {}", flag); }
                if !flags.insert(flag) { bail!("Duplicate cli_flag: {}", flag); }
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

        // Check for cap uniqueness if enforced
        if schema.validation_rules.cap_uniqueness.enforce {
            let mut seen_caps = std::collections::HashSet::new();
            for cap_ref in &schema.caps {
                let cap_name = match cap_ref {
                    CapReference::Inline(cap) => &cap.cap.name,
                    CapReference::Reference { cap_ref } => {
                        // Extract cap name from reference (basic heuristic)
                        cap_ref.split('/').last().unwrap_or(cap_ref).trim_end_matches(".json")
                    }
                };
                
                if !seen_caps.insert(cap_name) {
                    bail!("Duplicate cap in interface: {}", cap_name);
                }
            }
        }

        Ok(())
    }

    /// Validate manifest command implementation
    fn validate_plugin_manifest_command(
        &self,
        plugin_binary: &Path,
        interface: &PluginInterfaceSchema,
        report: &mut ValidationReport,
    ) -> PluginResult<()> {
        if !interface.global_requirements.plugin_manifest_command.required {
            return Ok(());
        }

        let output = Command::new(plugin_binary)
            .args(&["manifest"])
            .output()
            .context("Failed to execute manifest command")?;

        if !output.status.success() {
            report.add_error("manifest command failed".to_string());
            return Ok(());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let plugin_manifest: Value = serde_json::from_str(&stdout)
            .map_err(|e| {
                report.add_error(format!("manifest output is not valid JSON: {}", e));
                e
            })?;

        // Validate required fields
        let required_fields = ["name", "version", "caps"];
        for field in &required_fields {
            if !plugin_manifest.get(field).is_some() {
                report.add_error(format!("manifest missing required field: {}", field));
            }
        }

        report.add_success("manifest command validation passed".to_string());
        Ok(())
    }

    /// Validate a specific cap implementation
    fn validate_cap_implementation(
        &self,
        plugin_binary: &Path,
        cap: &CapSchema,
        report: &mut ValidationReport,
    ) -> PluginResult<()> {
        let cap_name = &cap.cap.name;
        
        // Test that the cap command is recognized
        let command_name = cap.command.trim_start_matches("--");
        let output = Command::new(plugin_binary)
            .args(&[command_name, "--help"])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() || result.status.code() == Some(2) {
                    // Exit code 2 is common for --help in many CLI tools
                    report.add_success(format!("Cap {} CLI flag recognized", cap_name));
                } else {
                    report.add_error(format!("Cap {} CLI flag not recognized", cap_name));
                }
            }
            Err(e) => {
                report.add_error(format!("Failed to test cap {}: {}", cap_name, e));
            }
        }

        Ok(())
    }

    /// Validate that cap outputs correct format (JSON for structured data)
    fn validate_cap_output_format(
        &self,
        plugin_binary: &Path,
        cap: &CapSchema,
        report: &mut ValidationReport,
    ) -> PluginResult<()> {
        let cap_name = &cap.cap.name;
        
        // Only test structured data caps that we can validate output format
        if !matches!(cap.response.response_type, ResponseType::Json) {
            return Ok(());
        }

        // Create a temporary test file if needed
        let test_file = if cap.args.iter().any(|arg| arg.media_urn == "media:string") {
            let temp_file = std::env::temp_dir().join(format!("plugin_test_{}.txt", uuid::Uuid::new_v4()));
            std::fs::write(&temp_file, "Test content for plugin validation\n\nSecond paragraph.").ok();
            Some(temp_file)
        } else {
            None
        };

        if let Some(ref test_file_path) = test_file {
            // Test the cap with the test file
            let test_file_str = test_file_path.to_string_lossy().to_string();
            let command_name = cap.command.trim_start_matches("--");
            let args = vec![command_name, &test_file_str];
            
            let output = Command::new(plugin_binary)
                .args(&args)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        let stdout = String::from_utf8_lossy(&result.stdout);
                        
                        // Verify it's valid JSON
                        match serde_json::from_str::<serde_json::Value>(&stdout) {
                            Ok(_) => {
                                // Check that it doesn't start with Debug format
                                if stdout.trim().starts_with(&format!("{}{{", cap.response.schema_ref.as_ref().unwrap_or(&"Data".to_string()).replace(".json", ""))) {
                                    report.add_error(format!("Cap {} outputs Debug format instead of JSON: starts with '{}{{", cap_name, cap.response.schema_ref.as_ref().unwrap_or(&"Data".to_string()).replace(".json", "")));
                                } else {
                                    report.add_success(format!("Cap {} outputs valid JSON", cap_name));
                                }
                            }
                            Err(e) => {
                                // Check if it's Debug format
                                if stdout.trim().starts_with("FileMetadata {") ||
                                   stdout.trim().starts_with("DisboundPage {") ||
                                   stdout.trim().starts_with("[DisboundPage {") ||
                                   stdout.trim().starts_with("DocumentOutline {") {
                                    report.add_error(format!("Cap {} outputs Debug format instead of JSON", cap_name));
                                } else {
                                    report.add_error(format!("Cap {} outputs invalid JSON: {}", cap_name, e));
                                }
                            }
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&result.stderr);
                        report.add_error(format!("Cap {} failed on test file: {}", cap_name, stderr));
                    }
                }
                Err(e) => {
                    report.add_error(format!("Failed to test cap {} output format: {}", cap_name, e));
                }
            }

            // Clean up test file
            std::fs::remove_file(test_file_path).ok();
        }

        Ok(())
    }

    /// Validate cap error handling
    fn validate_cap_error_handling(
        &self,
        plugin_binary: &Path,
        cap: &CapSchema,
        report: &mut ValidationReport,
    ) -> PluginResult<()> {
        let cap_name = &cap.cap.name;
        
        // Test with non-existent file if cap takes file input
        if cap.args.iter().any(|arg| arg.sources.iter().any(|s| matches!(s, ArgSource::Position{position:0}))) {
            let non_existent_file = "/tmp/non_existent_file_for_plugin_test.xyz";
            let non_existent_str = non_existent_file.to_string();
            let command_name = cap.command.trim_start_matches("--");
            
            let args = vec![command_name, &non_existent_str];
            
            let output = Command::new(plugin_binary)
                .args(&args)
                .output();

            match output {
                Ok(result) => {
                    if !result.status.success() {
                        let exit_code = result.status.code().unwrap_or(-1);
                        
                        // Check if it uses standard exit codes
                        if let Some(expected_code) = cap.error_handling.error_codes.get("FILE_NOT_FOUND") {
                            if exit_code == expected_code.code {
                                report.add_success(format!("Cap {} uses correct exit code for file not found", cap_name));
                            } else {
                                report.add_error(format!("Cap {} uses exit code {} instead of expected {} for file not found", cap_name, exit_code, expected_code.code));
                            }
                        } else {
                            if exit_code > 0 {
                                report.add_success(format!("Cap {} properly fails with non-zero exit code for invalid input", cap_name));
                            } else {
                                report.add_error(format!("Cap {} should fail with non-zero exit code for non-existent file", cap_name));
                            }
                        }
                    } else {
                        report.add_error(format!("Cap {} should fail when given non-existent file", cap_name));
                    }
                }
                Err(e) => {
                    report.add_error(format!("Failed to test cap {} error handling: {}", cap_name, e));
                }
            }
        }

        Ok(())
    }

    /// Validate plugin with test files from interface definition
    fn validate_with_test_files(
        &self,
        plugin_binary: &Path,
        interface: &PluginInterfaceSchema,
        report: &mut ValidationReport,
    ) -> PluginResult<()> {
        // Run test scenarios if defined
        for scenario in &interface.testing.test_scenarios {
            let cap_name = &scenario.cap;
            
            // Find the cap schema
            let cap_schema = self.find_cap_in_interface(interface, cap_name);
            if cap_schema.is_none() {
                report.add_error(format!("Test scenario references unknown cap: {}", cap_name));
                continue;
            }
            let cap_schema = cap_schema.unwrap();
            
            // Build command arguments from scenario
            let mut args_strings = Vec::new();
            let command_name = cap_schema.command.trim_start_matches("--");
            args_strings.push(command_name.to_string());
            
            // Add arguments from scenario
            if let Some(file_path) = scenario.arguments.get("file_path").and_then(|v| v.as_str()) {
                args_strings.push(file_path.to_string());
            }
            if let Some(width) = scenario.arguments.get("width").and_then(|v| v.as_u64()) {
                args_strings.push("--width".to_string());
                args_strings.push(width.to_string());
            }
            if let Some(height) = scenario.arguments.get("height").and_then(|v| v.as_u64()) {
                args_strings.push("--height".to_string());
                args_strings.push(height.to_string());
            }
            if let Some(output) = scenario.arguments.get("output").and_then(|v| v.as_str()) {
                args_strings.push("--output".to_string());
                args_strings.push(output.to_string());
            }
            
            let args: Vec<&str> = args_strings.iter().map(|s| s.as_str()).collect();
            
            // Execute test scenario
            let output = Command::new(plugin_binary)
                .args(&args)
                .output();

            match output {
                Ok(result) => {
                    let success = result.status.success();
                    let exit_code = result.status.code().unwrap_or(-1);
                    
                    if success == scenario.expected_result.success {
                        if let Some(expected_exit_code) = scenario.expected_result.exit_code {
                            if exit_code == expected_exit_code {
                                report.add_success(format!("Test scenario '{}' passed", scenario.name));
                            } else {
                                report.add_error(format!("Test scenario '{}' exit code mismatch: got {}, expected {}", scenario.name, exit_code, expected_exit_code));
                            }
                        } else {
                            report.add_success(format!("Test scenario '{}' passed", scenario.name));
                        }
                    } else {
                        report.add_error(format!("Test scenario '{}' success mismatch: got {}, expected {}", scenario.name, success, scenario.expected_result.success));
                    }
                }
                Err(e) => {
                    report.add_error(format!("Failed to run test scenario '{}': {}", scenario.name, e));
                }
            }
        }

        Ok(())
    }

    /// Find a cap schema in an interface
    fn find_cap_in_interface<'a>(&self, interface: &'a PluginInterfaceSchema, cap_name: &str) -> Option<&'a CapSchema> {
        for cap_ref in &interface.caps {
            match cap_ref {
                CapReference::Inline(cap_schema) => {
                    if cap_schema.cap.name == cap_name {
                        return Some(cap_schema);
                    }
                }
                CapReference::Reference { .. } => {
                    // For simplicity, skip referenced caps in this implementation
                    // In a full implementation, you'd load and check the referenced schema
                }
            }
        }
        None
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

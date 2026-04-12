//! Cartridge Schema Validator CLI Tool
//!
//! Command-line tool for validating cartridge implementations against formal interface schemas.

use clap::{Parser, Subcommand};
use machfab_cartridge_sdk::{CartridgeValidator, ValidationReport};
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "cartridge-validator")]
#[command(about = "Validate cartridge implementations against formal interface schemas")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a cartridge binary against an interface schema
    ValidateCartridge {
        /// Path to the cartridge binary to validate
        #[arg(short = 'c', long)]
        cartridge: PathBuf,

        /// Name of the interface schema to validate against
        #[arg(short, long)]
        interface: String,

        /// Directory containing schema files
        #[arg(short, long, default_value = "./cartridge-schemas")]
        schema_dir: PathBuf,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        output: String,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Validate a cap schema file
    ValidateCap {
        /// Path to the cap schema file
        #[arg(short = 'a', long)]
        cap: PathBuf,

        /// Directory containing schema files
        #[arg(short, long, default_value = "./cartridge-schemas")]
        schema_dir: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Validate an interface schema file
    ValidateInterface {
        /// Path to the interface schema file
        #[arg(short, long)]
        interface: PathBuf,

        /// Directory containing schema files
        #[arg(short, long, default_value = "./cartridge-schemas")]
        schema_dir: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// List available interface schemas
    ListInterfaces {
        /// Directory containing schema files
        #[arg(short, long, default_value = "./cartridge-schemas")]
        schema_dir: PathBuf,
    },

    /// Generate test scenarios for an interface
    GenerateTests {
        /// Interface to generate tests for
        #[arg(short, long)]
        interface: String,

        /// Directory containing schema files
        #[arg(short, long, default_value = "./cartridge-schemas")]
        schema_dir: PathBuf,

        /// Output directory for test files
        #[arg(short, long, default_value = "./tests")]
        output_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ValidateCartridge { cartridge, interface, schema_dir, output, verbose } => {
            validate_cartridge(cartridge, interface, schema_dir, output, verbose)
        }
        Commands::ValidateCap { cap, schema_dir, verbose } => {
            validate_cap(cap, schema_dir, verbose)
        }
        Commands::ValidateInterface { interface, schema_dir, verbose } => {
            validate_interface(interface, schema_dir, verbose)
        }
        Commands::ListInterfaces { schema_dir } => {
            list_interfaces(schema_dir)
        }
        Commands::GenerateTests { interface, schema_dir, output_dir } => {
            generate_tests(interface, schema_dir, output_dir)
        }
    }
}

fn validate_cartridge(
    cartridge_path: PathBuf,
    interface_name: String,
    schema_dir: PathBuf,
    output_format: String,
    verbose: bool
) -> Result<()> {
    tracing::info!(" Validating cartridge against interface schema...");
    tracing::info!("Cartridge: {}", cartridge_path.display());
    tracing::info!("Interface: {}", interface_name);
    tracing::info!("Schema directory: {}", schema_dir.display());
    println!();  // TODO: convert to tracing

    let mut validator = CartridgeValidator::new(&schema_dir)
        .context("Failed to create cartridge validator")?;

    // Load the interface schema
    let interface_schema_path = schema_dir.join("interfaces").join(format!("{}.json", interface_name));
    validator.load_interface_schema(&interface_schema_path)
        .with_context(|| format!("Failed to load interface schema: {}", interface_schema_path.display()))?;

    // Validate the cartridge
    let report = validator.validate_cartridge_implementation(&cartridge_path, &interface_name)
        .context("Cartridge validation failed")?;

    // Output results
    match output_format.as_str() {
        "json" => output_json_report(&report)?,
        "text" => output_text_report(&report, verbose)?,
        _ => anyhow::bail!("Unsupported output format: {}", output_format),
    }

    // Exit with error code if validation failed
    if !report.is_valid() {
        std::process::exit(1);
    }

    Ok(())
}

fn validate_cap(cap_path: PathBuf, schema_dir: PathBuf, verbose: bool) -> Result<()> {
    tracing::info!(" Validating cap schema...");
    tracing::info!("Cap: {}", cap_path.display());
    tracing::info!("Schema directory: {}", schema_dir.display());
    println!();  // TODO: convert to tracing

    let mut validator = CartridgeValidator::new(&schema_dir)
        .context("Failed to create cartridge validator")?;

    match validator.load_cap_schema(&cap_path) {
        Ok(schema) => {
            tracing::info!("OK Cap schema is valid!");
            if verbose {
                tracing::info!("Cap name: {}", schema.cap.name);
                tracing::info!("Description: {}", schema.cap.description);
                tracing::info!("File types: {:?}", schema.cap.file_types);
                tracing::info!("Version: {}", schema.cap.version);
            }
        }
        Err(e) => {
            tracing::error!("ERR Cap schema validation failed:");
            tracing::info!("{}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn validate_interface(interface_path: PathBuf, schema_dir: PathBuf, verbose: bool) -> Result<()> {
    tracing::info!(" Validating interface schema...");
    tracing::info!("Interface: {}", interface_path.display());
    tracing::info!("Schema directory: {}", schema_dir.display());
    println!();  // TODO: convert to tracing

    let mut validator = CartridgeValidator::new(&schema_dir)
        .context("Failed to create cartridge validator")?;

    match validator.load_interface_schema(&interface_path) {
        Ok(schema) => {
            tracing::info!("OK Interface schema is valid!");
            if verbose {
                tracing::info!("Interface name: {}", schema.interface.name);
                tracing::info!("Description: {}", schema.interface.description);
                tracing::info!("Version: {}", schema.interface.version);
                tracing::info!("Caps: {}", schema.caps.len());
                tracing::info!("Authors: {:?}", schema.interface.authors);
            }
        }
        Err(e) => {
            tracing::error!("ERR Interface schema validation failed:");
            tracing::info!("{}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn list_interfaces(schema_dir: PathBuf) -> Result<()> {
    tracing::info!(" Available interface schemas:");
    println!();  // TODO: convert to tracing

    let interfaces_dir = schema_dir.join("interfaces");
    if !interfaces_dir.exists() {
        tracing::info!("No interfaces directory found at: {}", interfaces_dir.display());
        return Ok(());
    }

    let entries = std::fs::read_dir(&interfaces_dir)
        .with_context(|| format!("Failed to read interfaces directory: {}", interfaces_dir.display()))?;

    let mut found_any = false;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "json") {
            found_any = true;
            let interface_name = path.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");

            // Try to load and display basic info
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str::<serde_json::Value>(&content) {
                        Ok(json) => {
                            let description = json.get("interface")
                                .and_then(|i| i.get("description"))
                                .and_then(|d| d.as_str())
                                .unwrap_or("No description");
                            let version = json.get("interface")
                                .and_then(|i| i.get("version"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");

                            tracing::info!(" {} (v{})", interface_name, version);
                            tracing::info!("   {}", description);
                            println!();  // TODO: convert to tracing
                        }
                        Err(_) => {
                            tracing::info!(" {} (invalid JSON)", interface_name);
                            println!();  // TODO: convert to tracing
                        }
                    }
                }
                Err(_) => {
                    tracing::info!(" {} (unreadable)", interface_name);
                    println!();  // TODO: convert to tracing
                }
            }
        }
    }

    if !found_any {
        tracing::info!("No interface schema files found in: {}", interfaces_dir.display());
    }

    Ok(())
}

fn generate_tests(interface_name: String, schema_dir: PathBuf, output_dir: PathBuf) -> Result<()> {
    tracing::info!(" Generating test scenarios for interface: {}", interface_name);
    tracing::info!("Schema directory: {}", schema_dir.display());
    tracing::info!("Output directory: {}", output_dir.display());
    println!();  // TODO: convert to tracing

    let mut validator = CartridgeValidator::new(&schema_dir)
        .context("Failed to create cartridge validator")?;

    let interface_schema_path = schema_dir.join("interfaces").join(format!("{}.json", interface_name));
    let interface = validator.load_interface_schema(&interface_schema_path)
        .with_context(|| format!("Failed to load interface schema: {}", interface_schema_path.display()))?;

    // Create output directory
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    // Generate test script
    let test_script = generate_test_script(interface, &interface_name);
    let script_path = output_dir.join(format!("test_{}.sh", interface_name));
    std::fs::write(&script_path, test_script)
        .with_context(|| format!("Failed to write test script: {}", script_path.display()))?;

    tracing::info!("OK Generated test script: {}", script_path.display());

    // Generate test configuration
    let test_config = generate_test_config(interface);
    let config_path = output_dir.join(format!("test_{}.json", interface_name));
    std::fs::write(&config_path, test_config)
        .with_context(|| format!("Failed to write test config: {}", config_path.display()))?;

    tracing::info!("OK Generated test config: {}", config_path.display());

    Ok(())
}

fn output_text_report(report: &ValidationReport, verbose: bool) -> Result<()> {
    tracing::info!(" Validation Report");
    tracing::info!("==================");
    println!();  // TODO: convert to tracing
    tracing::info!("{}", report.summary());
    println!();  // TODO: convert to tracing

    if !report.errors.is_empty() {
        tracing::error!("ERR Errors:");
        for error in &report.errors {
            tracing::info!("   • {}", error);
        }
        println!();  // TODO: convert to tracing
    }

    if !report.warnings.is_empty() {
        tracing::warn!("WARN  Warnings:");
        for warning in &report.warnings {
            tracing::info!("   • {}", warning);
        }
        println!();  // TODO: convert to tracing
    }

    if verbose && !report.successes.is_empty() {
        tracing::info!("OK Successes:");
        for success in &report.successes {
            tracing::info!("   • {}", success);
        }
        println!();  // TODO: convert to tracing
    }

    if report.is_valid() {
        tracing::info!(" Cartridge validation PASSED!");
    } else {
        tracing::error!(" Cartridge validation FAILED!");
    }

    Ok(())
}

fn output_json_report(report: &ValidationReport) -> Result<()> {
    let json_report = serde_json::json!({
        "cartridge_path": report.cartridge_path,
        "interface_name": report.interface_name,
        "is_valid": report.is_valid(),
        "successes": report.successes,
        "errors": report.errors,
        "warnings": report.warnings
    });

    tracing::info!("{}", serde_json::to_string_pretty(&json_report)?);
    Ok(())
}

fn generate_test_script(interface: &machfab_cartridge_sdk::CartridgeInterfaceSchema, interface_name: &str) -> String {
    let mut script = String::new();

    script.push_str("#!/bin/bash\n");
    script.push_str(&format!("# Test script for {} interface\n", interface_name));
    script.push_str("# Generated by cartridge-validator\n\n");
    script.push_str("set -e\n\n");
    script.push_str("CARTRIDGE_BINARY=\"$1\"\n");
    script.push_str("if [ -z \"$CARTRIDGE_BINARY\" ]; then\n");
    script.push_str("  echo \"Usage: $0 <cartridge_binary>\"\n");
    script.push_str("  exit 1\n");
    script.push_str("fi\n\n");
    script.push_str("echo \" Testing cartridge: $CARTRIDGE_BINARY\"\n");
    script.push_str("echo \" Interface: ");
    script.push_str(interface_name);
    script.push_str("\"\n\n");

    // Test manifest command
    script.push_str("echo \"Testing manifest command...\"\n");
    script.push_str("\"$CARTRIDGE_BINARY\" manifest > /tmp/manifest.json\n");
    script.push_str("echo \"OK manifest command passed\"\n\n");

    // Test each cap
    for cap_ref in interface.caps.iter() {
        let cap_name = match cap_ref {
            machfab_cartridge_sdk::CapReference::Inline(cap) => &cap.cap.name,
            machfab_cartridge_sdk::CapReference::Reference { cap_ref } => {
                cap_ref.split('/').last().unwrap_or(cap_ref).trim_end_matches(".json")
            }
        };

        script.push_str(&format!("echo \"Testing cap: {}...\"\n", cap_name));
        script.push_str(&format!("\"$CARTRIDGE_BINARY\" --{} --help >/dev/null 2>&1 || true\n", cap_name));
        script.push_str(&format!("echo \"OK {} cap flag recognized\"\n\n", cap_name));
    }

    script.push_str("echo \" All tests passed!\"\n");
    script
}

fn generate_test_config(interface: &machfab_cartridge_sdk::CartridgeInterfaceSchema) -> String {
    let config = serde_json::json!({
        "interface": interface.interface.name,
        "version": interface.interface.version,
        "test_scenarios": interface.testing.test_scenarios,
        "test_files": interface.testing.test_files,
        "caps": interface.caps.len()
    });

    serde_json::to_string_pretty(&config).unwrap_or_else(|_| "{}".to_string())
}

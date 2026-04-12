//! Comprehensive Cartridge Testing Tool
//!
//! Advanced testing tool that validates cartridge implementations with real data,
//! tests serialization integrity, and validates all edge cases.

use clap::{Parser, Subcommand};
use machfab_cartridge_sdk::{CartridgeValidator, ValidationReport};
use std::path::PathBuf;
use std::process::Command;
use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Parser)]
#[command(name = "cartridge-tester")]
#[command(about = "Comprehensive cartridge testing and validation")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run comprehensive tests on a cartridge
    TestCartridge {
        /// Path to the cartridge binary to test
        #[arg(short = 'c', long)]
        cartridge: PathBuf,

        /// Interface to test against
        #[arg(short, long)]
        interface: String,

        /// Directory containing schema files
        #[arg(short, long, default_value = "./cartridge-schemas")]
        schema_dir: PathBuf,

        /// Run stress tests
        #[arg(long)]
        stress: bool,

        /// Test with various file sizes
        #[arg(long)]
        file_sizes: bool,

        /// Test serialization integrity
        #[arg(long)]
        serialization: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Test serialization specifically
    TestSerialization {
        /// Path to the cartridge binary
        #[arg(short = 'c', long)]
        cartridge: PathBuf,

        /// Cap to test
        #[arg(short = 'a', long)]
        cap: String,

        /// Test file path
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Test error handling
    TestErrorHandling {
        /// Path to the cartridge binary
        #[arg(short = 'c', long)]
        cartridge: PathBuf,

        /// Interface to test against
        #[arg(short, long)]
        interface: String,

        /// Directory containing schema files
        #[arg(short, long, default_value = "./cartridge-schemas")]
        schema_dir: PathBuf,
    },

    /// Generate test files for cartridge validation
    GenerateTestFiles {
        /// Output directory
        #[arg(short, long, default_value = "./test-files")]
        output: PathBuf,

        /// File types to generate
        #[arg(short, long, default_values_t = vec!["txt".to_string(), "md".to_string()])]
        types: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::TestCartridge { cartridge, interface, schema_dir, stress, file_sizes, serialization, verbose } => {
            test_cartridge_comprehensive(cartridge, interface, schema_dir, stress, file_sizes, serialization, verbose)
        }
        Commands::TestSerialization { cartridge, cap, file } => {
            test_serialization_integrity(cartridge, cap, file)
        }
        Commands::TestErrorHandling { cartridge, interface, schema_dir } => {
            test_error_handling(cartridge, interface, schema_dir)
        }
        Commands::GenerateTestFiles { output, types } => {
            generate_test_files(output, types)
        }
    }
}

fn test_cartridge_comprehensive(
    cartridge: PathBuf,
    interface: String,
    schema_dir: PathBuf,
    stress: bool,
    file_sizes: bool,
    serialization: bool,
    verbose: bool,
) -> Result<()> {
    tracing::info!(" Running comprehensive cartridge tests...");
    tracing::info!("Cartridge: {}", cartridge.display());
    tracing::info!("Interface: {}", interface);
    println!();  // TODO: convert to tracing

    let mut validator = CartridgeValidator::new(&schema_dir)
        .context("Failed to create cartridge validator")?;

    // Load interface schema
    let interface_schema_path = schema_dir.join("interfaces").join(format!("{}.json", interface));
    validator.load_interface_schema(&interface_schema_path)
        .with_context(|| format!("Failed to load interface schema: {}", interface_schema_path.display()))?;

    // Run standard validation
    tracing::info!(" Running standard validation...");
    let report = validator.validate_cartridge_implementation(&cartridge, &interface)
        .context("Standard validation failed")?;

    if verbose {
        print_detailed_report(&report);
    } else {
        print_summary_report(&report);
    }

    // Run serialization tests if requested
    if serialization {
        tracing::info!("\n Testing serialization integrity...");
        test_all_caps_serialization(&cartridge, &interface)?;
    }

    // Run file size tests if requested
    if file_sizes {
        tracing::info!("\n Testing with various file sizes...");
        test_file_sizes(&cartridge)?;
    }

    // Run stress tests if requested
    if stress {
        tracing::info!("\n Running stress tests...");
        run_stress_tests(&cartridge)?;
    }

    if report.is_valid() {
        tracing::info!("\n All comprehensive tests PASSED!");
        Ok(())
    } else {
        tracing::error!("\n Some tests FAILED!");
        std::process::exit(1);
    }
}

fn test_serialization_integrity(cartridge: PathBuf, cap: String, file: Option<PathBuf>) -> Result<()> {
    tracing::info!(" Testing serialization integrity for cap: {}", cap);

    let test_file = if let Some(ref file_path) = file {
        file_path.clone()
    } else {
        // Create a temporary test file
        let temp_file = std::env::temp_dir().join("serialization_test.txt");
        std::fs::write(&temp_file, "Test content for serialization validation.\n\nThis is a second paragraph.\n\nAnd a third one with some content.")?;
        temp_file
    };

    // Test the cap (use as subcommand, not flag)
    let test_file_str = test_file.to_string_lossy().to_string();
    let output = Command::new(&cartridge)
        .args(&[&cap, &test_file_str])
        .output()
        .context("Failed to execute cartridge")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("ERR Cartridge execution failed: {}", stderr);
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for Debug format patterns
    let debug_patterns = [
        "FileMetadata {",
        "ExtractionInfo {",
    ];

    for pattern in &debug_patterns {
        if stdout.contains(pattern) {
            tracing::info!("ERR SERIALIZATION FAILURE: Output contains Debug format pattern: {}", pattern);
            tracing::info!("First 200 chars of output:");
            tracing::info!("{}", &stdout[..stdout.len().min(200)]);
            return Ok(());
        }
    }

    // Verify it's valid JSON
    match serde_json::from_str::<Value>(&stdout) {
        Ok(json_value) => {
            tracing::info!("OK Output is valid JSON");
            if stdout.len() > 1000 {
                tracing::info!(" Output size: {} bytes", stdout.len());
            }

            // Verify structure based on cap
            match cap.as_str() {
                "extract-metadata" => {
                    if json_value.get("file_path").is_some() && json_value.get("file_size_bytes").is_some() {
                        tracing::info!("OK Metadata structure validation passed");
                    } else {
                        tracing::warn!("WARN  Metadata missing expected fields");
                    }
                }
                "grind" => {
                    if json_value.get("pages").and_then(|p| p.as_array()).is_some() {
                        tracing::info!("OK Pages structure validation passed");
                    } else {
                        tracing::warn!("WARN  Pages missing expected structure");
                    }
                }
                "extract-outline" => {
                    if json_value.get("entries").is_some() {
                        tracing::info!("OK Outline structure validation passed");
                    } else {
                        tracing::warn!("WARN  Outline missing expected structure");
                    }
                }
                _ => {}
            }
        }
        Err(e) => {
            tracing::info!("ERR SERIALIZATION FAILURE: Output is not valid JSON: {}", e);
            tracing::info!("First 200 chars of output:");
            tracing::info!("{}", &stdout[..stdout.len().min(200)]);
        }
    }

    // Clean up temp file if we created it
    if file.is_none() {
        std::fs::remove_file(&test_file).ok();
    }

    Ok(())
}

fn test_error_handling(cartridge: PathBuf, _interface: String, _schema_dir: PathBuf) -> Result<()> {
    tracing::error!("CRIT Testing error handling...");

    // Test with non-existent file
    let non_existent = "/tmp/absolutely_non_existent_file_12345.xyz";

    let caps = ["extract-metadata", "grind", "extract-outline"];

    for cap in &caps {
        let output = Command::new(&cartridge)
            .args(&[cap, non_existent])
            .output()
            .context("Failed to execute cartridge")?;

        if output.status.success() {
            tracing::info!("ERR {} should fail with non-existent file", cap);
        } else {
            let exit_code = output.status.code().unwrap_or(-1);
            if exit_code > 0 {
                tracing::info!("OK {} properly fails with exit code {}", cap, exit_code);
            } else {
                tracing::error!("WARN  {} failed but with exit code {}", cap, exit_code);
            }
        }
    }

    // Test with invalid file types
    let invalid_file = std::env::temp_dir().join("test.invalid");
    std::fs::write(&invalid_file, "invalid content")?;

    for cap in &caps {
        let invalid_file_str = invalid_file.to_string_lossy();
        let output = Command::new(&cartridge)
            .args(&[*cap, invalid_file_str.as_ref()])
            .output()
            .context("Failed to execute cartridge")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                tracing::warn!("WARN  {} succeeded but produced no output for invalid file", cap);
            } else {
                tracing::info!("OK {} handled invalid file gracefully", cap);
            }
        } else {
            tracing::info!("OK {} properly rejected invalid file", cap);
        }
    }

    std::fs::remove_file(&invalid_file).ok();
    Ok(())
}

fn test_all_caps_serialization(cartridge: &PathBuf, _interface: &str) -> Result<()> {
    let caps = ["extract-metadata", "grind", "extract-outline"];

    for cap in &caps {
        tracing::info!("  Testing {} serialization...", cap);
        if let Err(e) = test_serialization_integrity(cartridge.clone(), cap.to_string(), None) {
            tracing::error!("  ERR Failed: {}", e);
        }
    }

    Ok(())
}

fn test_file_sizes(cartridge: &PathBuf) -> Result<()> {
    // Test with different file sizes
    let sizes = [
        ("small", 100),
        ("medium", 10_000),
        ("large", 100_000),
    ];

    for (name, size) in &sizes {
        tracing::info!("  Testing with {} file ({} bytes)...", name, size);

        let content = "A".repeat(*size);
        let test_file = std::env::temp_dir().join(format!("test_{}.txt", name));
        std::fs::write(&test_file, content)?;

        let test_file_str = test_file.to_string_lossy();
        let output = Command::new(cartridge)
            .args(&["extract-metadata", &test_file_str])
            .output()
            .context("Failed to execute cartridge")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if serde_json::from_str::<Value>(&stdout).is_ok() {
                tracing::info!("    OK {} file handled correctly", name);
            } else {
                tracing::info!("    ERR {} file produced invalid JSON", name);
            }
        } else {
            tracing::error!("    WARN  {} file processing failed", name);
        }

        std::fs::remove_file(&test_file).ok();
    }

    Ok(())
}

fn run_stress_tests(cartridge: &PathBuf) -> Result<()> {
    tracing::info!("  Running concurrent execution test...");

    let test_file = std::env::temp_dir().join("stress_test.txt");
    std::fs::write(&test_file, "Stress test content\n\nFor concurrent execution.")?;

    let handles: Vec<_> = (0..5).map(|i| {
        let cartridge = cartridge.clone();
        let test_file = test_file.clone();
        std::thread::spawn(move || {
            let test_file_str = test_file.to_string_lossy();
            let output = Command::new(&cartridge)
                .args(&["extract-metadata", &test_file_str])
                .output();
            (i, output)
        })
    }).collect();

    let mut successes = 0;
    for handle in handles {
        if let Ok((i, Ok(output))) = handle.join() {
            if output.status.success() {
                successes += 1;
            } else {
                tracing::error!("    WARN  Concurrent test {} failed", i);
            }
        }
    }

    if successes == 5 {
        tracing::info!("    OK All concurrent executions succeeded");
    } else {
        tracing::warn!("    WARN  {}/5 concurrent executions succeeded", successes);
    }

    std::fs::remove_file(&test_file).ok();
    Ok(())
}

fn generate_test_files(output_dir: PathBuf, types: Vec<String>) -> Result<()> {
    tracing::info!(" Generating test files in: {}", output_dir.display());

    std::fs::create_dir_all(&output_dir)?;

    for file_type in &types {
        match file_type.as_str() {
            "txt" => {
                let simple = output_dir.join("simple.txt");
                std::fs::write(&simple, "Simple text file.\n\nWith multiple paragraphs.")?;

                let complex = output_dir.join("complex.txt");
                std::fs::write(&complex, include_str!("../test_data/complex.txt"))?;

                tracing::info!("  OK Generated txt test files");
            }
            "md" => {
                let simple = output_dir.join("simple.md");
                std::fs::write(&simple, "# Simple Markdown\n\n## Section 1\n\nContent here.\n\n## Section 2\n\nMore content.")?;

                let complex = output_dir.join("complex.md");
                std::fs::write(&complex, include_str!("../test_data/complex.md"))?;

                tracing::info!("  OK Generated md test files");
            }
            _ => {
                tracing::warn!("  WARN  Unknown file type: {}", file_type);
            }
        }
    }

    Ok(())
}

fn print_detailed_report(report: &ValidationReport) {
    tracing::info!(" Detailed Validation Report");
    tracing::info!("=============================");
    tracing::info!("Cartridge: {}", report.cartridge_path.display());
    tracing::info!("Interface: {}", report.interface_name);
    tracing::info!("Valid: {}", if report.is_valid() { "OK YES" } else { "ERR NO" });
    println!();  // TODO: convert to tracing

    if !report.errors.is_empty() {
        tracing::error!("ERR Errors ({})", report.errors.len());
        for (i, error) in report.errors.iter().enumerate() {
            tracing::info!("  {}. {}", i + 1, error);
        }
        println!();  // TODO: convert to tracing
    }

    if !report.warnings.is_empty() {
        tracing::warn!("WARN  Warnings ({})", report.warnings.len());
        for (i, warning) in report.warnings.iter().enumerate() {
            tracing::info!("  {}. {}", i + 1, warning);
        }
        println!();  // TODO: convert to tracing
    }

    if !report.successes.is_empty() {
        tracing::info!("OK Successes ({})", report.successes.len());
        for (i, success) in report.successes.iter().enumerate() {
            tracing::info!("  {}. {}", i + 1, success);
        }
        println!();  // TODO: convert to tracing
    }
}

fn print_summary_report(report: &ValidationReport) {
    tracing::info!(" Validation Summary");
    tracing::info!("====================");
    tracing::info!("Status: {}", if report.is_valid() { "OK PASSED" } else { "ERR FAILED" });
    tracing::info!("Successes: {}", report.successes.len());
    tracing::warn!("Warnings: {}", report.warnings.len());
    tracing::error!("Errors: {}", report.errors.len());

    if !report.errors.is_empty() {
        tracing::error!("\nERR Errors:");
        for error in &report.errors {
            tracing::info!("  • {}", error);
        }
    }
}

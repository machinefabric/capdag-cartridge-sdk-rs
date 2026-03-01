//! Comprehensive Plugin Testing Tool
//! 
//! Advanced testing tool that validates plugin implementations with real data,
//! tests serialization integrity, and validates all edge cases.

use clap::{Parser, Subcommand};
use machfab_plugin_sdk::{PluginValidator, ValidationReport};
use std::path::PathBuf;
use std::process::Command;
use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Parser)]
#[command(name = "plugin-tester")]
#[command(about = "Comprehensive plugin testing and validation")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run comprehensive tests on a plugin
    TestPlugin {
        /// Path to the plugin binary to test
        #[arg(short, long)]
        plugin: PathBuf,
        
        /// Interface to test against
        #[arg(short, long)]
        interface: String,
        
        /// Directory containing schema files
        #[arg(short, long, default_value = "./plugin-schemas")]
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
        /// Path to the plugin binary
        #[arg(short, long)]
        plugin: PathBuf,
        
        /// Cap to test
        #[arg(short, long)]
        cap: String,
        
        /// Test file path
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    
    /// Test error handling
    TestErrorHandling {
        /// Path to the plugin binary
        #[arg(short, long)]
        plugin: PathBuf,
        
        /// Interface to test against
        #[arg(short, long)]
        interface: String,
        
        /// Directory containing schema files
        #[arg(short, long, default_value = "./plugin-schemas")]
        schema_dir: PathBuf,
    },
    
    /// Generate test files for plugin validation
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
        Commands::TestPlugin { plugin, interface, schema_dir, stress, file_sizes, serialization, verbose } => {
            test_plugin_comprehensive(plugin, interface, schema_dir, stress, file_sizes, serialization, verbose)
        }
        Commands::TestSerialization { plugin, cap, file } => {
            test_serialization_integrity(plugin, cap, file)
        }
        Commands::TestErrorHandling { plugin, interface, schema_dir } => {
            test_error_handling(plugin, interface, schema_dir)
        }
        Commands::GenerateTestFiles { output, types } => {
            generate_test_files(output, types)
        }
    }
}

fn test_plugin_comprehensive(
    plugin: PathBuf,
    interface: String,
    schema_dir: PathBuf,
    stress: bool,
    file_sizes: bool,
    serialization: bool,
    verbose: bool,
) -> Result<()> {
    println!(" Running comprehensive plugin tests...");
    println!("Plugin: {}", plugin.display());
    println!("Interface: {}", interface);
    println!();

    let mut validator = PluginValidator::new(&schema_dir)
        .context("Failed to create plugin validator")?;

    // Load interface schema
    let interface_schema_path = schema_dir.join("interfaces").join(format!("{}.json", interface));
    validator.load_interface_schema(&interface_schema_path)
        .with_context(|| format!("Failed to load interface schema: {}", interface_schema_path.display()))?;

    // Run standard validation
    println!(" Running standard validation...");
    let report = validator.validate_plugin_implementation(&plugin, &interface)
        .context("Standard validation failed")?;

    if verbose {
        print_detailed_report(&report);
    } else {
        print_summary_report(&report);
    }

    // Run serialization tests if requested
    if serialization {
        println!("\n Testing serialization integrity...");
        test_all_caps_serialization(&plugin, &interface)?;
    }

    // Run file size tests if requested
    if file_sizes {
        println!("\n Testing with various file sizes...");
        test_file_sizes(&plugin)?;
    }

    // Run stress tests if requested
    if stress {
        println!("\n Running stress tests...");
        run_stress_tests(&plugin)?;
    }

    if report.is_valid() {
        println!("\n All comprehensive tests PASSED!");
        Ok(())
    } else {
        println!("\n Some tests FAILED!");
        std::process::exit(1);
    }
}

fn test_serialization_integrity(plugin: PathBuf, cap: String, file: Option<PathBuf>) -> Result<()> {
    println!(" Testing serialization integrity for cap: {}", cap);
    
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
    let output = Command::new(&plugin)
        .args(&[&cap, &test_file_str])
        .output()
        .context("Failed to execute plugin")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("ERR Plugin execution failed: {}", stderr);
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for Debug format patterns
    let debug_patterns = [
        "FileMetadata {",
        "DocumentOutline {",
        "OutlineEntry {",
        "DisboundPage {",
        "ExtractionInfo {",
    ];

    for pattern in &debug_patterns {
        if stdout.contains(pattern) {
            println!("ERR SERIALIZATION FAILURE: Output contains Debug format pattern: {}", pattern);
            println!("First 200 chars of output:");
            println!("{}", &stdout[..stdout.len().min(200)]);
            return Ok(());
        }
    }

    // Verify it's valid JSON
    match serde_json::from_str::<Value>(&stdout) {
        Ok(json_value) => {
            println!("OK Output is valid JSON");
            if stdout.len() > 1000 {
                println!(" Output size: {} bytes", stdout.len());
            }
            
            // Verify structure based on cap
            match cap.as_str() {
                "extract-metadata" => {
                    if json_value.get("file_path").is_some() && json_value.get("file_size_bytes").is_some() {
                        println!("OK Metadata structure validation passed");
                    } else {
                        println!("WARN  Metadata missing expected fields");
                    }
                }
                "grind" => {
                    if json_value.get("pages").and_then(|p| p.as_array()).is_some() {
                        println!("OK Pages structure validation passed");
                    } else {
                        println!("WARN  Pages missing expected structure");
                    }
                }
                "extract-outline" => {
                    if json_value.get("entries").is_some() {
                        println!("OK Outline structure validation passed");
                    } else {
                        println!("WARN  Outline missing expected structure");
                    }
                }
                _ => {}
            }
        }
        Err(e) => {
            println!("ERR SERIALIZATION FAILURE: Output is not valid JSON: {}", e);
            println!("First 200 chars of output:");
            println!("{}", &stdout[..stdout.len().min(200)]);
        }
    }

    // Clean up temp file if we created it
    if file.is_none() {
        std::fs::remove_file(&test_file).ok();
    }

    Ok(())
}

fn test_error_handling(plugin: PathBuf, _interface: String, _schema_dir: PathBuf) -> Result<()> {
    println!("CRIT Testing error handling...");
    
    // Test with non-existent file
    let non_existent = "/tmp/absolutely_non_existent_file_12345.xyz";
    
    let caps = ["extract-metadata", "grind", "extract-outline"];
    
    for cap in &caps {
        let output = Command::new(&plugin)
            .args(&[cap, non_existent])
            .output()
            .context("Failed to execute plugin")?;

        if output.status.success() {
            println!("ERR {} should fail with non-existent file", cap);
        } else {
            let exit_code = output.status.code().unwrap_or(-1);
            if exit_code > 0 {
                println!("OK {} properly fails with exit code {}", cap, exit_code);
            } else {
                println!("WARN  {} failed but with exit code {}", cap, exit_code);
            }
        }
    }

    // Test with invalid file types
    let invalid_file = std::env::temp_dir().join("test.invalid");
    std::fs::write(&invalid_file, "invalid content")?;
    
    for cap in &caps {
        let invalid_file_str = invalid_file.to_string_lossy();
        let output = Command::new(&plugin)
            .args(&[*cap, invalid_file_str.as_ref()])
            .output()
            .context("Failed to execute plugin")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                println!("WARN  {} succeeded but produced no output for invalid file", cap);
            } else {
                println!("OK {} handled invalid file gracefully", cap);
            }
        } else {
            println!("OK {} properly rejected invalid file", cap);
        }
    }

    std::fs::remove_file(&invalid_file).ok();
    Ok(())
}

fn test_all_caps_serialization(plugin: &PathBuf, _interface: &str) -> Result<()> {
    let caps = ["extract-metadata", "grind", "extract-outline"];
    
    for cap in &caps {
        println!("  Testing {} serialization...", cap);
        if let Err(e) = test_serialization_integrity(plugin.clone(), cap.to_string(), None) {
            println!("  ERR Failed: {}", e);
        }
    }
    
    Ok(())
}

fn test_file_sizes(plugin: &PathBuf) -> Result<()> {
    // Test with different file sizes
    let sizes = [
        ("small", 100),
        ("medium", 10_000),
        ("large", 100_000),
    ];

    for (name, size) in &sizes {
        println!("  Testing with {} file ({} bytes)...", name, size);
        
        let content = "A".repeat(*size);
        let test_file = std::env::temp_dir().join(format!("test_{}.txt", name));
        std::fs::write(&test_file, content)?;
        
        let test_file_str = test_file.to_string_lossy();
        let output = Command::new(plugin)
            .args(&["extract-metadata", &test_file_str])
            .output()
            .context("Failed to execute plugin")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if serde_json::from_str::<Value>(&stdout).is_ok() {
                println!("    OK {} file handled correctly", name);
            } else {
                println!("    ERR {} file produced invalid JSON", name);
            }
        } else {
            println!("    WARN  {} file processing failed", name);
        }
        
        std::fs::remove_file(&test_file).ok();
    }
    
    Ok(())
}

fn run_stress_tests(plugin: &PathBuf) -> Result<()> {
    println!("  Running concurrent execution test...");
    
    let test_file = std::env::temp_dir().join("stress_test.txt");
    std::fs::write(&test_file, "Stress test content\n\nFor concurrent execution.")?;
    
    let handles: Vec<_> = (0..5).map(|i| {
        let plugin = plugin.clone();
        let test_file = test_file.clone();
        std::thread::spawn(move || {
            let test_file_str = test_file.to_string_lossy();
            let output = Command::new(&plugin)
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
                println!("    WARN  Concurrent test {} failed", i);
            }
        }
    }
    
    if successes == 5 {
        println!("    OK All concurrent executions succeeded");
    } else {
        println!("    WARN  {}/5 concurrent executions succeeded", successes);
    }
    
    std::fs::remove_file(&test_file).ok();
    Ok(())
}

fn generate_test_files(output_dir: PathBuf, types: Vec<String>) -> Result<()> {
    println!(" Generating test files in: {}", output_dir.display());
    
    std::fs::create_dir_all(&output_dir)?;
    
    for file_type in &types {
        match file_type.as_str() {
            "txt" => {
                let simple = output_dir.join("simple.txt");
                std::fs::write(&simple, "Simple text file.\n\nWith multiple paragraphs.")?;
                
                let complex = output_dir.join("complex.txt");
                std::fs::write(&complex, include_str!("../test_data/complex.txt"))?;
                
                println!("  OK Generated txt test files");
            }
            "md" => {
                let simple = output_dir.join("simple.md");
                std::fs::write(&simple, "# Simple Markdown\n\n## Section 1\n\nContent here.\n\n## Section 2\n\nMore content.")?;
                
                let complex = output_dir.join("complex.md");
                std::fs::write(&complex, include_str!("../test_data/complex.md"))?;
                
                println!("  OK Generated md test files");
            }
            _ => {
                println!("  WARN  Unknown file type: {}", file_type);
            }
        }
    }
    
    Ok(())
}

fn print_detailed_report(report: &ValidationReport) {
    println!(" Detailed Validation Report");
    println!("=============================");
    println!("Plugin: {}", report.plugin_path.display());
    println!("Interface: {}", report.interface_name);
    println!("Valid: {}", if report.is_valid() { "OK YES" } else { "ERR NO" });
    println!();

    if !report.errors.is_empty() {
        println!("ERR Errors ({})", report.errors.len());
        for (i, error) in report.errors.iter().enumerate() {
            println!("  {}. {}", i + 1, error);
        }
        println!();
    }

    if !report.warnings.is_empty() {
        println!("WARN  Warnings ({})", report.warnings.len());
        for (i, warning) in report.warnings.iter().enumerate() {
            println!("  {}. {}", i + 1, warning);
        }
        println!();
    }

    if !report.successes.is_empty() {
        println!("OK Successes ({})", report.successes.len());
        for (i, success) in report.successes.iter().enumerate() {
            println!("  {}. {}", i + 1, success);
        }
        println!();
    }
}

fn print_summary_report(report: &ValidationReport) {
    println!(" Validation Summary");
    println!("====================");
    println!("Status: {}", if report.is_valid() { "OK PASSED" } else { "ERR FAILED" });
    println!("Successes: {}", report.successes.len());
    println!("Warnings: {}", report.warnings.len());
    println!("Errors: {}", report.errors.len());
    
    if !report.errors.is_empty() {
        println!("\nERR Errors:");
        for error in &report.errors {
            println!("  • {}", error);
        }
    }
}
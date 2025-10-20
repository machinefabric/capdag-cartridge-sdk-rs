//! Standard plugin capability definitions with arguments
//!
//! This module provides the standard capability definitions used across
//! all LBVR plugins, including their formal argument specifications.

use capdef::{
    CapabilityId, Capability, CapabilityArgument, CapabilityArguments, CommandInterface,
    CapabilityOutput, ArgumentType, ArgumentValidation, OutputType
};
use crate::PluginCapabilities;
use std::collections::HashMap;

/// Create the standard extract-metadata capability with full argument definition
pub fn extract_metadata_capability() -> Capability {
    let id = CapabilityId::from_string("document:extract:metadata")
        .expect("Invalid capability ID");
    
    let command_interface = CommandInterface {
        cli_flag: "--extract-metadata".to_string(),
        usage_pattern: "plugin_binary --extract-metadata <file_path> [--output <output_file>]".to_string(),
    };
    
    let mut arguments = CapabilityArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapabilityArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: None,
        position: Some(0),
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            min_length: Some(1),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_required(file_path_arg);
    
    // Optional output argument
    let output_arg = CapabilityArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write output to specified file instead of stdout".to_string(),
        cli_flag: Some("--output".to_string()),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    let output = CapabilityOutput {
        output_type: OutputType::Object,
        schema_ref: Some("file-metadata.json".to_string()),
        content_type: Some("application/json".to_string()),
        validation: ArgumentValidation::default(),
        description: "Structured metadata including file properties, document properties, and format-specific metadata".to_string(),
    };
    
    Capability::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Extract document metadata including title, author, creation date, file size, and other properties".to_string()),
        HashMap::new(),
        Some(command_interface),
        arguments,
        Some(output),
    )
}

/// Create the standard generate-thumbnail capability with full argument definition
pub fn generate_thumbnail_capability() -> Capability {
    let id = CapabilityId::from_string("document:generate:thumbnail")
        .expect("Invalid capability ID");
    
    let command_interface = CommandInterface {
        cli_flag: "--generate-thumbnail".to_string(),
        usage_pattern: "plugin_binary --generate-thumbnail <file_path> [--width <width>] [--height <height>] [--output <output_file>] [--page <page>]".to_string(),
    };
    
    let mut arguments = CapabilityArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapabilityArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: None,
        position: Some(0),
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            min_length: Some(1),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_required(file_path_arg);
    
    // Optional width argument
    let width_arg = CapabilityArgument {
        name: "width".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Width of the thumbnail in pixels".to_string(),
        cli_flag: Some("--width".to_string()),
        position: None,
        validation: ArgumentValidation {
            min: Some(50.0),
            max: Some(2000.0),
            ..Default::default()
        },
        default: Some(serde_json::Value::Number(serde_json::Number::from(200))),
    };
    arguments.add_optional(width_arg);
    
    // Optional height argument
    let height_arg = CapabilityArgument {
        name: "height".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Height of the thumbnail in pixels".to_string(),
        cli_flag: Some("--height".to_string()),
        position: None,
        validation: ArgumentValidation {
            min: Some(50.0),
            max: Some(2000.0),
            ..Default::default()
        },
        default: Some(serde_json::Value::Number(serde_json::Number::from(300))),
    };
    arguments.add_optional(height_arg);
    
    // Optional output argument
    let output_arg = CapabilityArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write thumbnail to specified file instead of stdout".to_string(),
        cli_flag: Some("--output".to_string()),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("\\.(png|jpg|jpeg)$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    // Optional page argument
    let page_arg = CapabilityArgument {
        name: "page".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Page number to generate thumbnail from (1-based, default: 1)".to_string(),
        cli_flag: Some("--page".to_string()),
        position: None,
        validation: ArgumentValidation {
            min: Some(1.0),
            ..Default::default()
        },
        default: Some(serde_json::Value::Number(serde_json::Number::from(1))),
    };
    arguments.add_optional(page_arg);
    
    let output = CapabilityOutput {
        output_type: OutputType::Binary,
        schema_ref: None,
        content_type: Some("image/png".to_string()),
        validation: ArgumentValidation::default(),
        description: "PNG image data representing a thumbnail of the document".to_string(),
    };
    
    Capability::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Generate a thumbnail image preview of the document".to_string()),
        HashMap::new(),
        Some(command_interface),
        arguments,
        Some(output),
    )
}

/// Create the standard extract-outline capability with full argument definition
pub fn extract_outline_capability() -> Capability {
    let id = CapabilityId::from_string("document:extract:outline")
        .expect("Invalid capability ID");
    
    let command_interface = CommandInterface {
        cli_flag: "--extract-outline".to_string(),
        usage_pattern: "plugin_binary --extract-outline <file_path> [--max-depth <depth>] [--include-page-numbers] [--output <output_file>]".to_string(),
    };
    
    let mut arguments = CapabilityArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapabilityArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: None,
        position: Some(0),
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            min_length: Some(1),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_required(file_path_arg);
    
    // Optional max_depth argument
    let max_depth_arg = CapabilityArgument {
        name: "max_depth".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Maximum outline depth to extract (1-10)".to_string(),
        cli_flag: Some("--max-depth".to_string()),
        position: None,
        validation: ArgumentValidation {
            min: Some(1.0),
            max: Some(10.0),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(max_depth_arg);
    
    // Optional include_page_numbers argument
    let include_page_numbers_arg = CapabilityArgument {
        name: "include_page_numbers".to_string(),
        arg_type: ArgumentType::Boolean,
        description: "Include page numbers in the outline (default: true)".to_string(),
        cli_flag: Some("--include-page-numbers".to_string()),
        position: None,
        validation: ArgumentValidation::default(),
        default: Some(serde_json::Value::Bool(true)),
    };
    arguments.add_optional(include_page_numbers_arg);
    
    // Optional output argument
    let output_arg = CapabilityArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write output to specified file instead of stdout".to_string(),
        cli_flag: Some("--output".to_string()),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    let output = CapabilityOutput {
        output_type: OutputType::Object,
        schema_ref: Some("document-outline.json".to_string()),
        content_type: Some("application/json".to_string()),
        validation: ArgumentValidation::default(),
        description: "Hierarchical document outline with section titles and optional page numbers".to_string(),
    };
    
    Capability::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Extract document outline/table of contents with hierarchical structure".to_string()),
        HashMap::new(),
        Some(command_interface),
        arguments,
        Some(output),
    )
}

/// Create the standard extract-pages capability with full argument definition
pub fn extract_pages_capability() -> Capability {
    let id = CapabilityId::from_string("document:extract:pages")
        .expect("Invalid capability ID");
    
    let command_interface = CommandInterface {
        cli_flag: "--extract-pages".to_string(),
        usage_pattern: "plugin_binary --extract-pages <file_path> [--output <output_file>]".to_string(),
    };
    
    let mut arguments = CapabilityArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapabilityArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: None,
        position: Some(0),
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            min_length: Some(1),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_required(file_path_arg);
    
    // Optional output argument
    let output_arg = CapabilityArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write output to specified file instead of stdout".to_string(),
        cli_flag: Some("--output".to_string()),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    let output = CapabilityOutput {
        output_type: OutputType::Object,
        schema_ref: Some("document-pages.json".to_string()),
        content_type: Some("application/json".to_string()),
        validation: ArgumentValidation::default(),
        description: "Structured page content extracted from the document".to_string(),
    };
    
    Capability::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Extract structured page content from the document".to_string()),
        HashMap::new(),
        Some(command_interface),
        arguments,
        Some(output),
    )
}

/// Get all standard plugin capabilities
pub fn get_all_standard_capabilities() -> PluginCapabilities {
    let mut capabilities = PluginCapabilities::new();
    capabilities.add_capability(extract_metadata_capability());
    capabilities.add_capability(generate_thumbnail_capability());
    capabilities.add_capability(extract_outline_capability());
    capabilities.add_capability(extract_pages_capability());
    capabilities
}

/// Get a standard capability by name
pub fn get_standard_capability(name: &str) -> Option<Capability> {
    match name {
        "extract-metadata" => Some(extract_metadata_capability()),
        "generate-thumbnail" => Some(generate_thumbnail_capability()),
        "extract-outline" => Some(extract_outline_capability()),
        "extract-pages" => Some(extract_pages_capability()),
        _ => None,
    }
}

/// Get a standard capability by capability ID string
pub fn get_standard_capability_by_id(id_str: &str) -> Option<Capability> {
    match id_str {
        "document:extract:metadata" => Some(extract_metadata_capability()),
        "document:generate:thumbnail" => Some(generate_thumbnail_capability()),
        "document:extract:outline" => Some(extract_outline_capability()),
        "document:extract:pages" => Some(extract_pages_capability()),
        _ => None,
    }
}
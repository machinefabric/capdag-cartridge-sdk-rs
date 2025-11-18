//! Standard plugin cap definitions with arguments
//!
//! This module provides the standard cap definitions used across
//! all LBVR plugins, including their formal argument specifications.

use capdef::{
    CapCard, Cap, CapArgument, CapArguments,
    CapOutput, ArgumentType, ArgumentValidation, OutputType
};
use std::collections::HashMap;

/// Create the standard extract-metadata cap with full argument definition
pub fn extract_metadata_cap() -> Cap {
    let id = CapCard::from_string("action=extract;target=metadata;type=document")
        .expect("Invalid cap ID");
    
    let command = "extract-metadata".to_string();
    
    let mut arguments = CapArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: "file_path".to_string(),
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
    let output_arg = CapArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write output to specified file instead of stdout".to_string(),
        cli_flag: "--output".to_string(),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    let output = CapOutput {
        output_type: OutputType::Object,
        schema_ref: Some("file-metadata.json".to_string()),
        content_type: Some("application/json".to_string()),
        validation: ArgumentValidation::default(),
        description: "Structured metadata including file properties, document properties, and format-specific metadata".to_string(),
    };
    
    let mut cap = Cap::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Extract document metadata including title, author, creation date, file size, and other properties".to_string()),
        HashMap::new(),
        command,
        arguments,
        Some(output),
    );
    
    // Metadata extraction can accept stdin for direct file content processing
    cap.accepts_stdin = true;
    cap
}

/// Create the standard generate-thumbnail cap with full argument definition
pub fn generate_thumbnail_cap() -> Cap {
    let id = CapCard::from_string("action=generate;output=binary;target=thumbnail;type=document")
        .expect("Invalid cap ID");
    
    let command = "generate-thumbnail".to_string();
    
    let mut arguments = CapArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: "file_path".to_string(),
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
    let width_arg = CapArgument {
        name: "width".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Width of the thumbnail in pixels".to_string(),
        cli_flag: "--width".to_string(),
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
    let height_arg = CapArgument {
        name: "height".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Height of the thumbnail in pixels".to_string(),
        cli_flag: "--height".to_string(),
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
    let output_arg = CapArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write thumbnail to specified file instead of stdout".to_string(),
        cli_flag: "--output".to_string(),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("\\.(png|jpg|jpeg)$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    // Optional page argument
    let page_arg = CapArgument {
        name: "page".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Page number to generate thumbnail from (1-based, default: 1)".to_string(),
        cli_flag: "--page".to_string(),
        position: None,
        validation: ArgumentValidation {
            min: Some(1.0),
            ..Default::default()
        },
        default: Some(serde_json::Value::Number(serde_json::Number::from(1))),
    };
    arguments.add_optional(page_arg);
    
    let output = CapOutput {
        output_type: OutputType::Binary,
        schema_ref: None,
        content_type: Some("image/png".to_string()),
        validation: ArgumentValidation::default(),
        description: "PNG image data representing a thumbnail of the document".to_string(),
    };
    
    let mut cap = Cap::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Generate a thumbnail image preview of the document".to_string()),
        HashMap::new(),
        command,
        arguments,
        Some(output),
    );
    
    // Thumbnail generation can accept stdin for direct file content processing
    cap.accepts_stdin = true;
    cap
}

/// Create the standard extract-outline cap with full argument definition
pub fn extract_outline_cap() -> Cap {
    let id = CapCard::from_string("action=extract;target=outline;type=document")
        .expect("Invalid cap ID");
    
    let command = "extract-outline".to_string();
    
    let mut arguments = CapArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: "file_path".to_string(),
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
    let max_depth_arg = CapArgument {
        name: "max_depth".to_string(),
        arg_type: ArgumentType::Integer,
        description: "Maximum outline depth to extract (1-10)".to_string(),
        cli_flag: "--max-depth".to_string(),
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
    let include_page_numbers_arg = CapArgument {
        name: "include_page_numbers".to_string(),
        arg_type: ArgumentType::Boolean,
        description: "Include page numbers in the outline (default: true)".to_string(),
        cli_flag: "--include-page-numbers".to_string(),
        position: None,
        validation: ArgumentValidation::default(),
        default: Some(serde_json::Value::Bool(true)),
    };
    arguments.add_optional(include_page_numbers_arg);
    
    // Optional output argument
    let output_arg = CapArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write output to specified file instead of stdout".to_string(),
        cli_flag: "--output".to_string(),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    let output = CapOutput {
        output_type: OutputType::Object,
        schema_ref: Some("document-outline.json".to_string()),
        content_type: Some("application/json".to_string()),
        validation: ArgumentValidation::default(),
        description: "Hierarchical document outline with section titles and optional page numbers".to_string(),
    };
    
    let mut cap = Cap::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Extract document outline/table of contents with hierarchical structure".to_string()),
        HashMap::new(),
        command,
        arguments,
        Some(output),
    );
    
    // Outline extraction can accept stdin for direct file content processing
    cap.accepts_stdin = true;
    cap
}

/// Create the standard extract-pages cap with full argument definition
pub fn extract_pages_cap() -> Cap {
    let id = CapCard::from_string("action=extract;target=pages;type=document")
        .expect("Invalid cap ID");
    
    let command = "extract-pages".to_string();
    
    let mut arguments = CapArguments::new();
    
    // Required file_path argument
    let file_path_arg = CapArgument {
        name: "file_path".to_string(),
        arg_type: ArgumentType::String,
        description: "Path to the document file to process".to_string(),
        cli_flag: "file_path".to_string(),
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
    let output_arg = CapArgument {
        name: "output".to_string(),
        arg_type: ArgumentType::String,
        description: "Write output to specified file instead of stdout".to_string(),
        cli_flag: "--output".to_string(),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("^[^\\0]+$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(output_arg);
    
    // Optional page_range argument
    let page_range_arg = CapArgument {
        name: "page_range".to_string(),
        arg_type: ArgumentType::String,
        description: "Page range to extract (e.g., '1-5' or '10-')".to_string(),
        cli_flag: "--page-range".to_string(),
        position: None,
        validation: ArgumentValidation {
            pattern: Some("^\\d+(-\\d*)?$".to_string()),
            ..Default::default()
        },
        default: None,
    };
    arguments.add_optional(page_range_arg);
    
    let output = CapOutput {
        output_type: OutputType::Object,
        schema_ref: Some("document-pages.json".to_string()),
        content_type: Some("application/json".to_string()),
        validation: ArgumentValidation::default(),
        description: "Structured page content extracted from the document".to_string(),
    };
    
    let mut cap = Cap::with_full_definition(
        id,
        "1.0.0".to_string(),
        Some("Extract structured page content from the document".to_string()),
        HashMap::new(),
        command,
        arguments,
        Some(output),
    );
    
    // Page extraction can accept stdin for direct file content processing
    cap.accepts_stdin = true;
    cap
}

/// Get all standard plugin caps
pub fn get_all_standard_caps() -> Vec<Cap> {
    vec![
        extract_metadata_cap(),
        generate_thumbnail_cap(),
        extract_outline_cap(),
        extract_pages_cap(),
    ]
}

/// Get a standard cap by name
pub fn get_standard_cap(name: &str) -> Option<Cap> {
    match name {
        "extract-metadata" => Some(extract_metadata_cap()),
        "generate-thumbnail" => Some(generate_thumbnail_cap()),
        "extract-outline" => Some(extract_outline_cap()),
        "extract-pages" => Some(extract_pages_cap()),
        _ => None,
    }
}

/// Get a standard cap by cap ID string
pub fn get_standard_cap_by_id(id_str: &str) -> Option<Cap> {
    match id_str {
        "action=extract;target=metadata;type=document" => Some(extract_metadata_cap()),
        "action=generate;output=binary;target=thumbnail;type=document" => Some(generate_thumbnail_cap()),
        "action=extract;target=outline;type=document" => Some(extract_outline_cap()),
        "action=extract;target=pages;type=document" => Some(extract_pages_cap()),
        _ => None,
    }
}
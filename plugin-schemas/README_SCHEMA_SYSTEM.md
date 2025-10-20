# Plugin Interface Schema System

This directory contains a formal schema system for defining, validating, and testing plugin interfaces in the LBVR ecosystem. The system provides type-safe validation of plugin implementations against formal specifications.

## Overview

The schema system consists of:

1. **Capability Schemas** - Define individual operations with typed arguments
2. **Plugin Interface Schemas** - Aggregate capabilities into complete plugin interfaces  
3. **Validation Framework** - Type-safe validation of plugin implementations
4. **CLI Tools** - Command-line utilities for schema validation and testing

## Directory Structure

```
plugin-schemas/
├── README_SCHEMA_SYSTEM.md          # This file
├── capability-schema.json           # JSON Schema for capability definitions
├── plugin-interface-schema.json     # JSON Schema for plugin interfaces
├── capabilities/                    # Individual capability definitions
│   ├── extract-metadata.json
│   ├── extract-outline.json
│   └── generate-thumbnail.json
├── interfaces/                      # Complete plugin interface definitions
│   └── document-processor.json
└── examples/                        # Example schemas and usage
```

## Core Concepts

### Capabilities

A **capability** defines a single operation that a plugin can perform, such as `extract-metadata` or `generate-thumbnail`. Each capability specifies:

- **Name and Description** - Human-readable identification
- **File Types** - Which file types the capability supports (`pdf`, `txt`, `*` for all)
- **Arguments** - Required and optional parameters with type validation
- **Response Format** - Expected output type and schema
- **Error Handling** - Standard error codes and timeout settings
- **Command Interface** - Command

### Plugin Interfaces

A **plugin interface** combines multiple capabilities into a cohesive contract that plugins must implement. Interfaces define:

- **Capability Set** - Which capabilities must be implemented
- **Global Requirements** - Plugin-info command, JSON output support, etc.
- **Validation Rules** - Consistency checks and constraints
- **Test Scenarios** - Predefined tests for validation

## Usage

### 1. Validating Plugin Implementations

Use the CLI validator to check if a plugin correctly implements an interface:

```bash
# Build the validator tool
cd lbvr-plugin-sdk
cargo build --bin plugin-validator

# Validate a plugin against an interface
./target/debug/plugin-validator validate-plugin \
  --plugin ../pdfczar/target/debug/pdfczar \
  --interface document-processor \
  --schema-dir ./plugin-schemas

# List available interfaces
./target/debug/plugin-validator list-interfaces \
  --schema-dir ./plugin-schemas
```

### 2. Creating New Capability Schemas

Define a new capability by creating a JSON file following the capability schema:

```json
{
  "schema_version": "1.0",
  "capability": {
    "name": "extract-images",
    "description": "Extract embedded images from documents",
    "file_types": ["pdf", "epub"],
    "version": "1.0.0"
  },
  "command_interface": {
    "command": "extract-images",
  },
  "arguments": {
    "required": [
      {
        "name": "file_path",
        "type": "string",
        "description": "Path to the document file",
        "position": 0
      }
    ],
    "optional": [
      {
        "name": "output_dir",
        "type": "string",
        "description": "Directory to save extracted images",
        "command": "output-dir",
        "default": "./images"
      }
    ]
  },
  "response": {
    "type": "json",
    "schema_ref": "extracted-images.json",
    "description": "List of extracted image files with metadata"
  }
}
```

### 3. Creating Plugin Interfaces

Combine capabilities into interfaces by referencing existing capability schemas:

```json
{
  "schema_version": "1.0",
  "interface": {
    "name": "advanced-document-processor",
    "description": "Extended document processing with image extraction",
    "version": "1.0.0",
    "compatibility": {
      "min_sdk_version": "0.1.0"
    }
  },
  "capabilities": [
    {"capability_ref": "../capabilities/extract-metadata.json"},
    {"capability_ref": "../capabilities/extract-images.json"}
  ]
}
```

### 4. Validating Schema Files

Validate capability and interface schemas before using them:

```bash
# Validate a capability schema
./target/debug/plugin-validator validate-capability \
  --capability ./plugin-schemas/capabilities/extract-metadata.json

# Validate an interface schema  
./target/debug/plugin-validator validate-interface \
  --interface ./plugin-schemas/interfaces/document-processor.json
```

### 5. Generating Tests

Generate test scripts for an interface:

```bash
./target/debug/plugin-validator generate-tests \
  --interface document-processor \
  --schema-dir ./plugin-schemas \
  --output-dir ./tests
```

This creates:
- `test_document-processor.sh` - Bash script to test plugin implementations
- `test_document-processor.json` - Test configuration file

## Schema Validation Features

### Type Safety

Arguments are validated with full type checking:

```json
{
  "name": "width",
  "type": "integer",
  "validation": {
    "min": 50,
    "max": 2000
  }
}
```

### File Type Consistency

Related capabilities are checked for consistent file type support:

```json
{
  "validation_rules": {
    "file_type_consistency": {
      "enforce": true,
      "related_capabilities": [
        ["extract-metadata", "extract-outline", "extract-pages"]
      ]
    }
  }
}
```

### Error Code Standardization

Standard error codes ensure consistent error handling:

```json
{
  "error_handling": {
    "error_codes": {
      "FILE_NOT_FOUND": {
        "code": 3,
        "message": "File not found: {file_path}",
        "description": "The specified file does not exist"
      }
    }
  }
}
```

## Integration with Existing Plugins

### Current Plugin Format

The schema system is designed to work with existing "czar" plugins that follow the pattern:

```rust
#[derive(Subcommand)]
enum Commands {
    #[command(name = "plugin-info")]
    PluginInfo { /* ... */ },
    
    #[command(name = "extract-metadata")]
    ExtractMetadata { /* ... */ },
    
    // ... other capabilities
}
```

### Migration Path

1. **Define Schemas** - Create capability and interface schemas for existing plugins
2. **Validate Current Implementation** - Use the validator to check compliance
3. **Fix Issues** - Address any validation errors
4. **Add Tests** - Generate and run test scenarios
5. **Continuous Validation** - Integrate into CI/CD pipeline

## Best Practices

### Capability Design

- Use hyphenated names (`extract-metadata`, not `extract_metadata`)
- Be specific about file type support
- Include comprehensive argument validation
- Define clear error codes and messages

### Interface Design

- Group related capabilities logically
- Enforce consistency rules
- Provide comprehensive test scenarios
- Version interfaces semantically

### Plugin Implementation

- Implement all required capabilities
- Follow argument naming conventions
- Return consistent error codes
- Support JSON output where specified

## Troubleshooting

### Common Validation Errors

1. **CLI Flag Not Recognized**
   - Ensure plugin implements the specified CLI flag
   - Check flag naming (must start with `--`)

2. **Invalid Argument Types**
   - Verify argument types match schema definitions
   - Check validation constraints (min/max values)

3. **Missing plugin-info Command**
   - Implement `plugin-info` command (outputs JSON by default)
   - Return valid JSON with required fields

4. **File Type Inconsistency**
   - Ensure related capabilities support consistent file types
   - Review file type specifications in schemas

### Debug Output

Use verbose mode for detailed validation information:

```bash
./target/debug/plugin-validator validate-plugin \
  --plugin ./my-plugin \
  --interface document-processor \
  --verbose
```

## Future Enhancements

- **Schema Versioning** - Support for schema evolution and backwards compatibility
- **Performance Testing** - Automated performance benchmarks
- **Documentation Generation** - Auto-generate API documentation from schemas
- **IDE Integration** - Schema validation in development environments

## Contributing

When adding new capabilities or interfaces:

1. Follow the established naming conventions
2. Include comprehensive test scenarios
3. Document all arguments and responses
4. Validate schemas before submitting
5. Update this README with new concepts or patterns
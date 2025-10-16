# LBVR Plugin Schemas

This directory contains JSON schemas that define the data structures and interfaces for LBVR document processing plugins. These schemas serve as the canonical specification for plugin development across different programming languages.

## Schema Files

### Core Data Structures

- **`file-metadata.json`** - Consolidated metadata structure for any document type
- **`document-outline.json`** - Document outline/table of contents structure  
- **`document-pages.json`** - Document pages with text content organized by pages and paragraphs
- **`plugin-info.json`** - Plugin information and capabilities
- **`extracted-data.json`** - Combined output containing multiple types of extracted data

### Interfaces

- **`handler-interface.json`** - Core DocumentHandler interface specification

## Schema Overview

### FileMetadata
The consolidated metadata structure supports:
- Common document metadata (title, authors, dates, etc.)
- Format-specific fields (PDF-specific, EPUB-specific)
- Extensible metadata through `extended_metadata` HashMap
- Word/character/page counts

### DocumentOutline
Hierarchical table of contents structure with:
- Recursive nesting through `TocEntry.children`
- Page/section references
- Source references (filenames, anchors)

### DocumentPages
Page-based text content organization:
- Documents contain pages (1-indexed)
- Pages contain paragraphs (1-indexed within page)
- Word/character counts at paragraph level
- Optional source references

### PluginCapabilities
String-based capability system:
- `extract_metadata` - Can extract document metadata
- `extract_outline` - Can extract table of contents
- `extract_pages` - Can extract page-based text content
- `validate_file` - Can validate file integrity
- `generate_thumbnail` - Can generate thumbnail images
- `supports_json_output` - Supports JSON output format

## Interface Requirements

### DocumentHandler
All plugin handlers must implement:

**Required Methods:**
- `name()` - Handler name
- `version()` - Handler version  
- `supported_extensions()` - Supported file extensions
- `extract_metadata(file_path)` - Extract metadata
- `extract_outline(file_path)` - Extract outline/TOC
- `extract_pages(file_path)` - Extract pages with paragraphs
- `validate_file(file_path)` - Validate file
- `get_file_info(file_path)` - Get basic file info
- `generate_thumbnail(file_path, width, height)` - Generate thumbnail

**Optional Methods:**
- `can_handle(file_path)` - Check if handler supports file (defaults to extension check)
- `get_capabilities()` - Get handler capabilities (has default set)

## Validation Rules

### Page/Paragraph Numbering
- Pages are 1-indexed
- Paragraphs are 1-indexed within each page
- TOC entry pages are 1-indexed when provided

### Date Formats
- ISO 8601 format recommended for all dates
- Dates should include timezone information when possible

### File Paths
- All file paths should be absolute paths
- Use forward slashes for consistency across platforms

### Error Handling
- All async methods should return appropriate error information
- Warnings should be collected in extraction info

## SDK Implementations

### Rust SDK (`lbvr-plugin-sdk`)
- Located in `../lbvr-plugin-sdk/`
- Uses serde for JSON serialization
- Async trait implementation with anyhow error handling

### Go SDK (`lbvr-plugin-sdk-go`)
- Located in `../lbvr-plugin-sdk-go/` (to be created)
- Will implement the same schemas using Go structs and interfaces

## Version History

- **v1.0** - Initial schema specification
  - Consolidated metadata structure
  - Hierarchical outline support
  - Page-based content extraction
  - String-based capability system

## Usage

These schemas can be used for:
1. **Code Generation** - Generate language-specific data structures
2. **Validation** - Validate plugin outputs against schemas
3. **Documentation** - Reference specification for plugin developers
4. **Testing** - Create test fixtures that conform to schemas
5. **API Design** - Ensure consistency across different plugin implementations
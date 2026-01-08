//! Test tool to verify DocumentPages serialization works correctly

use fgnd_plugin_sdk::{DocumentPages, DocumentPage, ExtractionInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Testing DocumentPages serialization...");
    
    // Create test data
    let mut doc_pages = DocumentPages::new("test.txt", "text");
    doc_pages.document_title = Some("Test Document".to_string());
    doc_pages.extraction_info = ExtractionInfo::new("test-tool", "1.0.0");
    
    let mut page1 = DocumentPage::new(1);
    page1.add_paragraph(DocumentParagraph::new(1, "This is paragraph 1."));
    page1.add_paragraph(DocumentParagraph::new(2, "This is paragraph 2."));
    
    let mut page2 = DocumentPage::new(2);
    page2.add_paragraph(DocumentParagraph::new(1, "This is paragraph 1 of page 2."));
    
    doc_pages.add_page(page1);
    doc_pages.add_page(page2);
    
    // Test JSON serialization
    println!(" Serializing to JSON...");
    let json_output = serde_json::to_string_pretty(&doc_pages)?;
    println!("OK JSON serialization successful!");
    println!();
    println!("JSON output:");
    println!("{}", json_output);
    println!();
    
    // Test deserialization
    println!(" Testing deserialization...");
    let parsed_doc: DocumentPages = serde_json::from_str(&json_output)?;
    println!("OK Deserialization successful!");
    
    // Verify data integrity
    assert_eq!(parsed_doc.total_pages, 2);
    assert_eq!(parsed_doc.pages.len(), 2);
    assert_eq!(parsed_doc.document_title, Some("Test Document".to_string()));
    assert_eq!(parsed_doc.pages[0].paragraphs.len(), 2);
    assert_eq!(parsed_doc.pages[1].paragraphs.len(), 1);
    
    println!("OK Data integrity verified!");
    
    // Test Debug output (what's causing the problem)
    println!();
    println!("Debug output (this is what's causing the issue):");
    println!("{:?}", doc_pages);
    println!();
    
    println!(" All tests passed!");
    println!();
    println!("TIP The issue is likely a plugin using:");
    println!("   println!(\"{{:?}}\", document_pages);  // ERR Wrong - outputs Debug format");
    println!("   Instead of:");
    println!("   println!(\"{{}}\", serde_json::to_string(&document_pages)?);  // OK Correct - outputs JSON");
    
    Ok(())
}
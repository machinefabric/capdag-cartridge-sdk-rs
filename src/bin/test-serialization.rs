//! Test tool to verify DisboundPage array serialization works correctly

use machfab_plugin_sdk::DisboundPage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(" Testing Vec<DisboundPage> serialization...");

    // Create test data - now just an array of pages
    let pages: Vec<DisboundPage> = vec![
        DisboundPage::new_with_text(1, "This is paragraph 1.\nThis is paragraph 2."),
        DisboundPage::new_with_text(2, "This is paragraph 1 of page 2."),
    ];

    // Test JSON serialization
    tracing::info!(" Serializing to JSON...");
    let json_output = serde_json::to_string_pretty(&pages)?;
    tracing::info!("OK JSON serialization successful!");
    println!();  // TODO: convert to tracing
    tracing::info!("JSON output:");
    tracing::info!("{}", json_output);
    println!();  // TODO: convert to tracing

    // Test deserialization
    tracing::info!(" Testing deserialization...");
    let parsed_pages: Vec<DisboundPage> = serde_json::from_str(&json_output)?;
    tracing::info!("OK Deserialization successful!");

    // Verify data integrity
    assert_eq!(parsed_pages.len(), 2);
    assert_eq!(parsed_pages[0].order_index, 1);
    assert_eq!(parsed_pages[1].order_index, 2);
    assert!(!parsed_pages[0].text_content.is_empty());
    assert!(!parsed_pages[1].text_content.is_empty());

    tracing::info!("OK Data integrity verified!");

    // Test Debug output (what's causing the problem)
    println!();  // TODO: convert to tracing
    tracing::info!("Debug output (this is what's causing the issue):");
    tracing::info!("{:?}", pages);
    println!();  // TODO: convert to tracing

    tracing::info!(" All tests passed!");
    println!();  // TODO: convert to tracing
    tracing::info!("TIP The issue is likely a plugin using:");
    tracing::info!("   println!(\"{{:?}}\", pages);  // ERR Wrong - outputs Debug format");
    tracing::info!("   Instead of:");
    tracing::info!("   println!(\"{{}}\", serde_json::to_string(&pages)?);  // OK Correct - outputs JSON");

    Ok(())
}

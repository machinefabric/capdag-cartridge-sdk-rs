//! Test tool to verify DisboundPage array serialization works correctly

use macina_plugin_sdk::DisboundPage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Testing Vec<DisboundPage> serialization...");

    // Create test data - now just an array of pages
    let pages: Vec<DisboundPage> = vec![
        DisboundPage::new_with_text(1, "This is paragraph 1.\nThis is paragraph 2."),
        DisboundPage::new_with_text(2, "This is paragraph 1 of page 2."),
    ];

    // Test JSON serialization
    println!(" Serializing to JSON...");
    let json_output = serde_json::to_string_pretty(&pages)?;
    println!("OK JSON serialization successful!");
    println!();
    println!("JSON output:");
    println!("{}", json_output);
    println!();

    // Test deserialization
    println!(" Testing deserialization...");
    let parsed_pages: Vec<DisboundPage> = serde_json::from_str(&json_output)?;
    println!("OK Deserialization successful!");

    // Verify data integrity
    assert_eq!(parsed_pages.len(), 2);
    assert_eq!(parsed_pages[0].order_index, 1);
    assert_eq!(parsed_pages[1].order_index, 2);
    assert!(!parsed_pages[0].text_content.is_empty());
    assert!(!parsed_pages[1].text_content.is_empty());

    println!("OK Data integrity verified!");

    // Test Debug output (what's causing the problem)
    println!();
    println!("Debug output (this is what's causing the issue):");
    println!("{:?}", pages);
    println!();

    println!(" All tests passed!");
    println!();
    println!("TIP The issue is likely a plugin using:");
    println!("   println!(\"{{:?}}\", pages);  // ERR Wrong - outputs Debug format");
    println!("   Instead of:");
    println!("   println!(\"{{}}\", serde_json::to_string(&pages)?);  // OK Correct - outputs JSON");

    Ok(())
}

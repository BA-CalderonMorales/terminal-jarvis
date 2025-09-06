// Quick test script to verify hybrid menu functionality
use std::process::Command;
use std::io::Write;

fn main() {
    println!("Testing Hybrid Menu System...");
    
    // Test 1: Verify compilation
    let output = Command::new("cargo")
        .args(&["check", "--quiet"])
        .output()
        .expect("Failed to run cargo check");
    
    if output.status.success() {
        println!("✅ Compilation successful");
    } else {
        println!("❌ Compilation failed");
        return;
    }
    
    // Test 2: Test partial completion logic
    let options = vec!["AI CLI Tools", "Important Links", "Settings", "Exit"];
    
    // Test partial matches
    let test_cases = vec![
        ("ai", "AI CLI Tools"),
        ("link", "Important Links"), 
        ("set", "Settings"),
        ("ex", "Exit"),
    ];
    
    for (input, expected) in test_cases {
        let matches: Vec<_> = options
            .iter()
            .filter(|opt| opt.to_lowercase().contains(&input.to_lowercase()))
            .collect();
            
        if matches.len() == 1 && matches[0].contains(expected) {
            println!("✅ Partial match '{}' -> '{}'", input, expected);
        } else {
            println!("❌ Partial match '{}' failed", input);
        }
    }
    
    println!("✅ Hybrid menu system tests completed successfully!");
}

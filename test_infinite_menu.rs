// Test the infinite menu to check for double character input bug
use terminal_jarvis::cli_logic::cli_logic_infinite_menu::infinite_hybrid_menu_select;

#[tokio::main] 
async fn main() -> anyhow::Result<()> {
    println!("Testing infinite menu - type characters to test for double input bug");
    println!("Expected: Single character input should show only once");
    println!("Use arrows to test infinite cycling, tab for completion, enter to select");
    println!();
    
    let options = vec![
        "AI CLI Tools".to_string(),
        "Important Links".to_string(), 
        "Settings".to_string(),
        "Exit".to_string(),
    ];
    
    match infinite_hybrid_menu_select("Test Menu", options).await {
        Ok(selection) => {
            println!("Selected: {}", selection);
        }
        Err(e) => {
            println!("Cancelled or error: {}", e);
        }
    }
    
    Ok(())
}

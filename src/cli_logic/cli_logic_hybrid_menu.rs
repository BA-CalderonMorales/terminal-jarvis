use dialoguer::{Completion, Input, FuzzySelect, theme::ColorfulTheme};
use crate::tools::ToolManager;
use anyhow::Result;

/// Universal completion system for all menu options
pub struct UniversalCompletion {
    options: Vec<String>,
    context: String,
}

impl UniversalCompletion {
    pub fn new(options: Vec<String>, context: &str) -> Self {
        Self {
            options,
            context: context.to_string(),
        }
    }
}

impl Completion for UniversalCompletion {
    fn get(&self, input: &str) -> Option<String> {
        if input.is_empty() {
            return None;
        }

        let input_lower = input.to_lowercase();
        let matches: Vec<_> = self
            .options
            .iter()
            .filter(|option| option.to_lowercase().starts_with(&input_lower))
            .collect();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}

/// Hybrid menu system: Tab completion + Arrow navigation + Type-to-filter
pub async fn hybrid_menu_select(
    prompt: &str,
    options: Vec<String>,
    context: &str,
    allow_text_input: bool,
) -> Result<String> {
    if allow_text_input {
        // Show help message
        println!("🎯 Interactive Menu:");
        println!("  • Type and press Tab for completion");
        println!("  • Press Enter without typing for arrow navigation");
        println!("  • Available options: {}", options.join(", "));
        println!();

        let completion = UniversalCompletion::new(options.clone(), context);

        // First try text input with tab completion
        let text_input = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!("{} (Tab to complete, Enter for arrows)", prompt))
            .completion_with(&completion)
            .allow_empty(true)
            .interact_text()?;

        let input_trimmed = text_input.trim();

        if input_trimmed.is_empty() {
            // Fall back to arrow navigation
            return arrow_navigation_menu(prompt, options).await;
        } else {
            // Check for exact or partial matches and return the actual option
            return handle_text_input_with_selection(input_trimmed, options).await;
        }
    } else {
        // Direct arrow navigation
        return arrow_navigation_menu(prompt, options).await;
    }
}

/// Handle text input with fuzzy matching and return selected option
async fn handle_text_input_with_selection(input: &str, options: Vec<String>) -> Result<String> {
    let input_lower = input.to_lowercase();

    // Exact match
    if let Some(exact) = options.iter().find(|opt| opt.to_lowercase() == input_lower) {
        return Ok(exact.clone());
    }

    // Partial matches
    let partial_matches: Vec<_> = options
        .iter()
        .filter(|opt| opt.to_lowercase().contains(&input_lower))
        .collect();

    match partial_matches.len() {
        0 => {
            println!("❌ No matches found for: '{}'", input);
            println!("Available options: {}", options.join(", "));
            anyhow::bail!("No matches found");
        }
        1 => Ok(partial_matches[0].clone()),
        _ => {
            println!("🔍 Multiple matches found:");
            let filtered_options: Vec<String> = partial_matches.iter().map(|s| (*s).clone()).collect();
            arrow_navigation_menu("Choose from matches", filtered_options).await
        }
    }
}

/// Arrow navigation with fuzzy search
async fn arrow_navigation_menu(prompt: &str, options: Vec<String>) -> Result<String> {
    println!("🧭 Arrow Navigation Mode:");
    println!("  • Use ↑/↓ arrows to navigate (loops infinitely)");
    println!("  • Type to filter options in real-time");
    println!("  • Press Enter to select");
    println!();

    let selection = FuzzySelect::new()
        .with_prompt(prompt)
        .items(&options)
        .default(0)
        .interact()?;

    Ok(options[selection].clone())
}

/// Main menu options with tab completion
pub async fn main_menu_hybrid() -> Result<String> {
    let options = vec![
        "AI CLI Tools".to_string(),
        "Important Links".to_string(),
        "Settings".to_string(),
        "Exit".to_string(),
    ];

    hybrid_menu_select("Choose main menu option", options, "main_menu", true).await
}

/// AI tools menu with tab completion
pub async fn ai_tools_menu_hybrid() -> Result<String> {
    let tools = ToolManager::get_available_tools();
    let mut options: Vec<String> = tools.keys().map(|s| s.to_string()).collect();
    options.push("back".to_string());

    println!("🤖 Available AI Tools:");
    for (tool_name, tool_info) in tools.iter() {
        let status = if tool_info.is_installed { "INSTALLED" } else { "NOT INSTALLED" };
        println!("  • {} - {}", tool_name, status);
    }
    println!();

    hybrid_menu_select("Select AI tool", options, "ai_tools", true).await
}

/// Settings menu with tab completion
pub async fn settings_menu_hybrid() -> Result<String> {
    let options = vec![
        "Install Tools".to_string(),
        "Update Tools".to_string(),
        "List All Tools".to_string(),
        "Tool Information".to_string(),
        "Switch Theme".to_string(),
        "back".to_string(),
    ];

    hybrid_menu_select("Select settings option", options, "settings", true).await
}

/// Important links menu with tab completion
pub async fn important_links_menu_hybrid() -> Result<String> {
    let options = vec![
        "GitHub Repository".to_string(),
        "NPM Package".to_string(),
        "CHANGELOG.md".to_string(),
        "Cargo Package".to_string(),
        "Documentation".to_string(),
        "Homebrew Formula".to_string(),
        "back".to_string(),
    ];

    hybrid_menu_select("Select link to open", options, "links", true).await
}

/// Simple confirmation dialog (no completion needed)
pub async fn confirm_hybrid(prompt: &str) -> Result<bool> {
    use dialoguer::Confirm;
    
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()?)
}

/// Multi-select for tool installation (no completion needed)
pub async fn multiselect_tools_hybrid(prompt: &str, tools: Vec<String>) -> Result<Vec<usize>> {
    use dialoguer::MultiSelect;
    
    Ok(MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&tools)
        .interact()?)
}

use dialoguer::{Completion, Input, theme::ColorfulTheme};
use crate::tools::ToolManager;
use anyhow::Result;

/// Tool name completion for tab completion in tool selection
pub struct ToolCompletion {
    options: Vec<String>,
}

impl ToolCompletion {
    pub fn new() -> Self {
        let tools = ToolManager::get_available_tools();
        let mut options: Vec<String> = tools.keys().map(|s| s.to_string()).collect();
        options.push("back".to_string());
        options.push("exit".to_string());
        
        ToolCompletion { options }
    }
}

impl Completion for ToolCompletion {
    /// Tab completion for tool names - supports partial matching
    fn get(&self, input: &str) -> Option<String> {
        let matches: Vec<_> = self
            .options
            .iter()
            .filter(|option| option.starts_with(&input.to_lowercase()))
            .collect();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}

/// Interactive tool selection with tab completion
pub async fn select_tool_with_completion() -> Result<Option<String>> {
    let tools = ToolManager::get_available_tools();
    
    println!("Available AI tools:");
    for (tool_name, tool_info) in tools.iter() {
        let status = if tool_info.is_installed { "INSTALLED" } else { "NOT INSTALLED" };
        println!("  {} - {}", tool_name, status);
    }
    println!("  Type tool name (use Tab to complete) or 'back' to return:");
    println!();

    let completion = ToolCompletion::new();

    loop {
        let input = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Select tool")
            .completion_with(&completion)
            .interact_text()?;

        let input = input.trim().to_lowercase();

        if input == "back" || input == "exit" {
            return Ok(None);
        }

        if tools.contains_key(input.as_str()) {
            return Ok(Some(input));
        } else {
            println!("Tool '{}' not found. Available tools:", input);
            for tool_name in tools.keys() {
                print!("{} ", tool_name);
            }
            println!();
            println!("Try again or type 'back' to return:");
        }
    }
}

/// Interactive main menu selection with tab completion
pub async fn select_main_menu_with_completion() -> Result<Option<String>> {
    let options = vec![
        "tools".to_string(),
        "links".to_string(),
        "settings".to_string(),
        "exit".to_string(),
    ];

    let completion = MainMenuCompletion::new(options.clone());

    println!("Main menu options: tools, links, settings, exit");
    println!("Type option name (use Tab to complete):");
    println!();

    let input = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose option")
        .completion_with(&completion)
        .interact_text()?;

    let input = input.trim().to_lowercase();
    
    match input.as_str() {
        "tools" => Ok(Some("tools".to_string())),
        "links" => Ok(Some("links".to_string())),
        "settings" => Ok(Some("settings".to_string())),
        "exit" => Ok(None),
        _ => Ok(Some(input))
    }
}

/// Main menu completion for tab completion
pub struct MainMenuCompletion {
    options: Vec<String>,
}

impl MainMenuCompletion {
    pub fn new(options: Vec<String>) -> Self {
        MainMenuCompletion { options }
    }
}

impl Completion for MainMenuCompletion {
    /// Tab completion for main menu options
    fn get(&self, input: &str) -> Option<String> {
        let matches: Vec<_> = self
            .options
            .iter()
            .filter(|option| option.starts_with(&input.to_lowercase()))
            .collect();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}

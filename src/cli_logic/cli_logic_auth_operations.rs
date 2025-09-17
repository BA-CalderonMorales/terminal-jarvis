use anyhow::{anyhow, Result};
use inquire::{Confirm, Select, Text};
use std::collections::HashMap;

use crate::auth_manager::AuthManager;
use crate::theme::theme_global_config;
use crate::tools::{tools_config::get_tool_config_loader, ToolManager};

pub async fn handle_authentication_menu() -> Result<()> {
    let theme = theme_global_config::current_theme();
    loop {
        print!("\x1b[2J\x1b[H");
        println!("{}\n", theme.accent("Authentication Manager"));

        let options = vec![
            "Set credentials for a tool".to_string(),
            "Remove/Clear saved credentials".to_string(),
            "View current saved credentials".to_string(),
            "Back to Main Menu".to_string(),
        ];

        let selection = match Select::new("Choose an action:", options.clone())
            .with_render_config(crate::cli_logic::cli_logic_utilities::get_themed_render_config())
            .with_page_size(10)
            .prompt()
        {
            Ok(s) => s,
            Err(_) => return Ok(()),
        };

        match selection.as_str() {
            s if s.contains("Set credentials") => {
                if let Some(tool) = pick_tool_name().await? {
                    handle_auth_set(&tool).await?;
                } else {
                    continue;
                }
            }
            s if s.contains("Remove/Clear saved credentials") => {
                handle_auth_remove_menu().await?;
            }
            s if s.contains("View current saved") => {
                let loader = get_tool_config_loader();
                let all_tools = loader.get_tool_names();
                println!("\n{}", theme.secondary("Saved credentials:"));
                for tool in all_tools {
                    if let Ok(vars) = AuthManager::get_tool_credentials(&tool) {
                        if !vars.is_empty() {
                            println!("- {}:", tool);
                            for (k, _v) in vars.iter() {
                                println!("    {} = ***", k);
                            }
                        }
                    }
                }
                println!("\n{}", theme.accent("Press Enter to continue..."));
                let _ = std::io::stdin().read_line(&mut String::new());
            }
            _ => return Ok(()),
        }
    }
}

pub async fn handle_auth_help(tool: &str) -> Result<()> {
    let theme = theme_global_config::current_theme();
    let loader = get_tool_config_loader();
    let key = tool.to_string();
    if let Some(auth) = loader.get_auth_info(&key) {
        println!("\n{}", theme.secondary(&format!("Auth help for {}:", tool)));
        println!("  Required env vars: {}", auth.env_vars.join(", "));
        if !auth.setup_url.is_empty() {
            println!("  Setup URL: {}", auth.setup_url);
        }
        if let Some(instr) = &auth.auth_instructions {
            println!("  Instructions: {}", instr);
        }
        println!(
            "  Browser-based auth: {}",
            if auth.browser_auth { "yes" } else { "no" }
        );
    } else {
        println!("No auth info found for {}", tool);
    }
    Ok(())
}

pub async fn handle_auth_set(tool: &str) -> Result<()> {
    let theme = theme_global_config::current_theme();
    let loader = get_tool_config_loader();
    let key = tool.to_string();
    let auth = loader
        .get_auth_info(&key)
        .ok_or_else(|| anyhow!("Unknown tool '{}'", tool))?;

    println!("\n{}", theme.secondary(&format!("Set credentials for {}:", tool)));
    println!("  You can enter values for any of these env vars. Leave blank to skip.");

    let mut new_vars: HashMap<String, String> = HashMap::new();
    for var in &auth.env_vars {
        let existing = std::env::var(var).ok().or_else(|| {
            AuthManager::get_tool_credentials(&key)
                .ok()
                .and_then(|m| m.get(var).cloned())
        });
        let prompt = if existing.is_some() {
            format!("{} (current set, hit Enter to keep)", var)
        } else {
            var.to_string()
        };
        if let Ok(input) = Text::new(&prompt)
            .with_placeholder("leave blank to skip")
            .prompt()
        {
            let trimmed = input.trim().to_string();
            if !trimmed.is_empty() {
                new_vars.insert(var.clone(), trimmed);
            }
        }
    }

    if new_vars.is_empty() {
        println!("{}", theme.accent("No changes made."));
        return Ok(());
    }

    if Confirm::new("Save credentials to Terminal Jarvis config?")
        .with_default(true)
        .prompt()
        .unwrap_or(true)
    {
        AuthManager::save_tool_credentials(&key, &new_vars)?;
        // Also export to current process so immediate runs work
        for (k, v) in new_vars.iter() {
            std::env::set_var(k, v);
        }
        println!("{}", theme.accent("Credentials saved."));
    } else {
        println!("{}", theme.accent("Changes discarded."));
    }
    Ok(())
}

async fn pick_tool_name() -> Result<Option<String>> {
    let tools_map = ToolManager::get_available_tools();
    let mut options: Vec<String> = tools_map.keys().map(|s| s.to_string()).collect();
    options.sort();
    let prompt = Select::new("Select a tool:", options.clone())
        .with_render_config(crate::cli_logic::cli_logic_utilities::get_themed_render_config())
        .with_page_size(15)
        .prompt();
    match prompt {
        Ok(choice) => Ok(Some(choice)),
        Err(_) => Ok(None),
    }
}

async fn handle_auth_remove_menu() -> Result<()> {
    let theme = theme_global_config::current_theme();
    let options = vec![
        "Remove credentials for one tool".to_string(),
        "Remove credentials for multiple tools".to_string(),
        "Clear ALL credentials".to_string(),
        "Back".to_string(),
    ];
    let choice = Select::new("Choose a removal option:", options.clone())
        .with_render_config(crate::cli_logic::cli_logic_utilities::get_themed_render_config())
        .with_page_size(10)
        .prompt()?;

    match choice.as_str() {
        "Remove credentials for one tool" => {
            if let Some(tool) = pick_tool_name().await? {
                handle_remove_for_tool(&tool).await?;
            }
        }
        "Remove credentials for multiple tools" => {
            handle_remove_for_multiple_tools().await?;
        }
        "Clear ALL credentials" => {
            if Confirm::new("This will remove all saved credentials. Are you sure?")
                .with_default(false)
                .prompt()
                .unwrap_or(false)
            {
                AuthManager::clear_all_credentials()?;
                println!("{}", theme.accent("All credentials cleared."));
                println!("{}", theme.accent("Press Enter to continue..."));
                let _ = std::io::stdin().read_line(&mut String::new());
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_remove_for_tool(tool: &str) -> Result<()> {
    use inquire::MultiSelect;
    let theme = theme_global_config::current_theme();
    let existing = AuthManager::get_tool_credentials(tool)?;
    if existing.is_empty() {
        println!("{}", theme.accent("No saved credentials for this tool."));
        println!("{}", theme.accent("Press Enter to continue..."));
        let _ = std::io::stdin().read_line(&mut String::new());
        return Ok(());
    }
    let keys: Vec<String> = existing.keys().cloned().collect();
    let selections = MultiSelect::new("Select keys to remove (space to toggle):", keys.clone())
        .with_render_config(crate::cli_logic::cli_logic_utilities::get_themed_render_config())
        .with_page_size(10)
        .prompt()?;
    if selections.is_empty() {
        return Ok(());
    }
    let confirm_msg = if selections.len() == keys.len() {
        format!("Remove ALL credentials for {}?", tool)
    } else {
        format!("Remove {} selected key(s) for {}?", selections.len(), tool)
    };
    if Confirm::new(&confirm_msg).with_default(false).prompt().unwrap_or(false) {
        AuthManager::delete_tool_credentials(tool, &selections)?;
        println!("{}", theme.accent("Credentials removed."));
    }
    println!("{}", theme.accent("Press Enter to continue..."));
    let _ = std::io::stdin().read_line(&mut String::new());
    Ok(())
}

async fn handle_remove_for_multiple_tools() -> Result<()> {
    use inquire::MultiSelect;
    let theme = theme_global_config::current_theme();
    // Build a list of tools that currently have credentials
    let loader = get_tool_config_loader();
    let all_tools = loader.get_tool_names();
    let mut tools_with_creds: Vec<String> = vec![];
    for t in all_tools {
        if let Ok(m) = AuthManager::get_tool_credentials(&t) {
            if !m.is_empty() {
                tools_with_creds.push(t.to_string());
            }
        }
    }
    if tools_with_creds.is_empty() {
        println!("{}", theme.accent("No saved credentials found."));
        println!("{}", theme.accent("Press Enter to continue..."));
        let _ = std::io::stdin().read_line(&mut String::new());
        return Ok(());
    }
    let selected_tools = MultiSelect::new(
        "Select tools to remove credentials for:",
        tools_with_creds.clone(),
    )
    .with_render_config(crate::cli_logic::cli_logic_utilities::get_themed_render_config())
    .with_page_size(12)
    .prompt()?;
    if selected_tools.is_empty() {
        return Ok(());
    }
    if Confirm::new(&format!(
        "Remove ALL saved credentials for {} tool(s)?",
        selected_tools.len()
    ))
    .with_default(false)
    .prompt()
    .unwrap_or(false)
    {
        for t in selected_tools {
            AuthManager::delete_tool_credentials(&t, &[])?; // empty keys => remove tool entry
        }
        println!("{}", theme.accent("Selected tools' credentials removed."));
    }
    println!("{}", theme.accent("Press Enter to continue..."));
    let _ = std::io::stdin().read_line(&mut String::new());
    Ok(())
}

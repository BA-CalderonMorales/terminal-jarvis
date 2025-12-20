// CLI Logic Welcome Screen
// Displays ASCII art banner and system info on startup
// Each theme has a distinctly different visual style

use crate::cli_logic::cli_logic_first_run::get_tool_status_line;
use crate::theme::theme_global_config;
use std::env;

/// Display the welcome screen with theme-specific ASCII art banner
pub fn display_welcome_screen() {
    let theme = theme_global_config::current_theme();
    let version = env!("CARGO_PKG_VERSION");

    // Get current working directory
    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| String::from("unknown"));

    // Get tool status for display
    let tool_status = get_tool_status_line();

    println!();

    // Theme-specific welcome screens
    match theme.name {
        "Minimal" => {
            // Classic/Minimal: Ultra-clean, no box, just text
            println!("{}", theme.primary("Terminal Jarvis"));
            println!("{}", theme.secondary(&format!("v{}", version)));
            println!("{}", theme.secondary(&tool_status));
            println!("{}", theme.secondary(&cwd));
            println!("{}", theme.accent(":: /help for commands"));
        }
        "Terminal" => {
            // Matrix/Terminal: Hacker aesthetic with ASCII box
            println!("{}", theme.primary("+-------[ TERMINAL JARVIS ]-------+"));
            println!(
                "{}",
                theme.primary(&format!("|  VERSION: {}               |", version))
            );
            println!(
                "{}",
                theme.secondary(&format!(
                    "|  STATUS: {}  |",
                    truncate_status(&tool_status, 20)
                ))
            );
            println!(
                "{}",
                theme.secondary(&format!("|  PATH: {}  |", truncate_path(&cwd, 22)))
            );
            println!("{}", theme.accent("+---------------------------------+"));
            println!("{}", theme.accent("$ Type /help for command list"));
        }
        _ => {
            // Default/TJarvis: Modern with Unicode box
            println!("{}", theme.primary("   ┌─────┐  Terminal Jarvis"));
            println!("{}", theme.primary(&format!("   │ T.J │  v{}", version)));
            println!(
                "{}",
                theme.secondary(&format!("   │ ═ ═ │  {}", tool_status))
            );
            println!("{}", theme.secondary(&format!("   │     │  {}", cwd)));
            println!(
                "{}",
                theme.accent("   └─────┘  Type /help to see available commands")
            );
        }
    }

    println!();
}

/// Truncate tool status for terminal theme box
fn truncate_status(status: &str, max_len: usize) -> String {
    if status.len() <= max_len {
        format!("{:width$}", status, width = max_len)
    } else {
        format!("{}...", &status[..max_len - 3])
    }
}

/// Truncate path for terminal theme box
fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        format!("{:width$}", path, width = max_len)
    } else {
        // Show end of path (most relevant)
        let truncated = &path[path.len() - (max_len - 3)..];
        format!("...{}", truncated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_welcome_screen_does_not_panic() {
        // This test ensures the welcome screen can be displayed without panicking
        display_welcome_screen();
    }

    #[test]
    fn test_truncate_status_short() {
        let result = truncate_status("hello", 10);
        assert_eq!(result.len(), 10);
        assert!(result.starts_with("hello"));
    }

    #[test]
    fn test_truncate_status_long() {
        let result = truncate_status("this is a very long status", 10);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 10);
    }

    #[test]
    fn test_truncate_path_short() {
        let result = truncate_path("/home/user", 20);
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn test_truncate_path_long() {
        let result = truncate_path("/very/long/path/to/some/directory", 15);
        assert!(result.starts_with("..."));
    }

    #[test]
    fn test_version_formatting() {
        let version = env!("CARGO_PKG_VERSION");
        assert!(!version.is_empty());
        assert!(version.contains('.'));
    }
}

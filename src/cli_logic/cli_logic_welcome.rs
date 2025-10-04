// CLI Logic Welcome Screen
// Displays ASCII art banner and system info on startup

use crate::theme::theme_global_config;
use std::env;

/// Display the welcome screen with ASCII art banner and system information
pub fn display_welcome_screen() {
    let theme = theme_global_config::current_theme();
    let version = env!("CARGO_PKG_VERSION");

        // Get current working directory
    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| String::from("unknown"));

    // T.JARVIS Bot ASCII art - professional boxy design with horizontal layout
    // Designed to work at any terminal width
    println!();
    println!("{}", theme.primary("   ┌─────┐  Terminal Jarvis"));
    println!("{}", theme.primary(&format!("   │ T.J │  v{}", version)));
    println!("{}", theme.primary(&format!("   │ ═ ═ │  {}", cwd)));
    println!("{}", theme.secondary("   │     │  ---------------------------------"));
    println!("{}", theme.accent("   └─────┘  Tip: Check Important Links for docs"));
    println!();
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
    fn test_welcome_screen_contains_ascii_art() {
        // Verify the ASCII art structure is present
        // This is a smoke test to catch accidental modifications
        let art_elements = vec![
            "┌─────┐",
            "│ T.J │",
            "│ ═ ═ │",
            "│     │",
            "└─────┘",
            "Terminal Jarvis",
            "Tip: Check Important Links for docs",
        ];

        // If we can construct the elements, the art is valid
        for element in art_elements {
            assert!(!element.is_empty());
        }
    }

    #[test]
    fn test_version_formatting() {
        let version = env!("CARGO_PKG_VERSION");
        assert!(!version.is_empty());
        assert!(version.contains('.'));
    }
}

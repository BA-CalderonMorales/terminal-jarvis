use crate::cli_logic::cli_logic_utilities::get_themed_render_config;
use crate::config::ConfigManager;
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::ProgressContext;
use crate::services::PackageService;
use crate::theme_config;
use anyhow::Result;
use inquire::Select;

/// Handle the main interactive mode interface
pub async fn handle_interactive_mode() -> Result<()> {
    // Initialize theme configuration
    let _ = theme_config::initialize_theme_config();

    // Check NPM availability upfront
    let npm_available = InstallationManager::check_npm_available();

    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_config::current_theme();
        // Clear screen first
        print!("\x1b[2J\x1b[H"); // Clear screen

        display_welcome_interface(&theme, npm_available).await?;

        // Main menu options - clean styling without redundant indicators
        let options = vec![
            "AI CLI Tools".to_string(),
            "Important Links".to_string(),
            "Settings".to_string(),
            "Exit".to_string(),
        ];

        let selection = match Select::new("Choose an option:", options.clone())
            .with_render_config(get_themed_render_config())
            .with_page_size(10)
            .prompt()
        {
            Ok(selection) => selection,
            Err(_) => {
                // User interrupted (Ctrl+C) - show clean exit message
                println!();
                println!("Goodbye!");
                return Ok(());
            }
        };

        // Handle selection
        match selection.as_str() {
            s if s.contains("AI CLI Tools") => {
                handle_ai_tools_menu().await?;
            }
            s if s.contains("Important Links") => {
                handle_important_links().await?;
            }
            s if s.contains("Settings") => {
                handle_manage_tools_menu().await?;
            }
            s if s.contains("Exit") => {
                print!("{}", theme.reset()); // Reset all formatting
                print!("\x1b[2J\x1b[H"); // Clear screen
                println!("Goodbye!");
                break;
            }
            _ => continue,
        }
    }
    // Ensure terminal is reset when function exits
    print!("\x1b[0m"); // Reset all formatting
    print!("\x1b[2J\x1b[H"); // Clear screen
    Ok(())
}

/// Display the welcome interface with T.JARVIS branding
async fn display_welcome_interface(theme: &crate::theme::Theme, npm_available: bool) -> Result<()> {
    // Get terminal width for responsive design
    let term_width = if let Some((w, _)) = term_size::dimensions() {
        w
    } else {
        80 // fallback width
    };

    // Professional design with blue background in content areas - no initial line break
    // Use most of the terminal width for a more immersive experience
    let border_width = if term_width > 20 {
        term_width.saturating_sub(4) // Leave 2 chars padding on each side
    } else {
        term_width.saturating_sub(2) // Minimum border for very narrow terminals
    };
    let border_padding = format!("{}  {}", "\x1b[0m", "\x1b[0m"); // Explicit reset for padding spaces

    // Professional color scheme using T.JARVIS theme
    let top_border = format!(
        "{}╔{}╗{}",
        theme.colors.border,
        "═".repeat(border_width.saturating_sub(2)),
        theme.reset()
    );
    let bottom_border = format!(
        "{}╚{}╝{}",
        theme.colors.border,
        "═".repeat(border_width.saturating_sub(2)),
        theme.reset()
    );

    println!("{border_padding}{top_border}");

    let inner_width = border_width.saturating_sub(2); // Account for left and right border chars only

    // Helper function to print a line with proper border and background
    let print_border_line = |content: &str| {
        let full_line = theme.background_line_with_content(content, inner_width);
        println!(
            "{}{}║{}║{}",
            border_padding, theme.colors.border, full_line, theme.colors.border,
        );
    };

    display_ascii_logo(&print_border_line, theme, inner_width);
    display_version_info(&print_border_line, theme, inner_width).await;

    println!("{border_padding}{bottom_border}");
    println!();

    // Minimal setup warning
    if !npm_available {
        println!("Node.js required → https://nodejs.org/");
        println!();
    }

    Ok(())
}

/// Display the T.JARVIS ASCII logo
fn display_ascii_logo<F>(print_border_line: &F, theme: &crate::theme::Theme, inner_width: usize)
where
    F: Fn(&str),
{
    // T.JARVIS ASCII art - professional and clean
    let logo_lines = vec![
        "$$$$$$$$\\           $$$$$\\  $$$$$$\\  $$$$$$$\\  $$\\    $$\\ $$$$$$\\  $$$$$$\\  ",
        "\\__$$  __|          \\__$$ |$$  __$$\\ $$  __$$\\ $$ |   $$ |\\_$$  _|$$  __$$\\ ",
        "   $$ |                $$ |$$ /  $$ |$$ |  $$ |$$ |   $$ |  $$ |  $$ /  \\__|",
        "   $$ |                $$ |$$$$$$$$ |$$$$$$$  |\\$$\\  $$  /  $$ |  \\$$$$$$\\  ",
        "   $$ |          $$\\   $$ |$$  __$$ |$$  __$$<  \\$$\\$$  /   $$ |   \\____$$\\ ",
        "   $$ |          $$ |  $$ |$$ |  $$ |$$ |  $$ |  \\$$$  /    $$ |  $$\\   $$ |",
        "   $$ |$$\\       \\$$$$$$  |$$ |  $$ |$$ |  $$ |   \\$  /   $$$$$$\\ \\$$$$$$  |",
        "   \\__|\\__|       \\______/ \\__|  \\__|\\__|  \\__|    \\_/    \\______| \\______/ ",
    ];

    // Add empty line before logo
    print_border_line("");

    for line in logo_lines {
        let line_chars: Vec<char> = line.chars().collect();
        let line_char_len = line_chars.len();

        if line_char_len <= inner_width {
            // Create content with logo color formatting
            let content = format!("{}{}", theme.logo_no_reset(""), line);
            print_border_line(&content);
        } else {
            let simple_line = "T.JARVIS";
            // Create content with logo color formatting
            let content = format!("{}{}", theme.logo_no_reset(""), simple_line);
            print_border_line(&content);
        }
    }

    // Add empty line after logo
    print_border_line("");

    // Add elegant separator line with theme colors
    let separator_content =
        theme.background_line_with_content(&"─".repeat(inner_width), inner_width);
    print_border_line(&separator_content);
}

/// Display version and tagline information
async fn display_version_info<F>(
    print_border_line: &F,
    theme: &crate::theme::Theme,
    inner_width: usize,
) where
    F: Fn(&str),
{
    // Version and tagline in futuristic style - with NPM distribution tag if available
    let base_version = env!("CARGO_PKG_VERSION");

    // Initialize config manager for caching
    let config_manager = ConfigManager::new().unwrap_or_else(|_| {
        // Fallback if config manager fails - continue without caching
        ConfigManager::new().expect("Failed to create config manager")
    });

    // Show progress for NPM tag detection with caching
    let npm_progress = ProgressContext::new("Checking NPM distribution tags");
    let npm_tag = PackageService::get_cached_npm_dist_tag_info(&config_manager)
        .await
        .unwrap_or(None);
    npm_progress.finish_success("NPM tag info loaded");

    let version_text = if let Some(tag) = npm_tag {
        format!("v{base_version} (@{tag})")
    } else {
        format!("v{base_version}")
    };

    if version_text.len() <= inner_width {
        // Use background_line_with_content for proper background fill
        let content = format!("{}{}", theme.secondary_no_reset(""), version_text);
        print_border_line(&content);
    }

    let tagline = "AI Coding Assistant Command Center";
    // Calculate visual width accounting for emoji (which takes 2 columns)
    let tagline_visual_width = tagline
        .chars()
        .map(|c| {
            if c as u32 >= 0x1F300 {
                2
            } else {
                1
            } // Emoji range approximation
        })
        .sum::<usize>();

    if tagline_visual_width <= inner_width {
        // Use background_line_with_content for proper background fill
        let content = format!("{}{}", theme.secondary_no_reset(""), tagline);
        print_border_line(&content);
    }

    // Add another elegant separator
    let separator_content2 =
        theme.background_line_with_content(&"─".repeat(inner_width), inner_width);
    print_border_line(&separator_content2);

    // Short hint about Important Links - shortened to fit border
    let links_hint = "See 'Important Links' menu";
    // Calculate visual width accounting for emoji (which takes 2 columns)
    let links_visual_width = links_hint
        .chars()
        .map(|c| {
            if c as u32 >= 0x1F300 {
                2
            } else {
                1
            } // Emoji range approximation
        })
        .sum::<usize>();

    if links_visual_width <= inner_width {
        // Use background_line_with_content for proper background fill
        let content = format!("{}{}", theme.secondary_no_reset(""), links_hint);
        print_border_line(&content);
    }

    // Add empty line after links hint to match top padding
    print_border_line("");
}

// Forward declarations for menu functions that will be in other modules
async fn handle_ai_tools_menu() -> Result<()> {
    // This will be implemented in a separate function
    crate::cli_logic::handle_ai_tools_menu().await
}

async fn handle_important_links() -> Result<()> {
    // This will be implemented in a separate function
    crate::cli_logic::handle_important_links().await
}

async fn handle_manage_tools_menu() -> Result<()> {
    // This will be implemented in a separate function
    crate::cli_logic::handle_manage_tools_menu().await
}

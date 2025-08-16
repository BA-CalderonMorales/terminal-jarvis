use crate::services::GitHubService;
use crate::theme_config;
use anyhow::Result;

/// Handle initializing a new template repository
pub async fn handle_templates_init() -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_config::current_theme();

    println!("{}", theme.primary("Initializing template repository..."));
    println!("This requires gh CLI and will create a new GitHub repository for your templates.");

    github_service.init_template_repository().await
}

/// Handle creating a new template
pub async fn handle_templates_create(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_config::current_theme();

    println!("{}", theme.primary(&format!("Creating template: {}", name)));
    github_service.create_template(name).await
}

/// Handle listing all available templates
pub async fn handle_templates_list() -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_config::current_theme();

    println!("{}", theme.primary("Available templates:"));
    let templates = github_service.list_templates().await?;

    display_templates_list(&templates);

    Ok(())
}

/// Handle applying a specific template
pub async fn handle_templates_apply(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_config::current_theme();

    println!("{}", theme.primary(&format!("Applying template: {}", name)));
    github_service.apply_template(name).await
}

/// Display the templates list in a formatted way
fn display_templates_list(templates: &[String]) {
    let theme = theme_config::current_theme();

    if templates.is_empty() {
        println!("{}", theme.secondary(" No templates found."));
        println!("  Use 'terminal-jarvis templates create <name>' to create a template.");
    } else {
        for template in templates {
            println!(" {} {}", theme.accent("•"), template);
        }
        println!();
        println!("{}", theme.secondary(&format!("Found {} template(s)", templates.len())));
    }
}

/// Display template management help and guidance
pub async fn display_template_help() -> Result<()> {
    let theme = theme_config::current_theme();

    println!("{}", theme.primary("┌─ Template Management ──────────────────────────────────────┐"));
    println!("{}", theme.primary("│                                                             │"));
    println!("│ {:<59} │", theme.secondary("Templates help you quickly set up new projects with"));
    println!("│ {:<59} │", theme.secondary("predefined configurations and boilerplate code."));
    println!("{}", theme.primary("│                                                             │"));
    println!("│ {:<59} │", theme.accent("Available Commands:"));
    println!("│   {:<57} │", theme.secondary("init    - Initialize template repository"));
    println!("│   {:<57} │", theme.secondary("create  - Create a new template"));
    println!("│   {:<57} │", theme.secondary("list    - List available templates"));
    println!("│   {:<57} │", theme.secondary("apply   - Apply a template to current directory"));
    println!("{}", theme.primary("│                                                             │"));
    println!("│ {:<59} │", theme.accent("Requirements:"));
    println!("│   {:<57} │", theme.secondary("• GitHub CLI (gh) must be installed"));
    println!("│   {:<57} │", theme.secondary("• GitHub account with repository access"));
    println!("{}", theme.primary("│                                                             │"));
    println!("{}", theme.primary("└─────────────────────────────────────────────────────────────┘"));

    Ok(())
}

/// Check template system prerequisites
pub async fn check_template_prerequisites() -> Result<bool> {
    use tokio::process::Command;

    // Check if GitHub CLI is installed
    let gh_check = Command::new("gh").arg("--version").output().await;

    match gh_check {
        Ok(output) if output.status.success() => {
            let theme = theme_config::current_theme();
            println!("{}", theme.primary("✓ GitHub CLI is available"));
            Ok(true)
        }
        _ => {
            let theme = theme_config::current_theme();
            println!("{}", theme.accent("✗ GitHub CLI (gh) is required but not installed"));
            println!("  Install from: https://cli.github.com/");
            Ok(false)
        }
    }
}

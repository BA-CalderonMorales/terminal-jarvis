use crate::services::GitHubService;
use crate::theme::theme_global_config;
use anyhow::Result;

/// Handle initializing a new template repository
pub async fn handle_templates_init() -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_global_config::current_theme();

    println!("{}", theme.primary("Initializing template repository..."));
    println!("This requires gh CLI and will create a new GitHub repository for your templates.");

    github_service.init_template_repository().await
}

/// Handle creating a new template
pub async fn handle_templates_create(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_global_config::current_theme();

    println!("{}", theme.primary(&format!("Creating template: {}", name)));
    github_service.create_template(name).await
}

/// Handle listing all available templates
pub async fn handle_templates_list() -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_global_config::current_theme();

    println!("{}", theme.primary("Available templates:"));
    let templates = github_service.list_templates().await?;

    display_templates_list(&templates);

    Ok(())
}

/// Handle applying a specific template
pub async fn handle_templates_apply(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;
    let theme = theme_global_config::current_theme();

    println!("{}", theme.primary(&format!("Applying template: {}", name)));
    github_service.apply_template(name).await
}

/// Display the templates list in a formatted way
fn display_templates_list(templates: &[String]) {
    let theme = theme_global_config::current_theme();

    if templates.is_empty() {
        println!("{}", theme.secondary(" No templates found."));
        println!("  Use 'terminal-jarvis templates create <name>' to create a template.");
    } else {
        for template in templates {
            println!(" {} {}", theme.accent("•"), template);
        }
        println!();
        println!(
            "{}",
            theme.secondary(&format!("Found {} template(s)", templates.len()))
        );
    }
}

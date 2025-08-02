use crate::api::ToolApi;
use crate::services::{GitHubService, PackageService};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Supported AI coding tools
const SUPPORTED_TOOLS: &[&str] = &["claude-code", "gemini-cli", "qwen-code", "opencode"];

pub async fn handle_run_tool(tool: &str, args: &[String]) -> Result<()> {
    if !SUPPORTED_TOOLS.contains(&tool) {
        return Err(anyhow!(
            "Unsupported tool: {}. Supported tools: {:?}",
            tool,
            SUPPORTED_TOOLS
        ));
    }

    println!("Running {} with args: {:?}", tool, args);

    // Check if tool is installed
    let package_service = PackageService::new()?;
    if !package_service.is_tool_installed(tool).await? {
        println!("Tool {} is not installed. Installing...", tool);
        package_service.install_tool(tool).await?;
    }

    // Execute the tool
    package_service.run_tool(tool, args).await
}

pub async fn handle_update_packages(package: Option<&str>) -> Result<()> {
    let package_service = PackageService::new()?;

    match package {
        Some(pkg) => {
            if !SUPPORTED_TOOLS.contains(&pkg) {
                return Err(anyhow!(
                    "Unsupported package: {}. Supported packages: {:?}",
                    pkg,
                    SUPPORTED_TOOLS
                ));
            }
            println!("Updating package: {}", pkg);
            package_service.update_tool(pkg).await
        }
        None => {
            println!("Updating all packages...");
            for tool in SUPPORTED_TOOLS {
                println!("Updating {}...", tool);
                if let Err(e) = package_service.update_tool(tool).await {
                    eprintln!("Failed to update {}: {}", tool, e);
                }
            }
            Ok(())
        }
    }
}

pub async fn handle_list_tools() -> Result<()> {
    println!("Available AI coding tools:");
    println!();

    let package_service = PackageService::new()?;
    let tool_api = ToolApi::new();

    for tool in SUPPORTED_TOOLS {
        let is_installed = package_service
            .is_tool_installed(tool)
            .await
            .unwrap_or(false);
        let status = if is_installed {
            "✅ Installed"
        } else {
            "❌ Not installed"
        };
        let description = tool_api
            .get_tool_description(tool)
            .await
            .unwrap_or_else(|_| "No description available".to_string());

        println!("  {} - {} ({})", tool, description, status);
    }

    Ok(())
}

pub async fn handle_tool_info(tool: &str) -> Result<()> {
    if !SUPPORTED_TOOLS.contains(&tool) {
        return Err(anyhow!(
            "Unsupported tool: {}. Supported tools: {:?}",
            tool,
            SUPPORTED_TOOLS
        ));
    }

    let package_service = PackageService::new()?;
    let tool_api = ToolApi::new();

    let is_installed = package_service.is_tool_installed(tool).await?;
    let description = tool_api.get_tool_description(tool).await?;
    let version = if is_installed {
        package_service
            .get_tool_version(tool)
            .await
            .unwrap_or_else(|_| "Unknown".to_string())
    } else {
        "Not installed".to_string()
    };

    println!("Tool Information: {}", tool);
    println!("Description: {}", description);
    println!("Version: {}", version);
    println!(
        "Status: {}",
        if is_installed {
            "Installed"
        } else {
            "Not installed"
        }
    );

    Ok(())
}

pub async fn handle_templates_init() -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Initializing template repository...");
    println!("This requires gh CLI and will create a new GitHub repository for your templates.");

    github_service.init_template_repository().await
}

pub async fn handle_templates_create(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Creating template: {}", name);
    github_service.create_template(name).await
}

pub async fn handle_templates_list() -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Available templates:");
    let templates = github_service.list_templates().await?;

    if templates.is_empty() {
        println!("  No templates found. Use 'terminal-jarvis templates create <name>' to create a template.");
    } else {
        for template in templates {
            println!("  - {}", template);
        }
    }

    Ok(())
}

pub async fn handle_templates_apply(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Applying template: {}", name);
    github_service.apply_template(name).await
}

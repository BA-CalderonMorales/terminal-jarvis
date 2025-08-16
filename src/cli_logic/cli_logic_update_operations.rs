use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::services::PackageService;
use anyhow::Result;

/// Handle updating packages - either a specific package or all packages
pub async fn handle_update_packages(package: Option<&str>) -> Result<()> {
    let package_service = PackageService::new()?;

    match package {
        Some(pkg) => {
            update_single_package(&package_service, pkg).await
        }
        None => {
            update_all_packages(&package_service).await
        }
    }
}

/// Update a specific package with progress tracking
async fn update_single_package(package_service: &PackageService, pkg: &str) -> Result<()> {
    let update_progress = ProgressContext::new(&format!("Updating {pkg}"));

    // Add a small delay to show the progress indicator
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    update_progress.update_message(&format!("Downloading latest version of {pkg}..."));

    let result = package_service.update_tool(pkg).await;

    match result {
        Ok(_) => {
            update_progress.finish_success(&format!("{pkg} updated successfully"));
            Ok(())
        }
        Err(e) => {
            update_progress.finish_error(&format!("Failed to update {pkg}"));
            Err(e)
        }
    }
}

/// Update all packages with progress tracking and error handling
async fn update_all_packages(package_service: &PackageService) -> Result<()> {
    let overall_progress = ProgressContext::new("Updating all packages");
    let tools = InstallationManager::get_tool_names();
    let mut had_errors = false;

    for (index, tool) in tools.iter().enumerate() {
        overall_progress.update_message(&format!(
            "Updating {tool} ({}/{})...",
            index + 1,
            tools.len()
        ));

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        if let Err(e) = package_service.update_tool(tool).await {
            ProgressUtils::error_message(&format!("Failed to update {tool}: {e}"));
            had_errors = true;
        } else {
            ProgressUtils::success_message(&format!("{tool} updated successfully"));
        }
    }

    if had_errors {
        overall_progress.finish_error("Some packages failed to update");
    } else {
        overall_progress.finish_success("All packages updated successfully");
    }

    Ok(())
}

/// Display update status and recommendations  
pub fn show_update_recommendations() {
    let theme = crate::theme_config::current_theme();
    
    println!("{}", theme.secondary("Update Recommendations:"));
    println!("  • Run updates regularly to get the latest features and bug fixes");
    println!("  • Individual tool updates are faster than bulk updates");
    println!("  • Check tool documentation for breaking changes between versions");
    println!();
}

/// Check if updates are available for installed tools
pub async fn check_available_updates() -> Result<Vec<String>> {
    let package_service = PackageService::new()?;
    let tools = InstallationManager::get_tool_names();
    let tools_with_updates = Vec::new();
    
    for tool in tools {
        // This is a placeholder - in a real implementation, you'd check
        // the current version vs latest available version
        if let Ok(_) = package_service.is_tool_installed(tool).await {
            // For now, assume updates might be available
            // In practice, you'd call something like:
            // let current_version = package_service.get_tool_version(tool).await?;
            // let latest_version = package_service.get_latest_version(tool).await?;
            // if current_version != latest_version { ... }
        }
    }
    
    Ok(tools_with_updates)
}

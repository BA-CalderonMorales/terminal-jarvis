use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::services::PackageService;
use anyhow::Result;

/// Handle updating packages - either a specific package or all packages
pub async fn handle_update_packages(package: Option<&str>) -> Result<()> {
    let package_service = PackageService::new()?;

    match package {
        Some(pkg) => update_single_package(&package_service, pkg).await,
        None => update_all_packages(&package_service).await,
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

    // Only update installed tools, not all available tools
    let tools = crate::tools::ToolManager::get_installed_tools();
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

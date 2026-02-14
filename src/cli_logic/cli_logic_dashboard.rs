// Dashboard CLI - Flow state dashboard rendering and command handling
//
// Renders the tool health dashboard in both interactive (slash command)
// and non-interactive (CLI subcommand) modes.

use crate::progress_utils::ProgressContext;
use crate::theme::theme_global_config;
use crate::tools::tools_dashboard_scanner::DashboardScanner;
use crate::tools::tools_flow_state::{DashboardState, FlowState};
use anyhow::Result;

/// Handle the /dashboard or /status slash command (interactive mode)
pub async fn handle_dashboard() -> Result<()> {
    let theme = theme_global_config::current_theme();

    print!("\x1b[2J\x1b[H"); // Clear screen
    println!("{}\n", theme.primary("Terminal Jarvis Health Dashboard"));

    let progress = ProgressContext::new("Scanning tool health");
    let state = DashboardScanner::scan_all().await;
    progress.finish_success(&format!(
        "Scanned {} tools in {:.1}s",
        state.tools.len(),
        state.scan_duration.as_secs_f64()
    ));
    println!();

    render_dashboard(&state, &theme);

    println!("\n{}", theme.secondary("Press Enter to continue..."));
    let _ = std::io::stdin().read_line(&mut String::new());
    Ok(())
}

/// Handle `terminal-jarvis status` CLI command (non-interactive)
pub async fn handle_status_command() -> Result<()> {
    handle_dashboard().await
}

fn render_dashboard(state: &DashboardState, theme: &crate::theme::Theme) {
    // Summary line
    println!(
        "{}",
        theme.accent(&format!(
            "Summary: {} tools | {} FLOWING | {} NEEDS_WORK | {} BLOCKED | {} UNKNOWN",
            state.summary.total,
            state.summary.flowing,
            state.summary.needs_work,
            state.summary.blocked,
            state.summary.unknown,
        ))
    );
    println!();

    // Table header
    println!(
        "{}",
        theme.secondary(&format!(
            "{:<16} {:<8} {:<8} {:<8} {:<8} {}",
            "TOOL", "PREREQ", "INSTALL", "AUTH", "RUNTIME", "STATE"
        ))
    );
    println!("{}", theme.secondary(&"-".repeat(64)));

    // Table rows
    for tool in &state.tools {
        let line = format!(
            "{:<16} {:<8} {:<8} {:<8} {:<8} {}",
            tool.tool_name,
            tool.prerequisites.indicator(),
            tool.installation.indicator(),
            tool.authentication.indicator(),
            tool.runtime.indicator(),
            tool.overall.label(),
        );

        // Color the line based on overall state
        let colored = match tool.overall {
            FlowState::Flowing => theme.accent(&line),
            FlowState::NeedsWork => theme.secondary(&line),
            FlowState::Blocked => theme.primary(&line),
            FlowState::Unknown => theme.secondary(&line),
        };
        println!("{}", colored);
    }

    // Blockers section
    let all_blockers: Vec<_> = state
        .tools
        .iter()
        .filter(|t| !t.blockers.is_empty())
        .collect();
    if !all_blockers.is_empty() {
        println!("\n{}", theme.primary("BLOCKERS:"));
        for tool in all_blockers {
            for blocker in &tool.blockers {
                println!("  {}: {}", tool.tool_name, blocker);
            }
        }
    }

    // Suggestions section
    let all_suggestions: Vec<_> = state
        .tools
        .iter()
        .filter(|t| !t.suggestions.is_empty())
        .collect();
    if !all_suggestions.is_empty() {
        println!("\n{}", theme.accent("SUGGESTIONS:"));
        for tool in all_suggestions {
            for suggestion in &tool.suggestions {
                println!("  {}: {}", tool.tool_name, suggestion);
            }
        }
    }
}

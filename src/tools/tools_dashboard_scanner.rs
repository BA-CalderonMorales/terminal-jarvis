// Dashboard Scanner - Parallel tool health scanning
//
// Scans all configured tools using ToolPreflight and builds
// a DashboardState with sorted results and summary counts.

use super::tools_config::get_tool_config_loader;
use super::tools_flow_state::{DashboardState, DashboardSummary, FlowState, ToolFlowState};
use super::tools_preflight::ToolPreflight;

pub struct DashboardScanner;

impl DashboardScanner {
    /// Scan all configured tools in parallel.
    pub async fn scan_all() -> DashboardState {
        let start = std::time::Instant::now();
        let loader = get_tool_config_loader();
        let tool_names = loader.get_tool_names();

        // Spawn blocking tasks for each tool check (they do process spawning)
        let mut handles = Vec::new();
        for name in tool_names {
            handles.push(tokio::task::spawn_blocking(move || {
                let preflight = ToolPreflight::check(&name);
                ToolFlowState::from_preflight(&preflight)
            }));
        }

        let mut tools = Vec::new();
        for handle in handles {
            if let Ok(state) = handle.await {
                tools.push(state);
            }
        }

        // Sort by flow state priority (Blocked first, then NeedsWork, then Flowing)
        tools.sort_by(|a, b| {
            let priority = |s: &FlowState| match s {
                FlowState::Blocked => 0,
                FlowState::NeedsWork => 1,
                FlowState::Unknown => 2,
                FlowState::Flowing => 3,
            };
            priority(&a.overall)
                .cmp(&priority(&b.overall))
                .then(a.tool_name.cmp(&b.tool_name))
        });

        let summary = DashboardSummary::from_tools(&tools);
        DashboardState {
            tools,
            summary,
            scan_duration: start.elapsed(),
        }
    }

    /// Scan a single tool by name.
    #[allow(dead_code)]
    pub fn scan_tool(tool_name: &str) -> ToolFlowState {
        let preflight = ToolPreflight::check(tool_name);
        ToolFlowState::from_preflight(&preflight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_single_unknown_tool() {
        let state = DashboardScanner::scan_tool("nonexistent_tool_xyz_42");
        // Unknown tool: pm is ok (unknown pm defaults to ok), not installed
        assert_eq!(state.tool_name, "nonexistent_tool_xyz_42");
        assert_ne!(state.overall, FlowState::Flowing);
    }

    #[tokio::test]
    async fn test_scan_all_returns_dashboard_state() {
        let state = DashboardScanner::scan_all().await;
        // Should return a valid state (may have 0 tools in test environment)
        assert_eq!(state.summary.total, state.tools.len());
    }
}

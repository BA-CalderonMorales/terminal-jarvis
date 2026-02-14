// Flow State - Data model for tool health tracking
//
// Provides a structured representation of each tool's readiness
// across prerequisites, installation, authentication, and runtime.
// Used by the dashboard scanner and CLI dashboard renderer.

use super::tools_preflight::PreflightResult;

/// Overall flow state for a single tool
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowState {
    Flowing,   // Everything works end-to-end
    NeedsWork, // Something terminal-jarvis can help fix
    Blocked,   // External dependency not met
    Unknown,   // Cannot determine
}

impl FlowState {
    pub fn label(&self) -> &'static str {
        match self {
            FlowState::Flowing => "FLOWING",
            FlowState::NeedsWork => "NEEDS_WORK",
            FlowState::Blocked => "BLOCKED",
            FlowState::Unknown => "UNKNOWN",
        }
    }

    pub fn indicator(&self) -> &'static str {
        match self {
            FlowState::Flowing => "[OK]",
            FlowState::NeedsWork => "[!!]",
            FlowState::Blocked => "[XX]",
            FlowState::Unknown => "[??]",
        }
    }
}

impl std::fmt::Display for FlowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Detailed flow state for one tool
#[derive(Debug, Clone)]
pub struct ToolFlowState {
    pub tool_name: String,
    pub display_name: String,
    pub overall: FlowState,
    pub prerequisites: FlowState,
    pub installation: FlowState,
    pub authentication: FlowState,
    pub runtime: FlowState,
    pub blockers: Vec<String>,
    pub suggestions: Vec<String>,
    pub package_manager: String,
    pub version: Option<String>,
}

impl ToolFlowState {
    /// Derive from a PreflightResult
    pub fn from_preflight(preflight: &PreflightResult) -> Self {
        let prerequisites = if preflight.package_manager_ok {
            FlowState::Flowing
        } else {
            FlowState::Blocked
        };

        let installation = if preflight.is_installed {
            FlowState::Flowing
        } else {
            // TJ can install tools, so this is fixable
            FlowState::NeedsWork
        };

        let authentication = if preflight.auth.is_ready || preflight.auth.auth_mode == "none" {
            FlowState::Flowing
        } else {
            FlowState::NeedsWork
        };

        let runtime = if preflight.runtime_ok {
            FlowState::Flowing
        } else if preflight.is_installed {
            FlowState::Blocked
        } else {
            FlowState::Unknown
        };

        // Derive overall state
        let states = [&prerequisites, &installation, &authentication, &runtime];
        let overall = if states.iter().all(|s| **s == FlowState::Flowing) {
            FlowState::Flowing
        } else if states.iter().any(|s| **s == FlowState::Blocked) {
            FlowState::Blocked
        } else if states.iter().any(|s| **s == FlowState::NeedsWork) {
            FlowState::NeedsWork
        } else {
            FlowState::Unknown
        };

        // Build suggestions from what is missing
        let mut suggestions = Vec::new();
        if !preflight.is_installed {
            suggestions.push(format!(
                "Run: terminal-jarvis install {}",
                preflight.tool_name
            ));
        }
        if !preflight.auth.is_ready && preflight.auth.auth_mode != "none" {
            suggestions.push(format!(
                "Run: terminal-jarvis auth set --tool {}",
                preflight.tool_name
            ));
        }
        if !preflight.package_manager_ok {
            if let Some(ref msg) = preflight.package_manager_message {
                suggestions.push(msg.clone());
            }
        }

        ToolFlowState {
            tool_name: preflight.tool_name.clone(),
            display_name: preflight.tool_name.clone(),
            overall,
            prerequisites,
            installation,
            authentication,
            runtime,
            blockers: preflight.blockers.clone(),
            suggestions,
            package_manager: preflight.package_manager_label.clone(),
            version: preflight.tool_version.clone(),
        }
    }
}

/// Aggregate dashboard state
#[derive(Debug)]
pub struct DashboardState {
    pub tools: Vec<ToolFlowState>,
    pub summary: DashboardSummary,
    pub scan_duration: std::time::Duration,
}

/// Summary counts across all tools
#[derive(Debug)]
pub struct DashboardSummary {
    pub total: usize,
    pub flowing: usize,
    pub needs_work: usize,
    pub blocked: usize,
    pub unknown: usize,
}

impl DashboardSummary {
    pub fn from_tools(tools: &[ToolFlowState]) -> Self {
        let mut flowing = 0;
        let mut needs_work = 0;
        let mut blocked = 0;
        let mut unknown = 0;

        for tool in tools {
            match tool.overall {
                FlowState::Flowing => flowing += 1,
                FlowState::NeedsWork => needs_work += 1,
                FlowState::Blocked => blocked += 1,
                FlowState::Unknown => unknown += 1,
            }
        }

        DashboardSummary {
            total: tools.len(),
            flowing,
            needs_work,
            blocked,
            unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth_manager::auth_preflight::AuthPreflightResult;

    fn make_preflight(
        tool_name: &str,
        pm_ok: bool,
        installed: bool,
        auth_ready: bool,
        auth_mode: &str,
        runtime_ok: bool,
    ) -> PreflightResult {
        PreflightResult {
            tool_name: tool_name.to_string(),
            package_manager_ok: pm_ok,
            package_manager_label: "npm".to_string(),
            package_manager_message: if pm_ok {
                None
            } else {
                Some("npm not found".to_string())
            },
            is_installed: installed,
            install_message: if installed {
                None
            } else {
                Some(format!("'{}' is not installed", tool_name))
            },
            tool_version: if installed {
                Some("1.0.0".to_string())
            } else {
                None
            },
            auth: AuthPreflightResult {
                tool_name: tool_name.to_string(),
                is_ready: auth_ready,
                missing_vars: if auth_ready {
                    vec![]
                } else {
                    vec!["API_KEY".to_string()]
                },
                set_vars: vec![],
                auth_mode: auth_mode.to_string(),
                setup_url: None,
                auth_instructions: None,
                cli_auth_command: None,
                providers: vec![],
                default_env_var: None,
                key_format_hint: None,
            },
            runtime_ok,
            runtime_message: None,
            blockers: vec![],
            can_proceed: pm_ok && installed && auth_ready && runtime_ok,
            can_fix: !pm_ok || !installed || !auth_ready,
        }
    }

    #[test]
    fn test_from_preflight_all_flowing() {
        let pf = make_preflight("test-tool", true, true, true, "any", true);
        let state = ToolFlowState::from_preflight(&pf);

        assert_eq!(state.overall, FlowState::Flowing);
        assert_eq!(state.prerequisites, FlowState::Flowing);
        assert_eq!(state.installation, FlowState::Flowing);
        assert_eq!(state.authentication, FlowState::Flowing);
        assert_eq!(state.runtime, FlowState::Flowing);
        assert!(state.suggestions.is_empty());
    }

    #[test]
    fn test_from_preflight_not_installed() {
        let pf = make_preflight("test-tool", true, false, true, "any", false);
        let state = ToolFlowState::from_preflight(&pf);

        assert_eq!(state.overall, FlowState::NeedsWork);
        assert_eq!(state.installation, FlowState::NeedsWork);
        assert_eq!(state.runtime, FlowState::Unknown);
        assert!(state.suggestions.iter().any(|s| s.contains("install")));
    }

    #[test]
    fn test_from_preflight_blocked_pm() {
        let pf = make_preflight("test-tool", false, false, true, "any", false);
        let state = ToolFlowState::from_preflight(&pf);

        assert_eq!(state.overall, FlowState::Blocked);
        assert_eq!(state.prerequisites, FlowState::Blocked);
    }

    #[test]
    fn test_from_preflight_auth_needs_work() {
        let pf = make_preflight("test-tool", true, true, false, "any", true);
        let state = ToolFlowState::from_preflight(&pf);

        assert_eq!(state.overall, FlowState::NeedsWork);
        assert_eq!(state.authentication, FlowState::NeedsWork);
        assert!(state.suggestions.iter().any(|s| s.contains("auth")));
    }

    #[test]
    fn test_from_preflight_auth_none_always_flowing() {
        let pf = make_preflight("test-tool", true, true, false, "none", true);
        let state = ToolFlowState::from_preflight(&pf);

        // auth_mode "none" means auth is always flowing even if is_ready=false
        // But we check is_ready || auth_mode=="none", so this should be Flowing
        assert_eq!(state.authentication, FlowState::Flowing);
    }

    #[test]
    fn test_from_preflight_runtime_blocked() {
        let pf = make_preflight("test-tool", true, true, true, "any", false);
        let state = ToolFlowState::from_preflight(&pf);

        assert_eq!(state.runtime, FlowState::Blocked);
        assert_eq!(state.overall, FlowState::Blocked);
    }

    #[test]
    fn test_flow_state_labels() {
        assert_eq!(FlowState::Flowing.label(), "FLOWING");
        assert_eq!(FlowState::NeedsWork.label(), "NEEDS_WORK");
        assert_eq!(FlowState::Blocked.label(), "BLOCKED");
        assert_eq!(FlowState::Unknown.label(), "UNKNOWN");
    }

    #[test]
    fn test_flow_state_indicators() {
        assert_eq!(FlowState::Flowing.indicator(), "[OK]");
        assert_eq!(FlowState::NeedsWork.indicator(), "[!!]");
        assert_eq!(FlowState::Blocked.indicator(), "[XX]");
        assert_eq!(FlowState::Unknown.indicator(), "[??]");
    }

    #[test]
    fn test_flow_state_display() {
        assert_eq!(format!("{}", FlowState::Flowing), "FLOWING");
        assert_eq!(format!("{}", FlowState::Blocked), "BLOCKED");
    }

    #[test]
    fn test_dashboard_summary_from_tools() {
        let pf_ok = make_preflight("tool-a", true, true, true, "any", true);
        let pf_needs = make_preflight("tool-b", true, false, true, "any", false);
        let pf_blocked = make_preflight("tool-c", false, false, true, "any", false);

        let tools = vec![
            ToolFlowState::from_preflight(&pf_ok),
            ToolFlowState::from_preflight(&pf_needs),
            ToolFlowState::from_preflight(&pf_blocked),
        ];

        let summary = DashboardSummary::from_tools(&tools);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.flowing, 1);
        // tool-b: not installed -> NeedsWork, runtime Unknown -> NeedsWork overall
        // tool-c: pm blocked -> Blocked overall
        assert_eq!(summary.blocked, 1);
    }

    #[test]
    fn test_dashboard_summary_empty() {
        let summary = DashboardSummary::from_tools(&[]);
        assert_eq!(summary.total, 0);
        assert_eq!(summary.flowing, 0);
        assert_eq!(summary.needs_work, 0);
        assert_eq!(summary.blocked, 0);
        assert_eq!(summary.unknown, 0);
    }
}

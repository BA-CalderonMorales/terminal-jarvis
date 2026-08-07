use crate::contracts::{CapabilityPlan, SupportState};

pub fn validate(prefix: &str, plan: &CapabilityPlan, errors: &mut Vec<String>) {
    let command = plan.command.render().to_ascii_lowercase();
    let help = help_fallback(&command);
    if help && plan.support != SupportState::Stub {
        errors.push(format!("{prefix} help fallback must be classified as stub"));
    }
    if plan.support == SupportState::Stub && !help {
        errors.push(format!(
            "{prefix} stub must be deterministic local guidance"
        ));
    }
    if claims_execution(plan.support) && unsafe_pipe(&command) {
        errors.push(format!(
            "{prefix} support claim cannot use a curl-pipe installer"
        ));
    }
    if command.split_whitespace().any(|word| word == "sudo") {
        errors.push(format!("{prefix} command must not use sudo"));
    }
    if placeholder(&command) && plan.support != SupportState::Disabled {
        errors.push(format!(
            "{prefix} fail-closed placeholder must be classified as disabled"
        ));
    }
}

fn claims_execution(support: SupportState) -> bool {
    matches!(support, SupportState::Verified | SupportState::Expected)
}

fn help_fallback(command: &str) -> bool {
    command
        .split_whitespace()
        .any(|word| word.trim_matches(['\'', '"']) == "--help")
}

fn unsafe_pipe(command: &str) -> bool {
    command.contains("curl ") && (command.contains("| sh") || command.contains("| bash"))
}

fn placeholder(command: &str) -> bool {
    command.contains("not configured") && command.contains("exit 1")
}

#[cfg(test)]
#[path = "../tests/command_truth_props.rs"]
mod props;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_pipe_needs_both_parts() {
        assert!(unsafe_pipe("curl https://x.sh | sh"));
        assert!(unsafe_pipe("curl https://x.sh | bash"));
        assert!(!unsafe_pipe("echo x | sh"));
        assert!(!unsafe_pipe("curl https://x.sh --help"));
    }

    #[test]
    fn placeholder_needs_both_markers() {
        assert!(placeholder("echo 'not configured'; exit 1"));
        assert!(!placeholder("not configured but no exit"));
        assert!(!placeholder("exit 1 without notice"));
    }
}

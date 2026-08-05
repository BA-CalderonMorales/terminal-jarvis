pub fn normalize(raw: &str) -> Option<&'static str> {
    match raw {
        "env" | "source" => Some("source"),
        "github-release" | "github-release-cache" | "npm" => Some("npm"),
        "homebrew" => Some("homebrew"),
        "cargo" => Some("cargo"),
        "direct" => Some("direct"),
        _ => None,
    }
}

pub fn channel() -> Option<&'static str> {
    if let Some(raw) = std::env::var(crate::context::constants::env::DISTRIBUTION)
        .ok()
        .filter(|value| !value.trim().is_empty())
    {
        return Some(normalize(&raw).unwrap_or("unknown"));
    }
    if std::env::var(crate::context::constants::env::WRAPPER)
        .ok()
        .is_some_and(|value| !value.trim().is_empty())
    {
        return Some("npm");
    }
    if let Some(raw) =
        option_env!("TERMINAL_JARVIS_DISTRIBUTION_STAMPED").filter(|value| !value.trim().is_empty())
    {
        return Some(normalize(raw).unwrap_or("unknown"));
    }
    let executable = std::env::current_exe()
        .ok()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_default();
    if source_build(&executable) {
        return Some("source");
    }
    homebrew_path(&executable).then_some("homebrew")
}

pub fn source_build(path: &str) -> bool {
    !path.contains("/deps/")
        && (path.contains("/target/debug/") || path.contains("/target/release/"))
}

pub fn homebrew_path(path: &str) -> bool {
    path.contains("homebrew") || path.contains("Cellar")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_channels_are_normalized_without_passthrough_claims() {
        for raw in ["env", "source"] {
            assert_eq!(normalize(raw), Some("source"));
        }
        for raw in ["github-release", "github-release-cache", "npm"] {
            assert_eq!(normalize(raw), Some("npm"));
        }
        for raw in ["homebrew", "cargo", "direct"] {
            assert_eq!(normalize(raw), Some(raw));
        }
        assert_eq!(normalize("custom"), None);
    }

    #[test]
    fn source_build_classifies_workspace_binaries_only() {
        assert!(source_build("/srv/target/debug/terminal-jarvis"));
        assert!(source_build("/srv/target/release/tj"));
        assert!(!source_build("/srv/target/debug/deps/lib-test"));
        assert!(!source_build("/home/caldo/.cargo/bin/terminal-jarvis"));
        assert!(!source_build(""));
    }
}

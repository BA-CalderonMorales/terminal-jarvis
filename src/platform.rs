pub fn id() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH, libc()) {
        ("linux", "x86_64", "gnu") => Some("linux-x64-gnu"),
        ("linux", "aarch64", "gnu") => Some("linux-arm64-gnu"),
        ("macos", "x86_64", _) => Some("macos-x64"),
        ("macos", "aarch64", _) => Some("macos-arm64"),
        ("windows", "x86_64", _) => Some("windows-x64-msvc"),
        _ => None,
    }
}

pub fn libc() -> &'static str {
    if cfg!(target_os = "android") {
        "bionic"
    } else if cfg!(all(target_os = "linux", target_env = "gnu")) {
        "gnu"
    } else if cfg!(all(target_os = "linux", target_env = "musl")) {
        "musl"
    } else {
        "n/a"
    }
}

pub fn shell() -> String {
    let value = if cfg!(windows) { "COMSPEC" } else { "SHELL" };
    std::env::var(value)
        .ok()
        .filter(|shell| !shell.trim().is_empty())
        .and_then(|shell| {
            std::path::Path::new(&shell)
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn wsl() -> &'static str {
    if !cfg!(target_os = "linux") {
        return "no";
    }
    let release = std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default();
    if release
        .to_ascii_lowercase()
        .contains("microsoft-standard-wsl2")
    {
        "wsl2"
    } else if release.to_ascii_lowercase().contains("microsoft") {
        "wsl1-or-unknown"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_returns_basename() {
        let shell_path = std::env::var("SHELL").unwrap_or_default();
        let result = super::shell();
        if shell_path.is_empty() {
            assert_eq!(result, "unknown");
        } else {
            let expected = std::path::Path::new(&shell_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn shell_empty_returns_unknown() {
        let result = super::shell();
        assert!(!result.is_empty());
    }

    #[test]
    fn wsl_returns_valid_value() {
        let result = wsl();
        assert!(
            matches!(result, "no" | "wsl1-or-unknown" | "wsl2"),
            "wsl() should return no, wsl1-or-unknown, or wsl2, got {result}"
        );
    }
}

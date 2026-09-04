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
    // current_exe() returns backslash-separated paths on Windows; normalize
    // so the forward-slash matching below works on every platform.
    let path = path.replace('\\', "/");
    let Some(index) = path.find("/target/") else {
        return false;
    };
    let rest = &path[index + "/target/".len()..];
    if rest.contains("/deps/") {
        return false;
    }
    let in_build = rest.starts_with("debug/")
        || rest.starts_with("release/")
        || rest.contains("/debug/")
        || rest.contains("/release/");
    in_build
        && std::path::Path::new(&path[..index])
            .join("Cargo.toml")
            .is_file()
}

pub fn homebrew_path(path: &str) -> bool {
    path.contains("homebrew") || path.contains("Cellar")
}

#[cfg(test)]
#[path = "../tests/distribution.rs"]
mod tests;

use std::path::Path;

const REPO: &str = "https://github.com/BA-CalderonMorales/terminal-jarvis";

pub fn text(verbose: bool, catalog: &Path, home: &Path) -> String {
    let version = env!("CARGO_PKG_VERSION");
    if !verbose {
        let channel = distribution_channel();
        let suffix = if channel.is_empty() {
            String::new()
        } else {
            format!(" ({channel})")
        };
        return format!("terminal-jarvis {version}{suffix}\n");
    }
    let binary = std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let git_sha = option_env!("TERMINAL_JARVIS_GIT_SHA").unwrap_or("unknown");
    let distribution = nonempty_env("TERMINAL_JARVIS_DISTRIBUTION", || "unknown".to_string());
    let wrapper = std::env::var("TERMINAL_JARVIS_WRAPPER").unwrap_or_default();
    let release = nonempty_env("TERMINAL_JARVIS_RELEASE_URL", || {
        format!("{REPO}/releases/tag/v{version}")
    });
    let cache = nonempty_env("TERMINAL_JARVIS_CACHE", || "unavailable".to_string());
    let wrapper_line = if wrapper.is_empty() {
        String::new()
    } else {
        format!("wrapper: {wrapper}\n")
    };
    format!(
        "terminal-jarvis {version}\n\
         binary: {binary}\n\
         distribution: {distribution}\n\
         git commit: {git_sha}\n\
         release: {release}\n\
         cache: {cache}\n\
         catalog: {}\n\
         home: {}\n\
         {wrapper_line}",
        catalog.display(),
        home.display()
    )
}

fn nonempty_env<F>(key: &str, fallback: F) -> String
where
    F: FnOnce() -> String,
{
    std::env::var(key)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(fallback)
}

fn distribution_channel() -> String {
    let raw = std::env::var("TERMINAL_JARVIS_DISTRIBUTION").unwrap_or_default();
    if !raw.is_empty() {
        return match raw.as_str() {
            "env" | "source" => "source".to_string(),
            "github-release" | "github-release-cache" => "npm".to_string(),
            other => other.to_string(),
        };
    }
    if std::env::var("TERMINAL_JARVIS_WRAPPER").is_ok() {
        return "npm".to_string();
    }
    std::env::current_exe()
        .ok()
        .and_then(|p| {
            let s = p.to_string_lossy().to_string();
            if s.contains("homebrew") || s.contains("Cellar") {
                Some("homebrew".to_string())
            } else {
                None
            }
        })
        .unwrap_or_default()
}

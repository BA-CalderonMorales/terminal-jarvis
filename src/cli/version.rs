use super::{style, table};
use std::path::Path;

const REPO: &str = "https://github.com/BA-CalderonMorales/terminal-jarvis";

pub fn text(verbose: bool, catalog: &Path, home: &Path) -> String {
    let version = env!("CARGO_PKG_VERSION");
    if !verbose {
        let suffix = crate::distribution::channel()
            .map(|channel| format!(" ({channel})"))
            .unwrap_or_default();
        return format!("terminal-jarvis {version}{suffix}\n");
    }
    let binary = std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let git_sha = option_env!("TERMINAL_JARVIS_GIT_SHA").unwrap_or("unknown");
    let distribution = crate::distribution::channel()
        .unwrap_or("unknown")
        .to_string();
    let wrapper = std::env::var("TERMINAL_JARVIS_WRAPPER").unwrap_or_default();
    let release = nonempty_env("TERMINAL_JARVIS_RELEASE_URL", || {
        format!("{REPO}/releases/tag/v{version}")
    });
    let cache = nonempty_env("TERMINAL_JARVIS_CACHE", || "unavailable".to_string());
    let mut details = vec![
        ("BINARY", binary),
        ("DISTRIBUTION", distribution),
        ("GIT COMMIT", git_sha.to_string()),
        ("RELEASE", release),
        ("CACHE", cache),
        ("CATALOG", catalog.display().to_string()),
        ("HOME", home.display().to_string()),
    ];
    if !wrapper.is_empty() {
        details.push(("WRAPPER", wrapper));
    }
    if style::plain() {
        return format!("terminal-jarvis {version}\n{}", table::fields("", &details));
    }
    table::fields(&format!("Terminal Jarvis {version}"), &details)
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

#[cfg(test)]
#[path = "version_test.rs"]
mod tests;

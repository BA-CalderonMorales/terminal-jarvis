use std::path::Path;

const REPO: &str = "https://github.com/BA-CalderonMorales/terminal-jarvis";

pub fn text(verbose: bool, catalog: &Path, home: &Path) -> String {
    let version = env!("CARGO_PKG_VERSION");
    if !verbose {
        return format!("terminal-jarvis {version}\n");
    }
    let binary = std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let git_sha = option_env!("TERMINAL_JARVIS_GIT_SHA").unwrap_or("unknown");
    let distribution = nonempty_env("TERMINAL_JARVIS_DISTRIBUTION", "unknown");
    let wrapper = std::env::var("TERMINAL_JARVIS_WRAPPER").unwrap_or_default();
    let release = nonempty_env(
        "TERMINAL_JARVIS_RELEASE_URL",
        &format!("{REPO}/releases/tag/v{version}"),
    );
    let cache = nonempty_env("TERMINAL_JARVIS_CACHE", "unavailable");
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

fn nonempty_env(key: &str, fallback: &str) -> String {
    std::env::var(key)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

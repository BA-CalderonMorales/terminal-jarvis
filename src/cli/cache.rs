use std::env;

pub fn handle(words: &[String]) -> Result<String, String> {
    match words {
        [] => Ok(status()),
        [action] if action == "status" => Ok(status()),
        [action] if action == "clear" => Ok(clear()),
        [action, ..] if action == "refresh" => Ok(refresh()),
        _ => Err("usage: terminal-jarvis cache [status|clear|refresh]".to_string()),
    }
}

fn status() -> String {
    let distribution = distribution();
    let release = release_line();
    match cache_path() {
        Some(path) => format!("cache: {path}\ndistribution: {distribution}\n{release}"),
        None => format!("cache: unavailable\ndistribution: {distribution}\n{release}"),
    }
}

fn clear() -> String {
    match cache_path() {
        Some(path) => format!("cache clear: remove {path} after terminal-jarvis exits\n"),
        None => "cache clear: no wrapper cache path is active\n".to_string(),
    }
}

fn refresh() -> String {
    match cache_path() {
        Some(path) => format!("cache refresh: remove {path}; the wrapper will fetch again\n"),
        None => "cache refresh: no wrapper cache path is active\n".to_string(),
    }
}

fn cache_path() -> Option<String> {
    env::var("TERMINAL_JARVIS_CACHE")
        .ok()
        .filter(|value| !value.is_empty())
}

fn distribution() -> String {
    env::var("TERMINAL_JARVIS_DISTRIBUTION")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

fn release_line() -> String {
    match env::var("TERMINAL_JARVIS_RELEASE_URL") {
        Ok(url) if !url.is_empty() => format!("release: {url}\n"),
        _ => String::new(),
    }
}

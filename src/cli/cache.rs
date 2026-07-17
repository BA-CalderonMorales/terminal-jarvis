use super::{style, table};
use std::env;

pub fn handle(words: &[String]) -> Result<String, String> {
    match words {
        [] => Ok(status()),
        [action] if action == "status" => Ok(status()),
        [action] if action == "clear" => Ok(maintenance("clear", "after terminal-jarvis exits")),
        [action, ..] if action == "refresh" => {
            Ok(maintenance("refresh", "the wrapper will fetch again"))
        }
        _ => Err("usage: terminal-jarvis cache [status|clear|refresh]".to_string()),
    }
}

fn status() -> String {
    let cache = cache_path().unwrap_or_else(|| {
        "unavailable (set TERMINAL_JARVIS_CACHE or run via the npm launcher)".to_string()
    });
    let distribution = distribution();
    let release = release_url();
    if style::plain() {
        let mut out = format!("cache: {cache}\ndistribution: {distribution}\n");
        if let Some(release) = release {
            out.push_str(&format!("release: {release}\n"));
        }
        return out;
    }
    let mut fields = vec![("CACHE", cache), ("DISTRIBUTION", distribution)];
    if let Some(release) = release {
        fields.push(("RELEASE", release));
    }
    table::fields("Cache Status", &fields)
}

fn maintenance(action: &str, suffix: &str) -> String {
    let detail = match cache_path() {
        Some(path) => format!("remove {path}; {suffix}"),
        None => "no wrapper cache path is active".to_string(),
    };
    if style::plain() {
        return format!("cache {action}: {detail}\n");
    }
    table::fields(
        "Cache Maintenance",
        &[("REQUEST", action.to_string()), ("NEXT STEP", detail)],
    )
}

fn cache_path() -> Option<String> {
    env::var("TERMINAL_JARVIS_CACHE")
        .ok()
        .filter(|value| !value.is_empty())
}

fn distribution() -> String {
    crate::distribution::channel()
        .unwrap_or("unknown")
        .to_string()
}

fn release_url() -> Option<String> {
    env::var("TERMINAL_JARVIS_RELEASE_URL")
        .ok()
        .filter(|value| !value.is_empty())
}

//! Working-directory label for the home dashboard: rooted at `~` when it
//! lives under HOME, then left-ellipsized to at most 32 columns, cutting at
//! component boundaries; a single oversized component is hard-ellipsized
//! from the left so the cap is absolute.

pub fn cwd_label() -> String {
    let full = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    cwd_label_for(&full, std::env::var("HOME").ok().as_deref())
}

fn cwd_label_for(full: &str, home: Option<&str>) -> String {
    let rooted = home
        .and_then(|home| full.strip_prefix(home).map(|rest| format!("~{rest}")))
        .unwrap_or_else(|| full.to_string());
    if rooted.chars().count() <= 32 {
        return rooted;
    }
    const BUDGET: usize = 32 - 4;
    let mut tail: Vec<&str> = Vec::new();
    let mut length = 0;
    for component in rooted.split('/').rev() {
        if component.chars().count() > BUDGET {
            let keep = BUDGET - 1;
            let head: String = component
                .chars()
                .skip(component.chars().count() - keep)
                .collect();
            return format!("…{head}");
        }
        let joins = tail.len();
        if length + joins + component.chars().count() > BUDGET {
            break;
        }
        tail.push(component);
        length += component.chars().count();
    }
    tail.reverse();
    format!(".../{}", tail.join("/"))
}

#[cfg(test)]
#[path = "../tests/cwd.rs"]
mod tests;

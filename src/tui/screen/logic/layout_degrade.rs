//! HeaderDegrade: which clauses a narrow header keeps, behind the `tui-pages`
//! Labs flag. The identity shortens to its initials and the verdict is the
//! clause that never disappears, so a 40-column header reads
//! `TJ · READY 1/1 ready` instead of a bare count or a cut title.

use super::sanitize;

/// Shorter header candidates, longest first. The verdict is the last clause;
/// the identity is the first.
pub fn candidates(core: &str) -> Vec<String> {
    let clauses: Vec<&str> = core.split(" · ").collect();
    if clauses.len() < 2 {
        return Vec::new();
    }
    let verdict = clauses[clauses.len() - 1];
    let identity = abbreviate(clauses[0]);
    vec![format!("{identity} · {verdict}"), verdict.to_string()]
}

/// The product name reduced to its initials ("Terminal Jarvis" -> "TJ").
fn abbreviate(identity: &str) -> String {
    let styled = sanitize::keep_color(identity);
    match styled.split_once(' ') {
        Some((first, second)) => {
            let initials = format!(
                "{}{}",
                first.chars().next().unwrap_or(' '),
                second.chars().next().unwrap_or(' ')
            );
            identity.replace(&styled, &initials)
        }
        None => identity.to_string(),
    }
}

//! HeaderDegrade: which clauses a narrow header keeps, behind the `tui-pages`
//! Labs flag. The identity shortens to its initials and the verdict is the
//! clause that never disappears, so a 40-column header reads
//! `TJ · READY 1/1 ready` instead of a bare count or a cut title.

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

/// The product name reduced to its initials ("Terminal Jarvis" -> "TJ"). The
/// initials come from the unstyled text, so no escape sequence ever leaks in.
fn abbreviate(identity: &str) -> String {
    let plain = strip_escapes(identity);
    let mut words = plain.split_whitespace();
    match (words.next(), words.next()) {
        (Some(first), Some(second)) => {
            let initial = |word: &str| word.chars().next().unwrap_or(' ');
            format!("{}{}", initial(first), initial(second))
        }
        _ => plain,
    }
}

/// Removes CSI escape sequences (`ESC [ ... <letter>`) and keeps visible text.
fn strip_escapes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for next in chars.by_ref() {
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
#[path = "../../tests/layout_degrade.rs"]
mod tests;

//! Complete: Tab expands the line's last token against the command grammar
//! and the fleet. Pure -- the vocabulary arrives as a slice -- so the keys
//! are witnessed without a terminal.

use crate::tui::input::Editor;

/// First-word grammar; later words complete against harness names.
pub const COMMANDS: &[&str] = &[
    "auth",
    "cache",
    "clear",
    "config",
    "converse",
    "exit",
    "gate",
    "graveyard",
    "help",
    "home",
    "install",
    "list",
    "plan",
    "quit",
    "run",
    "security",
    "show",
    "uninstall",
    "update",
    "use",
];

/// Completes the last token in place; false when there is nothing to add.
pub fn apply(editor: &mut Editor, names: &[String]) -> bool {
    let Some(next) = line(editor.line(), names) else {
        return false;
    };
    editor.rewrite(&next);
    true
}

/// The line with its last token extended to the longest prefix its candidate
/// words share; `None` when nothing matches or the token already goes that far.
pub fn line(text: &str, names: &[String]) -> Option<String> {
    let start = text.rfind(char::is_whitespace).map_or(0, |at| at + 1);
    let token = &text[start..];
    if token.is_empty() {
        return None;
    }
    let mut words: Vec<&str> = names.iter().map(String::as_str).collect();
    if start == 0 {
        words.extend(COMMANDS.iter().copied());
    }
    let mut matches = words.into_iter().filter(|word| word.starts_with(token));
    let mut prefix = matches.next()?.to_string();
    for word in matches {
        let kept = common(&prefix, word).len();
        prefix.truncate(kept);
    }
    (prefix.len() > token.len()).then(|| format!("{}{prefix}", &text[..start]))
}

/// The longest shared prefix of two words.
fn common<'a>(a: &'a str, b: &str) -> &'a str {
    let mut end = 0;
    for (x, y) in a.chars().zip(b.chars()) {
        if x != y {
            break;
        }
        end += x.len_utf8();
    }
    &a[..end]
}

#[cfg(test)]
#[path = "../tests/complete.rs"]
mod tests;

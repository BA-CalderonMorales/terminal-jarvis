//! Sanitize: strips ANSI escapes and control bytes from captured harness
//! output -- opencode (and friends) emit color codes that would poison the
//! frame's width math.

/// Strips ANSI escapes and control bytes from captured harness output --
/// opencode (and friends) emit color codes that would poison width math.
pub fn clean(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut escape = false;
    for character in text.chars() {
        match character {
            '\x1b' => escape = true,
            _ if escape => escape = !character.is_ascii_alphabetic(),
            _ if character.is_control() && character != '\n' => {}
            _ => out.push(character),
        }
    }
    out.trim().to_string()
}

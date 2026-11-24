//! Quote-aware comment stripping: `#` starts a comment only outside a
//! double-quoted string (`description = "the #1 scanner"` keeps its `#`).

pub fn strip(line: &str) -> &str {
    let mut quoted = false;
    let mut escaped = false;
    for (at, character) in line.char_indices() {
        match character {
            _ if escaped => escaped = false,
            '\\' if quoted => escaped = true,
            '"' => quoted = !quoted,
            '#' if !quoted => return &line[..at],
            _ => {}
        }
    }
    line
}

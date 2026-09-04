//! Sanitize: the frame trusts only its own chrome. Any line entering the
//! painted viewport keeps color (SGR) but loses every other escape -- OSC,
//! cursor movement, queries -- and all control characters. Catalog strings
//! and command output are data, never commands to the terminal.

const MAX_SEQUENCE: usize = 32;

/// True when `line` carries nothing terminal-directive-shaped. Decoded
/// chars only -- UTF-8 continuation bytes are text, never C1 controls.
pub fn is_plain(line: &str) -> bool {
    !line
        .chars()
        .any(|c| c.is_control() || ('\u{80}'..='\u{9f}').contains(&c))
}

/// Strips every escape/control character except SGR color sequences.
pub fn keep_color(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\x1b' => escape(&mut out, &mut chars),
            c if is_control(c) => {}
            c => out.push(c),
        }
    }
    out
}

fn is_control(c: char) -> bool {
    c.is_control() || ('\u{80}'..='\u{9f}').contains(&c)
}

fn escape<I: Iterator<Item = char>>(out: &mut String, chars: &mut std::iter::Peekable<I>) {
    match chars.peek() {
        Some('[') => csi(out, chars),
        // OSC (and other ESC-introduced introducers): drop through ST/BEL.
        Some(']') => {
            chars.next();
            let mut previous = '\0';
            for c in chars.by_ref() {
                if c == '\x07' || (previous == '\x1b' && c == '\\') {
                    break;
                }
                previous = c;
            }
        }
        _ => {
            chars.next();
        }
    }
}

fn csi<I: Iterator<Item = char>>(out: &mut String, chars: &mut std::iter::Peekable<I>) {
    let mut candidate = String::from("\x1b[");
    chars.next();
    let mut kept_sgr = false;
    for (step, c) in chars.by_ref().enumerate() {
        let is_param = matches!(
            c,
            ';' | ':' | '?' | '0'..='9' | '<' | '=' | '>' | '$' | '"' | '\'' | ' '
        );
        if step < MAX_SEQUENCE && is_param {
            candidate.push(c);
            continue;
        }
        // First non-param byte terminates the sequence.
        if step < MAX_SEQUENCE && c == 'm' {
            kept_sgr = true;
            candidate.push(c);
        }
        break;
    }
    if kept_sgr {
        out.push_str(&candidate);
    }
}

#[cfg(test)]
#[path = "../../tests/screen_sanitize.rs"]
mod tests;

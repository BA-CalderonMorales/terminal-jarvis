//! Frames: read a pty byte stream back as the frames the tui painted, so a
//! test can assert the shape of what the user actually saw -- including that a
//! frame never grows past the terminal's height, which scrolls it.
#![cfg(unix)]

/// Every frame the tui painted, as the bytes after its cursor-home.
pub fn frames(bytes: &[u8]) -> Vec<&[u8]> {
    let needle = b"\x1b[H";
    let mut out = Vec::new();
    let mut at = 0;
    while let Some(pos) = bytes[at..].windows(needle.len()).position(|w| w == needle) {
        let start = at + pos + needle.len();
        let end = bytes[start..]
            .windows(needle.len())
            .position(|w| w == needle)
            .map(|p| start + p)
            .unwrap_or(bytes.len());
        out.push(&bytes[start..end]);
        at = end;
    }
    out
}

/// How many rows one frame painted.
pub fn height(frame: &[u8]) -> usize {
    frame.windows(2).filter(|w| w == b"\r\n").count() + 1
}

/// One frame's rows, escapes stripped.
pub fn rows(frame: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(frame)
        .split('\n')
        .map(|row| {
            let mut visible = String::new();
            let mut escape = false;
            for c in row.chars() {
                match (escape, c) {
                    (true, c) => escape = !c.is_ascii_alphabetic(),
                    (false, '\x1b') => escape = true,
                    (false, '\r') => {}
                    (false, c) => visible.push(c),
                }
            }
            visible
        })
        .collect()
}

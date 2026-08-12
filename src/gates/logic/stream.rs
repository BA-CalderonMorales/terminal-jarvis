use std::io::{Read, Write};

/// Copies a child gate's pipe to stderr (live, tee-style) while capturing
/// the full bytes for the caller. The stderr copy is success-path narration:
/// it happens only when `narrate` is on, so a quiet tui never sees the raw
/// scan stream, but the capture (and therefore the block summary) is intact.
pub fn tee(pipe: &mut dyn Read, narrate: bool) -> String {
    let mut captured = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match pipe.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => {
                if narrate {
                    let _ = std::io::stderr().write_all(&chunk[..read]);
                    let _ = std::io::stderr().flush();
                }
                captured.extend_from_slice(&chunk[..read]);
            }
        }
    }
    String::from_utf8_lossy(&captured).trim().to_string()
}

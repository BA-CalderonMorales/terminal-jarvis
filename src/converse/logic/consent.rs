//! Consent: the experimental gate. The first `converse` run in a session
//! home shows the warning and stops; running the same command again is the
//! acceptance, recorded as a memo file so the warning never nags twice.

use std::path::Path;

pub const DISCORD_URL: &str = "https://discord.gg/terminal-jarvis";
pub const ISSUES_URL: &str = "https://github.com/BA-CalderonMorales/terminal-jarvis/issues";

/// True once the user has seen the warning (memo file in the session home).
pub fn seen(state_home: &Path) -> bool {
    state_home.join("converse-consent").exists()
}

/// Records the acceptance after the warning was shown once.
pub fn mark(state_home: &Path) {
    let _ = std::fs::create_dir_all(state_home);
    let _ = std::fs::write(state_home.join("converse-consent"), b"warned once\n");
}

/// The always-on banner lines every conversation opens with, plus the
/// first-run consent notice when the memo is not yet recorded.
pub fn opening_lines(warned: bool) -> Vec<String> {
    let mut lines = vec![
        "⚠ experimental surface — in active development".to_string(),
        format!("feedback: {DISCORD_URL} or {ISSUES_URL}"),
    ];
    if !warned {
        lines.push("run the same converse command again to accept and start".to_string());
    }
    lines
}

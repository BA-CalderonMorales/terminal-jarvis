//! Art contract: the welcome body teaches the tool in a bounded number of
//! lines and always names the active harness truthfully.

use super::welcome;

#[test]
fn welcome_is_compact_and_teaches_the_commands() {
    let lines = welcome("codex", 3, 25);
    assert!(lines.len() <= 10, "primer must fit small viewports");
    let joined = lines.join("\n");
    // Identity lives in the header and tagline now; the primer teaches
    // commands only -- including how to leave.
    assert!(!joined.contains("codex"), "no identity duplication");
    assert!(!joined.contains("3/25"), "no readiness duplication");
    assert!(joined.contains("list"));
    assert!(joined.contains("status"));
    assert!(joined.contains("exit"));
    assert!(
        !lines.iter().any(|line| line.contains("╔")),
        "no brand box: the header owns identity"
    );
}

//! Art contract: the welcome body greets with the live fleet state, then
//! teaches the tool in a bounded number of lines.

use super::welcome;

#[test]
fn welcome_greets_then_teaches_the_commands() {
    let lines = welcome("codex", 3, 25);
    assert!(lines.len() <= 13, "primer must fit small viewports");
    let joined = lines.join("\n");
    // The greeting carries the live fleet state truthfully.
    assert!(joined.contains("codex is at the helm"));
    assert!(joined.contains("3 of 25 harnesses are ready"));
    assert!(joined.contains("list"));
    assert!(joined.contains("status"));
    assert!(joined.contains("exit"));
    assert!(
        !lines.iter().any(|line| line.contains("╔")),
        "no brand box: the header owns identity"
    );
}

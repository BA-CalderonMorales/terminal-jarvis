//! Art contract: the welcome body teaches the tool in a bounded number of
//! lines and always names the active harness truthfully.

use super::welcome;

#[test]
fn welcome_is_compact_and_names_the_active_harness() {
    let lines = welcome("codex", 3, 25);
    assert!(lines.len() <= 12, "welcome must fit small viewports");
    let joined = lines.join("\n");
    assert!(joined.contains("codex"));
    assert!(joined.contains("3/25"));
    assert!(joined.contains("list"));
    assert!(joined.contains("status"));
    assert!(lines.iter().any(|line| line.contains("T E R M I N A L")));
}

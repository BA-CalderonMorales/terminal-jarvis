use super::tests::harness;
use crate::cli::logic::{output, style};

const TODAY: &str = "2026-11-20";

#[test]
fn list_shows_status_and_counts_retired_and_retiring_apart() {
    let fleet = [
        harness("codex", None),
        harness("jules", Some(("2026-12-31", None))),
        harness("gemini", Some(("2026-06-18", None))),
    ];
    let before = style::set(false, true);
    let table = output::list_on(&fleet, TODAY);
    style::set(true, true);
    let plain = output::list_on(&fleet, TODAY);
    style::restore(before);
    assert!(table.contains("STATUS"), "{table}");
    assert!(!table.contains("SUPPORT"), "{table}");
    // The name column is the bare name, so no row wraps into a phantom one.
    assert_eq!(table.lines().filter(|l| l.contains("jules")).count(), 1);
    assert!(!table.contains("jules ("), "{table}");
    assert!(
        table.contains("3 harnesses · 1 retired · 1 retiring"),
        "{table}"
    );
    assert!(plain.contains("jules support="), "{plain}");
    assert!(plain.contains("status=retiring"), "{plain}");
    assert!(plain.contains("status=retired"), "{plain}");
    assert!(plain.contains("status=maintained"), "{plain}");
}

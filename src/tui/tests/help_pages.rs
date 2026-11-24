//! Grouped help (the `tui-pages` flag): keys first, commands under groups,
//! and the purpose column dropped on narrow frames instead of wrapping.
//! Every case goes through `help::text`, the one place that chooses the page.

use crate::tui::pages_env;
use crate::tui::screen::visible_width;
use crate::tui::shell::help::text;

#[test]
fn grouped_help_puts_the_keys_first_and_groups_the_commands() {
    let _env = pages_env::set(Some("1"));
    let body = text(120);
    let keys = body.find("Keys").expect("keys section");
    let agents = body.find("Your agents").expect("agents group");
    let preview = body.find("Preview and change").expect("preview group");
    assert!(keys < agents && agents < preview, "order: {body}");
    assert!(body.contains("this help"));
    assert!(body.contains("installs and runs pass the Trivy gate"));
}

#[test]
fn grouped_help_keeps_the_command_rows_of_the_flat_help() {
    let _env = pages_env::set(Some("1"));
    let body = text(120);
    for command in [
        "list [filter]",
        "gate [status|enable|disable|run]",
        "/debug [on|off]",
        "settings [set <flag> on|off | reset <flag>]",
    ] {
        assert!(body.contains(command), "missing `{command}`");
    }
}

#[test]
fn narrow_grouped_help_drops_the_purpose_column_and_fits() {
    let _env = pages_env::set(Some("1"));
    let body = text(40);
    assert!(body.contains("list [filter]"), "{body}");
    assert!(
        !body.contains("numbered fleet picker"),
        "purpose wrapped: {body}"
    );
    for line in body.lines() {
        assert!(visible_width(line) <= 40, "{line}");
    }
}

#[test]
fn flag_off_help_has_no_groups() {
    let _env = pages_env::set(None);
    assert!(!text(120).contains("Your agents"));
}

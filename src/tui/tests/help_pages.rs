//! Grouped help (the `tui-pages` flag): keys first, commands under groups,
//! wording shared with the flat help, and the purpose column dropped on
//! narrow frames instead of wrapping. Every case goes through `help::text`,
//! the one place that chooses the page.

use super::super::help_table::ROWS;
use super::groups::GROUPS;
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
fn every_command_row_is_grouped_once_and_wears_the_table_wording() {
    let _env = pages_env::set(Some("1"));
    let grouped: Vec<&str> = GROUPS
        .iter()
        .flat_map(|(_, cmds)| cmds.iter().copied())
        .collect();
    for (command, purpose) in ROWS {
        assert_eq!(
            grouped.iter().filter(|name| *name == command).count(),
            1,
            "`{command}` must appear in exactly one group"
        );
    }
    for command in &grouped {
        assert!(
            ROWS.iter().any(|(name, _)| name == command),
            "group names `{command}`, which the table lacks"
        );
    }
    let body = text(120);
    for (command, purpose) in ROWS {
        assert!(body.contains(command), "missing `{command}`");
        assert!(body.contains(purpose), "`{command}` lost its wording");
    }
}

#[test]
fn grouped_and_flat_help_share_every_purpose_verbatim() {
    let flat = {
        let _env = pages_env::set(None);
        text(120)
    };
    let _env = pages_env::set(Some("1"));
    let grouped = text(120);
    for (_, purpose) in ROWS {
        assert!(flat.contains(purpose), "flat help lacks `{purpose}`");
        assert!(grouped.contains(purpose), "grouped help lacks `{purpose}`");
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

#[test]
fn a_settings_file_that_enables_the_flag_does_not_leak_into_flag_off_tests() {
    let _env = pages_env::isolated(None, Some("[flags]\ntui-pages = true\n"));
    assert!(!text(120).contains("Your agents"));
}

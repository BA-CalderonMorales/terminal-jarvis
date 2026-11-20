//! A script installer has no derivable inverse, so every `sh`/`bash` download
//! row must declare its own uninstaller or sit on the allowlist below.
use std::path::Path;
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::Capability;

/// Script installers with no vendor-documented noninteractive remover.
/// `uninstall <name>` fails for these with a pointer to the row's `source`.
/// Do not add a row here without checking the vendor docs first.
const NO_DOCUMENTED_UNINSTALLER: [(&str, &str); 10] = [
    ("amp", "ampcode.com documents no CLI remover"),
    (
        "claude",
        "native installer; removal is manual file deletion",
    ),
    ("cursor-agent", "installer only; no documented remover"),
    ("droid", "Factory documents no CLI remover"),
    ("eca", "install script only; no documented remover"),
    (
        "forge",
        "antinomy.ai installer and forgecode docs name no remover",
    ),
    ("goose", "download_cli.sh only; no documented remover"),
    ("hermes", "install.sh only; no documented remover"),
    ("ollama", "install.sh only; docs describe manual removal"),
    ("vibe", "install.sh only; no documented remover"),
];

#[test]
fn script_installers_declare_an_uninstaller_or_are_allowlisted() {
    let harnesses = catalog::load(Path::new("harnesses")).expect("catalog loads");
    for harness in &harnesses {
        let Some(download) = harness.plan(Capability::Download) else {
            continue;
        };
        if !matches!(download.command.command.as_str(), "sh" | "bash") {
            continue;
        }
        let allowed = NO_DOCUMENTED_UNINSTALLER
            .iter()
            .any(|(name, reason)| *name == harness.name && !reason.is_empty());

        assert!(
            download.uninstall.is_some() || allowed,
            "{} is a script installer with no declared uninstaller",
            harness.name
        );
        if download.uninstall.is_some() {
            assert!(
                !allowed,
                "{} declares one; drop it from the allowlist",
                harness.name
            );
        }
    }
}

/// The catalog parser keeps inner quotes and does not unescape them, so a
/// quoted path reaches the shell with literal quote characters. Script rows
/// write `$HOME/...` unquoted; `set -f; IFS=` already stops word-splitting.
#[test]
fn shell_rows_carry_no_backslash_or_double_quote_in_args() {
    let harnesses = catalog::load(Path::new("harnesses")).expect("catalog loads");
    for harness in &harnesses {
        for plan in &harness.capabilities {
            if !matches!(plan.command.command.as_str(), "sh" | "bash") {
                continue;
            }
            for arg in &plan.command.args {
                assert!(
                    !arg.contains('\\') && !arg.contains('"'),
                    "{} {:?}: quote or backslash in args",
                    harness.name,
                    plan.capability
                );
            }
        }
    }
}

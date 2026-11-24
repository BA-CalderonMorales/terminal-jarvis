//! The tui's one command table: each command row and its purpose. The flat
//! help and the grouped help both read these rows, so the wording lives here
//! once and the two pages cannot drift apart.

pub const ROWS: &[(&str, &str)] = &[
    ("list [filter]", "numbered fleet picker; text narrows it"),
    ("graveyard", "retired harnesses, why, and their successor"),
    ("show <harness>", "one harness in detail"),
    ("plan [harness] <cap>", "preview, don't run"),
    (
        "use <harness> | <number> | <name>",
        "switch the active harness",
    ),
    ("run <harness> [args]", "open the harness ui"),
    ("install <harness>", "install behind the security gate"),
    ("uninstall <harness>", "remove a harness and its binary"),
    ("update <harness>", "update through the same gate"),
    (
        "gate [status|enable|disable|run]",
        "control the optional Trivy gate",
    ),
    (
        "auth | config | cache | security",
        "inspect local setup and posture",
    ),
    (
        "settings [set <flag> on|off | reset <flag>]",
        "optional features, on or off",
    ),
    ("/debug [on|off]", "raw, verbose view"),
    (
        "converse <turns> <a> <b> <topic...>",
        "two agents talk; dangerous plans need --allow-dangerous",
    ),
    ("home | clear", "reset the frame"),
    ("exit | quit", "leave without state changes"),
];

/// The purpose text for one command row, if the table has that command.
pub fn purpose(command: &str) -> Option<&'static str> {
    ROWS.iter()
        .find(|(name, _)| *name == command)
        .map(|(_, purpose)| *purpose)
}

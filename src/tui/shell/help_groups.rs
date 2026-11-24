//! The grouped help's rows: the keys, then the command table under the groups
//! a user thinks in. The wording is main's (`help::text`), so only the order
//! and the grouping live here.

pub type Row = (&'static str, &'static str);

pub const KEYS: &[Row] = &[
    ("↑ ↓ / j k", "scroll the transcript"),
    ("i", "back to typing"),
    ("esc", "leave typing (normal mode)"),
    ("tab", "complete a command or harness"),
    ("?", "this help"),
    ("q", "quits"),
    ("exit / ctrl-d", "leave the command center"),
];

pub const GROUPS: &[(&str, &[Row])] = &[
    (
        "Your agents",
        &[
            ("list [filter]", "numbered fleet picker; text narrows it"),
            ("graveyard", "retired harnesses, why, and their successor"),
            ("show <harness>", "one harness in detail"),
            (
                "use <harness> | <number> | <name>",
                "switch the active harness",
            ),
            ("run <harness> [args]", "open the harness ui"),
        ],
    ),
    (
        "Preview and change",
        &[
            ("plan [harness] <cap>", "preview, don't run"),
            ("install <harness>", "install behind the security gate"),
            ("uninstall <harness>", "remove a harness and its binary"),
            ("update <harness>", "update through the same gate"),
        ],
    ),
    (
        "Security and setup",
        &[
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
        ],
    ),
    (
        "Session",
        &[
            (
                "converse <turns> <a> <b> <topic...>",
                "two agents talk; dangerous plans need --allow-dangerous",
            ),
            ("/debug [on|off]", "raw, verbose view"),
            ("home | clear", "reset the frame"),
            ("exit | quit", "leave without state changes"),
        ],
    ),
];

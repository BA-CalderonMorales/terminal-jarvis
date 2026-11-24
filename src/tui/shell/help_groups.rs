//! The grouped help's layout: the keys (the tui's own key vocabulary, which
//! has no table row) and the command groups. Groups name commands only; each
//! command's wording comes from `help_table`, so nothing is copied here.

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

pub const GROUPS: &[(&str, &[&str])] = &[
    (
        "Your agents",
        &[
            "list [filter]",
            "graveyard",
            "show <harness>",
            "use <harness> | <number> | <name>",
            "run <harness> [args]",
        ],
    ),
    (
        "Preview and change",
        &[
            "plan [harness] <cap>",
            "install <harness>",
            "uninstall <harness>",
            "update <harness>",
        ],
    ),
    (
        "Security and setup",
        &[
            "gate [status|enable|disable|run]",
            "auth | config | cache | security",
            "settings [set <flag> on|off | reset <flag>]",
        ],
    ),
    (
        "Session",
        &[
            "converse <turns> <a> <b> <topic...>",
            "/debug [on|off]",
            "home | clear",
            "exit | quit",
        ],
    ),
];

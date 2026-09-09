//! ShellState: the mutable session state one command can change -- body,
//! history, scroll offset, hint -- plus the typed-flag overlay that lets
//! a command line carry the same options the headless cli speaks.

use crate::cli::args;

/// The mutable session state one command can change.
pub struct LoopState {
    pub body: Vec<String>,
    pub history: Vec<String>,
    pub converse: Option<crate::converse::Live>,
    /// The shared scroll position: turns and prompts move the same view.
    pub offset: usize,
    pub hint: String,
    pub options: args::Options,
    pub debug: bool,
    pub indicator: crate::tui::input::Indicator,
}

/// Flags typed on a command line overlay the session's for that command:
/// boolean modes OR in; a typed `--confirm` replaces the session's.
/// `narrate` is session-owned: the headless parser defaults it on, and the
/// tui's clean view deliberately keeps its own silence.
pub fn overlay(base: &args::Options, typed: &args::Options) -> args::Options {
    args::Options {
        confirm: typed.confirm.clone().or_else(|| base.confirm.clone()),
        no_input: base.no_input || typed.no_input,
        verbose: base.verbose || typed.verbose,
        dry_run: base.dry_run || typed.dry_run,
        allow_dangerous: base.allow_dangerous || typed.allow_dangerous,
        ..base.clone()
    }
}

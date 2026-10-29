//! ShellState: the mutable session state one command can change -- body,
//! history, scroll offset, hint -- plus the typed-flag overlay that lets
//! a command line carry the same options the headless cli speaks.

use crate::cli::args;

/// The mutable session state one command can change.
pub struct LoopState {
    pub body: Vec<String>,
    pub history: Vec<String>,
    pub converse: Option<crate::converse::Live>,
    /// A first-use notice already shown: Enter accepts it, anything else
    /// cancels. The seed is kept so the notice never costs a retype.
    pub pending_converse: Option<crate::converse::Seed>,
    /// The shared scroll position: turns and prompts move the same view.
    pub offset: usize,
    pub hint: String,
    /// Anchor the body to the bottom: a log or a transcript grows upward from
    /// the prompt, while a primer or a one-shot card stays centered.
    pub bottom: bool,
    pub options: args::Options,
    pub debug: bool,
    pub indicator: crate::tui::input::Indicator,
}

/// An empty line accepts a pending first-use notice; any typed command
/// cancels it. The seed survives the notice so accepting costs one Enter
/// rather than a retyped command line.
pub fn notice(state: &mut LoopState, input: &str) -> Option<crate::converse::Seed> {
    if !input.is_empty() {
        state.pending_converse = None;
        return None;
    }
    state.pending_converse.take()
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

#[cfg(test)]
#[path = "../tests/state.rs"]
mod tests;

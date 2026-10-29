//! Next: the shapes a resolved line takes and the outcome the loop applies.
//! Data shapes live apart from the resolver so the grammar stays one screen
//! of code and the loop's vocabulary is readable on its own.

use crate::cli::args;

pub enum Resolved {
    Empty,
    Help,
    Exit,
    Home,
    /// `list [filter]`: the picker, optionally narrowed to a text match.
    List(Option<String>),
    Run(args::Action, args::Options),
    Debug(Option<bool>),
    Theme(Option<String>),
    Converse(Option<crate::converse::Seed>),
    Error(String),
}

#[derive(Debug)]
pub enum Next {
    Exit,
    /// `reset` restores the pristine body (the home primer).
    Again {
        picker_shown: bool,
        reset: bool,
    },
    Debug(Option<bool>),
    Converse(Option<crate::converse::Seed>),
    /// A streaming action plus its typed options (session-overlaid).
    Stream {
        action: args::Action,
        options: args::Options,
    },
}

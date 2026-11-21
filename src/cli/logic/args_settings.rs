//! Grammar and option rules for `settings`.

use super::{Action, Options};

pub(super) fn valid_settings(words: &[String]) -> Result<Vec<String>, String> {
    match words {
        [] => Ok(Vec::new()),
        [action] if action == "list" => Ok(words.to_vec()),
        [action, _, state] if action == "set" && matches!(state.as_str(), "on" | "off") => {
            Ok(words.to_vec())
        }
        [action, _] if action == "reset" => Ok(words.to_vec()),
        _ => Err("usage: terminal-jarvis settings [list|set <flag> on|off|reset <flag>]".into()),
    }
}

/// `--dry-run` alone is the one lifecycle option `settings` accepts.
pub(super) fn preview_only(action: &Action, options: &Options) -> bool {
    matches!(action, Action::Settings(_))
        && options.dry_run
        && options.confirm.is_none()
        && !options.allow_dangerous
        && !options.no_input
}

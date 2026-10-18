//! StreamFinish: the human completion card for a streamed install or
//! update -- the same verdict the clean view speaks, adoption included.

use super::state::LoopState;
use crate::contracts::Harness;
use crate::tui::screen::{paint, Role};
use std::path::Path;
use std::time::Duration;

/// The human verdict for a streamed install/update: the same card the
/// clean view speaks, adoption included.
pub fn settle(
    state: &mut LoopState,
    harnesses: &[Harness],
    state_home: &Path,
    name: &str,
    verb: &str,
    code: i32,
    elapsed: Duration,
) {
    let binary_on_path = harnesses
        .iter()
        .find(|harness| harness.name == name)
        .is_some_and(|harness| crate::security::command_on_path(&harness.binary));
    let outcome = Ok((code, String::new()));
    let (adopted, text, persisted) =
        super::verdict::text(name, verb, binary_on_path, &outcome, elapsed);
    let row = if code == 0 {
        paint(Role::Ok, &format!("✓ {text}"))
    } else {
        paint(Role::Err, &format!("✗ {text}"))
    };
    state.body.push(row);
    if !persisted && code == 0 {
        state.body.push(paint(
            Role::Warn,
            &format!(
                "warning: {name}'s binary was not found on PATH; restart the shell or add its install directory"
            ),
        ));
    }
    if adopted && persisted {
        if let Err(cause) = crate::context::save(state_home, name) {
            let failure = crate::cli::session_write_error(cause);
            state
                .body
                .push(crate::cli::style::warning(&failure.rendered()));
        }
    }
}

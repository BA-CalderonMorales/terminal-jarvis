use super::error;
use crate::context::{self, Session};
use crate::contracts::Harness;
use std::path::Path;

pub fn session(home: &Path) -> error::Result<Option<Session>> {
    context::load(home).map_err(session_error)
}

pub fn session_error(cause: impl std::fmt::Display) -> error::Failure {
    error::Failure::state(
        "session_invalid",
        cause.to_string(),
        "repair or remove the Terminal Jarvis session file",
    )
}

pub fn selected_name(explicit: Option<String>, home: &Path) -> error::Result<String> {
    explicit.map_or_else(
        || {
            session(home)?
                .map(|value| value.active_harness)
                .ok_or_else(|| {
                    error::Failure::state(
                        "active_harness_missing",
                        "no active harness is selected",
                        "run `terminal-jarvis use <harness>` or pass a harness",
                    )
                })
        },
        Ok,
    )
}

pub fn find<'a>(harnesses: &'a [Harness], name: &str) -> error::Result<&'a Harness> {
    harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| {
            error::Failure::unavailable(
                "harness_unknown",
                format!("unknown harness '{name}'"),
                "run `terminal-jarvis list`",
            )
        })
}

pub fn state_error(message: String) -> error::Failure {
    error::Failure::state(
        "state_invalid",
        message,
        "run `terminal-jarvis check --verbose`",
    )
}

pub fn unavailable_error(message: String) -> error::Failure {
    error::Failure::unavailable("unavailable", message, "run `terminal-jarvis check`")
}

pub fn experimental_error(message: String) -> error::Failure {
    if message.contains("disabled") {
        return error::Failure::unavailable(
            "feature_disabled",
            message,
            "set TERMINAL_JARVIS_EXPERIMENTAL_UI=1 to opt in",
        );
    }
    state_error(message)
}

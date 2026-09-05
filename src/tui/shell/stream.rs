//! StreamWire: runs one streaming-eligible action inside the live frame.
//! The child's output arrives as classified splunk rows; the body grows
//! and repaints live, so the user never leaves the tui to watch a task.

use super::session;
use super::stream_finish;
use super::stream_plan;
use super::viewport::paint;
use crate::cli::{args, stream_invocation};
use crate::contracts::Harness;
use std::path::Path;
use std::time::{Duration, Instant};

pub fn apply(
    action: &args::Action,
    options: &args::Options,
    state: &mut super::state::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    let planned = match stream_plan::for_action(action, state, harnesses, state_home) {
        Some(planned) => planned,
        None => return true,
    };
    let label = &planned.label;
    state.body.push(format!("── {label} ──"));
    let started = Instant::now();
    let outcome = {
        let super::state::LoopState {
            body,
            offset,
            indicator,
            hint,
            ..
        } = state;
        let mut since_paint = Instant::now();
        stream_invocation(
            planned.invocation,
            options,
            harnesses,
            state_home,
            &mut |line| {
                for row in line.split('\n') {
                    body.push(row.to_string());
                }
                *offset = super::viewport::pinned(body);
                let now = Instant::now();
                if now.duration_since(since_paint) > Duration::from_millis(180) {
                    paint(
                        indicator,
                        hint,
                        harnesses,
                        catalog_root,
                        state_home,
                        body,
                        *offset,
                    );
                    since_paint = now;
                }
            },
        )
    };
    match outcome {
        Ok(code) => {
            if let Some((name, verb)) = &planned.lifecycle {
                stream_finish::settle(
                    state,
                    harnesses,
                    state_home,
                    name,
                    verb,
                    code,
                    started.elapsed(),
                );
            } else {
                state
                    .body
                    .push(session::recap(label, Some(code), started.elapsed()));
            }
            state.hint = super::status::modeline(state_home, false, state.debug);
        }
        Err(message) => {
            state
                .body
                .push(format!("✗ {} -- {}", message.code, message.message));
        }
    }
    true
}

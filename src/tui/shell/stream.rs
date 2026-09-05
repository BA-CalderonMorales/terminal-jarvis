//! StreamWire: runs one streaming-eligible action inside the live frame.
//! The child's output arrives as classified splunk rows; the body grows
//! and repaints live, so the user never leaves the tui to watch a task.

use super::outcome;
use super::stream_plan;
use super::viewport::paint;
use crate::cli::{args, stream_invocation};
use crate::contracts::Harness;
use std::path::Path;
use std::time::{Duration, Instant};

pub fn apply(
    action: &args::Action,
    state: &mut outcome::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    let (invocation, label) = match stream_plan::for_action(action, state, harnesses, state_home) {
        Some(pair) => pair,
        None => return true,
    };
    state.body.push(format!("── {label} ──"));
    let started = Instant::now();
    let outcome = {
        let outcome::LoopState {
            body,
            indicator,
            hint,
            options,
            ..
        } = state;
        let mut since_paint = Instant::now();
        stream_invocation(invocation, options, harnesses, state_home, &mut |line| {
            for row in line.split('\n') {
                body.push(row.to_string());
            }
            let now = Instant::now();
            if now.duration_since(since_paint) > Duration::from_millis(180) {
                paint(indicator, hint, harnesses, catalog_root, state_home, body);
                since_paint = now;
            }
        })
    };
    let seconds = started.elapsed().as_secs_f32();
    match outcome {
        Ok(code) => {
            state.body.push(format!(
                "── {} · exit {} · {:.1}s ──",
                if code == 0 { "done" } else { "failed" },
                code,
                seconds
            ));
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

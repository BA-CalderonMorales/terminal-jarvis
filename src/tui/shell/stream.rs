//! StreamWire: runs one streaming-eligible action inside the live frame: the
//! child's output arrives as classified rows and a run anchors to the prompt.
use super::session;
use super::stream_finish;
use super::stream_plan;
use super::viewport::paint as repaint;
use crate::cli::{args, stream_invocation};
use crate::contracts::Harness;
use crate::tui::screen::{paint, Role};
use std::path::Path;
use std::time::Instant;

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
    // The primer and the last command's output yield to the task under way.
    state.body.clear();
    let chapter = paint(Role::Accent, &session::chapter(label));
    state.body.push(chapter);
    state.offset = 0;
    // A run is a log: it grows upward from the prompt, newest row nearest it.
    state.bottom = true;
    let started = Instant::now();
    let outcome = {
        let super::state::LoopState {
            body,
            offset,
            indicator,
            hint,
            ..
        } = state;
        // Hold raw mode for the whole run: a typed key must not echo.
        let _raw = crate::tui::term::enable_raw();
        let mut throttle = super::stream_rows::Throttle::new();
        stream_invocation(
            planned.invocation,
            options,
            harnesses,
            state_home,
            &mut |line| {
                let rows = line.split('\n').count();
                for row in line.split('\n') {
                    body.push(super::stream_rows::render(row));
                }
                *offset = super::viewport::pinned(body);
                if throttle.due(rows) {
                    repaint(
                        indicator,
                        hint,
                        harnesses,
                        catalog_root,
                        state_home,
                        body,
                        *offset,
                        true,
                    );
                }
            },
        )
    };
    match outcome {
        Ok(crate::runtime::CANCELLED) => {
            state.body.push(paint(Role::Warn, "cancelled"));
        }
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
                // an adopted install changes the active harness
                super::status::refresh_indicator(&mut state.indicator, state_home, state.debug);
            } else {
                let recap = session::recap(label, Some(code), started.elapsed());
                state.body.push(paint(Role::Dim, &recap));
            }
        }
        Err(message) => {
            let line = format!("✗ {} -- {}", message.code, message.message);
            state.body.push(paint(Role::Err, &line));
        }
    }
    state.hint = super::status::modeline(state_home, false, state.debug);
    true
}

//! GuardExecute: the shared tail of every guarded invocation. `on_line`
//! switches the final hop from the blocking runner to the streaming one.

use crate::cli::logic::{
    args::Options, dispatch_support, error, gate_skip, guard_ask, guard_intent, guard_policy,
    invoke, output, package_advisory, resolve,
};
use crate::contracts::Harness;
use crate::gates;
use std::io::IsTerminal;
use std::path::Path;

type Lines<'a> = Option<&'a mut dyn FnMut(&str)>;

pub(super) fn execute(
    invocation: resolve::Invocation,
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
    explicit: bool,
    mut on_line: Lines<'_>,
) -> error::Result<(i32, String)> {
    let harness = dispatch_support::find(harnesses, &invocation.harness)?;
    let plan = harness.plan(invocation.capability).ok_or_else(|| {
        error::Failure::state(
            "catalog_incomplete",
            format!("{} lacks {}", harness.name, invocation.capability),
            "repair the harness catalog",
        )
    })?;
    guard_policy::check(harness, plan, std::io::stdin().is_terminal())?;
    let asked = on_line.is_some();
    let intent = match on_line.as_mut() {
        Some(paint) => guard_intent::check_with(
            harness,
            plan,
            &invocation.extra,
            options,
            explicit,
            &mut |lead: &str, token: &str| {
                for line in lead.lines() {
                    paint(line);
                }
                paint(&format!("Continue with {token}? [y/N]"));
                // one keystroke answers: raw mode so [y/N] means [y/N],
                // never a cooked line waiting for an Enter that reads
                // as a hang
                let _raw = crate::tui::term::enable_raw();
                let key = crate::tui::input::read_key();
                // the tail of a typed answer ("es" of "yes") must never
                // leak into the prompt buffer
                crate::tui::input::drain_answer(std::time::Duration::from_millis(150));
                let (row, answer) = guard_ask::in_frame(key);
                paint(row);
                guard_ask::consent(answer)
            },
        ),
        None => guard_intent::check(harness, plan, &invocation.extra, options, explicit),
    };
    intent?;
    if options.dry_run {
        return Ok((
            0,
            output::plan_with_extra(harness, invocation.capability, &invocation.extra),
        ));
    }
    let target = format!("{}:{}", invocation.capability, invocation.harness);
    gates::preflight(home, options.narrate)
        .map_err(|m| error::Failure::safety("gate_blocked", m, "run `terminal-jarvis gate status`"))
        .and_then(|verdict| gate_skip::route(options, verdict, &target))?;
    package_advisory::check_quiet(harness, plan, options, home, asked, &mut |line| {
        if let Some(paint) = on_line.as_deref_mut() {
            paint(line);
        }
    })?;
    match on_line {
        Some(paint) => {
            let code = crate::runtime::run_streaming(plan, &invocation.extra, &mut |line: &str| {
                paint(&crate::runtime::classify(line))
            });
            match code {
                Ok(code) => Ok((code, String::new())),
                Err(error) => Err(dispatch_support::unavailable_error(error.to_string())),
            }
        }
        None => invoke::invocation(invocation, harnesses, options.narrate)
            .map_err(dispatch_support::unavailable_error),
    }
}

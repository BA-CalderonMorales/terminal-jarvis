//! GuardExecute: the shared tail of every guarded invocation. `on_line`
//! switches the final hop from the blocking runner to the streaming one.

use crate::cli::logic::{
    args::Options, dispatch_support, error, guard_ask, invoke, output, package_advisory, resolve,
};
use crate::contracts::Harness;
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
    let asked = on_line.is_some();
    let request = super::super::guard_authorize::Request {
        harness,
        plan,
        extra: &invocation.extra,
        options,
        home,
        explicit,
    };
    let _authorized = match on_line.as_mut() {
        Some(paint) => {
            super::super::guard_authorize::check(request, &mut |lead, token, default_yes| {
                super::super::guard_prompt::ask(paint, lead, token, default_yes)
            })
        }
        None => super::super::guard_authorize::check(request, &mut guard_ask::ask_in_terminal),
    }?;
    if options.dry_run {
        return Ok((
            0,
            output::plan_with_extra(harness, invocation.capability, &invocation.extra),
        ));
    }
    package_advisory::check_quiet(harness, plan, options, home, asked, &mut |line| {
        if let Some(paint) = on_line.as_deref_mut() {
            paint(line);
        }
    })?;
    match on_line {
        Some(paint) => {
            let code = crate::runtime::run_streaming(
                plan,
                &invocation.extra,
                &mut |line: &str| paint(&crate::runtime::classify(line)),
                // Ctrl+C or Esc cancels a streamed install: the child has no
                // terminal to reach and no polite shutdown worth waiting for.
                &mut || crate::tui::input::cancel_requested(),
            );
            match code {
                Ok(code) => Ok((code, String::new())),
                Err(error) => Err(dispatch_support::unavailable_error(error.to_string())),
            }
        }
        None => invoke::invocation(invocation, harnesses, options.narrate)
            .map_err(dispatch_support::unavailable_error),
    }
}

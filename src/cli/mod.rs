mod action;
pub mod args;
mod cache;
mod compat;
mod compat_support;
mod dispatch;
#[path = "dispatch_compat.rs"]
mod dispatch_compat;
#[path = "dispatch_security.rs"]
mod dispatch_security;
#[path = "dispatch_support.rs"]
mod dispatch_support;
mod entry;
mod error;
mod execute;
mod experimental;
mod gate_cmd;
mod guard;
#[path = "guard_intent.rs"]
mod guard_intent;
#[path = "guard_policy.rs"]
mod guard_policy;
mod help;
#[path = "help_command.rs"]
mod help_command;
#[path = "help_text.rs"]
mod help_text;
mod invoke;
mod json;
mod output;
#[path = "output_plan.rs"]
mod output_plan;
#[path = "output_truth.rs"]
mod output_truth;
mod resolve;
mod response;
mod self_update;
#[path = "self_update_intent.rs"]
mod self_update_intent;
mod style;
mod table;
#[cfg(test)]
pub(crate) mod test_support;
mod version;
use std::path::Path;

pub fn run<I>(args: I, catalog_root: &Path, home: &Path) -> i32
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let all = args.into_iter().map(Into::into).collect::<Vec<String>>();
    let parsed = match args::parse_cli(all.clone()) {
        Ok(parsed) => parsed,
        Err(error) => return entry::parse_failure(&all, &error),
    };
    let command = entry::action_name(&parsed.action);
    let mode = parsed.options.output;
    let plain = mode != args::OutputMode::Rich;
    let previous = style::set(plain, parsed.options.no_color || plain);
    let result = execute::run(parsed, catalog_root, home);
    let code = match result {
        Ok(response) => {
            let response::Response {
                exit_code: code,
                body,
                json: document,
            } = response;
            if mode == args::OutputMode::Json {
                print!(
                    "{}",
                    document.unwrap_or_else(|| json::outcome(&command, code, &body))
                );
            } else if !body.is_empty() {
                print!("{body}");
            }
            code
        }
        Err(error) => {
            if mode == args::OutputMode::Json {
                print!(
                    "{}",
                    json::failure(
                        &command,
                        error.exit_code,
                        error.code,
                        &error.message,
                        &error.next_action,
                    )
                );
            } else {
                eprint!("{}", style::error(&error.message));
                eprintln!("next action: {}", error.next_action);
            }
            error.exit_code
        }
    };
    style::restore(previous);
    code
}

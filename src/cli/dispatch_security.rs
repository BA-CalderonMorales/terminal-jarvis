use super::{dispatch_support, error, output};
use crate::contracts::{Capability, Harness};

pub fn run(words: &[String], harnesses: &[Harness]) -> error::Result<(i32, String)> {
    match words {
        [] => Ok((0, output::status(harnesses))),
        [action] if action == "status" => Ok((0, output::status(harnesses))),
        [action] if action == "audit" => Ok((0, output::audit(harnesses))),
        [name] => Ok((
            0,
            output::plan(
                dispatch_support::find(harnesses, name)?,
                Capability::Security,
            ),
        )),
        _ => Err(error::Failure::state(
            "parser_contract",
            "security arguments reached dispatch without validation",
            "report this Terminal Jarvis bug",
        )),
    }
}

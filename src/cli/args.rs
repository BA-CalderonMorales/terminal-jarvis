pub use super::action::Action;
use crate::contracts::Capability;
#[rustfmt::skip]
fn hlp(words: &[String]) -> bool { words.iter().skip(1).any(|w| w == "--help" || w == "-h") }
#[rustfmt::skip]
pub fn parse<I>(args: I) -> Result<Action, String>
where I: IntoIterator, I::Item: Into<String>,
{
    let words = args.into_iter().map(Into::into).skip(1).collect::<Vec<_>>();
    if words.is_empty() { return Ok(Action::Help); }
    match words[0].as_str() {
        "help" | "--help" | "-h" => Ok(Action::Help),
        "version" => version(&words[1..]),
        "--version" | "-v" if words.len() == 1 => Ok(Action::Version { verbose: false }),
        "-v" if words.len() == 2 && words[1] == "version" => version(&[String::from("-v")]),
        "--version" | "-v" => Err(format!("unexpected argument '{}' after --version/-v flag", words[1])),
        "--info" if words.len() == 1 => Ok(Action::Version { verbose: true }),
        "--info" => Err(format!("unexpected argument '{}' after --info flag", words[1])),
        "list" | "tools" if hlp(&words) => Ok(Action::Help),
        "list" | "tools" => Ok(Action::List),
        "check" | "status" if hlp(&words) => Ok(Action::Help),
        "check" | "status" => Ok(Action::Check),
        "current" if hlp(&words) => Ok(Action::Help),
        "current" => Ok(Action::Current),
        "use" if hlp(&words) => Ok(Action::Help),
        "use" => one(&words, "use").map(Action::Use),
        "show" | "info" if hlp(&words) => Ok(Action::Help),
        "show" | "info" => one(&words, words[0].as_str()).map(Action::Show),
        "plan" if hlp(&words) => Ok(Action::Help),
        "plan" => plan(&words[1..]),
        "run" if words.get(1).is_some_and(|w| w == "--help" || w == "-h") => Ok(Action::Help),
        "run" => Ok(Action::Run(words[1..].to_vec())),
        "install" if hlp(&words) => Ok(Action::Help),
        "install" => one(&words, "install").map(Action::Install),
        "update" if hlp(&words) => Ok(Action::Help),
        "update" => optional_one(&words, "update").map(Action::Update),
        "--update" if words.len() == 1 => Ok(Action::SelfUpdate),
        "auth" if hlp(&words) => Ok(Action::Help),
        "auth" => Ok(Action::Auth(words[1..].to_vec())),
        "config" if hlp(&words) => Ok(Action::Help),
        "config" => Ok(Action::Config(words[1..].to_vec())),
        "cache" if hlp(&words) => Ok(Action::Help),
        "cache" => Ok(Action::Cache(words[1..].to_vec())),
        "security" if hlp(&words) => Ok(Action::Help),
        "security" => Ok(Action::Security(words[1..].to_vec())),
        "templates" | "db" if hlp(&words) => Ok(Action::Help),
        "templates" | "db" => Ok(Action::Legacy(words[0].clone())),
        other if other.starts_with('-') => Err(format!("unknown flag '{other}'; use --help, --version, -v, or --info")),
        other => Ok(Action::Direct { harness: other.to_string(), extra: words[1..].to_vec() }),
    }
}
#[rustfmt::skip]
fn version(words: &[String]) -> Result<Action, String> {
    let mut verbose = false;
    for flag in words { match flag.as_str() {
        "-v" => verbose = false,
        "--verbose" | "--info" => verbose = true,
        "--help" | "-h" => return Ok(Action::Help),
        _ => return Err(format!("unknown flag '{flag}'; usage: terminal-jarvis version [--verbose|--info|-v]")),
    } }
    Ok(Action::Version { verbose })
}
#[rustfmt::skip]
fn one(w: &[String], c: &str) -> Result<String, String> { match w { [_, v] => Ok(v.clone()), _ => Err(format!("usage: terminal-jarvis {c} <harness>")) } }
#[rustfmt::skip]
fn optional_one(w: &[String], c: &str) -> Result<Option<String>, String> { match w { [_] => Ok(None), [_, v] => Ok(Some(v.clone())), _ => Err(format!("usage: terminal-jarvis {c} [harness]")) } }
#[rustfmt::skip]
fn plan(words: &[String]) -> Result<Action, String> { match words { [c] => Ok(Action::Plan { harness: None, capability: cap(c)? }), [h, c] => Ok(Action::Plan { harness: Some(h.clone()), capability: cap(c)? }), _ => Err("usage: terminal-jarvis plan [harness] <capability>".to_string()) } }
#[rustfmt::skip]
fn cap(value: &str) -> Result<Capability, String> { Capability::parse(value).ok_or_else(|| format!("unknown capability '{value}'; expected one of: {}", Capability::ALL.iter().map(|c| c.as_str()).collect::<Vec<_>>().join(", "))) }

#[cfg(test)]
#[path = "args_test.rs"]
mod tests;
#[cfg(test)]
#[path = "args_test_extra.rs"]
mod tests_extra;

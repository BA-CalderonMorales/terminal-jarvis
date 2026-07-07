use crate::context;
use crate::contracts::{Capability, Harness};
use std::path::Path;

pub struct Invocation {
    pub harness: String,
    pub capability: Capability,
    pub extra: Vec<String>,
}

pub fn run(words: &[String], harnesses: &[Harness], home: &Path) -> Result<Invocation, String> {
    if words.is_empty() {
        return Ok(invocation(active(home)?, Capability::Ui, Vec::new()));
    }
    let first = &words[0];
    if has_harness(harnesses, first) {
        return Ok(for_harness(first, &words[1..]));
    }
    if first == "headless" {
        return Ok(invocation(
            active(home)?,
            Capability::Headless,
            words[1..].to_vec(),
        ));
    }
    if let Some(capability) = Capability::parse(first) {
        if words.len() == 1 {
            return Ok(invocation(active(home)?, capability, Vec::new()));
        }
        return Ok(invocation(
            active(home)?,
            Capability::Headless,
            words.to_vec(),
        ));
    }
    let selected = active(home)?;
    if has_harness(harnesses, &selected) {
        return Ok(invocation(selected, Capability::Headless, words.to_vec()));
    }
    Err(format!("active harness '{selected}' is not in the catalog"))
}

pub fn direct(
    harness: &str,
    extra: &[String],
    harnesses: &[Harness],
) -> Result<Invocation, String> {
    if !has_harness(harnesses, harness) {
        return Err(format!(
            "unknown command or harness '{harness}'; run `terminal-jarvis list`"
        ));
    }
    Ok(invocation(
        harness.to_string(),
        Capability::Ui,
        extra.to_vec(),
    ))
}

fn for_harness(harness: &str, rest: &[String]) -> Invocation {
    match rest.split_first() {
        None => invocation(harness.to_string(), Capability::Ui, Vec::new()),
        Some((first, extra)) => Capability::parse(first)
            .map(|capability| invocation(harness.to_string(), capability, extra.to_vec()))
            .unwrap_or_else(|| {
                invocation(harness.to_string(), Capability::Headless, rest.to_vec())
            }),
    }
}

fn active(home: &Path) -> Result<String, String> {
    context::load(home)
        .map_err(|error| error.to_string())?
        .map(|session| session.active_harness)
        .ok_or_else(|| {
            "no active harness; run `terminal-jarvis use <harness>` or pass a harness".to_string()
        })
}

fn has_harness(harnesses: &[Harness], name: &str) -> bool {
    harnesses.iter().any(|harness| harness.name == name)
}

fn invocation(harness: String, capability: Capability, extra: Vec<String>) -> Invocation {
    Invocation {
        harness,
        capability,
        extra,
    }
}

#[cfg(test)]
#[path = "resolve_test.rs"]
mod tests;

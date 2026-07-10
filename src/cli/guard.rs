use super::{invoke, resolve};
use crate::contracts::{Capability, Harness};
use crate::gates;
use std::path::Path;

pub fn run(words: &[String], harnesses: &[Harness], home: &Path) -> Result<(i32, String), String> {
    let invocation = resolve::run(words, harnesses, home)?;
    gates::preflight(home)?;
    invoke::invocation(invocation, harnesses)
}

pub fn direct(
    name: &str,
    extra: &[String],
    harnesses: &[Harness],
    home: &Path,
) -> Result<(i32, String), String> {
    let invocation = resolve::direct(name, extra, harnesses)?;
    gates::preflight(home)?;
    invoke::invocation(invocation, harnesses)
}

pub fn capability(
    harnesses: &[Harness],
    name: &str,
    capability: Capability,
    home: &Path,
) -> Result<(i32, String), String> {
    known(harnesses, name)?;
    gates::preflight(home)?;
    invoke::capability(harnesses, name, capability, &[])
}

fn known(harnesses: &[Harness], name: &str) -> Result<(), String> {
    harnesses
        .iter()
        .any(|harness| harness.name == name)
        .then_some(())
        .ok_or_else(|| format!("unknown harness '{name}'"))
}

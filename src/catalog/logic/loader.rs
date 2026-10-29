use crate::contracts::{Capability, CapabilityPlan, Harness};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::embedded;
use crate::catalog::logic::parser::{self, Fields};
use crate::catalog::structs::metadata;

pub fn load(root: &Path) -> io::Result<Vec<Harness>> {
    if should_use_embedded(root) {
        return embedded::load();
    }
    let mut harnesses = Vec::new();
    for dir in dirs(root)? {
        if dir.is_dir() {
            harnesses.push(load_harness(&dir)?);
        }
    }
    super::validate::checked(harnesses)
}

fn should_use_embedded(root: &Path) -> bool {
    !catalog_env_set() && embedded_for(root, root.is_dir())
}

// The policy is split from the filesystem probe it depends on: whether a
// selected root is served from embedded data is a pure decision on the root
// and its existence, so it can be witnessed without borrowing the process
// working directory or the process environment.
fn embedded_for(root: &Path, root_exists: bool) -> bool {
    root == Path::new(crate::context::EMBEDDED_CATALOG)
        || (root == Path::new("harnesses") && !root_exists)
}

fn catalog_env_set() -> bool {
    env::var_os("TERMINAL_JARVIS_CATALOG").is_some_and(|value| !value.is_empty())
}

fn load_harness(dir: &Path) -> io::Result<Harness> {
    let meta = fields(&dir.join("index.toml"))?;
    let mut capabilities = Vec::new();
    for capability in Capability::ALL {
        capabilities.push(load_capability(dir, capability)?);
    }
    metadata::harness(&meta, capabilities).map_err(invalid)
}

fn load_capability(dir: &Path, capability: Capability) -> io::Result<CapabilityPlan> {
    let path = dir.join(capability.as_str()).join("index.toml");
    let data = fields(&path)?;
    metadata::capability(&data, capability).map_err(invalid)
}

fn fields(path: &Path) -> io::Result<Fields> {
    let data = fs::read_to_string(path)?;
    parser::parse(&data).map_err(invalid)
}

fn dirs(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs = fs::read_dir(root)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<io::Result<Vec<_>>>()?;
    dirs.sort();
    Ok(dirs)
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(test)]
#[path = "../tests/embedded_policy.rs"]
mod tests;

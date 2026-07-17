use crate::contracts::{Capability, CapabilityPlan, Harness};
use std::collections::BTreeMap;
use std::io;

use super::{
    metadata,
    parser::{self, Fields},
};

include!(concat!(env!("OUT_DIR"), "/embedded_catalog.rs"));

pub fn load() -> io::Result<Vec<Harness>> {
    let mut grouped: BTreeMap<&str, BTreeMap<&str, &str>> = BTreeMap::new();
    for (path, data) in FILES {
        if let Some((name, file)) = path.split_once('/') {
            grouped.entry(name).or_default().insert(file, data);
        }
    }
    let harnesses = grouped
        .into_values()
        .map(|files| load_harness(&files))
        .collect::<io::Result<Vec<_>>>()?;
    super::validate::checked(harnesses)
}

fn load_harness(files: &BTreeMap<&str, &str>) -> io::Result<Harness> {
    let meta = fields(files, "index.toml")?;
    let mut capabilities = Vec::new();
    for capability in Capability::ALL {
        capabilities.push(load_capability(files, capability)?);
    }
    metadata::harness(&meta, capabilities).map_err(invalid)
}

fn load_capability(
    files: &BTreeMap<&str, &str>,
    capability: Capability,
) -> io::Result<CapabilityPlan> {
    let path = format!("{}/index.toml", capability.as_str());
    let data = fields(files, &path)?;
    metadata::capability(&data, capability).map_err(invalid)
}

fn fields(files: &BTreeMap<&str, &str>, path: &str) -> io::Result<Fields> {
    let data = files
        .get(path)
        .ok_or_else(|| invalid(format!("missing {path}")))?;
    parser::parse(data).map_err(invalid)
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(test)]
#[path = "embedded_test.rs"]
mod tests;

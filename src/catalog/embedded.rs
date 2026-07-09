use crate::contracts::{Capability, CapabilityPlan, CommandPlan, EnvMode, Harness};
use std::collections::BTreeMap;
use std::io;

use super::parser::{self, Fields};

include!(concat!(env!("OUT_DIR"), "/embedded_catalog.rs"));

pub fn load() -> io::Result<Vec<Harness>> {
    let mut grouped: BTreeMap<&str, BTreeMap<&str, &str>> = BTreeMap::new();
    for (path, data) in FILES {
        if let Some((name, file)) = path.split_once('/') {
            grouped.entry(name).or_default().insert(file, data);
        }
    }
    grouped
        .into_values()
        .map(|files| load_harness(&files))
        .collect()
}

fn load_harness(files: &BTreeMap<&str, &str>) -> io::Result<Harness> {
    let meta = fields(files, "index.toml")?;
    let mut capabilities = Vec::new();
    for capability in Capability::ALL {
        capabilities.push(load_capability(files, capability)?);
    }
    Ok(Harness {
        name: parser::string(&meta, "name").map_err(invalid)?,
        display: parser::string(&meta, "display").map_err(invalid)?,
        description: parser::string(&meta, "description").map_err(invalid)?,
        binary: parser::string(&meta, "binary").map_err(invalid)?,
        env_mode: EnvMode::parse(&parser::string(&meta, "env_mode").map_err(invalid)?)
            .map_err(invalid)?,
        env: parser::list(&meta, "env").map_err(invalid)?,
        capabilities,
    })
}

fn load_capability(
    files: &BTreeMap<&str, &str>,
    capability: Capability,
) -> io::Result<CapabilityPlan> {
    let path = format!("{}/index.toml", capability.as_str());
    let data = fields(files, &path)?;
    let command = parser::string(&data, "command").map_err(invalid)?;
    Ok(CapabilityPlan {
        capability,
        summary: parser::string(&data, "summary").map_err(invalid)?,
        command: CommandPlan::new(command, parser::list(&data, "args").map_err(invalid)?),
    })
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

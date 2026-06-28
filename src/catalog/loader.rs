use crate::contracts::{Capability, CapabilityPlan, CommandPlan, EnvMode, Harness};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::{
    embedded,
    parser::{self, Fields},
};

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
    Ok(harnesses)
}

fn should_use_embedded(root: &Path) -> bool {
    env::var_os("TERMINAL_JARVIS_CATALOG").is_none()
        && root == Path::new("harnesses")
        && !root.is_dir()
}

fn load_harness(dir: &Path) -> io::Result<Harness> {
    let meta = fields(&dir.join("index.toml"))?;
    let mut capabilities = Vec::new();
    for capability in Capability::ALL {
        capabilities.push(load_capability(dir, capability)?);
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

fn load_capability(dir: &Path, capability: Capability) -> io::Result<CapabilityPlan> {
    let path = dir.join(capability.as_str()).join("index.toml");
    let data = fields(&path)?;
    let command = parser::string(&data, "command").map_err(invalid)?;
    Ok(CapabilityPlan {
        capability,
        summary: parser::string(&data, "summary").map_err(invalid)?,
        command: CommandPlan::new(command, parser::list(&data, "args").map_err(invalid)?),
    })
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

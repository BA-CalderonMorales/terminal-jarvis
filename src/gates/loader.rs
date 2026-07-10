use crate::catalog::parser;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const EMBEDDED_TRIVY: &str = include_str!("../../gates/trivy/index.toml");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gate {
    pub name: String,
    pub display: String,
    pub description: String,
    pub binary: String,
    pub args: Vec<String>,
    pub install_hint: String,
}

pub fn load(root: &Path) -> io::Result<Vec<Gate>> {
    if !root.is_dir() {
        if configured_root() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                root.display().to_string(),
            ));
        }
        return Ok(vec![parse(EMBEDDED_TRIVY)?]);
    }
    let mut dirs = fs::read_dir(root)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<io::Result<Vec<PathBuf>>>()?;
    dirs.sort();
    let gates = dirs
        .into_iter()
        .filter(|path| path.is_dir())
        .map(|path| read(&path.join("index.toml")))
        .collect::<io::Result<Vec<_>>>()?;
    if gates.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "gate catalog is empty",
        ));
    }
    Ok(gates)
}

fn configured_root() -> bool {
    std::env::var_os("TERMINAL_JARVIS_GATES").is_some_and(|value| !value.is_empty())
}

fn read(path: &Path) -> io::Result<Gate> {
    parse(&fs::read_to_string(path)?)
}

fn parse(data: &str) -> io::Result<Gate> {
    let fields = parser::parse(data).map_err(invalid)?;
    Ok(Gate {
        name: parser::string(&fields, "name").map_err(invalid)?,
        display: parser::string(&fields, "display").map_err(invalid)?,
        description: parser::string(&fields, "description").map_err(invalid)?,
        binary: parser::string(&fields, "binary").map_err(invalid)?,
        args: parser::list(&fields, "args").map_err(invalid)?,
        install_hint: parser::string(&fields, "install_hint").map_err(invalid)?,
    })
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(test)]
#[path = "loader_test.rs"]
mod tests;

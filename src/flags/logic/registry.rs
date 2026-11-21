//! Loads the registry: embedded by default, a directory when overridden.

use super::{embedded, registry_parse::parse};
use crate::flags::structs::flag::Flag;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Every declared flag, sorted by name. A missing override directory and any
/// schema violation are errors; an empty registry is valid.
pub fn load(root: &Path) -> Result<Vec<Flag>, String> {
    if root == Path::new(crate::context::EMBEDDED_FLAGS) {
        return build(
            embedded::FILES.iter().filter_map(|(path, data)| {
                path.strip_suffix("/index.toml").map(|dir| (dir, *data))
            }),
        );
    }
    if !root.is_dir() {
        return Err(format!(
            "flag registry missing at {}; fix TERMINAL_JARVIS_FLAGS or unset it",
            root.display()
        ));
    }
    let mut sources = BTreeMap::new();
    let entries = fs::read_dir(root).map_err(|error| error.to_string())?;
    for entry in entries.flatten().filter(|entry| entry.path().is_dir()) {
        let dir = entry.file_name().to_string_lossy().to_string();
        let path = entry.path().join("index.toml");
        let data = fs::read_to_string(&path)
            .map_err(|error| format!("flag '{dir}': cannot read {}: {error}", path.display()))?;
        sources.insert(dir, data);
    }
    build(
        sources
            .iter()
            .map(|(dir, data)| (dir.as_str(), data.as_str())),
    )
}

/// Parses `(directory, contents)` pairs: the string entry point tests use.
pub fn build<'a>(files: impl Iterator<Item = (&'a str, &'a str)>) -> Result<Vec<Flag>, String> {
    let mut flags: Vec<Flag> = Vec::new();
    for (dir, data) in files {
        let flag = parse(dir, data)?;
        if flags.iter().any(|known| known.name == flag.name) {
            return Err(format!("flag '{}' is declared twice", flag.name));
        }
        flags.push(flag);
    }
    flags.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(flags)
}

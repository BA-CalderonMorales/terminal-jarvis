use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    pub name: String,
    pub source: &'static str,
}

pub fn selected(home: &Path) -> io::Result<Option<Selection>> {
    if let Some(value) = env::var_os("TERMINAL_JARVIS_GATE").filter(|value| !value.is_empty()) {
        let name = value.to_string_lossy().to_string();
        return Ok((name != "off").then_some(Selection {
            name,
            source: "environment",
        }));
    }
    let path = home.join("gate.toml");
    if !path.exists() {
        return Ok(None);
    }
    let name = fs::read_to_string(path)?
        .lines()
        .find_map(|line| line.trim().strip_prefix("enabled = "))
        .and_then(|value| value.strip_prefix('"'))
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_string);
    Ok(name.map(|name| Selection {
        name,
        source: "config",
    }))
}

pub fn enable(home: &Path, name: &str) -> io::Result<()> {
    fs::create_dir_all(home)?;
    fs::write(home.join("gate.toml"), format!("enabled = \"{name}\"\n"))
}

pub fn disable(home: &Path) -> io::Result<()> {
    let path = home.join("gate.toml");
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

//! The strict `settings.toml` reader and writer. Only the `[flags]` table
//! matters: `name = true|false`. Other tables are skipped; anything else
//! malformed makes the whole file fall back to defaults (with a warning).

use crate::flags::constants::fixed::SETTINGS_TABLE;
use std::collections::BTreeMap;

pub type Choices = BTreeMap<String, bool>;

pub fn parse(data: &str) -> Result<Choices, String> {
    let mut choices = Choices::new();
    let mut table = None;
    for (index, raw) in data.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(header) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            table = Some(header.trim().to_string());
            continue;
        }
        if table.as_deref().is_some_and(|name| name != SETTINGS_TABLE) {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {} is missing '='", index + 1))?;
        let value = match value.trim() {
            "true" => true,
            "false" => false,
            other => {
                return Err(format!(
                    "line {}: '{other}' is not true or false",
                    index + 1
                ))
            }
        };
        if table.is_none() {
            return Err(format!("line {} is outside a table", index + 1));
        }
        choices.insert(key.trim().to_string(), value);
    }
    Ok(choices)
}

pub fn render(choices: &Choices) -> String {
    let mut out = format!("# Terminal Jarvis settings\n[{SETTINGS_TABLE}]\n");
    for (name, value) in choices {
        out.push_str(&format!("{name} = {value}\n"));
    }
    out
}

//! One flag file -> one validated `Flag`. Every failure is a schema error
//! that names the flag directory and the rule that was broken.

use crate::catalog::parser;
use crate::flags::constants::fixed::{MAX_DESCRIPTION, MAX_NAME};
use crate::flags::structs::flag::{Flag, Stage};

const KEYS: [&str; 5] = ["name", "default", "stage", "description", "since"];

pub fn parse(dir: &str, data: &str) -> Result<Flag, String> {
    let fail = |message: String| format!("flag '{dir}': {message}");
    let fields = parser::parse(data).map_err(fail)?;
    if let Some(key) = fields.keys().find(|key| !KEYS.contains(&key.as_str())) {
        return Err(fail(format!("unknown field '{key}'")));
    }
    let name = parser::string(&fields, "name").map_err(fail)?;
    if name != dir {
        return Err(fail(format!("name '{name}' must equal its directory")));
    }
    if !kebab(&name) {
        return Err(fail("name must be kebab-case (a-z, 0-9, dashes)".into()));
    }
    let default = match fields.get("default").map(String::as_str) {
        Some("true") => true,
        Some("false") => false,
        _ => return Err(fail("default must be true or false".into())),
    };
    let stage = match parser::string(&fields, "stage").map_err(fail)?.as_str() {
        "stable" => Stage::Stable,
        "preview" => Stage::Preview,
        "labs" => Stage::Labs,
        other => {
            return Err(fail(format!(
                "stage '{other}' must be stable, preview, or labs"
            )))
        }
    };
    if stage == Stage::Labs && default {
        return Err(fail("a labs flag must default to false".into()));
    }
    let description = parser::string(&fields, "description").map_err(fail)?;
    if description.trim().is_empty() || description.chars().count() > MAX_DESCRIPTION {
        return Err(fail(format!(
            "description must be 1-{MAX_DESCRIPTION} characters"
        )));
    }
    let since = parser::string(&fields, "since").map_err(fail)?;
    if !version(&since) {
        return Err(fail(format!("since '{since}' must look like 0.1.26")));
    }
    Ok(Flag {
        name,
        default,
        stage,
        description,
        since,
    })
}

pub fn kebab(name: &str) -> bool {
    let part = |part: &str| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    };
    name.len() <= MAX_NAME
        && name.starts_with(|c: char| c.is_ascii_lowercase())
        && name.split('-').all(part)
}

fn version(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}

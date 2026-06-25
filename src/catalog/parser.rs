use std::collections::BTreeMap;

pub type Fields = BTreeMap<String, String>;

pub fn parse(input: &str) -> Result<Fields, String> {
    let mut fields = BTreeMap::new();
    for (index, raw) in input.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {} is missing '='", index + 1))?;
        fields.insert(key.trim().to_string(), value.trim().to_string());
    }
    Ok(fields)
}

pub fn string(fields: &Fields, key: &str) -> Result<String, String> {
    let value = fields.get(key).ok_or_else(|| format!("missing '{key}'"))?;
    strip_quotes(value.trim())
}

pub fn list(fields: &Fields, key: &str) -> Result<Vec<String>, String> {
    let Some(value) = fields.get(key) else {
        return Ok(Vec::new());
    };
    let inner = value
        .trim()
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| format!("'{key}' must be a string array"))?;
    if inner.trim().is_empty() {
        return Ok(Vec::new());
    }
    inner
        .split(',')
        .map(|part| strip_quotes(part.trim()))
        .collect()
}

fn strip_quotes(value: &str) -> Result<String, String> {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_string)
        .ok_or_else(|| format!("expected quoted string, got {value}"))
}

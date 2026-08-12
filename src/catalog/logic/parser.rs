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
        let key = key.trim().to_string();
        if fields
            .insert(key.clone(), value.trim().to_string())
            .is_some()
        {
            return Err(format!("duplicate key '{key}' on line {}", index + 1));
        }
    }
    Ok(fields)
}

pub fn string(fields: &Fields, key: &str) -> Result<String, String> {
    let value = fields.get(key).ok_or_else(|| format!("missing '{key}'"))?;
    strip_quotes(value.trim())
}

pub fn optional_string(fields: &Fields, key: &str) -> Result<Option<String>, String> {
    let Some(value) = fields.get(key) else {
        return Ok(None);
    };
    strip_quotes(value.trim()).map(Some)
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
    split_list(inner)
}

fn split_list(input: &str) -> Result<Vec<String>, String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    for character in input.chars() {
        match character {
            '"' => {
                quoted = !quoted;
                current.push(character);
            }
            ',' if !quoted => {
                values.push(strip_quotes(current.trim())?);
                current.clear();
            }
            _ => current.push(character),
        }
    }
    if quoted {
        return Err("unterminated quoted string array value".to_string());
    }
    values.push(strip_quotes(current.trim())?);
    Ok(values)
}

fn strip_quotes(value: &str) -> Result<String, String> {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_string)
        .ok_or_else(|| format!("expected quoted string, got {value}"))
}

#[cfg(test)]
#[path = "../tests/parser_test.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/parser_props.rs"]
mod props;

use std::path::{Path, PathBuf};

pub struct Redactor<'a> {
    home: Option<&'a Path>,
    temp: Option<&'a Path>,
}

impl<'a> Redactor<'a> {
    pub fn new(home: Option<&'a PathBuf>, temp: Option<&'a PathBuf>) -> Self {
        Self {
            home: home.map(PathBuf::as_path),
            temp: temp.map(PathBuf::as_path),
        }
    }

    pub fn full(&self, path: &Path) -> String {
        if let Some(value) = prefixed(path, self.home, "~") {
            return value;
        }
        if let Some(value) = prefixed(path, self.temp, "$TMP") {
            return value;
        }
        clean(&path.to_string_lossy())
    }

    pub fn minimal(&self, path: &Path) -> String {
        path.file_name()
            .map(|name| clean(&name.to_string_lossy()))
            .unwrap_or_else(|| "<path>".into())
    }
}

pub fn segment(value: &str) -> String {
    if !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
    {
        value.to_string()
    } else {
        "invalid".to_string()
    }
}

pub fn clean(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_control() { '?' } else { c })
        .collect()
}

fn prefixed(path: &Path, prefix: Option<&Path>, label: &str) -> Option<String> {
    let rest = path.strip_prefix(prefix?).ok()?;
    if rest.as_os_str().is_empty() {
        return Some(label.to_string());
    }
    Some(format!(
        "{label}/{}",
        clean(&rest.to_string_lossy()).trim_start_matches(['/', '\\'])
    ))
}

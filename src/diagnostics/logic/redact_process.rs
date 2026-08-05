use super::redact::{clean, prefixed};
use std::path::{Path, PathBuf};

pub fn text(value: &str) -> String {
    let mut output = clean(value);
    for (name, label) in [
        ("HOME", "~"),
        ("USERPROFILE", "~"),
        ("TMPDIR", "$TMP"),
        ("TMP", "$TMP"),
        ("TEMP", "$TMP"),
    ] {
        if let Some(prefix) = std::env::var_os(name) {
            output = replace_prefix(&output, &prefix.to_string_lossy(), label);
        }
    }
    replace_prefix(&output, &std::env::temp_dir().to_string_lossy(), "$TMP")
}

pub fn path(path: &Path) -> String {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from);
    if let Some(value) = prefixed(path, home.as_deref(), "~") {
        return value;
    }
    if path.starts_with(std::env::temp_dir()) {
        return path
            .file_name()
            .map(|name| format!("$TMP/{}", clean(&name.to_string_lossy())))
            .unwrap_or_else(|| "$TMP".to_string());
    }
    path.file_name()
        .map(|name| clean(&name.to_string_lossy()))
        .unwrap_or_else(|| "<path>".to_string())
}

fn replace_prefix(value: &str, prefix: &str, label: &str) -> String {
    if prefix.len() > 1 {
        value.replacen(prefix, label, 1)
    } else {
        value.to_string()
    }
}

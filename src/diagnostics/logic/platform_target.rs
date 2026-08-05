use super::DiagnosticInput;

pub fn supported(input: &DiagnosticInput) -> bool {
    matches!(
        (
            input.platform.os.as_str(),
            input.platform.arch.as_str(),
            input.platform.libc.as_str()
        ),
        ("linux", "x86_64" | "aarch64", "gnu")
            | ("macos", "x86_64" | "aarch64", "n/a")
            | ("windows", "x86_64", "n/a")
    ) && input.platform.wsl != "wsl1-or-unknown"
}

pub fn name(input: &DiagnosticInput) -> String {
    match (
        input.platform.os.as_str(),
        input.platform.arch.as_str(),
        input.platform.libc.as_str(),
    ) {
        ("linux", "x86_64", "gnu") => "linux-x64-gnu".into(),
        ("linux", "aarch64", "gnu") => "linux-arm64-gnu".into(),
        ("macos", "x86_64", "n/a") => "macos-x64".into(),
        ("macos", "aarch64", "n/a") => "macos-arm64".into(),
        ("windows", "x86_64", "n/a") => "windows-x64-msvc".into(),
        _ => "unsupported".into(),
    }
}

pub fn allowed<'a>(value: &'a str, values: &[&str]) -> Option<&'a str> {
    values.contains(&value).then_some(value)
}

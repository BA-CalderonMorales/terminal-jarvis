use super::{Code, DiagnosticInput, Record, Severity, ValueState};
use std::path::Path;

pub fn collect(input: &DiagnosticInput) -> (Vec<Record>, bool) {
    let os = super::platform_target::allowed(&input.platform.os, &["linux", "macos", "windows"]);
    let arch = super::platform_target::allowed(&input.platform.arch, &["x86_64", "aarch64"]);
    let libc = super::platform_target::allowed(&input.platform.libc, &["gnu", "n/a"]);
    let target = super::platform_target::supported(input);
    let mut records = vec![
        fact("platform.os", os),
        fact("platform.arch", arch),
        fact("platform.libc", libc),
        shell(input),
        wsl(input),
        Record::new(
            "platform.target",
            if target {
                Code::Ready
            } else {
                Code::Unsupported
            },
            if target {
                Severity::Info
            } else {
                Severity::Error
            },
            super::platform_target::name(input),
        ),
    ];
    if !target {
        records.last_mut().unwrap().action = Some("use a supported native target".into());
    }
    (records, target)
}

fn fact(key: &str, value: Option<&str>) -> Record {
    match value {
        Some(value) => Record::new(key, Code::Ready, Severity::Info, value),
        None => Record::new(key, Code::Unsupported, Severity::Error, "other")
            .action("use a supported native target"),
    }
}

fn shell(input: &DiagnosticInput) -> Record {
    let name = if input.platform.os == "windows" {
        "COMSPEC"
    } else {
        "SHELL"
    };
    let state = input.environment.state(name);
    let value = if state == ValueState::Present {
        input
            .environment
            .text(name)
            .and_then(|v| Path::new(v).file_name())
            .map(|v| super::redact::clean(&v.to_string_lossy()))
            .unwrap_or_else(|| "unknown".into())
    } else {
        state.as_str().to_string()
    };
    Record::new("platform.shell", state.code(), Severity::Warning, value)
}

fn wsl(input: &DiagnosticInput) -> Record {
    match input.platform.wsl.as_str() {
        "no" | "wsl2" => Record::new(
            "platform.wsl",
            Code::Ready,
            Severity::Info,
            &input.platform.wsl,
        ),
        "wsl1-or-unknown" => Record::new(
            "platform.wsl",
            Code::Unsupported,
            Severity::Error,
            "wsl1-or-unknown",
        ),
        _ => Record::new("platform.wsl", Code::Malformed, Severity::Error, "unknown"),
    }
}

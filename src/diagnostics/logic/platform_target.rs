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
    name_for(
        &input.platform.os,
        &input.platform.arch,
        &input.platform.libc,
    )
}

fn name_for(os: &str, arch: &str, libc: &str) -> String {
    match (os, arch, libc) {
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

#[cfg(test)]
mod tests {
    use super::super::{PlatformInput, RuntimeInput};
    use super::*;
    use std::path::Path;

    fn target(os: &str, arch: &str, libc: &str) -> DiagnosticInput {
        let mut input = DiagnosticInput::local(
            Path::new("/tmp/tj-catalog"),
            Path::new("/tmp/tj-home"),
            None,
            &[],
            RuntimeInput {
                gate: "/tmp/tj-gate".into(),
                stdout_tty: false,
                stderr_tty: false,
                color: false,
                width: 0,
                update_route: String::new(),
                checksum: String::new(),
                probes: false,
            },
        );
        input.platform = PlatformInput {
            os: os.into(),
            arch: arch.into(),
            libc: libc.into(),
            wsl: "no".into(),
        };
        input
    }

    #[test]
    fn supported_accepts_known_triples_and_rejects_others() {
        assert!(supported(&target("linux", "x86_64", "gnu")));
        assert!(supported(&target("macos", "aarch64", "n/a")));
        assert!(supported(&target("windows", "x86_64", "n/a")));
        assert!(!supported(&target("freebsd", "x86_64", "n/a")));
        assert!(!supported(&target("linux", "x86_64", "musl")));
        let mut wsl1 = target("linux", "x86_64", "gnu");
        wsl1.platform.wsl = "wsl1-or-unknown".into();
        assert!(!supported(&wsl1));
    }

    #[test]
    fn name_for_maps_known_platforms() {
        assert_eq!(name_for("linux", "x86_64", "gnu"), "linux-x64-gnu");
        assert_eq!(name_for("linux", "aarch64", "gnu"), "linux-arm64-gnu");
        assert_eq!(name_for("macos", "x86_64", "n/a"), "macos-x64");
        assert_eq!(name_for("macos", "aarch64", "n/a"), "macos-arm64");
        assert_eq!(name_for("windows", "x86_64", "n/a"), "windows-x64-msvc");
        assert_eq!(name_for("linux", "x86_64", "musl"), "unsupported");
        assert_eq!(name(&target("linux", "x86_64", "gnu")), "linux-x64-gnu");
    }

    #[test]
    fn allowed_returns_value_only_when_listed() {
        assert_eq!(allowed("xh", &["xh", "claude"]), Some("xh"));
        assert_eq!(allowed("none", &["xh"]), None);
    }
}

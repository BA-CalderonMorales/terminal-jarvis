#![cfg(unix)]
use crate::logic::cli_driver::Fixture;
use crate::logic::pty;
use crate::logic::tui::{launch, EXIT, FRAME, SERIAL};
#[test]
fn direct_lifecycle_checks_the_agent_without_adopting_it() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for (command, capability, target_present) in [
        ("fixture download\n", "download", false),
        ("fixture download\n", "download", true),
        ("run fixture download\n", "download", false),
        ("fixture update\n", "update", false),
        ("run fixture update\n", "update", false),
    ] {
        let fixture = Fixture::new(
            "expected",
            "expected",
            "#!/bin/sh\necho installer-completed\n",
        );
        let root = fixture.root.join("catalog/fixture");
        let other = fixture.root.join("catalog/other");
        std::fs::create_dir_all(&other).unwrap();
        for entry in std::fs::read_dir(&root).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                std::fs::create_dir_all(other.join(entry.file_name())).unwrap();
                std::fs::copy(
                    entry.path().join("index.toml"),
                    other.join(entry.file_name()).join("index.toml"),
                )
                .unwrap();
            }
        }
        let descriptor = std::fs::read_to_string(root.join("index.toml")).unwrap();
        std::fs::write(
            other.join("index.toml"),
            descriptor.replace("name = \"fixture\"", "name = \"other\""),
        )
        .unwrap();
        if !target_present {
            std::fs::write(
                root.join("index.toml"),
                descriptor.replace(
                    "binary = \"fixture-child\"",
                    "binary = \"tj-absent-target-binary\"",
                ),
            )
            .unwrap();
        }
        fixture.run(&["--plain", "use", "other"]);
        let prompt = format!("Continue with {capability}:fixture? [Y/n]");
        let result = if target_present {
            "downloaded fixture"
        } else {
            "binary was not found on PATH"
        };
        let (status, _) = pty::run_pty_probe(
            launch(&fixture, &[]),
            &[
                (b"", Some(FRAME)),
                (command.as_bytes(), None),
                (b"y\n", Some(prompt.as_bytes())),
                (b"", Some(result.as_bytes())),
                EXIT,
            ],
        );
        assert!(status.success());
        let current = fixture.run(&["--plain", "current"]);
        assert!(String::from_utf8_lossy(&current.stdout).contains("active harness = other"));
    }
}

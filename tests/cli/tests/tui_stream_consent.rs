#![cfg(unix)]
use crate::logic::cli_driver::Fixture;
use crate::logic::pty;
use crate::logic::tui::{launch, EXIT, FRAME, SERIAL};

#[test]
fn an_explicit_dangerous_request_still_runs_after_confirmation() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let fixture = Fixture::new(
        "expected",
        "expected",
        "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\necho agent-completed\n",
    );
    let path = fixture.root.join("catalog/fixture/headless/index.toml");
    let text = std::fs::read_to_string(&path).unwrap();
    std::fs::write(
        path,
        text.replace("effect = \"read-only\"", "effect = \"dangerous\""),
    )
    .unwrap();
    fixture.run(&["--plain", "use", "fixture"]);
    let (status, _) = pty::run_pty_probe(
        launch(&fixture, &[]),
        &[
            (b"", Some(FRAME)),
            (b"run fixture headless --allow-dangerous\n", None),
            (b"y\n", Some(b"Continue with headless:fixture? [y/N]")),
            (b"", Some(b"agent-completed")),
            EXIT,
        ],
    );
    assert!(status.success());
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}

#[test]
fn streaming_keeps_the_original_dangerous_intent_requirement() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for command in [
        "run headless --allow-dangerous\n",
        "fixture headless --allow-dangerous\n",
    ] {
        let fixture = Fixture::new(
            "expected",
            "expected",
            "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n",
        );
        let path = fixture.root.join("catalog/fixture/headless/index.toml");
        let text = std::fs::read_to_string(&path).unwrap();
        std::fs::write(
            path,
            text.replace("effect = \"read-only\"", "effect = \"dangerous\""),
        )
        .unwrap();
        fixture.run(&["--plain", "use", "fixture"]);
        let (status, _) = pty::run_pty_probe(
            launch(&fixture, &[]),
            &[
                (b"", Some(FRAME)),
                (command.as_bytes(), None),
                (b"", Some(b"dangerous_opt_in_required")),
                EXIT,
            ],
        );
        assert!(status.success());
        assert!(!fixture.spawned());
        assert!(!fixture.gate_spawned());
    }
}

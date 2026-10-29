#![cfg(unix)]
use crate::logic::cli_driver::Fixture;
use crate::logic::pty;
use crate::logic::tui::{launch, EXIT, FRAME, SERIAL};

#[test]
fn dangerous_conversations_require_an_opt_in_and_a_visible_confirmation() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for opt_in in [false, true] {
        let fixture = Fixture::new(
            "expected",
            "expected",
            concat!(
                "#!/bin/sh\nif [ $# -gt 0 ]; then : > \"$TJ_FIXTURE_MARKER\"; fi\necho reply\n"
            ),
        );
        let path = fixture.root.join("catalog/fixture/headless/index.toml");
        let source = std::fs::read_to_string(&path).unwrap();
        std::fs::write(
            path,
            source.replace("effect = \"read-only\"", "effect = \"dangerous\""),
        )
        .unwrap();
        fixture.run(&["--plain", "use", "fixture"]);
        let command = format!(
            "converse {}1 fixture fixture topic\n",
            if opt_in { "--allow-dangerous " } else { "" }
        );
        let mut stages: Vec<(&[u8], Option<&[u8]>)> = vec![
            (b"", Some(FRAME)),
            (command.as_bytes(), None),
            (b"", Some(b"again to accept")),
            (command.as_bytes(), None),
        ];
        if opt_in {
            stages.push((b"n\n", Some(b"Continue with headless:fixture? [y/N]")));
        } else {
            stages.push((b"", Some(b"dangerous_opt_in_required")));
        }
        stages.push((b"", Some(b"converse ended early")));
        stages.push(EXIT);
        let (status, _) =
            pty::run_pty_probe(launch(&fixture, &[("TJ_DEBUG_SIZE", "80x24")]), &stages);
        assert!(status.success());
        assert!(
            !fixture.spawned(),
            "a refused conversation spawned an agent"
        );
        assert!(
            !fixture.gate_spawned(),
            "refusal must precede the workspace scan"
        );
    }
}

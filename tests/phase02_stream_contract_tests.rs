#![cfg(unix)]

mod phase02_fixture;

use phase02_fixture::Fixture;

#[test]
fn child_bytes_stay_on_matching_streams_and_exit_is_exact() {
    let fixture = Fixture::new(
        "expected",
        "expected",
        "#!/bin/sh\nprintf 'out\\377\\n'\nprintf 'err\\376\\n' >&2\nexit 37\n",
    );
    let output = fixture.run(&["--plain", "run", "fixture", "headless", "--", "payload"]);
    assert_eq!(output.status.code(), Some(37));
    assert_eq!(output.stdout, b"out\xff\n");
    assert!(output.stderr.starts_with(b"err\xfe\n"));
    let diagnostic = String::from_utf8_lossy(&output.stderr[b"err\xfe\n".len()..]);
    assert!(diagnostic.contains("capability 'headless' failed with exit 37"));
    assert!(!fixture.spawned());
    assert!(fixture.gate_spawned());
}

#[test]
fn successful_child_stderr_is_not_discarded_or_crossed() {
    let fixture = Fixture::new(
        "expected",
        "expected",
        "#!/bin/sh\nprintf 'primary\\n'\nprintf 'warning\\n' >&2\n",
    );
    let output = fixture.run(&["--plain", "run", "fixture", "headless"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"primary\n");
    assert_eq!(output.stderr, b"warning\n");
}

#[test]
fn unix_signal_exit_is_preserved_with_a_parent_diagnostic() {
    let fixture = Fixture::new(
        "expected",
        "expected",
        "#!/bin/sh\nprintf 'before\\n'\nprintf 'signal\\n' >&2\nkill -TERM $$\n",
    );
    let output = fixture.run(&["--plain", "run", "fixture", "headless"]);
    assert_eq!(output.status.code(), Some(143));
    assert_eq!(output.stdout, b"before\n");
    assert!(output.stderr.starts_with(b"signal\n"));
    assert!(String::from_utf8_lossy(&output.stderr).contains("failed with exit 143"));
}

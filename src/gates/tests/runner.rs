use crate::gates::logic::loader::Gate;
use crate::gates::logic::runner::preflight;
use crate::gates::logic::stream::run;

use crate::gates::tests_util::*;

#[cfg(unix)]
#[test]
fn block_summary_drops_info_lines_and_keeps_the_signal_tail() {
    let verbatim = "2026 INFO [vuln] enabled\n2026 WARN spam\n2026 INFO secret\n2026 FATAL fs scan error: stat file: no such file\n";
    assert_eq!(
        crate::gates::logic::stream::block_summary(verbatim),
        "2026 WARN spam\n2026 FATAL fs scan error: stat file: no such file"
    );
    assert_eq!(
        crate::gates::logic::stream::block_summary("2026 INFO [vuln] enabled\n"),
        "2026 INFO [vuln] enabled"
    );
}
#[cfg(unix)]
#[test]
fn preflight_accepts_success_and_reports_blocking_exit() {
    let _guard = lock();
    let root = std::env::temp_dir().join(format!("tj-preflight-{}", std::process::id()));
    let home = root.join("home");
    let catalog = root.join("catalog");
    let _ = std::fs::remove_dir_all(&root);
    write_gate(&catalog, "pass", "true");
    write_gate(&catalog, "block", "false");
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    crate::gates::enable(&home, "pass").unwrap();
    assert!(preflight(&home, true).is_ok());
    crate::gates::enable(&home, "block").unwrap();
    let error = preflight(&home, true).unwrap_err();
    assert!(error.contains("blocked harness execution (exit 1)"));
    restore_gates_env(previous);
    let _ = std::fs::remove_dir_all(root);
}
#[cfg(unix)]
#[test]
fn run_streams_gate_output_and_captures_it() {
    let script = "#!/bin/sh\nprintf 'streamed-1\\n'\nprintf 'streamed-2\\n' >&2\n";
    let dir = std::env::temp_dir().join(format!("tj-gate-stream-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let bin = dir.join("scan");
    write_executable(&bin, script);
    let gate = Gate {
        name: "scan".to_string(),
        display: "Scan".to_string(),
        description: "test".to_string(),
        binary: bin.to_string_lossy().into_owned(),
        args: vec![],
        install_hint: "install".to_string(),
    };
    let (code, body) = run(&gate, true).unwrap();
    assert_eq!(code, 0);
    assert!(body.contains("streamed-1"));
    assert!(body.contains("streamed-2"));
    let _ = std::fs::remove_dir_all(&dir);
}
#[cfg(unix)]
#[test]
fn preflight_reports_exit_128_as_blocked_and_signals_as_interrupted() {
    let _guard = lock();
    let root = std::env::temp_dir().join(format!("tj-preflight-codes-{}", std::process::id()));
    let home = root.join("home");
    let catalog = root.join("catalog");
    let _ = std::fs::remove_dir_all(&root);
    for (name, script) in [
        ("hundred", "#!/bin/sh\nexit 128\n"),
        ("hung", "#!/bin/sh\nkill -HUP $$\n"),
    ] {
        write_gate(&catalog, name, "placeholder");
        let binary = catalog.join(name).join("scan");
        write_executable(&binary, script);
        std::fs::write(
            catalog.join(name).join("index.toml"),
            format!(
                "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{}\"\nargs = []\ninstall_hint = \"install\"\n",
                binary.display()
            ),
        )
        .unwrap();
    }
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    crate::gates::enable(&home, "hundred").unwrap();
    let error = preflight(&home, true).unwrap_err();
    assert!(
        error.contains("blocked harness execution (exit 128)"),
        "{error}"
    );
    crate::gates::enable(&home, "hung").unwrap();
    let error = preflight(&home, false).unwrap_err();
    assert!(error.contains("was interrupted (Ctrl+C)"), "{error}");
    restore_gates_env(previous);
    let _ = std::fs::remove_dir_all(root);
}

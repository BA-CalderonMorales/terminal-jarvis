use super::*;

fn harness(name: &str) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: "probe".into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

fn options() -> args::Options {
    args::Options::default()
}

fn local() -> (std::path::PathBuf, std::path::PathBuf) {
    (std::env::temp_dir(), std::env::temp_dir())
}

fn call(hs: &[Harness], root: &std::path::Path, home: &std::path::Path, input: &str) -> Next {
    let mut sink = Vec::new();
    let previous = crate::cli::style::set(true, true);
    let next = handle(&mut sink, hs, root, home, &options(), input);
    crate::cli::style::restore(previous);
    next
}

#[test]
fn handle_dispatch_exit_empty_home_and_actions() {
    let hs = [harness("alpha")];
    let (root, home) = local();
    assert!(matches!(call(&hs, &root, &home, "/exit"), Next::Exit));
    again_false(call(&hs, &root, &home, ""));
    again_true(call(&hs, &root, &home, "/home"));
    again_true(call(&hs, &root, &home, "/clear"));
}

#[test]
fn actions_the_headless_cli_pre_routes_never_panic() {
    let hs = [harness("alpha")];
    let (root, home) = local();
    for input in ["version", "/version", "help run", "self-update", "tui"] {
        again_false(call(&hs, &root, &home, input));
    }
}

#[test]
fn handle_marks_picker_shown_only_after_list() {
    let hs = [harness("alpha")];
    let (root, home) = local();
    assert!(matches!(
        call(&hs, &root, &home, "/list"),
        Next::Again {
            picker_shown: true,
            reset: false
        }
    ));
    again_false(call(&hs, &root, &home, "1"));
    again_false(call(&hs, &root, &home, "/bogus"));
}

fn again_false(next: Next) {
    assert!(
        matches!(
            next,
            Next::Again {
                picker_shown: false,
                reset: false
            }
        ),
        "{next:?}"
    );
}

fn again_true(next: Next) {
    assert!(
        matches!(
            next,
            Next::Again {
                picker_shown: false,
                reset: true
            }
        ),
        "{next:?}"
    );
}

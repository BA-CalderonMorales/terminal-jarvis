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

#[test]
fn handle_dispatch_exit_empty_home_and_actions() {
    let previous = crate::cli::style::set(true, true);
    let (catalog_root, state_home) = local();
    let harnesses = [harness("alpha")];
    assert!(matches!(
        handle(&harnesses, &catalog_root, &state_home, &options(), "/exit"),
        Next::Exit
    ));
    assert!(matches!(
        handle(&harnesses, &catalog_root, &state_home, &options(), ""),
        Next::Again {
            picker_shown: false
        }
    ));
    assert!(matches!(
        handle(&harnesses, &catalog_root, &state_home, &options(), "/home"),
        Next::Again {
            picker_shown: false
        }
    ));
    assert!(matches!(
        handle(&harnesses, &catalog_root, &state_home, &options(), "/clear"),
        Next::Again {
            picker_shown: false
        }
    ));
    crate::cli::style::restore(previous);
}

#[test]
fn actions_the_headless_cli_pre_routes_never_panic() {
    let previous = crate::cli::style::set(true, true);
    let (catalog_root, state_home) = local();
    let harnesses = [harness("alpha")];
    for input in ["version", "/version", "help run", "self-update", "tui"] {
        assert!(
            matches!(
                handle(&harnesses, &catalog_root, &state_home, &options(), input),
                Next::Again {
                    picker_shown: false
                }
            ),
            "{input}"
        );
    }
    crate::cli::style::restore(previous);
}

#[test]
fn handle_marks_picker_shown_only_after_list() {
    let previous = crate::cli::style::set(true, true);
    let (catalog_root, state_home) = local();
    let harnesses = [harness("alpha")];
    assert!(matches!(
        handle(&harnesses, &catalog_root, &state_home, &options(), "/list"),
        Next::Again { picker_shown: true }
    ));
    assert!(matches!(
        handle(&harnesses, &catalog_root, &state_home, &options(), "1"),
        Next::Again {
            picker_shown: false
        }
    ));
    assert!(matches!(
        handle(&harnesses, &catalog_root, &state_home, &options(), "/bogus"),
        Next::Again {
            picker_shown: false
        }
    ));
    crate::cli::style::restore(previous);
}

use super::*;

fn candidates_nonempty(command: String, windows: bool, path_ext: String) -> bool {
    !candidates(&command, windows, &path_ext).is_empty()
}

/// The exact original command string is always one of the candidates (as a
/// fallback if nothing else). On Windows without an existing extension it is
/// now the *last* candidate, not the first — PATHEXT-suffixed names are
/// tried first so a bare, unrunnable file (an extensionless POSIX shim next
/// to a real `.cmd`/`.exe`) does not shadow the real entry point.
fn candidates_preserve_command(command: String, windows: bool, path_ext: String) -> bool {
    let names = candidates(&command, windows, &path_ext);
    names.iter().any(|name| name == &command)
}

fn candidates_with_extension_are_singular(
    command: String,
    windows: bool,
    path_ext: String,
) -> bool {
    if command.is_empty() || command.contains('/') || command.contains('\\') {
        return true;
    }
    let with_ext = format!("{command}.sh");
    candidates(&with_ext, windows, &path_ext) == [with_ext.clone()]
}

fn missing_env_none_is_empty(env: Vec<String>) -> bool {
    let harness = Harness {
        name: "h".into(),
        display: "h".into(),
        description: "h".into(),
        binary: "h".into(),
        env_mode: EnvMode::None,
        env,
        capabilities: Vec::new(),
    };
    missing_env(&harness).is_empty()
}

#[test]
fn checks_properties() {
    quickcheck::quickcheck(candidates_nonempty as fn(String, bool, String) -> bool);
    quickcheck::quickcheck(candidates_preserve_command as fn(String, bool, String) -> bool);
    quickcheck::quickcheck(
        candidates_with_extension_are_singular as fn(String, bool, String) -> bool,
    );
    quickcheck::quickcheck(missing_env_none_is_empty as fn(Vec<String>) -> bool);
}

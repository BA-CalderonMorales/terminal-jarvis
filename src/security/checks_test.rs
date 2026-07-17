use super::{candidates, command_on_path, nonempty_env};

#[test]
fn windows_candidates_include_pathext_extensions() {
    assert_eq!(
        candidates("trivy", true, ".EXE;.CMD"),
        ["trivy", "trivy.EXE", "trivy.CMD"]
    );
}

#[test]
fn executable_extension_is_not_duplicated() {
    assert_eq!(candidates("trivy.exe", true, ".EXE"), ["trivy.exe"]);
}

#[cfg(unix)]
#[test]
fn explicit_path_requires_an_executable_file() {
    use std::os::unix::fs::PermissionsExt;

    let path = std::env::temp_dir().join(format!("tj-exec-probe-{}", std::process::id()));
    std::fs::write(&path, "probe").unwrap();
    assert!(!command_on_path(path.to_str().unwrap()));
    let mut permissions = std::fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o700);
    std::fs::set_permissions(&path, permissions).unwrap();
    assert!(command_on_path(path.to_str().unwrap()));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn directories_are_not_commands() {
    assert!(!command_on_path(std::env::temp_dir().to_str().unwrap()));
}

#[test]
fn environment_values_must_contain_non_whitespace() {
    let key = format!("TJ_ENV_PROBE_{}", std::process::id());
    std::env::set_var(&key, "");
    assert!(!nonempty_env(&key));
    std::env::set_var(&key, " \t ");
    assert!(!nonempty_env(&key));
    std::env::set_var(&key, "ready");
    assert!(nonempty_env(&key));
    std::env::remove_var(key);
}

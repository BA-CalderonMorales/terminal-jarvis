use super::{candidates, command_on_path};

#[test]
fn windows_candidates_try_pathext_extensions_before_the_bare_name() {
    assert_eq!(
        candidates("trivy", true, ".EXE;.CMD"),
        ["trivy.EXE", "trivy.CMD", "trivy"]
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

#[cfg(unix)]
#[test]
fn relative_slash_paths_depend_on_the_path_search() {
    use std::os::unix::fs::PermissionsExt;

    let cwd = std::env::current_dir().unwrap();
    let absolute = cwd.join("target/tj-rel-probe");
    std::fs::write(&absolute, "probe").unwrap();
    let mut permissions = std::fs::metadata(&absolute).unwrap().permissions();
    permissions.set_mode(0o700);
    std::fs::set_permissions(&absolute, permissions).unwrap();
    let probe = absolute
        .strip_prefix(&cwd)
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert!(command_on_path(&probe));
    std::fs::remove_file(&absolute).unwrap();
}

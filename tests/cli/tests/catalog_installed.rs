use crate::logic::cli_driver::Fixture;
use std::path::Path;
use std::process::Command;

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let target = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_tree(&path, &target);
        } else {
            std::fs::copy(path, target).unwrap();
        }
    }
}

#[test]
fn installed_catalog_discovery_keeps_prefix_priority_without_workspace_trust() {
    let fixture = Fixture::new("expected", "expected", "");
    let prefix = fixture.root.join("installation");
    let bin = prefix.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let executable = bin.join(format!("terminal-jarvis{}", std::env::consts::EXE_SUFFIX));
    std::fs::copy(env!("CARGO_BIN_EXE_terminal-jarvis"), &executable).unwrap();
    let workspace = fixture.root.join("workspace");
    std::fs::create_dir_all(workspace.join("harnesses/poison")).unwrap();
    let share = prefix.join("share/terminal-jarvis/harnesses");
    copy_tree(&fixture.root.join("catalog"), &share);
    let invoke = |args: &[&str]| spawn(&executable, &workspace, args);
    let list = invoke(&["--plain", "list"]);
    assert!(list.status.success(), "{list:?}");
    assert!(String::from_utf8_lossy(&list.stdout).contains("fixture support="));
    for root in [&share, &prefix.join("harnesses"), &bin.join("harnesses")] {
        if root != &share {
            copy_tree(&fixture.root.join("catalog"), root);
        }
        let version = invoke(&["--plain", "version", "--verbose"]);
        assert!(version.status.success());
        assert!(
            String::from_utf8_lossy(&version.stdout).contains(root.to_str().unwrap()),
            "wrong installed catalog selected: {version:?}"
        );
    }
}

/// A freshly copied binary can answer ETXTBSY on Linux while a forked sibling
/// still holds the write descriptor. Retry the exec; never skip the assertion.
fn spawn(executable: &Path, workspace: &Path, args: &[&str]) -> std::process::Output {
    for _ in 0..40 {
        let outcome = Command::new(executable)
            .args(args)
            .env_remove("TERMINAL_JARVIS_CATALOG")
            .current_dir(workspace)
            .output();
        match outcome {
            Err(error) if busy(&error) => {
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            result => return result.unwrap(),
        }
    }
    panic!("{} stayed busy", executable.display());
}

/// ETXTBSY, by raw errno: the typed kind needs a newer toolchain than the
/// repository's MSRV, and Windows has no equivalent race.
fn busy(error: &std::io::Error) -> bool {
    #[cfg(unix)]
    {
        error.raw_os_error() == Some(26)
    }
    #[cfg(not(unix))]
    {
        let _ = error;
        false
    }
}

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
    let invoke = |args: &[&str]| {
        Command::new(&executable)
            .args(args)
            .env_remove("TERMINAL_JARVIS_CATALOG")
            .current_dir(&workspace)
            .output()
            .unwrap()
    };
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

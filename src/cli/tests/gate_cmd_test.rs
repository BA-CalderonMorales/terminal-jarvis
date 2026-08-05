use super::*;
use crate::gates::Gate;

fn words(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn home() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("tj-gate-cmd-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    path
}

fn fake(name: &str) -> Gate {
    let (binary, args) = if cfg!(windows) {
        ("cmd", vec!["/c", "echo marker & exit /b 7"])
    } else {
        ("sh", vec!["-c", "printf marker; exit 7"])
    };
    Gate {
        name: name.to_string(),
        display: name.to_string(),
        description: "test".to_string(),
        binary: binary.to_string(),
        args: args.into_iter().map(str::to_string).collect(),
        install_hint: "install".to_string(),
    }
}

#[test]
fn handle_routes_every_gate_form() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let home = home();
    let default = handle(&[], &home).unwrap();
    assert_eq!(default, handle(&words(&["status"]), &home).unwrap());
    assert!(handle(&words(&["list"]), &home)
        .unwrap()
        .1
        .contains("Trivy"));
    assert!(handle(&words(&["enable"]), &home).is_ok());
    assert_eq!(
        crate::gates::selected(&home).unwrap().unwrap().name,
        "trivy"
    );
    assert!(handle(&words(&["disable"]), &home).is_ok());
    assert!(crate::gates::selected(&home).unwrap().is_none());
    assert!(handle(&words(&["enable", "trivy"]), &home).is_ok());
    assert!(handle(&words(&["run", "ghost"]), &home)
        .unwrap_err()
        .contains("unknown gate"));
    assert!(handle(&words(&["bogus", "trivy"]), &home)
        .unwrap_err()
        .starts_with("usage:"));
    assert!(handle(&words(&["bogus"]), &home)
        .unwrap_err()
        .starts_with("usage:"));
    let _ = std::fs::remove_dir_all(home);
}

#[test]
fn default_run_routes_to_trivy() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let root = std::env::temp_dir().join(format!("tj-gate-run-{}", std::process::id()));
    let gate = root.join("trivy");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&gate).unwrap();
    std::fs::write(
        gate.join("index.toml"),
        "name = \"trivy\"\ndisplay = \"Trivy\"\ndescription = \"test\"\nbinary = \"/definitely/missing/trivy\"\nargs = []\ninstall_hint = \"install\"\n",
    )
    .unwrap();
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &root);
    let error = handle(&words(&["run"]), &home()).unwrap_err();
    assert!(error.contains("optional gate 'trivy' is enabled"));
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_GATES", value);
    } else {
        std::env::remove_var("TERMINAL_JARVIS_GATES");
    }
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn run_and_names_preserve_gate_results() {
    let gate = fake("alpha");
    let (code, body) = run(&gate).unwrap();
    assert_eq!(code, 7);
    assert!(body.contains("marker"));
    assert_eq!(names(&[gate, fake("beta")]), "alpha, beta");
}

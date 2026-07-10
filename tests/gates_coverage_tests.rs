use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use terminal_jarvis::gates::{self, Gate};

static ID: AtomicUsize = AtomicUsize::new(0);

fn dir() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "tj-gatecov-{}-{}-{}",
        std::process::id(),
        ID.fetch_add(1, Ordering::Relaxed),
        std::thread::current().name().unwrap_or("t")
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn home() -> PathBuf {
    let path = dir().join("home");
    fs::create_dir_all(&path).unwrap();
    path
}

fn gate() -> Gate {
    Gate {
        name: "trivy".to_string(),
        display: "Trivy".to_string(),
        description: "scan".to_string(),
        binary: "definitely-not-a-real-binary-xyz".to_string(),
        args: vec![],
        install_hint: "install".to_string(),
    }
}

#[test]
fn embedded_gate_catalog_defaults_to_trivy() {
    let gates = gates::load(Path::new("no-such-gates-dir-xyz")).unwrap();
    assert_eq!(gates.len(), 1);
    assert_eq!(gates[0].name, "trivy");
}

#[test]
fn loader_reads_gate_directory_and_rejects_bad_data() {
    let root = dir();
    let good = root.join("good");
    fs::create_dir_all(&good).unwrap();
    fs::write(
        good.join("index.toml"),
        "name = \"good\"\ndisplay = \"Good\"\ndescription = \"g\"\nbinary = \"good\"\nargs = [\"x\"]\ninstall_hint = \"hint\"\n",
    )
    .unwrap();
    let loaded = gates::load(&root).unwrap();
    assert_eq!(loaded[0].name, "good");

    let bad = root.join("bad");
    fs::create_dir_all(&bad).unwrap();
    fs::write(bad.join("index.toml"), "this is = not valid toml [[[").unwrap();
    assert!(gates::load(&root).is_err());

    let empty = dir();
    assert!(gates::load(&empty).is_err());
}

#[test]
fn preflight_is_noop_without_selection_and_errors_on_unknown_gate() {
    assert!(gates::preflight(&home()).is_ok());

    let h = home();
    gates::enable(&h, "ghost").unwrap();
    let err = gates::preflight(&h).unwrap_err();
    assert!(err.contains("not in the gate catalog"));
}

#[test]
fn run_reports_missing_binary_and_succeeds_with_present_binary() {
    let err = gates::run(&gate()).unwrap_err();
    assert!(err.contains("not on PATH"));

    let (bin, args) = if cfg!(windows) {
        (
            "cmd".to_string(),
            vec!["/c".to_string(), "exit".to_string(), "0".to_string()],
        )
    } else {
        ("true".to_string(), vec![])
    };
    let present = Gate {
        name: "present".to_string(),
        display: "Present".to_string(),
        description: "d".to_string(),
        binary: bin,
        args,
        install_hint: "h".to_string(),
    };
    let (code, _) = gates::run(&present).unwrap();
    assert_eq!(code, 0);
}

//! Gates that name `flag` load only while that flag is on. Tests pin the
//! home, registry and gate env so the caller's machine cannot change them.

use crate::gates::load;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::MutexGuard;

const FLAG_VAR: &str = "TJ_FLAG_SCANNER_CHOICE";
const BUNDLED: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/gates");
const PINNED: [&str; 4] = [
    "TERMINAL_JARVIS_HOME",
    "TERMINAL_JARVIS_FLAGS",
    "TERMINAL_JARVIS_GATES",
    FLAG_VAR,
];

struct Pinned {
    _lock: MutexGuard<'static, ()>,
    saved: Vec<(&'static str, Option<OsString>)>,
}

impl Drop for Pinned {
    fn drop(&mut self) {
        for (key, value) in self.saved.drain(..) {
            value.map_or_else(|| std::env::remove_var(key), |v| std::env::set_var(key, v));
        }
    }
}

/// Runs `body` with an empty home, the embedded registry, and the flag set to
/// `flag` (or unset), then restores the caller's environment.
fn with_flag<T>(flag: Option<&str>, body: impl FnOnce() -> T) -> T {
    let lock = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let saved = PINNED
        .iter()
        .map(|key| (*key, std::env::var_os(key)))
        .collect();
    let _pinned = Pinned { _lock: lock, saved };
    let home = scratch("home");
    std::env::set_var("TERMINAL_JARVIS_HOME", &home);
    std::env::remove_var("TERMINAL_JARVIS_FLAGS");
    std::env::remove_var("TERMINAL_JARVIS_GATES");
    match flag {
        Some(value) => std::env::set_var(FLAG_VAR, value),
        None => std::env::remove_var(FLAG_VAR),
    }
    body()
}

fn scratch(tag: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("tj-scanner-choice-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn names(gates: &[crate::gates::Gate]) -> Vec<&str> {
    gates.iter().map(|gate| gate.name.as_str()).collect()
}

#[test]
fn flag_off_loads_only_trivy_from_bundled_data_and_embedded() {
    with_flag(None, || {
        let from_dir = load(Path::new(BUNDLED)).unwrap();
        assert_eq!(names(&from_dir), ["trivy"]);
        let embedded = load(Path::new(crate::context::EMBEDDED_GATES)).unwrap();
        assert_eq!(names(&embedded), ["trivy"]);
    });
}

#[test]
fn flag_on_loads_the_three_scanner_gates_beside_trivy() {
    with_flag(Some("1"), || {
        let gates = load(Path::new(BUNDLED)).unwrap();
        assert_eq!(names(&gates), ["grype", "opengrep", "semgrep", "trivy"]);
        let embedded = load(Path::new(crate::context::EMBEDDED_GATES)).unwrap();
        assert_eq!(names(&embedded), ["trivy"]);
    });
}

#[test]
fn a_gate_naming_an_unknown_flag_is_a_load_error() {
    with_flag(None, || {
        let root = scratch("unknown");
        let directory = root.join("bogus");
        fs::create_dir_all(&directory).unwrap();
        let data = "name = \"bogus\"\ndisplay = \"b\"\ndescription = \"d\"\nbinary = \"b\"\n\
                    args = []\ninstall_hint = \"i\"\nflag = \"no-such-flag\"\n";
        fs::write(directory.join("index.toml"), data).unwrap();
        let error = load(&root).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("unknown flag 'no-such-flag'"));
    });
}

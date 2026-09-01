use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::Capability;

static ID: AtomicUsize = AtomicUsize::new(0);

fn temp_root() -> PathBuf {
    // A thread name is not a safe path component: Rust's default
    // test-thread names are the full `::`-separated test path, and `:` is
    // invalid in Windows paths. process id + counter already guarantee
    // uniqueness on their own.
    std::env::temp_dir().join(format!(
        "terminal-jarvis-loader-{}-{}",
        std::process::id(),
        ID.fetch_add(1, Ordering::Relaxed)
    ))
}

#[test]
fn loader_rejects_invalid_catalog_data() {
    let root = temp_root();
    let tool = root.join("bad");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&tool).unwrap();
    fs::write(root.join("not-a-dir"), "ignored").unwrap();
    fs::write(
        tool.join("index.toml"),
        "name = \"bad\"\ndisplay = \"bad\"\ndescription = \"bad\"\nbinary = \"bad\"\nenv_mode = \"sometimes\"\n",
    )
    .unwrap();
    for capability in Capability::ALL {
        let dir = tool.join(capability.as_str());
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("index.toml"),
            "summary = \"test\"\ncommand = \"sh\"\n",
        )
        .unwrap();
    }
    assert!(catalog::load(&root).is_err());
}

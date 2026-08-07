use std::fs;
use std::path::{Path, PathBuf};
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::{Capability, Effect};

fn root() -> PathBuf {
    std::env::temp_dir().join(format!(
        "terminal-jarvis-catalog-strict-{}",
        std::process::id()
    ))
}

fn write_catalog(root: &Path, root_extra: &str, plan_extra: &str) {
    let harness = root.join("test");
    fs::create_dir_all(&harness).unwrap();
    fs::write(
        harness.join("index.toml"),
        format!(
            "name = \"test\"\ndisplay = \"Test\"\ndescription = \"Test\"\n\
             binary = \"sh\"\nenv_mode = \"none\"\nenv = []\n{root_extra}"
        ),
    )
    .unwrap();
    for capability in Capability::ALL {
        let dir = harness.join(capability.as_str());
        fs::create_dir_all(&dir).unwrap();
        let effect = match capability {
            Capability::Yolo => Effect::Dangerous,
            Capability::Download | Capability::Update | Capability::Ui => Effect::StateChanging,
            _ => Effect::ReadOnly,
        };
        let effect = match effect {
            Effect::ReadOnly => "read-only",
            Effect::StateChanging => "state-changing",
            Effect::Dangerous => "dangerous",
        };
        let network = matches!(
            capability,
            Capability::Download | Capability::Update | Capability::Ui | Capability::Yolo
        );
        let interaction = if capability == Capability::Ui {
            "interactive"
        } else {
            "noninteractive"
        };
        let extra = if capability == Capability::Stats {
            plan_extra
        } else {
            ""
        };
        fs::write(
            dir.join("index.toml"),
            format!(
                "summary = \"Dangerous test\"\ncommand = \"sh\"\nargs = []\n\
                 support = \"unknown\"\nevidence = \"deterministic\"\neffect = \"{effect}\"\n\
                 network = {network}\ninteraction = \"{interaction}\"\nplatforms = []\n\
                 executable = \"sh\"\nsource = \"internal:test\"\n\
                 verified_at = \"2026-07-17T00:00:00Z\"\n{extra}"
            ),
        )
        .unwrap();
    }
}

#[test]
fn loader_rejects_duplicate_and_unknown_metadata() {
    let root = root();
    let _ = fs::remove_dir_all(&root);
    write_catalog(&root, "name = \"again\"\n", "");
    assert!(catalog::load(&root)
        .unwrap_err()
        .to_string()
        .contains("duplicate key"));

    fs::remove_dir_all(&root).unwrap();
    write_catalog(&root, "", "future = \"no\"\n");
    assert!(catalog::load(&root)
        .unwrap_err()
        .to_string()
        .contains("metadata keys must be exactly"));
    fs::remove_dir_all(root).unwrap();
}

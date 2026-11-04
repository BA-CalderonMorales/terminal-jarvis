//! Synthetic dates isolate intent/readiness tests from bundled evidence aging.
//! This disposable catalog is not evidence of upstream agent compatibility.
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub fn root() -> &'static Path {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("tj-intent-catalog-{nonce}"));
        copy(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("harnesses"),
            &root,
        );
        root
    })
}

fn copy(source: &Path, target: &Path) {
    std::fs::create_dir_all(target).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let output = target.join(entry.file_name());
        if path.is_dir() {
            copy(&path, &output);
        } else {
            let text = std::fs::read_to_string(path).unwrap();
            let synthetic = text
                .lines()
                .map(|line| {
                    if line.starts_with("verified_at =") {
                        format!(
                            "verified_at = \"{}\"",
                            terminal_jarvis::catalog::today_utc()
                        )
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::write(output, synthetic).unwrap();
        }
    }
}

use std::{fs, path::Path};

/// Copies the changelog generator into a temp repo so preflight can call it.
pub fn copy_generator(root: &Path) {
    let generator = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/python/release");
    for rel in [
        "index.py",
        "constants/paths.py",
        "constants/sections.py",
        "logic/cli.py",
        "logic/changelog.py",
        "logic/group.py",
        "logic/parse.py",
    ] {
        let dest = root.join("scripts/python/release").join(rel);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(generator.join(rel), dest).unwrap();
    }
}

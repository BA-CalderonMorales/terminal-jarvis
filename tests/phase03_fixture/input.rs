use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub fn binary() -> OsString {
    std::env::var_os("TJ_PHASE03_BIN").map_or_else(
        || OsString::from(env!("CARGO_BIN_EXE_terminal-jarvis")),
        |path| canonical(path).into_os_string(),
    )
}

pub fn catalog_root() -> PathBuf {
    std::env::var_os("TJ_PHASE03_CATALOG")
        .map(canonical)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("harnesses"))
}

fn canonical(path: OsString) -> PathBuf {
    let path = PathBuf::from(path);
    std::fs::canonicalize(&path).unwrap_or_else(|cause| {
        panic!(
            "{} is not a readable fixture input: {cause}",
            path.display()
        )
    })
}

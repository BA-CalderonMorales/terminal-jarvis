use std::env;
use std::path::PathBuf;

pub fn gates_root() -> PathBuf {
    if let Some(path) =
        env::var_os(crate::context::constants::env::GATES).filter(|path| !path.is_empty())
    {
        return PathBuf::from(path);
    }
    candidates()
        .into_iter()
        .find(|path| path.is_dir())
        .unwrap_or_else(|| PathBuf::from(crate::context::EMBEDDED_GATES))
}

fn candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = env::current_exe() {
        if let Some(bin) = exe.parent() {
            roots.push(bin.join("gates"));
            if let Some(root) = bin.parent() {
                roots.push(root.join("gates"));
                roots.push(root.join("share/terminal-jarvis/gates"));
            }
        }
    }
    roots
}

#[cfg(test)]
#[path = "../tests/gates.rs"]
mod context_gates_tests;

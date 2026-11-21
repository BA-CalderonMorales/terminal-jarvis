use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=harnesses");
    println!("cargo:rerun-if-env-changed=TERMINAL_JARVIS_DISTRIBUTION");
    if let Some(value) = env::var("TERMINAL_JARVIS_DISTRIBUTION")
        .ok()
        .filter(|value| !value.trim().is_empty())
    {
        println!("cargo:rustc-env=TERMINAL_JARVIS_DISTRIBUTION_STAMPED={value}");
    }
    embed("harnesses", "embedded_catalog.rs");
    println!("cargo:rerun-if-changed=flags");
    embed("flags", "embedded_flags.rs");
}

/// Embeds every `index.toml` under `dir` as `(relative path, contents)`. A
/// missing directory embeds nothing, so an empty flag registry still builds.
fn embed(dir: &str, module: &str) {
    let root = Path::new(dir);
    let mut files = Vec::new();
    if root.is_dir() {
        collect(root, root, &mut files).expect("embedded data files are readable");
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    write_module(module, files).expect("embedded data module is writable");
}

fn collect(root: &Path, dir: &Path, files: &mut Vec<(String, PathBuf)>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect(root, &path, files)?;
        } else if path.file_name().is_some_and(|name| name == "index.toml") {
            let rel = path
                .strip_prefix(root)
                .expect("catalog path is under root")
                .to_string_lossy()
                .replace('\\', "/");
            println!("cargo:rerun-if-changed={}", path.display());
            files.push((rel, fs::canonicalize(path)?));
        }
    }
    Ok(())
}

fn write_module(name: &str, files: Vec<(String, PathBuf)>) -> io::Result<()> {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set"));
    let mut module = fs::File::create(out.join(name))?;
    writeln!(module, "pub const FILES: &[(&str, &str)] = &[")?;
    for (rel, path) in files {
        writeln!(
            module,
            "    ({rel:?}, include_str!({:?})),",
            path.to_string_lossy()
        )?;
    }
    writeln!(module, "];")
}

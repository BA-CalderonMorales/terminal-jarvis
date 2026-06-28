use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=harnesses");
    let root = Path::new("harnesses");
    let mut files = Vec::new();
    collect(root, root, &mut files).expect("catalog files are readable");
    files.sort_by(|left, right| left.0.cmp(&right.0));
    write_module(files).expect("embedded catalog module is writable");
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

fn write_module(files: Vec<(String, PathBuf)>) -> io::Result<()> {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set"));
    let mut module = fs::File::create(out.join("embedded_catalog.rs"))?;
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

//! DirIndex: one read of every PATH directory into an in-memory name table.
//! Binary resolution used to stat every PATH directory for every harness --
//! on slow filesystems (WSL /mnt) that made a full diagnostics run take
//! ~17s. Indexing reads each directory once per collect, after which
//! resolution is pure set lookups.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

pub struct DirIndex {
    dirs: Vec<(PathBuf, BTreeSet<String>)>,
}

impl DirIndex {
    pub fn from_paths(paths: &[PathBuf]) -> DirIndex {
        let mut seen = BTreeSet::new();
        let mut dirs = Vec::new();
        for path in paths {
            if !seen.insert(path) {
                continue;
            }
            let Ok(entries) = fs::read_dir(path) else {
                continue;
            };
            let mut names = BTreeSet::new();
            for entry in entries.flatten() {
                names.insert(entry.file_name().to_string_lossy().into_owned());
            }
            dirs.push((path.clone(), names));
        }
        DirIndex { dirs }
    }

    pub fn locate(&self, candidates: &[String]) -> Vec<PathBuf> {
        let mut found = Vec::new();
        for (directory, names) in &self.dirs {
            for candidate in candidates {
                if names.contains(candidate.as_str()) {
                    found.push(directory.join(candidate));
                }
            }
        }
        found
    }
}

#[cfg(test)]
#[path = "../tests/dir_index.rs"]
mod tests;

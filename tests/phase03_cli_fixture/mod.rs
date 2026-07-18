mod catalog;
mod child;
mod output;
#[cfg(unix)]
mod pty;

pub use output::{assert_json_document, assert_width, text};
#[cfg(unix)]
pub use pty::{run_pty, run_pty_input};

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
pub enum State {
    Expected,
    Unknown,
}

pub struct Fixture {
    root: PathBuf,
    catalog: PathBuf,
    home: PathBuf,
    bin: PathBuf,
}

impl Fixture {
    pub fn new(state: State) -> Self {
        let leaf = format!(
            "terminal-jarvis-phase03-cli-{}-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed),
            "unbroken".repeat(8)
        );
        let root = std::env::temp_dir().join(leaf);
        let _ = std::fs::remove_dir_all(&root);
        let catalog = root.join("catalog");
        let home = root.join("home");
        let bin = root.join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        std::fs::create_dir_all(root.join("tmp")).unwrap();
        catalog::write(&catalog, state);
        Self {
            root,
            catalog,
            home,
            bin,
        }
    }

    pub fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
        command
            .env_clear()
            .env("PATH", &self.bin)
            .env("HOME", &self.root)
            .env("XDG_CONFIG_HOME", self.root.join("config"))
            .env("TMPDIR", self.root.join("tmp"))
            .env("SHELL", "/bin/sh")
            .env("TERM", "xterm-256color")
            .env("TERMINAL_JARVIS_CATALOG", &self.catalog)
            .env("TERMINAL_JARVIS_GATE", "off")
            .env("TERMINAL_JARVIS_HOME", &self.home)
            .current_dir(&self.root);
        if let Some(value) = std::env::var_os("SystemRoot") {
            command.env("SystemRoot", value);
        }
        command
    }

    pub fn run(&self, args: &[&str]) -> Output {
        self.command().args(args).output().expect("CLI runs")
    }

    pub fn child(&self, executable: bool) -> PathBuf {
        child::write(&self.bin, executable)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

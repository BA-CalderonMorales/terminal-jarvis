//! A temp home plus an optional fixture registry, and the real binary.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static ID: AtomicUsize = AtomicUsize::new(0);

pub struct Sandbox {
    pub home: PathBuf,
    pub flags: Option<PathBuf>,
}

pub fn flag_file(name: &str, default: bool, stage: &str) -> String {
    format!(
        "name = \"{name}\"\ndefault = {default}\nstage = \"{stage}\"\n\
         description = \"A fixture flag for {name}.\"\nsince = \"0.1.26\"\n"
    )
}

impl Sandbox {
    /// `files` are `(name, default, stage)`; `None` uses the embedded registry.
    pub fn new(files: Option<&[(&str, bool, &str)]>) -> Self {
        let root = std::env::temp_dir().join(format!(
            "tj-flagsit-{}-{}",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&root);
        let home = root.join("home");
        fs::create_dir_all(&home).unwrap();
        let flags = files.map(|files| {
            let dir = root.join("flags");
            fs::create_dir_all(&dir).unwrap();
            for (name, default, stage) in files {
                fs::create_dir_all(dir.join(name)).unwrap();
                let data = flag_file(name, *default, stage);
                fs::write(dir.join(name).join("index.toml"), data).unwrap();
            }
            dir
        });
        Self { home, flags }
    }

    pub fn run(&self, args: &[&str], env: &[(&str, &str)]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
        command.args(args).env("TERMINAL_JARVIS_HOME", &self.home);
        for (key, _) in std::env::vars().filter(|(key, _)| key.starts_with("TJ_FLAG_")) {
            command.env_remove(key);
        }
        match &self.flags {
            Some(dir) => command.env("TERMINAL_JARVIS_FLAGS", dir),
            None => command.env_remove("TERMINAL_JARVIS_FLAGS"),
        };
        command.envs(env.iter().copied()).output().unwrap()
    }

    pub fn settings_file(&self) -> PathBuf {
        self.home.join("settings.toml")
    }
}

pub fn out(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn err(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

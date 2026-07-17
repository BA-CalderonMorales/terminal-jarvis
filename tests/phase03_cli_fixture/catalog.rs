use super::State;
use std::path::Path;

const CAPABILITIES: [&str; 9] = [
    "download", "update", "headless", "version", "stats", "models", "security", "yolo", "ui",
];

pub fn write(root: &Path, state: State) {
    let harness = root.join("fixture");
    std::fs::create_dir_all(&harness).unwrap();
    std::fs::write(
        harness.join("index.toml"),
        concat!(
            "name = \"fixture\"\n",
            "display = \"Fixture 漢字 é 🚀\"\n",
            "description = \"Unicode and a verylongunbrokenfieldverylongunbrokenfieldverylongunbrokenfield with deterministic wrapping\"\n",
            "binary = \"fixture-child\"\n",
            "env_mode = \"none\"\n",
            "env = []\n",
        ),
    )
    .unwrap();
    for capability in CAPABILITIES {
        write_capability(&harness, capability, state);
    }
}

fn write_capability(root: &Path, capability: &str, state: State) {
    let directory = root.join(capability);
    std::fs::create_dir_all(&directory).unwrap();
    let support = match state {
        State::Expected => "expected",
        State::Unknown => "unknown",
    };
    let platforms = match state {
        State::Expected => format!("[\"{}\"]", platform()),
        State::Unknown => "[]".to_string(),
    };
    let (effect, network, interaction) = behavior(capability);
    let summary = if capability == "yolo" {
        "Dangerous Unicode fixture plan 漢字 é 🚀"
    } else {
        "Unicode fixture plan 漢字 é 🚀 with unbrokenunbrokenunbrokenunbroken data"
    };
    let body = format!(
        "summary = \"{summary}\"\ncommand = \"fixture-child\"\nargs = []\n\
         support = \"{support}\"\nevidence = \"deterministic\"\neffect = \"{effect}\"\n\
         network = {network}\ninteraction = \"{interaction}\"\nplatforms = {platforms}\n\
         executable = \"fixture-child\"\nsource = \"internal:phase03-cli\"\n\
         verified_at = \"2026-07-17T00:00:00Z\"\n"
    );
    std::fs::write(directory.join("index.toml"), body).unwrap();
}

fn behavior(capability: &str) -> (&'static str, bool, &'static str) {
    match capability {
        "download" | "update" => ("state-changing", true, "noninteractive"),
        "ui" => ("state-changing", true, "interactive"),
        "yolo" => ("dangerous", true, "noninteractive"),
        _ => ("read-only", false, "noninteractive"),
    }
}

fn platform() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "linux-x64-gnu",
        ("linux", "aarch64") => "linux-arm64-gnu",
        ("macos", "x86_64") => "macos-x64",
        ("macos", "aarch64") => "macos-arm64",
        ("windows", "x86_64") => "windows-x64-msvc",
        pair => panic!("unsupported Phase 03 CLI fixture host: {pair:?}"),
    }
}

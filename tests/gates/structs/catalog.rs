use std::path::Path;

const CAPABILITIES: [&str; 9] = [
    "download", "update", "headless", "version", "stats", "models", "security", "yolo", "ui",
];

pub fn write(root: &Path, download: &str, yolo: &str) {
    let harness = root.join("fixture");
    std::fs::create_dir_all(&harness).unwrap();
    std::fs::write(
        harness.join("index.toml"),
        concat!(
            "name = \"fixture\"\n",
            "display = \"Fixture\"\n",
            "description = \"Disposable acceptance harness\"\n",
            "binary = \"fixture-child\"\n",
            "env_mode = \"none\"\n",
            "env = []\n",
        ),
    )
    .unwrap();
    for capability in CAPABILITIES {
        let support = match capability {
            "download" => download,
            "yolo" => yolo,
            _ => "expected",
        };
        write_capability(&harness, capability, support);
    }
}

fn write_capability(root: &Path, capability: &str, support: &str) {
    let directory = root.join(capability);
    std::fs::create_dir_all(&directory).unwrap();
    let (effect, network, interaction) = behavior(capability);
    let evidence = match support {
        "verified" => "disposable-real",
        "manual" => "manual",
        "unsupported" => "unsupported",
        _ => "deterministic",
    };
    let platforms = if matches!(support, "verified" | "expected" | "manual") {
        format!("[\"{}\"]", platform())
    } else {
        "[]".to_string()
    };
    let summary = if capability == "yolo" {
        "Dangerous disposable fixture capability"
    } else {
        "Disposable fixture capability"
    };
    let body = format!(
        "summary = \"{summary}\"\ncommand = \"fixture-child\"\nargs = []\n\
support = \"{support}\"\nevidence = \"{evidence}\"\neffect = \"{effect}\"\n\
network = {network}\ninteraction = \"{interaction}\"\nplatforms = {platforms}\n\
executable = \"fixture-child\"\nsource = \"internal:test\"\n\
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
        pair => panic!("unsupported acceptance-test host: {pair:?}"),
    }
}

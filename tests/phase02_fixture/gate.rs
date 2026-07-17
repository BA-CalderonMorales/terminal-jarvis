use std::path::Path;

pub fn write(root: &Path) {
    let directory = root.join("acceptance");
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(
        directory.join("index.toml"),
        concat!(
            "name = \"acceptance\"\n",
            "display = \"Acceptance gate\"\n",
            "description = \"Disposable gate marker\"\n",
            "binary = \"fixture-gate\"\n",
            "args = []\n",
            "install_hint = \"use the bundled acceptance fixture\"\n",
        ),
    )
    .unwrap();
}

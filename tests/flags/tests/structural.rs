//! The tests that keep flags from rotting. Each holds for any number of
//! embedded flags (vacuously at zero) and is proven live on a fixture.

use crate::logic::sandbox::{err, out, Sandbox};
use std::path::Path;
use terminal_jarvis::flags::{self, Stage};

const THREE: [(&str, bool, &str); 3] = [
    ("calm-mode", true, "stable"),
    ("quick-look", false, "preview"),
    ("wild-idea", false, "labs"),
];

fn embedded() -> Vec<flags::Flag> {
    flags::load(Path::new(terminal_jarvis::context::EMBEDDED_FLAGS)).unwrap()
}

/// Every flag is listed, stable/preview before the Labs heading, labs after.
fn assert_listed(sandbox: &Sandbox, declared: &[(String, bool)]) -> usize {
    let plain = out(&sandbox.run(&["--plain", "settings"], &[]));
    let rich = out(&sandbox.run(&["--no-color", "settings"], &[]));
    let labs_at = rich.find("Labs").unwrap_or(rich.len());
    for (name, is_labs) in declared {
        let row = plain
            .lines()
            .find(|line| line.starts_with(&format!("flag={name} ")));
        let row = row.unwrap_or_else(|| panic!("{name} missing from plain output:\n{plain}"));
        assert_eq!(row.contains("stage=labs"), *is_labs, "{row}");
        let at = rich
            .find(&format!("  {name}  "))
            .unwrap_or_else(|| panic!("{name} missing:\n{rich}"));
        assert_eq!(
            at > labs_at,
            *is_labs,
            "{name} is in the wrong section:\n{rich}"
        );
    }
    declared.len()
}

#[test]
fn every_embedded_flag_appears_in_settings_in_its_section() {
    let declared: Vec<_> = embedded()
        .iter()
        .map(|flag| (flag.name.clone(), flag.stage == Stage::Labs))
        .collect();
    assert_listed(&Sandbox::new(None), &declared);
    let sandbox = Sandbox::new(Some(&THREE));
    let fixture = [
        ("calm-mode".to_string(), false),
        ("quick-look".into(), false),
        ("wild-idea".into(), true),
    ];
    assert_eq!(assert_listed(&sandbox, &fixture), 3);
}

fn boot_each_flag_on_and_off(sandbox: &Sandbox, names: &[String]) -> usize {
    let mut runs = 0;
    for name in names {
        let var = flags::env_name(name);
        for value in ["1", "0"] {
            for command in ["check", "list"] {
                let output = sandbox.run(&[command], &[(var.as_str(), value)]);
                assert_eq!(
                    output.status.code(),
                    Some(0),
                    "{command} {var}={value}: {}",
                    err(&output)
                );
                runs += 1;
            }
        }
    }
    runs
}

#[test]
fn the_binary_boots_with_every_flag_forced_on_and_off() {
    let names: Vec<_> = embedded().into_iter().map(|flag| flag.name).collect();
    assert_eq!(
        boot_each_flag_on_and_off(&Sandbox::new(None), &names),
        names.len() * 4
    );
    let fixture: Vec<_> = THREE.iter().map(|(name, ..)| name.to_string()).collect();
    assert_eq!(
        boot_each_flag_on_and_off(&Sandbox::new(Some(&THREE)), &fixture),
        12
    );
}

use crate::catalog::logic::parser::Fields;

const HARNESS_KEYS: [&str; 6] = [
    "name",
    "display",
    "description",
    "binary",
    "env_mode",
    "env",
];
const PLAN_KEYS: [&str; 12] = [
    "summary",
    "command",
    "args",
    "support",
    "evidence",
    "effect",
    "network",
    "interaction",
    "platforms",
    "executable",
    "source",
    "verified_at",
];
const OPTIONAL_PLAN_KEYS: [&str; 3] = ["package", "uninstall_command", "uninstall_args"];
const SUNSET_KEYS: [&str; 3] = ["sunset_at", "sunset_note", "sunset_replaced_by"];

pub fn exact_fields(fields: &Fields, expected: &[&str]) -> Result<(), String> {
    let actual = fields.keys().map(String::as_str).collect::<Vec<_>>();
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "metadata keys must be exactly {}; found {}",
            expected.join(", "),
            actual.join(", ")
        ))
    }
}

pub fn plan_keys(fields: &Fields) -> Result<(), String> {
    let missing: Vec<_> = PLAN_KEYS
        .iter()
        .filter(|key| !fields.contains_key(**key))
        .copied()
        .collect();
    let unknown: Vec<_> = fields
        .keys()
        .filter(|key| {
            !PLAN_KEYS.contains(&key.as_str()) && !OPTIONAL_PLAN_KEYS.contains(&key.as_str())
        })
        .map(String::as_str)
        .collect();
    if missing.is_empty() && unknown.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "plan keys must be the required set plus optional keys; missing {}, unknown {}",
            missing.join(", "),
            unknown.join(", ")
        ))
    }
}

pub fn harness_keys() -> &'static [&'static str] {
    &HARNESS_KEYS
}

/// The facade's required keys, plus whichever retirement keys the tool
/// carries. Retirement is optional, so a maintained tool needs none of them
/// and a retired one still has to spell out the rest of the facade exactly.
pub fn harness_keys_match(fields: &Fields) -> Result<(), String> {
    let mut expected = harness_keys().to_vec();
    expected.extend(
        SUNSET_KEYS
            .iter()
            .copied()
            .filter(|key| fields.contains_key(*key)),
    );
    exact_fields(fields, &expected)
}

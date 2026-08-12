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
const OPTIONAL_PLAN_KEYS: [&str; 1] = ["package"];

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

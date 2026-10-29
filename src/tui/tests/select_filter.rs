use super::*;

fn harness(name: &str) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: "probe".into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

fn fleet() -> [Harness; 3] {
    [harness("alpha"), harness("beta"), harness("gamma")]
}

#[test]
fn filter_narrows_rows_and_keeps_their_picker_numbers() {
    let harnesses = fleet();
    let rendered = pick(&harnesses, None, &BTreeSet::new(), Some("eta"), 80);
    assert!(rendered.starts_with(" 2  beta"));
    assert!(!rendered.contains("alpha"));
    assert!(!rendered.contains("gamma"));
    assert!(rendered.contains("1 of 3 tools match \"eta\""));
    assert!(rendered.contains("stable picker positions"));
}

#[test]
fn filter_matches_the_description_as_well_as_the_name() {
    let harnesses = fleet();
    let rendered = pick(&harnesses, None, &BTreeSet::new(), Some("PROBE"), 80);
    assert_eq!(rendered.lines().count(), 4);
    assert!(rendered.contains("3 of 3 tools match \"probe\""));
}

#[test]
fn filter_with_no_match_names_the_whole_fleet() {
    let harnesses = fleet();
    let rendered = pick(&harnesses, None, &BTreeSet::new(), Some("zzz"), 80);
    assert_eq!(rendered.lines().count(), 1);
    assert!(rendered.contains("no harness matches \"zzz\""));
    assert!(rendered.contains("list shows all 3"));
}

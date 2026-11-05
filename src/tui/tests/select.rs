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
        sunset: None,
    }
}

fn tokens(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

#[test]
fn selection_uses_the_visible_order() {
    let harnesses = [harness("alpha"), harness("beta")];
    assert_eq!(
        select("1", &harnesses),
        Some(args::Action::Use("alpha".into()))
    );
    assert_eq!(
        select("2", &harnesses),
        Some(args::Action::Use("beta".into()))
    );
}

#[test]
fn selection_refuses_zero_out_of_range_and_garbage() {
    let harnesses = [harness("alpha")];
    assert_eq!(select("0", &harnesses), None);
    assert_eq!(select("2", &harnesses), None);
    assert_eq!(select("", &harnesses), None);
    assert_eq!(select("nope", &harnesses), None);
}

#[test]
fn pick_is_numbered_with_readiness_and_the_active_row_marked() {
    let harnesses = [harness("alpha"), harness("beta")];
    let ready = BTreeSet::from(["alpha".to_string()]);
    let rendered = pick(&harnesses, Some("beta"), &ready, None, 80);
    let alpha = rendered.lines().next().unwrap();
    let beta = rendered.lines().nth(1).unwrap();
    assert_eq!(tokens(alpha), ["1", "alpha", "[ready]", "probe"]);
    assert_eq!(
        tokens(beta),
        ["2", "beta", "[install]", "probe", "(current)"]
    );
    assert!(rendered.contains("pick a number to switch agents"));
    assert_eq!(rendered.lines().count(), 3);
}

#[test]
fn narrow_frames_drop_the_description_before_the_verdict() {
    let harnesses = [harness("alpha")];
    let rendered = pick(&harnesses, None, &BTreeSet::new(), None, 30);
    assert!(rendered.contains("[install]"));
    assert!(!rendered.contains("probe"));
}

fn long_harness() -> Harness {
    Harness {
        name: "alpha".into(),
        display: "alpha".into(),
        description:
            "a very long description that should be clipped at the char limit, and then some more"
                .into(),
        binary: "alpha".into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
        sunset: None,
    }
}

#[test]
fn pick_truncates_overlong_descriptions() {
    let rendered = pick(&[long_harness()], None, &BTreeSet::new(), None, 80);
    assert!(rendered.contains("..."));
    assert!(!rendered.contains("and then some more"));
}

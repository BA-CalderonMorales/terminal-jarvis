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
fn pick_is_numbered_with_the_active_row_marked() {
    let harnesses = [harness("alpha"), harness("beta")];
    let rendered = pick(&harnesses, Some("beta"));
    assert!(rendered.starts_with(" 1  alpha  probe\n"));
    assert!(rendered.lines().nth(1).unwrap().starts_with(" 2  beta"));
    assert!(rendered.contains("probe  (current)"));
    assert!(rendered.contains("pick a number to switch agents"));
    assert_eq!(rendered.lines().count(), 3);
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
    }
}

#[test]
fn pick_truncates_overlong_descriptions() {
    let rendered = pick(&[long_harness()], None);
    assert!(rendered.contains("..."));
    assert!(!rendered.contains("and then some more"));
}

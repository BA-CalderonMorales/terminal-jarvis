use crate::cli::args::{Options, OutputMode};
use crate::cli::logic::test_support;
use crate::cli::logic::{output_plan, prompt_lead};
use crate::contracts::{Capability, EnvMode, Harness, Sunset};

const TODAY: &str = "2026-11-20";

pub(super) fn harness(name: &str, sunset: Option<(&str, Option<&str>)>) -> Harness {
    let mut plan = test_support::plan(Capability::Download, "npm", vec!["i".into(), "pkg".into()]);
    plan.package = Some("@google/jules".into());
    Harness {
        name: name.into(),
        display: name.into(),
        description: "d".into(),
        binary: "sh".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![plan],
        sunset: sunset.map(|(at, by)| Sunset {
            at: at.into(),
            note: "n".into(),
            replaced_by: by.map(Into::into),
        }),
    }
}

fn quiet() -> Options {
    Options {
        output: OutputMode::Plain,
        no_color: false,
        verbose: false,
        dry_run: false,
        no_input: false,
        confirm: None,
        allow_dangerous: false,
        narrate: false,
    }
}

#[test]
fn plan_carries_the_retirement_before_the_flags() {
    let h = harness("jules", Some(("2026-12-31", Some("antigravity"))));
    let plan = &h.capabilities[0];
    let text = output_plan::plain_on(&h, plan, &plan.command, TODAY);
    let line = "retirement: retires 2026-12-31 · replaced by antigravity\n";
    assert!(text.contains(line), "{text}");
    assert!(text.find(line) < text.find("intent:"), "{text}");
    let done = output_plan::plain_on(&h, plan, &plan.command, "2027-01-01");
    assert!(done.contains("retirement: retired 2026-12-31"), "{done}");
    let card = output_plan::groups_on(&h, plan, &plan.command, TODAY);
    let rows: Vec<&str> = card
        .iter()
        .flat_map(|g| &g.rows)
        .map(|r| &*r.label)
        .collect();
    assert!(
        rows.iter().position(|l| *l == "retirement") < rows.iter().position(|l| *l == "intent")
    );
}

#[test]
fn a_maintained_plan_shows_no_retirement() {
    let h = harness("codex", None);
    let plan = &h.capabilities[0];
    assert!(!output_plan::plain_on(&h, plan, &plan.command, TODAY).contains("retire"));
    let card = output_plan::groups_on(&h, plan, &plan.command, TODAY);
    assert!(card
        .iter()
        .flat_map(|g| &g.rows)
        .all(|r| r.label != "retirement"));
}

#[test]
fn a_retirement_without_a_successor_names_none() {
    let h = harness("old", Some(("2026-01-01", None)));
    let line = output_plan::retirement(&h, TODAY).unwrap();
    assert_eq!(line, "retired 2026-01-01");
}

#[test]
fn the_consent_lead_carries_retirement_and_names_the_package() {
    let h = harness("jules", Some(("2026-12-31", Some("antigravity"))));
    let plan = &h.capabilities[0];
    let lead = prompt_lead::confirm_lead_on(&quiet(), &h, plan, &[], TODAY);
    assert_eq!(
        lead,
        "install jules · npm · @google/jules · retires 2026-12-31 · replaced by antigravity\n"
    );
    let kept = harness("codex", None);
    let lead = prompt_lead::confirm_lead_on(&quiet(), &kept, &kept.capabilities[0], &[], TODAY);
    assert_eq!(lead, "install codex · npm · @google/jules\n");
}

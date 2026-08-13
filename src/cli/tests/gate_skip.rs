use crate::cli::logic::args::Options;
use crate::cli::logic::gate_skip::{allow, route};
use crate::gates::Verdict;

#[test]
fn skip_consent_is_exactly_terminal_and_not_no_input_and_yes() {
    quickcheck::quickcheck(consent_matrix as fn(bool, bool, bool) -> bool);
    fn consent_matrix(no_input: bool, promptable: bool, confirmed: bool) -> bool {
        allow(no_input, promptable, confirmed) == (promptable && !no_input && confirmed)
    }
}

#[test]
fn the_eight_consent_states_are_witnessed() {
    assert!(!allow(true, true, true));
    assert!(!allow(true, true, false));
    assert!(!allow(true, false, true));
    assert!(!allow(true, false, false));
    assert!(!allow(false, false, true));
    assert!(!allow(false, false, false));
    assert!(allow(false, true, true));
    assert!(!allow(false, true, false));
}

#[test]
fn a_passed_or_blocked_scan_is_routed_without_a_prompt() {
    let options = Options::default();
    assert!(route(&options, Verdict::Passed, "download:fixture").is_ok());
    let error = route(
        &options,
        Verdict::Blocked(
            "security gate 'scan' blocked harness execution (exit 1)\nCRITICAL minimist".into(),
        ),
        "download:fixture",
    )
    .unwrap_err();
    assert_eq!(error.code, "gate_blocked");
    assert!(error.message.contains("CRITICAL minimist"));
}

#[test]
fn an_interrupted_scan_refuses_when_no_one_can_answer() {
    let options = Options {
        no_input: true,
        ..Default::default()
    };
    let error = route(
        &options,
        Verdict::Interrupted {
            gate: "scan".into(),
        },
        "download:fixture",
    )
    .unwrap_err();
    assert_eq!(error.code, "gate_interrupted");
    assert!(error.message.contains("was interrupted (Ctrl+C)"));
    assert!(error.message.contains("scan cancelled"));
}

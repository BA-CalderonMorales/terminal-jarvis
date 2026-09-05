use super::*;
use crate::cli::args::Options;
use crate::cli::logic::guard_ask::{bracket, consent, in_frame};

fn options(
    dry_run: bool,
    no_input: bool,
    allow_dangerous: bool,
    confirm: Option<String>,
) -> Options {
    Options {
        output: crate::cli::args::OutputMode::Plain,
        no_color: false,
        verbose: false,
        dry_run,
        no_input,
        confirm,
        allow_dangerous,
        narrate: true,
    }
}

#[test]
fn reject_irrelevant_errors_on_lifecycle_options() {
    let opts = options(false, true, false, None); // no_input
    let err = reject_irrelevant(&opts).unwrap_err();
    assert_eq!(err.exit_code, 2);
    assert!(err.message.contains("lifecycle options are not valid"));
}

#[test]
fn reject_irrelevant_errors_for_dry_run_only() {
    let opts = options(true, false, false, None);
    let err = reject_irrelevant(&opts).unwrap_err();
    assert_eq!(err.exit_code, 2);
}

#[test]
fn reject_irrelevant_errors_for_confirm_token_only() {
    let opts = options(false, false, false, Some("cap:h".to_string()));
    let err = reject_irrelevant(&opts).unwrap_err();
    assert_eq!(err.exit_code, 2);
    assert!(err.message.contains("lifecycle options are not valid"));
}

#[test]
fn reject_irrelevant_errors_for_allow_dangerous_only() {
    let opts = options(false, false, true, None);
    let err = reject_irrelevant(&opts).unwrap_err();
    assert_eq!(err.exit_code, 2);
    assert!(err.message.contains("lifecycle options are not valid"));
}

#[test]
fn reject_irrelevant_ok_when_no_options() {
    let opts = options(false, false, false, None);
    assert!(reject_irrelevant(&opts).is_ok());
}

#[test]
fn confirm_error_contains_token() {
    let err = confirm_error("test:token");
    assert!(err.message.contains("test:token"));
    assert!(err.next_action.contains("test:token"));
}

#[test]
fn the_in_frame_key_speaks_the_same_y_n_grammar_as_the_terminal() {
    use crate::tui::input::Key;
    // pressing y must confirm: the painted row AND the consent answer
    let (row, answer) = in_frame(Some(Key::Char('y')), false);
    assert_eq!(row, "confirmed");
    assert!(consent(answer).is_ok(), "'y' must pass the consent gate");
    let (row, answer) = in_frame(Some(Key::Char('Y')), true);
    assert_eq!(row, "confirmed");
    assert!(consent(answer).is_ok(), "'Y' must pass the consent gate");
    // the bracket's default is real: Enter confirms the add/update
    // direction (install, update) and declines the destructive one
    let (row, answer) = in_frame(Some(Key::Enter), true);
    assert_eq!(row, "confirmed");
    assert!(consent(answer).is_ok(), "Enter confirms a [Y/n] prompt");
    let (row, answer) = in_frame(Some(Key::Enter), false);
    assert_eq!(row, "cancelled -- nothing was run");
    assert!(consent(answer).is_err(), "Enter declines a [y/N] prompt");
    // n declines in both directions; EOF/Ctrl-D never confirms
    for key in [Some(Key::Char('n')), Some(Key::Escape), None] {
        let (row, answer) = in_frame(key, true);
        assert_eq!(row, "cancelled -- nothing was run");
        assert!(consent(answer).is_err());
        let (_, answer) = in_frame(key, false);
        assert!(consent(answer).is_err());
    }
}

#[test]
fn the_bracket_tells_the_truth_about_the_default() {
    assert_eq!(bracket(true), "[Y/n]");
    assert_eq!(bracket(false), "[y/N]");
}

use super::*;
use crate::cli::args::Options;

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

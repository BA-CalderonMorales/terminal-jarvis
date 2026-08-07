use std::io::IsTerminal;

use super::*;

fn a(args: &[&str]) -> Action {
    parse(args.iter().map(|s| s.to_string())).unwrap()
}
fn e(args: &[&str]) -> Result<Action, String> {
    parse(args.iter().map(|s| s.to_string()))
}

#[test]
fn help_variants() {
    assert_eq!(a(&["tj", "help"]), Action::Help);
    assert_eq!(a(&["tj", "--help"]), Action::Help);
    assert_eq!(a(&["tj", "-h"]), Action::Help);
    assert_eq!(a(&["tj"]), Action::Help);
}
#[test]
fn version_variants() {
    assert_eq!(a(&["tj", "version"]), Action::Version { verbose: false });
    assert_eq!(
        a(&["tj", "version", "--verbose"]),
        Action::Version { verbose: true }
    );
    assert_eq!(
        a(&["tj", "version", "--info"]),
        Action::Version { verbose: true }
    );
    assert_eq!(a(&["tj", "--version"]), Action::Version { verbose: false });
    assert_eq!(a(&["tj", "-v"]), Action::Version { verbose: false });
    assert_eq!(
        a(&["tj", "-v", "version"]),
        Action::Version { verbose: false }
    );
    assert_eq!(a(&["tj", "--info"]), Action::Version { verbose: true });
}
#[test]
fn version_non_flag_rejects_with_exact_usage() {
    assert_eq!(
        super::values::version(&["bogus".into()]),
        Err("usage: terminal-jarvis version [--verbose|--info|-v]".into())
    );
    assert_eq!(
        super::values::version(&["-x".into()]),
        Err("unknown flag '-x'; usage: terminal-jarvis version [--verbose|--info|-v]".into())
    );
    assert_eq!(
        super::values::version(&["-v".into()]),
        Ok(Action::Version { verbose: false })
    );
    assert_eq!(
        super::values::version(&["--info".into()]),
        Ok(Action::Version { verbose: true })
    );
}

#[test]
fn bare_launch_mirrors_stdin_stdout_terminals() {
    assert_eq!(
        super::action_parser::bare_launch(&[], &super::Options::default()),
        std::io::stdout().is_terminal() && std::io::stdin().is_terminal()
    );
    assert!(!super::action_parser::bare_launch(
        &["list".into()],
        &super::Options::default()
    ));
}

#[test]
fn version_rejects_extra() {
    assert!(e(&["tj", "--version", "bogus"]).is_err());
    assert!(e(&["tj", "--info", "bogus"]).is_err());
    assert!(e(&["tj", "version", "bogus"]).is_err());
    assert!(e(&["tj", "-v", "bogus"]).is_err());
}
#[test]
fn list_status_check_current_use_show() {
    assert_eq!(a(&["tj", "list"]), Action::List);
    assert_eq!(a(&["tj", "tools"]), Action::List);
    assert_eq!(a(&["tj", "check"]), Action::Check);
    assert_eq!(a(&["tj", "status"]), Action::Check);
    assert_eq!(a(&["tj", "current"]), Action::Current);
    assert_eq!(
        a(&["tj", "use", "opencode"]),
        Action::Use("opencode".to_string())
    );
    assert_eq!(
        a(&["tj", "show", "opencode"]),
        Action::Show("opencode".to_string())
    );
    assert!(e(&["tj", "use"]).is_err());
    assert!(e(&["tj", "show"]).is_err());
}

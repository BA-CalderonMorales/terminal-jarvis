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

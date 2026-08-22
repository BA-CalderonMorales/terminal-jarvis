//! The session seam's error faces: load failures keep the repair-the-file
//! advice, write failures point at the home location instead.

use super::session_error;

#[test]
fn load_failures_advise_repairing_the_session_file() {
    let failure = session_error("corrupt session");
    assert_eq!(failure.code, "session_invalid");
    assert_eq!(
        failure.next_action,
        "repair or remove the Terminal Jarvis session file"
    );
}

#[test]
fn write_failures_advise_on_the_home_location() {
    let failure =
        super::session_write_error(std::io::Error::from(std::io::ErrorKind::ReadOnlyFilesystem));
    assert_eq!(failure.code, "session_unwritable");
    assert_eq!(failure.exit_code, 3);
    assert!(failure.next_action.contains("TERMINAL_JARVIS_HOME"));
}

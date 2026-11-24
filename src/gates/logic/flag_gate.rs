//! Gate data may name the flag that turns it on (`flag = "<name>"`). A name the
//! flags registry does not declare is a schema error; a declared one loads the
//! gate exactly when `flags::enabled` says it is on.

use std::io;

/// Whether a gate naming `flag` loads now.
pub fn admitted(flag: &str) -> io::Result<bool> {
    if !crate::flags::is_declared(flag) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("gate names unknown flag '{flag}'"),
        ));
    }
    Ok(crate::flags::enabled(flag))
}

/// Test pin: the scanner-choice gates read their flag through `flags::enabled`
/// like any other, so the declared/used scan sees it. Not used in release code.
#[cfg(test)]
pub fn scanner_choice_on() -> bool {
    crate::flags::enabled("scanner-choice")
}

//! Gate data may name the flag that turns it on. Only the flags listed in
//! `KNOWN` are valid in gate data; each entry is a literal `flags::enabled`
//! call, so the flags registry scan sees the flag as used.

use std::io;

type Check = fn() -> bool;

const KNOWN: &[(&str, Check)] = &[("scanner-choice", scanner_choice)];

fn scanner_choice() -> bool {
    crate::flags::enabled("scanner-choice")
}

/// Whether a gate naming `flag` loads now. A name no row knows is a schema error.
pub fn admitted(flag: &str) -> io::Result<bool> {
    KNOWN
        .iter()
        .find(|(name, _)| *name == flag)
        .map(|(_, check)| check())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("gate names unknown flag '{flag}'"),
            )
        })
}

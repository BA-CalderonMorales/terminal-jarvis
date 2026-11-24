//! Header initials under color: the styled product name must abbreviate to its
//! plain initials, never to a leaked escape character.

use super::candidates;

#[test]
fn a_colored_identity_abbreviates_to_plain_initials() {
    let core = "\x1b[1;36mTerminal Jarvis\x1b[0m · ACTIVE none · READY 1/1 ready";
    let kept = candidates(core);
    assert_eq!(kept[0], "TJ · READY 1/1 ready");
    assert!(kept.iter().all(|line| !line.contains('\x1b')), "{kept:?}");
}

#[test]
fn a_plain_identity_abbreviates_the_same_way() {
    let kept = candidates("Terminal Jarvis · ACTIVE none · READY 0/23");
    assert_eq!(kept[0], "TJ · READY 0/23");
    assert_eq!(kept[1], "READY 0/23");
}

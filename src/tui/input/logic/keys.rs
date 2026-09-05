//! Keys: decodes terminal bytes into semantic key events; escape soup is
//! swallowed whole, never leaked into the command line.

use std::io::{self, Read};

/// One decoded input event from the raw terminal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    Char(char),
    Escape,
    Enter,
    Backspace,
    ClearLine,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Ignored,
    Dead,
}

/// Blocks until one key arrives; `None` means EOF or Ctrl-D -- the session
/// ends. Signal interrupts (resize ticks) retry instead of dying.
pub fn read_key() -> Option<Key> {
    // Scoped lock: the escape resolver's reader thread needs the stdin
    // lock free, so the first byte reads and releases before the dance.
    let first = {
        let mut sin = io::stdin().lock();
        next_byte(&mut sin)?
    };
    if first == 0x1b {
        return super::escape::resolve_and_decode(first);
    }
    let mut sin = io::stdin().lock();
    decode(&mut sin, first)
}

fn next_byte(sin: &mut impl Read) -> Option<u8> {
    if let Some(byte) = super::escape::pending() {
        return Some(byte);
    }
    let mut one = [0u8; 1];
    loop {
        break match sin.read(&mut one) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Ok(0) => None,
            Ok(_) => Some(one[0]),
            Err(_) => None,
        };
    }
}

pub(crate) fn decode(sin: &mut impl Read, first: u8) -> Option<Key> {
    let key = match first {
        0x04 => return None,
        b'\r' | b'\n' => Key::Enter,
        0x7f | 0x08 => Key::Backspace,
        0x15 => Key::ClearLine,
        0x1b => match (next_byte(sin), next_byte(sin)) {
            (Some(b'[' | b'O'), Some(last)) => csi(sin, last),
            _ => Key::Ignored,
        },
        0x20..=0x7e => Key::Char(first as char),
        lead @ 0xc2..=0xf4 => utf8(sin, lead),
        _ => Key::Ignored,
    };
    Some(key)
}

fn csi(sin: &mut impl Read, code: u8) -> Key {
    match code {
        b'A' => Key::Up,
        b'B' => Key::Down,
        b'H' => Key::Home,
        b'F' => Key::End,
        digit @ (b'5' | b'6') => match (digit, next_byte(sin)) {
            (b'5', Some(b'~')) => Key::PageUp,
            (b'6', Some(b'~')) => Key::PageDown,
            _ => Key::Ignored,
        },
        _ => Key::Ignored,
    }
}

fn utf8(sin: &mut impl Read, lead: u8) -> Key {
    let extra = match lead {
        0xc2..=0xdf => 1,
        0xe0..=0xef => 2,
        _ => 3,
    };
    let mut bytes = vec![lead];
    for _ in 0..extra {
        match next_byte(sin) {
            Some(byte) => bytes.push(byte),
            None => return Key::Ignored,
        }
    }
    match std::str::from_utf8(&bytes) {
        Ok(text) => text.chars().next().map(Key::Char).unwrap_or(Key::Ignored),
        Err(_) => Key::Ignored,
    }
}

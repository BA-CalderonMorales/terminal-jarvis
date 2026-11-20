//! Read one bounded CSI/SS3 sequence, leaving the next key untouched.
pub fn continuation(second: u8, mut next: impl FnMut() -> Option<u8>) -> Vec<u8> {
    let mut tail = vec![second];
    while tail.len() < 32 {
        let Some(byte) = next() else { break };
        tail.push(byte);
        if (0x40..=0x7e).contains(&byte) {
            break;
        }
    }
    tail
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sequences_stop_at_the_final_byte() {
        for (intro, bytes, expected) in [
            (b'[', b"5~x".as_slice(), b"[5~".as_slice()),
            (b'[', b"6~x".as_slice(), b"[6~".as_slice()),
            (b'O', b"Bx".as_slice(), b"OB".as_slice()),
            (b'[', b"1;5Ax".as_slice(), b"[1;5A".as_slice()),
        ] {
            let mut bytes = bytes.iter().copied();
            assert_eq!(continuation(intro, || bytes.next()), expected);
            assert_eq!(bytes.next(), Some(b'x'));
        }
    }
    #[test]
    fn unfinished_sequences_have_a_finite_budget() {
        assert_eq!(continuation(b'[', || None), b"[");
        assert!(continuation(b'[', || Some(b'1')).len() <= 32);
    }
}

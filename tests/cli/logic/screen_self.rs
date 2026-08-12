use super::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_sequence_keeps_the_badge_on_the_prompt_row() {
        let bytes =
            b"READY 1 / 1 ready\r\n\r\n[>_]::fixture::0.1.13 \n\x1b[2mactive: fixture\x1b[0m\x1b[1A\r[>_]::fixture::0.1.13 ";
        let screen = render(bytes);
        let lines = screen.lines();
        eprintln!("lines: {lines:?}");
        assert!(screen.contains("[>_]::fixture::0.1.13"));
    }

    #[test]
    fn erase_followed_by_reprint_leaves_no_control_glyph() {
        let bytes = b"\x1b[2K[>_]::fixture::0.1.13 \r\x1b[2K[>_]::fixture::0.1.13 ";
        let screen = render(bytes);
        eprintln!("lines: {:?}", screen.lines());
        assert!(screen.contains("[>_]::fixture::0.1.13"));
        assert!(screen.no_line_contains("^"), "lines: {:?}", screen.lines());
    }
}

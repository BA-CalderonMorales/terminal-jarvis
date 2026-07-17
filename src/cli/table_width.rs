pub(super) fn display_width(value: &str) -> usize {
    value.chars().map(character_width).sum()
}

pub(super) fn pad(value: &str, target: usize) -> String {
    format!(
        "{value}{}",
        " ".repeat(target.saturating_sub(display_width(value)))
    )
}

pub(super) fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .map(|width| width.clamp(40, 120))
        .unwrap_or(100)
}

pub(super) fn character_width(character: char) -> usize {
    let value = character as u32;
    if character.is_control() || combining(value) {
        0
    } else if wide(value) {
        2
    } else {
        1
    }
}

fn combining(value: u32) -> bool {
    matches!(
        value,
        0x0300..=0x036f
            | 0x1ab0..=0x1aff
            | 0x1dc0..=0x1dff
            | 0x20d0..=0x20ff
            | 0xfe00..=0xfe0f
            | 0xfe20..=0xfe2f
            | 0xe0100..=0xe01ef
    )
}

fn wide(value: u32) -> bool {
    matches!(
        value,
        0x1100..=0x115f
            | 0x2329..=0x232a
            | 0x2e80..=0xa4cf
            | 0xac00..=0xd7a3
            | 0xf900..=0xfaff
            | 0xfe10..=0xfe19
            | 0xfe30..=0xfe6f
            | 0xff00..=0xff60
            | 0xffe0..=0xffe6
            | 0x1f300..=0x1faff
            | 0x20000..=0x3fffd
    )
}

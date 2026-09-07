//! ViewportPage: bubble-to-bubble paging -- one `╭─` row per press.

/// Jumps the scroll offset one bubble down (`true`) or up.
pub fn page(body: &[String], offset: usize, down: bool) -> usize {
    let starts: Vec<usize> = body
        .iter()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with("╭─"))
        .map(|(index, _)| index)
        .collect();
    if down {
        starts
            .iter()
            .find(|&&start| start > offset)
            .copied()
            .unwrap_or(offset)
    } else {
        starts
            .iter()
            .rev()
            .find(|&&start| start < offset)
            .copied()
            .unwrap_or(offset)
    }
}

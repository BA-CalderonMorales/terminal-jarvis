use super::super::width::{character_width, display_width as width};

pub(super) fn wrap(value: &str, limit: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for part in value.split('\n') {
        let start = lines.len();
        let mut line = String::new();
        for token in part.split_whitespace() {
            let segments = segments(token);
            for (index, segment) in segments.iter().enumerate() {
                if width(segment) > limit {
                    if !line.is_empty() {
                        lines.push(std::mem::take(&mut line));
                    }
                    chunks(segment, limit, &mut lines);
                    continue;
                }
                let glued = index > 0 && width(&line) + width(segment) <= limit;
                let spaced =
                    index == 0 && !line.is_empty() && width(&line) + 1 + width(segment) <= limit;
                if glued {
                    line.push_str(segment);
                } else if spaced {
                    line.push(' ');
                    line.push_str(segment);
                } else if line.is_empty() {
                    line = segment.clone();
                } else {
                    lines.push(std::mem::take(&mut line));
                    line = segment.to_string();
                }
            }
        }
        if !line.is_empty() {
            lines.push(line);
        } else if start == lines.len() {
            lines.push(String::new());
        }
    }
    lines
}

pub(super) fn segments(token: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut run = String::new();
    for character in token.chars() {
        run.push(character);
        if breakable(character) {
            segments.push(std::mem::take(&mut run));
        }
    }
    if !run.is_empty() {
        segments.push(run);
    }
    segments
}

fn breakable(character: char) -> bool {
    matches!(
        character,
        '/' | '.' | ',' | ':' | ';' | '-' | '_' | '=' | '\\' | '@' | '+'
    )
}

fn chunks(word: &str, limit: usize, lines: &mut Vec<String>) {
    let mut chunk = String::new();
    for character in word.chars() {
        if !chunk.is_empty() && width(&chunk) + character_width(character) > limit {
            lines.push(std::mem::take(&mut chunk));
        }
        chunk.push(character);
        if width(&chunk) >= limit {
            lines.push(std::mem::take(&mut chunk));
        }
    }
    if !chunk.is_empty() {
        lines.push(chunk);
    }
}

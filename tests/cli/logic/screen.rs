#[derive(Default)]
pub struct Screen {
    rows: Vec<Vec<char>>,
    pub row: usize,
    pub col: usize,
}

impl Screen {
    pub fn lines(&self) -> Vec<String> {
        self.rows.iter().map(|row| row.iter().collect()).collect()
    }

    pub fn displayed(&self) -> String {
        self.lines().join("\n")
    }

    pub fn contains(&self, needle: &str) -> bool {
        self.lines().iter().any(|line| line.contains(needle))
    }

    pub fn no_line_contains(&self, needle: &str) -> bool {
        self.lines().iter().all(|line| !line.contains(needle))
    }
}

pub fn render(bytes: &[u8]) -> Screen {
    let mut screen = Screen::default();
    let chars: Vec<char> = String::from_utf8_lossy(bytes).chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '\x1b' {
            index = escape(&mut screen, &chars, index);
        } else {
            write_char(&mut screen, chars[index]);
            index += 1;
        }
    }
    screen
}

fn write_char(screen: &mut Screen, character: char) {
    let col = screen.col;
    match character {
        '\r' => screen.col = 0,
        '\n' => {
            screen.row += 1;
            screen.col = 0;
        }
        _ => {
            if screen.row >= screen.rows.len() {
                screen.rows.resize(screen.row + 1, Vec::new());
            }
            let row = &mut screen.rows[screen.row];
            if col >= row.len() {
                row.resize(col + 1, ' ');
            }
            row[col] = character;
            screen.col += 1;
        }
    }
}

fn escape(screen: &mut Screen, chars: &[char], start: usize) -> usize {
    if start + 1 < chars.len() && chars[start + 1] == '[' {
        let mut index = start + 2;
        while index < chars.len() && !matches!(chars[index], '@'..='~') {
            index += 1; // skip the parameter run
        }
        let params: String = chars[start + 2..index].iter().collect();
        if index < chars.len() {
            apply(screen, &params, chars[index]);
            index += 1;
        }
        index
    } else {
        start + 2
    }
}

fn apply(screen: &mut Screen, params: &str, final_: char) {
    let first = params
        .trim_start_matches('?')
        .split(';')
        .next()
        .and_then(|p| p.parse::<usize>().ok());
    match final_ {
        'A' => screen.row = screen.row.saturating_sub(first.unwrap_or(1)),
        'B' => screen.row += first.unwrap_or(1),
        'K' if screen.row < screen.rows.len() => {
            let at = if first == Some(2) { 0 } else { screen.col };
            screen.rows[screen.row].truncate(at);
            screen.col = 0;
        }
        _ => {}
    }
}

#[cfg(test)]
#[path = "screen_self.rs"]
mod self_tests;

use super::*;

fn unquote(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars().peekable();
    let mut quoted = false;
    while let Some(character) = chars.next() {
        if quoted {
            if character == '\'' {
                quoted = false;
            } else {
                out.push(character);
            }
        } else if character == '\'' {
            quoted = true;
        } else if character == '\\' {
            if chars.peek() == Some(&'\'') {
                chars.next();
                out.push('\'');
            } else {
                out.push('\\');
            }
        } else {
            out.push(character);
        }
    }
    out
}

fn tokens(value: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = value.chars().peekable();
    let mut quoted = false;
    while let Some(character) = chars.next() {
        match character {
            '\'' => {
                quoted = !quoted;
                current.push(character);
            }
            ' ' if !quoted => {
                words.push(std::mem::take(&mut current));
            }
            '\\' if !quoted && chars.peek() == Some(&'\'') => {
                current.push(character);
                current.push(chars.next().unwrap());
            }
            _ => current.push(character),
        }
    }
    words.push(current);
    words
}

fn render_roundtrips(command: String, args: Vec<String>) -> bool {
    if command.is_empty() {
        return true;
    }
    let plan = CommandPlan::new(command.clone(), args.clone());
    let binding = plan.render();
    let rendered = tokens(&binding);
    if unquote(&rendered[0]) != command {
        return false;
    }
    args.iter().enumerate().all(|(index, arg)| {
        rendered
            .get(index + 1)
            .is_some_and(|word| unquote(word) == *arg)
    })
}

#[test]
fn render_roundtrips_all_words() {
    quickcheck::quickcheck(render_roundtrips as fn(String, Vec<String>) -> bool);
}

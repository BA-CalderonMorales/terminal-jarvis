pub fn outcome(command: &str, code: i32, body: &str) -> String {
    format!(
        "{{\"schema_version\":1,\"command\":\"{}\",\"ok\":{},\"exit_code\":{},\"data\":{{\"text\":\"{}\"}},\"error\":null}}\n",
        escape(command),
        code == 0,
        code,
        escape(body)
    )
}

pub fn failure(
    command: &str,
    exit_code: i32,
    code: &str,
    message: &str,
    next_action: &str,
) -> String {
    format!(
        "{{\"schema_version\":1,\"command\":\"{}\",\"ok\":false,\"exit_code\":{},\"data\":null,\"error\":{{\"code\":\"{}\",\"message\":\"{}\",\"next_action\":\"{}\"}}}}\n",
        escape(command),
        exit_code,
        escape(code),
        escape(message),
        escape(next_action)
    )
}

fn escape(value: &str) -> String {
    let mut out = String::new();
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            value if value < '\u{20}' => out.push_str(&format!("\\u{:04x}", value as u32)),
            value => out.push(value),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_escapes_control_characters() {
        let output = outcome("check", 0, "a\n\"b\\c");
        assert!(output.contains("a\\n\\\"b\\\\c"));
        assert_eq!(output.lines().count(), 1);
    }
}

use super::escape;

fn unescape(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            out.push(character);
            continue;
        }
        match chars.next() {
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                    if let Some(decoded) = char::from_u32(code) {
                        out.push(decoded);
                    }
                }
            }
            _ => out.push('\\'),
        }
    }
    out
}

fn escape_roundtrips(value: String) -> bool {
    unescape(&escape(&value)) == value
}

fn escape_has_no_low_control_characters(value: String) -> bool {
    !escape(&value).chars().any(|character| character < '\u{20}')
}

fn outcome_json_fields(value: String) -> bool {
    let output = super::outcome("cmd", 0, &value);
    output.contains("\\n") == value.contains('\n')
        && output.lines().count() == 1
        && output.starts_with("{\"schema_version\":1,\"command\":\"cmd\"")
}

#[test]
fn escape_properties() {
    quickcheck::quickcheck(escape_roundtrips as fn(String) -> bool);
    quickcheck::quickcheck(escape_has_no_low_control_characters as fn(String) -> bool);
    quickcheck::quickcheck(outcome_json_fields as fn(String) -> bool);
}

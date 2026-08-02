use super::*;
use std::collections::BTreeMap;

fn clean_key(value: &str) -> String {
    let cleaned: String = value
        .chars()
        .filter(|character| *character != '=' && *character != '#' && *character != '\n')
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "_".to_string()
    } else {
        trimmed.to_string()
    }
}

fn clean_value(value: &str) -> String {
    value
        .chars()
        .filter(|character| *character != '#' && *character != '\n')
        .collect::<String>()
        .trim()
        .to_string()
}

fn parse_roundtrips(pairs: Vec<(String, String)>) -> bool {
    let mut expected = BTreeMap::new();
    for (key, value) in pairs {
        expected.insert(clean_key(&key), clean_value(&value));
    }
    let input = expected
        .iter()
        .map(|(key, value)| format!("{key} = {value}"))
        .collect::<Vec<_>>()
        .join("\n");
    parse(&input) == Ok(expected)
}

fn list_roundtrips(values: Vec<String>) -> bool {
    let sanitized: Vec<String> = values
        .into_iter()
        .map(|value| value.replace('"', ""))
        .collect();
    let rendered = format!(
        "[{}]",
        sanitized
            .iter()
            .map(|value| format!("\"{value}\""))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut fields = Fields::new();
    fields.insert("k".to_string(), rendered);
    list(&fields, "k") == Ok(sanitized)
}

fn list_is_total(payload: String) -> bool {
    let mut fields = Fields::new();
    fields.insert("k".to_string(), format!("[{payload}]"));
    list(&fields, "k").is_ok() || list(&fields, "k").is_err()
}

fn parse_is_total(input: String) -> bool {
    parse(&input).is_ok() || parse(&input).is_err()
}

#[test]
fn parser_roundtrips_and_totality() {
    quickcheck::quickcheck(parse_roundtrips as fn(Vec<(String, String)>) -> bool);
    quickcheck::quickcheck(list_roundtrips as fn(Vec<String>) -> bool);
    quickcheck::quickcheck(list_is_total as fn(String) -> bool);
    quickcheck::quickcheck(parse_is_total as fn(String) -> bool);
}

#[test]
fn list_missing_key_is_empty() {
    assert_eq!(list(&Fields::new(), "absent"), Ok(Vec::new()));
}

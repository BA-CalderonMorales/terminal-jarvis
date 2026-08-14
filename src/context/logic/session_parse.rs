//! The strict session parser, shared by runtime loading and diagnostics.
//! One authoritative parse: unknown keys are malformed, duplicate entries
//! must agree, and values must be quoted and non-empty. Runtime loading
//! degrades to defaults with a warning; diagnostics reports the exact code.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    Malformed,
    Conflicting,
}

pub fn parse(data: &str) -> Result<String, ParseError> {
    let mut values = Vec::new();
    for line in data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let (key, value) = line.split_once('=').ok_or(ParseError::Malformed)?;
        if key.trim() != "active_harness" {
            return Err(ParseError::Malformed);
        }
        let value = value
            .trim()
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .filter(|value| !value.trim().is_empty())
            .ok_or(ParseError::Malformed)?;
        values.push(value.to_string());
    }
    let Some(first) = values.first().cloned() else {
        return Err(ParseError::Malformed);
    };
    if values.iter().any(|value| value != &first) {
        Err(ParseError::Conflicting)
    } else {
        Ok(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_single_quoted_value() {
        assert_eq!(parse("active_harness = \"codex\"\n"), Ok("codex".into()));
    }

    #[test]
    fn rejects_unquoted_empty_and_unknown_keys() {
        assert_eq!(
            parse("active_harness = codex\n"),
            Err(ParseError::Malformed)
        );
        assert_eq!(parse("active_harness = \"\"\n"), Err(ParseError::Malformed));
        assert_eq!(parse("name = \"x\"\n"), Err(ParseError::Malformed));
        assert_eq!(parse(""), Err(ParseError::Malformed));
    }

    #[test]
    fn rejects_conflicting_duplicates_and_accepts_identical_ones() {
        let conflict = "active_harness = \"a\"\nactive_harness = \"b\"\n";
        assert_eq!(parse(conflict), Err(ParseError::Conflicting));
        let same = "active_harness = \"a\"\nactive_harness = \"a\"\n";
        assert_eq!(parse(same), Ok("a".into()));
    }

    #[test]
    fn tolerates_comments_and_spacing() {
        assert_eq!(
            parse("# note\nactive_harness=\"codex\"\n"),
            Ok("codex".into())
        );
    }
}

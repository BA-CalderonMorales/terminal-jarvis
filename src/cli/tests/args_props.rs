use super::*;
use quickcheck::{Arbitrary, Gen};

impl Arbitrary for OutputMode {
    fn arbitrary(gen: &mut Gen) -> Self {
        match u8::arbitrary(gen) % 3 {
            0 => OutputMode::Rich,
            1 => OutputMode::Plain,
            _ => OutputMode::Json,
        }
    }
}

fn parse_cli_is_total(words: Vec<String>) -> bool {
    parse_cli(words.clone()).is_ok() || parse_cli(words).is_err()
}

fn parse_is_total(words: Vec<String>) -> bool {
    parse(words.clone()).is_ok() || parse(words).is_err()
}

fn plain_json_mutually_exclusive() -> bool {
    let parsed = parse_cli(["tj", "--plain", "--json"].map(String::from));
    parsed.is_err()
}

fn verbose_only_for_check_or_version(words: Vec<String>) -> bool {
    match parse_cli(words) {
        Ok(parsed) if parsed.options.verbose => {
            matches!(parsed.action, Action::Check | Action::Version { .. })
        }
        _ => true,
    }
}

fn output_mode_roundtrip(mode: OutputMode) -> bool {
    let flag = match mode {
        OutputMode::Rich => "--plain",
        OutputMode::Plain => "--plain",
        OutputMode::Json => "--json",
    };
    let parsed = parse_cli(["tj", flag, "list"].map(String::from)).unwrap();
    let expected = if mode == OutputMode::Rich {
        OutputMode::Plain
    } else {
        mode
    };
    parsed.options.output == expected
}

#[test]
fn args_parser_properties() {
    quickcheck::quickcheck(parse_cli_is_total as fn(Vec<String>) -> bool);
    quickcheck::quickcheck(parse_is_total as fn(Vec<String>) -> bool);
    quickcheck::quickcheck(verbose_only_for_check_or_version as fn(Vec<String>) -> bool);
    quickcheck::quickcheck(output_mode_roundtrip as fn(OutputMode) -> bool);
    assert!(plain_json_mutually_exclusive());
}

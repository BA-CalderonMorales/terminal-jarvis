use super::support::flag_file;
use crate::flags::{build, env_name, parse_switch, resolve, Choices, Flag, Source};

fn flag(default: bool) -> Flag {
    let file = flag_file("two-words", default, "preview");
    build([("two-words", file.as_str())].into_iter())
        .unwrap()
        .remove(0)
}

fn env_with(value: Option<&'static str>) -> impl Fn(&str) -> Option<String> {
    move |name| {
        (name == "TJ_FLAG_TWO_WORDS")
            .then(|| value.map(String::from))
            .flatten()
    }
}

#[test]
fn env_names_upper_case_and_swap_dashes() {
    assert_eq!(env_name("two-words"), "TJ_FLAG_TWO_WORDS");
}

#[test]
fn switches_accept_the_documented_spellings_only() {
    for on in ["1", "on", "ON", "True", " true "] {
        assert_eq!(parse_switch(on), Some(true), "{on}");
    }
    for off in ["0", "off", "Off", "FALSE"] {
        assert_eq!(parse_switch(off), Some(false), "{off}");
    }
    for junk in ["", "yes", "2", "enabled"] {
        assert_eq!(parse_switch(junk), None, "{junk}");
    }
}

#[test]
fn env_beats_settings_beats_default() {
    let mut choices = Choices::new();
    let none = env_with(None);
    let row = resolve(&flag(false), &choices, &none);
    assert_eq!((row.enabled, row.source), (false, Source::Default));
    choices.insert("two-words".into(), true);
    let row = resolve(&flag(false), &choices, &none);
    assert_eq!((row.enabled, row.source), (true, Source::Settings));
    let row = resolve(&flag(false), &choices, &env_with(Some("0")));
    assert_eq!((row.enabled, row.source), (false, Source::Env));
    let row = resolve(&flag(false), &Choices::new(), &env_with(Some("ON")));
    assert_eq!((row.enabled, row.source), (true, Source::Env));
}

#[test]
fn an_unreadable_env_value_is_ignored() {
    let mut choices = Choices::new();
    choices.insert("two-words".into(), true);
    let row = resolve(&flag(false), &choices, &env_with(Some("banana")));
    assert_eq!((row.enabled, row.source), (true, Source::Settings));
}

#[test]
fn unknown_names_in_the_file_are_ignored() {
    let mut choices = Choices::new();
    choices.insert("ghost".into(), true);
    let row = resolve(&flag(false), &choices, &env_with(None));
    assert_eq!(row.source, Source::Default);
}

//! Quickcheck properties for the tui line grammar: bare names switch, bare
//! unknown words run the active agent, stray numbers error, and no input ever
//! resolves to an unhandled state.

use super::*;

const RESERVED: &[&str] = &[
    "exit", "quit", "home", "clear", "list", "tools", "status", "check", "help", "current", "use",
    "show", "info", "plan", "run", "install", "update", "auth", "config", "cache", "security",
    "gate", "tui", "version",
];

fn to_harness(name: &str) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: "probe".into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

fn distinct_names(names: Vec<String>) -> Vec<String> {
    let mut unique = std::collections::BTreeSet::new();
    names
        .into_iter()
        .filter(|name| {
            !name.is_empty()
                && name
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric())
                && !name.chars().all(|character| character.is_ascii_digit())
                && !RESERVED.contains(&name.as_str())
        })
        .filter(|name| unique.insert(name.clone()))
        .collect()
}

fn bare_known_harness_switches(names: Vec<String>) -> bool {
    let names = distinct_names(names);
    if names.is_empty() {
        return true;
    }
    let harnesses: Vec<Harness> = names.iter().map(|name| to_harness(name)).collect();
    let pick = names[0].clone();
    matches!(resolve(&pick, &harnesses), Resolved::Run(args::Action::Use(switched)) if switched == pick)
}

fn bare_unknown_words_run_the_active_agent(names: Vec<String>) -> bool {
    let names = distinct_names(names);
    if names.is_empty() {
        return true;
    }
    let phrase = names.join(" ");
    matches!(resolve(&phrase, &[]), Resolved::Run(args::Action::Run(words)) if words == names)
}

fn out_of_range_and_zero_numbers_error(number: usize) -> bool {
    matches!(
        resolve(&number.to_string(), &[]),
        Resolved::Error(message) if message.contains("no harness at position")
    )
}

fn every_input_is_resolved_or_empty(input: String) -> bool {
    if input.trim().is_empty() {
        matches!(resolve(&input, &[]), Resolved::Empty)
    } else {
        !matches!(resolve(&input, &[]), Resolved::Empty)
    }
}

#[test]
fn resolve_holds_grammar_properties() {
    quickcheck::quickcheck(bare_known_harness_switches as fn(Vec<String>) -> bool);
    quickcheck::quickcheck(bare_unknown_words_run_the_active_agent as fn(Vec<String>) -> bool);
    quickcheck::quickcheck(out_of_range_and_zero_numbers_error as fn(usize) -> bool);
    quickcheck::quickcheck(every_input_is_resolved_or_empty as fn(String) -> bool);
}

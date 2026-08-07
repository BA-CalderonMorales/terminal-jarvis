//! Quickcheck properties for the picker: the rendered numbered rows always
//! round-trip back into `select`, and `clip` either passes text through or
//! truncates inside the advertised bound.

use super::*;

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
        .filter(|name| !name.is_empty() && !name.chars().any(char::is_whitespace))
        .filter(|name| unique.insert(name.clone()))
        .collect()
}

fn pick_rows_round_trip_to_selection(names: Vec<String>) -> bool {
    let names = distinct_names(names);
    if names.is_empty() {
        return true;
    }
    let harnesses: Vec<Harness> = names.iter().map(|name| to_harness(name)).collect();
    let rendered = pick(&harnesses, None);
    (0..names.len()).all(|index| {
        let Some(line) = rendered.lines().nth(index) else {
            return false;
        };
        let label = (index + 1).to_string();
        line.split_whitespace().next() == Some(label.as_str())
            && line.split_whitespace().nth(1) == Some(names[index].as_str())
            && select(&label, &harnesses) == Some(args::Action::Use(harnesses[index].name.clone()))
    })
}

fn clip_passes_short_text_through(description: String) -> bool {
    clip(&description, 10_000) == description
}

fn clip_truncates_bounded(description: String, limit: usize) -> bool {
    let limit = limit % 200;
    let clipped = clip(&description, limit);
    let count = description.chars().count();
    if count <= limit {
        clipped == description
    } else {
        clipped.chars().count() == limit + 3 && clipped.ends_with("...")
    }
}

#[test]
fn switcher_holds_picker_properties() {
    quickcheck::quickcheck(pick_rows_round_trip_to_selection as fn(Vec<String>) -> bool);
    quickcheck::quickcheck(clip_passes_short_text_through as fn(String) -> bool);
    quickcheck::quickcheck(clip_truncates_bounded as fn(String, usize) -> bool);
}

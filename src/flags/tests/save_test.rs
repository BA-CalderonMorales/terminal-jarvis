use super::support::temp;
use crate::flags::{load_choices, parse_choices, save, settings_path};
use std::fs;
use std::io::ErrorKind;

#[test]
fn save_refuses_a_corrupt_file_and_leaves_it_byte_identical() {
    let home = temp("corrupt");
    let corrupt = "[flags]\nkeep-me = true\nbroken = maybe\n";
    fs::write(settings_path(&home), corrupt).unwrap();
    let error = save(&home, "other", Some(true)).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::InvalidData);
    assert!(error.to_string().contains("could not be parsed"), "{error}");
    assert_eq!(fs::read_to_string(settings_path(&home)).unwrap(), corrupt);
    assert!(!home.join("settings.toml.lock").exists());
}

#[test]
fn save_treats_only_a_missing_file_as_empty() {
    let home = temp("missing");
    save(&home, "a", Some(true)).unwrap();
    assert_eq!(load_choices(&home).len(), 1);
    let dir_home = temp("unreadable");
    fs::create_dir_all(settings_path(&dir_home)).unwrap();
    assert!(save(&dir_home, "a", Some(true)).is_err());
}

#[test]
fn other_tables_are_skipped_before_values_are_validated() {
    let choices = parse_choices("[other]\ntheme = \"dark\"\nnote\n[flags]\na = true\n").unwrap();
    assert_eq!(choices.len(), 1);
    assert!(choices["a"]);
    assert!(parse_choices("[flags]\na = \"dark\"\n").is_err());
}

#[test]
fn concurrent_saves_never_lose_a_choice() {
    let home = temp("concurrent");
    let writers: Vec<_> = (0..8)
        .map(|index| {
            let home = home.clone();
            std::thread::spawn(move || save(&home, &format!("flag-{index}"), Some(true)))
        })
        .collect();
    for writer in writers {
        writer.join().unwrap().unwrap();
    }
    assert_eq!(load_choices(&home).len(), 8);
}

#[test]
fn inline_comments_on_headers_and_values_keep_every_choice() {
    let data = "# settings\n[flags] # mine\nwild-idea = true # try this feature\n\
                calm-mode = false\n[other]\nnote = \"a # b\" # c\n";
    let choices = parse_choices(data).unwrap();
    assert_eq!(choices.len(), 2);
    assert!(choices["wild-idea"]);
    assert!(!choices["calm-mode"]);
    let home = temp("inline");
    fs::write(settings_path(&home), data).unwrap();
    assert_eq!(load_choices(&home).len(), 2);
}

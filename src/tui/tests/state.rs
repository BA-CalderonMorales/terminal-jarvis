use super::*;

fn seed() -> crate::converse::Seed {
    (
        3,
        "codex".to_string(),
        "gemini".to_string(),
        "caching".to_string(),
        false,
    )
}

fn fresh() -> LoopState {
    LoopState {
        body: Vec::new(),
        history: Vec::new(),
        converse: None,
        pending_converse: Some(seed()),
        offset: 0,
        hint: String::new(),
        bottom: false,
        options: args::Options {
            output: args::OutputMode::Rich,
            no_color: false,
            verbose: false,
            dry_run: false,
            no_input: false,
            confirm: None,
            allow_dangerous: false,
            narrate: false,
        },
        debug: false,
        indicator: crate::tui::input::Indicator {
            active: "none".into(),
            debug: false,
        },
    }
}

#[test]
fn enter_accepts_the_notice_with_the_original_seed() {
    let mut state = fresh();
    assert_eq!(notice(&mut state, ""), Some(seed()));
    assert!(state.pending_converse.is_none(), "one Enter accepts once");
    assert_eq!(notice(&mut state, ""), None, "a later blank line is empty");
}

#[test]
fn any_typed_command_cancels_the_notice() {
    let mut state = fresh();
    assert_eq!(notice(&mut state, "list"), None);
    assert!(
        state.pending_converse.is_none(),
        "a command is not a notice answer"
    );
}

#[test]
fn a_blank_line_without_a_notice_stays_empty() {
    let mut state = fresh();
    state.pending_converse = None;
    assert_eq!(notice(&mut state, ""), None);
}

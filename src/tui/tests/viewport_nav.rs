use super::super::viewport_chrome::ViewportState;
use super::super::viewport_raw::Session;
use super::*;

#[test]
fn eof_ends_a_normal_mode_session() {
    let state = ViewportState {
        header: String::new(),
        cwd: String::new(),
        tagline: String::new(),
        prefix: String::new(),
        prefix_cells: 0,
    };
    let body = Vec::new();
    let history = Vec::new();
    let names = vec!["codex".to_string()];
    let mut offset = 0;
    let mut session = Session {
        state: &state,
        hint: "",
        body: &body,
        history: &history,
        offset: &mut offset,
        names: &names,
    };
    let mut mode = Mode::Normal;
    let mut editor = Editor::default();
    assert!(matches!(
        key(&mut mode, &mut editor, Key::Dead, &mut session, 0),
        Flow::Dead
    ));
}

#[test]
fn tab_completes_in_insert_and_is_inert_in_normal() {
    let state = ViewportState {
        header: String::new(),
        cwd: String::new(),
        tagline: String::new(),
        prefix: String::new(),
        prefix_cells: 0,
    };
    let body = Vec::new();
    let history = Vec::new();
    let names = vec!["codex".to_string()];
    let mut offset = 0;
    let mut session = Session {
        state: &state,
        hint: "",
        body: &body,
        history: &history,
        offset: &mut offset,
        names: &names,
    };
    let mut editor = Editor::default();
    for character in "run co".chars() {
        editor.feed(Key::Char(character));
    }
    let mut insert = Mode::Insert;
    assert!(matches!(
        key(&mut insert, &mut editor, Key::Char('\t'), &mut session, 0),
        Flow::Continue(_)
    ));
    assert_eq!(editor.line(), "run codex");

    let before = editor.line().to_string();
    let mut normal = Mode::Normal;
    assert!(matches!(
        key(&mut normal, &mut editor, Key::Char('\t'), &mut session, 0),
        Flow::Continue(_)
    ));
    assert_eq!(editor.line(), before, "Normal scrolls; it never edits");
}

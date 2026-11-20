use super::super::viewport_chrome::ViewportState;
use super::super::viewport_raw::Session;
use super::*;

#[test]
fn a_resize_continues_without_touching_the_prompt_mode_or_offset() {
    let state = ViewportState {
        header: String::new(),
        cwd: String::new(),
        tagline: String::new(),
        prefix: String::new(),
        prefix_cells: 0,
    };
    let body: Vec<String> = (0..50).map(|line| line.to_string()).collect();
    let history = Vec::new();
    let names = vec!["codex".to_string()];
    let mut offset = 7;
    let mut session = Session {
        state: &state,
        hint: "",
        body: &body,
        history: &history,
        offset: &mut offset,
        names: &names,
    };
    let mut editor = Editor::default();
    editor.feed(Key::Char('h'));
    editor.feed(Key::Char('i'));

    let mut insert = Mode::Insert;
    assert!(matches!(
        key(&mut insert, &mut editor, Key::Resized, &mut session, 0),
        Flow::Continue(_)
    ));
    assert!(
        matches!(insert, Mode::Insert),
        "a resize never leaves Insert mode"
    );
    assert_eq!(*session.offset, 7, "a resize never scrolls the body");
    assert_eq!(
        editor.tail_view(0, 40),
        "hi",
        "a resize never edits the line"
    );

    let mut normal = Mode::Normal;
    assert!(matches!(
        key(&mut normal, &mut editor, Key::Resized, &mut session, 0),
        Flow::Continue(_)
    ));
    assert_eq!(
        *session.offset, 7,
        "a resize never scrolls in Normal mode either"
    );
}

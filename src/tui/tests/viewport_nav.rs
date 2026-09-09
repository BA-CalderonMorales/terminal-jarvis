use super::super::viewport_raw::{Session, ViewportState};
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
    let mut offset = 0;
    let mut session = Session {
        state: &state,
        hint: "",
        body: &body,
        history: &history,
        offset: &mut offset,
    };
    let mut mode = Mode::Normal;
    let mut editor = Editor::default();
    assert!(matches!(
        key(&mut mode, &mut editor, Key::Dead, &mut session, 0),
        Flow::Dead
    ));
}

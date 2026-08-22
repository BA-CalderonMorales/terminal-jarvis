//! Viewport prompt: paints the composed frame and reads one line parked
//! past the prompt prefix; chat mode delegates to the classic widget.

use crate::contracts::Harness;
use std::io::Write;
use std::path::Path;

pub fn welcome(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> Vec<String> {
    let o = crate::tui::home::collect(harnesses, catalog_root, state_home);
    crate::tui::screen::welcome(&o.name, o.ready, o.total)
}

#[allow(clippy::too_many_arguments)]
pub fn prompt(
    indicator: &crate::tui::input::Indicator,
    hint: &str,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    body: &[String],
) -> Option<String> {
    let _ = std::io::stdout().flush();
    let size = crate::tui::screen::size();
    let o = crate::tui::home::collect(harnesses, catalog_root, state_home);
    let prefix = indicator.render(true);
    let draft = crate::tui::screen::Draft {
        title: format!("TERMINAL JARVIS v{}", env!("CARGO_PKG_VERSION")),
        status: crate::tui::home::styled(&o),
        body: body.to_vec(),
        prompt: prefix.clone(),
        hint: hint.to_string(),
    };
    let cells = crate::tui::screen::visible_width(&prefix) + 2;
    let painted = crate::tui::screen::parked(crate::tui::screen::frame(size, &draft), size, cells);
    print!("{painted}");
    std::io::stdout().flush().ok()?;
    crate::tui::input::raw_line()
}

/// Chat-mode boot banner: the welcome frame above the first prompt.
pub fn chat_banner(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) {
    let mut out = std::io::stdout();
    crate::tui::home::render(&mut out, harnesses, catalog_root, state_home);
}

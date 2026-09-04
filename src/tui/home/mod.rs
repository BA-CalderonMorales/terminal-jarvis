//! Home dashboard: the welcome shown when the tui starts. Deliberately small
//! -- banner, active harness, readiness, working directory, one hint line.
//! The tool list is not part of the welcome; `list` renders it and a bare
//! number selects, so first-run users see something they can admire.

#[path = "logic/overview.rs"]
mod overview;

#[path = "cwd.rs"]
mod cwd;

pub use cwd::cwd_label;
pub use overview::{collect, header, plain, styled, Overview};

use crate::contracts::Harness;
use std::io::Write;
use std::path::Path;

/// Chat-mode banner: title left, status right when the width allows.
pub fn render(out: &mut dyn Write, harnesses: &[Harness], catalog_root: &Path, state_home: &Path) {
    let o = collect(harnesses, catalog_root, state_home);
    let status = styled(&o);
    let title = "Terminal Jarvis";
    let subtitle =
        "Command center for orchestrating context switching between coding-agent harnesses";
    let width = crate::tui::term::columns();
    if title.chars().count() + 3 + plain(&o).chars().count() > width {
        let _ = writeln!(
            out,
            "{}\n{}\n{}\n",
            crate::tui::screen::accent(title),
            crate::tui::screen::dim(subtitle),
            status
        );
    } else {
        let gap = " ".repeat(width - title.chars().count() - plain(&o).chars().count());
        let _ = writeln!(
            out,
            "{}{}{}\n{}\n",
            crate::tui::screen::accent(title),
            gap,
            status,
            crate::tui::screen::dim(subtitle)
        );
    }
}

//! ViewportChrome: the frozen frame chrome for one prompt session -- header,
//! cwd, status rule, prompt prefix, and the fleet vocabulary the prompt
//! completes against. What does not change while keys edit the line or scroll
//! the body.

use crate::contracts::Harness;
use crate::tui::home;
use crate::tui::input::Indicator;
use crate::tui::screen::Draft;
use std::path::Path;

/// The frozen frame chrome for one prompt session: what does not change while
/// keys edit the line or scroll the body.
pub struct ViewportState {
    pub header: String,
    pub cwd: String,
    pub tagline: String,
    pub prefix: String,
    pub prefix_cells: usize,
}

impl ViewportState {
    pub fn collect(
        indicator: &Indicator,
        harnesses: &[Harness],
        catalog_root: &Path,
        state_home: &Path,
    ) -> Self {
        let o = home::collect(harnesses, catalog_root, state_home);
        let prefix = indicator.render(true);
        Self {
            header: home::header(&o),
            cwd: o.cwd,
            tagline: crate::tui::screen::tagline(
                &o.name,
                o.ready,
                o.total,
                gate(state_home).as_deref(),
            ),
            prefix_cells: crate::tui::screen::visible_width(&prefix) + 1,
            prefix,
        }
    }

    pub fn base_draft(&self, hint: &str, body: &[String], bottom: bool) -> Draft {
        Draft {
            header: self.header.clone(),
            cwd: self.cwd.clone(),
            tagline: self.tagline.clone(),
            body: body.to_vec(),
            prompt: self.prefix.clone(),
            offset: 0,
            hint: hint.to_string(),
            bottom,
        }
    }
}

fn gate(state_home: &Path) -> Option<String> {
    crate::gates::selected(state_home)
        .ok()
        .flatten()
        .map(|selection| selection.name)
}

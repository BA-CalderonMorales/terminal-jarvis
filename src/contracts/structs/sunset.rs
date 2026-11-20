//! A retired tool's dated record. The catalog keeps a retired harness so its
//! binary, install channel, and removal path stay inspectable; this shape is
//! what `graveyard` reads, and it is the only place "why did it stop working?"
//! gets an answer.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sunset {
    /// The day the upstream project retires the tool, `YYYY-MM-DD`. It may be
    /// in the future: a vendor announces a shutdown before it happens, and the
    /// announcement is the useful news.
    pub at: String,
    /// What happened, in the project's own terms.
    pub note: String,
    /// The tool the vendor points users to, when one is named.
    pub replaced_by: Option<String>,
}

/// Whether the retirement date has passed, so prose can say "retired" rather
/// than "retires" about a date the reader can see is still ahead of them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Scheduled,
    Done,
}

impl Sunset {
    pub fn successor(&self) -> &str {
        self.replaced_by.as_deref().unwrap_or("no named successor")
    }

    /// `today` is passed in rather than read here: the contracts are the pure
    /// data model, and a date that can be injected is a date a test can pin.
    /// `YYYY-MM-DD` strings compare correctly as strings, so this needs no
    /// date arithmetic.
    pub fn state(&self, today: &str) -> State {
        if !today.is_empty() && self.at.as_str() <= today {
            State::Done
        } else {
            State::Scheduled
        }
    }

    /// The verb a sentence uses: "retired 2026-06-18" versus "retires
    /// 2026-12-31". An announced shutdown has not happened yet.
    pub fn verb(&self, today: &str) -> &'static str {
        match self.state(today) {
            State::Done => "retired",
            State::Scheduled => "retires",
        }
    }

    /// The adjective a column or chip uses, where a verb would not fit.
    pub fn adjective(&self, today: &str) -> &'static str {
        match self.state(today) {
            State::Done => "retired",
            State::Scheduled => "retiring",
        }
    }
}

//! A retired tool's dated record. The catalog keeps a retired harness so its
//! binary, install channel, and removal path stay inspectable; this shape is
//! what `graveyard` reads, and it is the only place "why did it stop working?"
//! gets an answer.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sunset {
    /// The day the upstream project retired the tool, `YYYY-MM-DD`.
    pub at: String,
    /// What happened, in the project's own terms.
    pub note: String,
    /// The tool the vendor points users to, when one is named.
    pub replaced_by: Option<String>,
}

impl Sunset {
    pub fn successor(&self) -> &str {
        self.replaced_by.as_deref().unwrap_or("no named successor")
    }
}

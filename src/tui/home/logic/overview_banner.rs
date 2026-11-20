//! The dashboard's one-line verdict: active harness, working directory,
//! readiness, and -- only when the catalog has any -- how many tools are
//! retired. Split from the overview data so each file stays one idea.

use super::overview::Overview;

/// Only a catalog that has retirements pays for the extra clause, so a
/// maintained-only fleet keeps the header it always had. A scheduled
/// retirement is its own word: the tool still runs today.
pub fn retirement(o: &Overview) -> String {
    let mut tail = String::new();
    for (count, word) in [(o.retired, "retired"), (o.retiring, "retiring")] {
        if count > 0 {
            tail.push_str(&format!(" · {count} {word}"));
        }
    }
    tail
}

/// The viewport header: one merged line, priority-ordered so a narrow
/// terminal loses the working directory before the readiness verdict. Every
/// clause names a role, so the header follows the theme like the body does.
pub fn header(o: &Overview) -> String {
    use crate::tui::screen::{paint, Role};
    let ready_role = if o.ready == o.total {
        Role::Ok
    } else {
        Role::Warn
    };
    format!(
        "{} · {} {} · {} {}{}",
        paint(Role::Title, "Terminal Jarvis"),
        paint(Role::Dim, "ACTIVE"),
        paint(Role::Accent, &o.name),
        paint(Role::Dim, "READY"),
        paint(ready_role, &format!("{}/{} ready", o.ready, o.total)),
        if o.retired + o.retiring == 0 {
            String::new()
        } else {
            format!(
                " · {} {}",
                paint(Role::Dim, "RETIRED"),
                paint(Role::Warn, &format!("{}{}", o.retired, retiring_clause(o)))
            )
        }
    )
}

/// `· 1 retiring` after the retired count, empty when nothing is scheduled.
fn retiring_clause(o: &Overview) -> String {
    if o.retiring == 0 {
        String::new()
    } else {
        format!(" · {} retiring", o.retiring)
    }
}

fn verdict(o: &Overview) -> String {
    use crate::cli::style;
    if o.ready == o.total {
        format!("{}/{} ready", o.ready, o.total)
    } else {
        style::warning(&format!("{}/{} ready", o.ready, o.total))
    }
}

/// The same line, styled for a terminal that honors ANSI.
pub fn styled(o: &Overview) -> String {
    use crate::cli::style;
    format!(
        "{} {} · {} {} · {} {}{}",
        style::label("ACTIVE"),
        style::heading(&o.name),
        style::label("CWD"),
        o.cwd,
        style::label("READY"),
        verdict(o),
        if o.retired + o.retiring == 0 {
            String::new()
        } else {
            format!(
                " · {} {}",
                style::label("RETIRED"),
                style::warning(&format!("{}{}", o.retired, retiring_clause(o)))
            )
        }
    )
}

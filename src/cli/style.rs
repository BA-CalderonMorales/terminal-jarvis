use std::cell::Cell;
use std::io::IsTerminal;

#[path = "style_diagnostics.rs"]
mod diagnostics;
pub use diagnostics::decisions as diagnostic_decisions;

#[derive(Clone, Copy)]
pub struct Options {
    plain: bool,
    no_color: bool,
}

thread_local! {
    static OPTIONS: Cell<Options> = const { Cell::new(Options { plain: false, no_color: false }) };
}

pub fn set(plain: bool, no_color: bool) -> Options {
    OPTIONS.with(|cell| cell.replace(Options { plain, no_color }))
}

pub fn restore(options: Options) {
    OPTIONS.with(|cell| cell.set(options));
}

pub fn plain() -> bool {
    OPTIONS.with(|cell| cell.get().plain)
}

pub fn heading(value: &str) -> String {
    paint(value, "1;36")
}

pub fn label(value: &str) -> String {
    paint(value, "1;37")
}

pub fn success(value: &str) -> String {
    paint(value, "1;32")
}

pub fn warning(value: &str) -> String {
    paint(value, "1;33")
}

pub fn error(value: &str) -> String {
    format!(
        "{}\n",
        paint_for(&format!("error: {value}"), "1;31", Stream::Stderr)
    )
}

pub fn banner(title: &str, subtitle: &str) -> String {
    if plain() {
        return format!("{title}\n{subtitle}\n\n");
    }
    format!("{}\n{}\n\n", heading(title), paint(subtitle, "2"))
}

fn paint(value: &str, code: &str) -> String {
    paint_for(value, code, Stream::Stdout)
}

#[derive(Clone, Copy)]
enum Stream {
    Stdout,
    Stderr,
}

fn paint_for(value: &str, code: &str, stream: Stream) -> String {
    let term = std::env::var("TERM").ok();
    let terminal = match stream {
        Stream::Stdout => std::io::stdout().is_terminal(),
        Stream::Stderr => std::io::stderr().is_terminal(),
    };
    if color_enabled_for(
        terminal,
        OPTIONS.with(|cell| cell.get().no_color),
        std::env::var_os("NO_COLOR").is_some(),
        term_is_dumb(term.as_deref()),
    ) {
        format!("\x1b[{code}m{value}\x1b[0m")
    } else {
        value.to_string()
    }
}

fn term_is_dumb(term: Option<&str>) -> bool {
    term == Some("dumb")
}

fn color_enabled_for(terminal: bool, no_color: bool, env_no_color: bool, dumb: bool) -> bool {
    terminal && !no_color && !env_no_color && !dumb
}

#[cfg(test)]
#[path = "style_test.rs"]
mod tests;

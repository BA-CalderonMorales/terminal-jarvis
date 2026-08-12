//! Shell: the chat-style read-prompt loop -- results print above the input box.
use crate::{cli::args, contracts::Harness};
use std::path::Path;

#[path = "./canonical.rs"]
mod canonical;
#[path = "./handle.rs"]
mod handle;
#[path = "./help.rs"]
mod help;
#[path = "./session.rs"]
mod session;
#[path = "./status.rs"]
mod status;
#[path = "./verdict.rs"]
mod verdict;
pub use handle::handle;

pub fn run(
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    mut options: args::Options,
) {
    let mut debug = false;
    let mut indicator = super::input::Indicator {
        active: "none".into(),
        debug: false,
    };
    super::sigint::guarded(|| {
        let mut hint = status::modeline(state_home, false, debug);
        status::refresh_indicator(&mut indicator, state_home, debug);
        while let Some(input) = super::input::read_line(&indicator, &hint) {
            let next = handle(harnesses, catalog_root, state_home, &options, &input);
            match next {
                Next::Exit => break,
                Next::Again { picker_shown } => {
                    hint = status::modeline(state_home, picker_shown, debug);
                    status::refresh_indicator(&mut indicator, state_home, debug);
                    println!();
                }
                Next::Debug(toggle) => {
                    debug = toggle.unwrap_or(!debug);
                    options.narrate = debug;
                    hint = status::modeline(state_home, false, debug);
                    status::refresh_indicator(&mut indicator, state_home, debug);
                    println!("debug view {}", if debug { "on" } else { "off" });
                }
            }
        }
    });
    println!();
}

#[path = "./parse.rs"]
mod parse;
pub use parse::{resolve, Next, Resolved};

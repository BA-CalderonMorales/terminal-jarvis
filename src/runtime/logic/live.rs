//! Live: a piped child whose stdout and stderr arrive as tagged lines while
//! the caller drives the wait loop -- so a streaming surface can keep
//! painting (and scrolling) in step with the child, silence included.

use super::stream::status_code;
use crate::contracts::CapabilityPlan;
use std::io::{self, BufRead};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

/// One tagged child line: the reply stream or the noise stream.
pub enum Line {
    Out(String),
    Err(String),
}

/// What one wait step saw: a line, an idle tick, or the end of the pipe.
pub enum Event {
    Line(Line),
    Idle,
    Done,
}

/// A spawned child with tagged line pumps; `wait` joins the child.
pub struct Running {
    child: Child,
    rx: Receiver<Line>,
}

impl Running {
    /// The next line within `timeout`: `Idle` keeps the caller's ticker
    /// alive while the child thinks, `Done` means both pipes closed.
    pub fn next(&mut self, timeout: Duration) -> Event {
        match self.rx.recv_timeout(timeout) {
            Ok(line) => Event::Line(line),
            Err(RecvTimeoutError::Timeout) => Event::Idle,
            Err(RecvTimeoutError::Disconnected) => Event::Done,
        }
    }

    /// The exit status once the pipes have closed.
    pub fn wait(&mut self) -> i32 {
        status_code(self.child.wait().expect("child status"))
    }
}

/// Spawns the plan fully piped (stdin null, so headless runs never prompt)
/// with one pump thread per stream feeding tagged lines.
pub fn spawn(plan: &CapabilityPlan, extra: &[String]) -> io::Result<Running> {
    let mut command = Command::new(crate::security::resolved(&plan.command.command).as_ref());
    command
        .args(&plan.command.args)
        .args(extra)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    super::runner::reset_sigint_in_child(&mut command);
    let mut child = command.spawn()?;
    let (tx, rx) = mpsc::channel::<Line>();
    for lines in child
        .stdout
        .take()
        .map(|pipe| pump(pipe, Line::Out))
        .into_iter()
        .chain(child.stderr.take().map(|pipe| pump(pipe, Line::Err)))
    {
        let tx = tx.clone();
        std::thread::spawn(move || {
            for line in lines {
                if tx.send(line).is_err() {
                    return;
                }
            }
        });
    }
    drop(tx);
    Ok(Running { child, rx })
}

fn pump<R: io::Read + Send + 'static>(pipe: R, tag: fn(String) -> Line) -> mpsc::IntoIter<Line> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for line in io::BufReader::new(pipe).lines().map_while(Result::ok) {
            if tx.send(tag(line)).is_err() {
                return;
            }
        }
    });
    rx.into_iter()
}

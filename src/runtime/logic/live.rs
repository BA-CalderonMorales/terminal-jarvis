//! A piped invocation owns its process tree and bounded output queue.
use super::stream::status_code;
#[cfg(unix)]
use super::tree_unix as tree;
#[cfg(windows)]
use super::tree_windows as tree;
use crate::contracts::CapabilityPlan;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;
use std::{
    io,
    process::{Child, Command, Stdio},
};

pub enum Line {
    Out(String),
    Err(String),
}
pub enum Event {
    Line(Line),
    Idle,
    Done,
}
pub struct Running {
    child: Child,
    rx: Receiver<Line>,
    tree: tree::Tree,
}
impl Running {
    pub fn next(&mut self, timeout: Duration) -> Event {
        if self.child.try_wait().ok().flatten().is_some() {
            self.tree.kill();
        }
        match self.rx.recv_timeout(timeout) {
            Ok(line) => Event::Line(line),
            Err(RecvTimeoutError::Timeout) => {
                if self.child.try_wait().ok().flatten().is_some() {
                    self.tree.kill();
                }
                Event::Idle
            }
            Err(RecvTimeoutError::Disconnected) => {
                if self.child.try_wait().ok().flatten().is_some() {
                    Event::Done
                } else {
                    std::thread::sleep(timeout);
                    Event::Idle
                }
            }
        }
    }
    pub fn wait(&mut self) -> i32 {
        self.child.wait().map(status_code).unwrap_or(3)
    }
    pub fn kill(&mut self) {
        self.tree.kill();
        let _ = self.child.kill();
    }
}
impl Drop for Running {
    fn drop(&mut self) {
        self.kill();
        let _ = self.child.wait();
    }
}

pub fn spawn(plan: &CapabilityPlan, extra: &[String]) -> io::Result<Running> {
    let mut command = Command::new(crate::security::resolved(&plan.command.command).as_ref());
    command
        .args(&plan.command.args)
        .args(extra)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let (mut child, tree) = tree::spawn(&mut command)?;
    let (tx, rx) = mpsc::sync_channel(128);
    super::live_pump::pump(
        child.stdout.take().expect("piped stdout"),
        Line::Out,
        tx.clone(),
    );
    super::live_pump::pump(child.stderr.take().expect("piped stderr"), Line::Err, tx);
    Ok(Running { child, rx, tree })
}

#[cfg(all(test, unix))]
#[path = "../tests/live_drop.rs"]
mod drop_tests;

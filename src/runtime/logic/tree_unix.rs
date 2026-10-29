//! A private session owns ordinary descendants of a piped invocation.
use std::{
    io,
    process::{Child, Command},
};
pub struct Tree(Option<u32>);
pub fn spawn(command: &mut Command) -> io::Result<(Child, Tree)> {
    super::runner::harden_child(command, true);
    let child = command.spawn()?;
    let tree = Tree(Some(child.id()));
    Ok((child, tree))
}
impl Tree {
    pub fn kill(&mut self) {
        if let Some(pid) = self.0.take() {
            unsafe {
                kill(-(pid as i32), 9);
            }
        }
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        self.kill();
    }
}
unsafe extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
}

#[cfg(test)]
#[path = "../tests/tree.rs"]
mod tests;

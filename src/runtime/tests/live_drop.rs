use super::*;
use crate::{cli::logic::test_support, contracts::Capability};
#[test]
fn dropping_a_live_invocation_reaps_its_immediate_child() {
    let (command, args) = test_support::shell("sleep 60", "exit 0");
    let plan = test_support::plan(Capability::Headless, &command, args);
    let running = spawn(&plan, &[]).unwrap();
    let pid = running.child.id() as i32;
    drop(running);
    let exists = unsafe { kill(pid, 0) } == 0;
    if exists {
        unsafe {
            waitpid(pid, std::ptr::null_mut(), 0);
        }
    }
    assert!(!exists, "terminated child was left unreaped");
}
unsafe extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
    fn waitpid(pid: i32, status: *mut i32, options: i32) -> i32;
}

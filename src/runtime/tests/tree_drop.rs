use super::*;
use std::time::{Duration, Instant};
#[test]
fn dropping_tree_ownership_without_a_running_wrapper_terminates_the_child() {
    let (mut child, tree) = spawn(Command::new("sleep").arg("60")).unwrap();
    drop(tree);
    let deadline = Instant::now() + Duration::from_secs(2);
    let exited = loop {
        if child.try_wait().unwrap().is_some() {
            break true;
        }
        if Instant::now() >= deadline {
            break false;
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    if !exited {
        let _ = child.kill();
    }
    let _ = child.wait();
    assert!(exited, "tree owner dropped without terminating its child");
}

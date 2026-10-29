use crate::{cli::logic::test_support, contracts::Capability, runtime};
use std::time::{Duration, Instant};
fn start(script: &str) -> runtime::LiveRunning {
    let (command, args) = test_support::shell(script, "exit 0");
    runtime::spawn_live(
        &test_support::plan(Capability::Headless, &command, args),
        &[],
    )
    .unwrap()
}
fn descendant(running: &mut runtime::LiveRunning) -> i32 {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        assert!(
            Instant::now() < deadline,
            "child did not announce descendant"
        );
        if let runtime::LiveEvent::Line(runtime::LiveLine::Out(pid)) =
            running.next(Duration::from_millis(20))
        {
            return pid.parse().unwrap();
        }
    }
}
fn stopped(pid: i32) -> bool {
    if unsafe { kill(pid, 0) } != 0 {
        return true;
    }
    #[cfg(target_os = "linux")]
    return std::fs::read_to_string(format!("/proc/{pid}/stat"))
        .is_ok_and(|stat| stat.split_whitespace().nth(2) == Some("Z"));
    #[cfg(not(target_os = "linux"))]
    false
}
unsafe extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
}
#[test]
fn dropping_an_invocation_terminates_its_descendant() {
    let mut running = start("sleep 60 & echo $!; wait");
    let pid = descendant(&mut running);
    drop(running);
    let deadline = Instant::now() + Duration::from_secs(3);
    while !stopped(pid) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(stopped(pid), "descendant survived invocation drop");
}
#[test]
fn an_exited_parent_cannot_leave_the_pipe_open_forever() {
    let mut running = start("sleep 60 & echo $!; exit 0");
    let pid = descendant(&mut running);
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        assert!(Instant::now() < deadline, "descendant kept the pipe open");
        if matches!(
            running.next(Duration::from_millis(20)),
            runtime::LiveEvent::Done
        ) {
            break;
        }
    }
    assert_eq!(running.wait(), 0);
    assert!(stopped(pid));
}
#[test]
fn a_long_line_is_delivered_in_bounded_chunks_without_losing_bytes() {
    let mut running = start("printf '%65536s' x");
    let mut total = 0;
    loop {
        match running.next(Duration::from_millis(20)) {
            runtime::LiveEvent::Line(runtime::LiveLine::Out(line)) => {
                assert!(line.len() <= 16384);
                total += line.len();
            }
            runtime::LiveEvent::Done => break,
            _ => {}
        }
    }
    assert_eq!(total, 65536);
    assert_eq!(running.wait(), 0);
}

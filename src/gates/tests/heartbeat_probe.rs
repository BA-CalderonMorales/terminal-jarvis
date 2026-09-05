use crate::gates::logic::heartbeat::Heartbeat;
use crate::gates::logic::live::TICK;

#[cfg(unix)]
#[test]
fn heartbeat_probe() {
    if std::env::var("TJ_HB_PROBE").as_deref() != Ok("ticks") {
        return;
    }
    let mut pump = Heartbeat::start("security scan (scan) ...");
    std::thread::sleep(TICK + TICK);
    pump.stop();
}

#[cfg(unix)]
#[test]
fn the_first_tick_lands_exactly_on_five_seconds() {
    let exe = std::env::current_exe().unwrap();
    let output = std::process::Command::new(&exe)
        .args(["heartbeat_probe", "--nocapture"])
        .env("TJ_HB_PROBE", "ticks")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("    5s"), "first tick at 5s:\n{stderr}");
    assert!(!stderr.contains("    1s"), "no premature tick:\n{stderr}");
    assert!(!stderr.contains("    0s"), "no zero-second tick:\n{stderr}");
}

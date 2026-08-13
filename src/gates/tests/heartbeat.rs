use crate::gates::logic::heartbeat::{live_line, live_width, Heartbeat, TICK};
use crate::gates::tests_util::{lock, scan_gate};

#[test]
fn live_line_starts_on_cr_and_names_the_scan() {
    let line = live_line("security scan (scan) ...", 10);
    assert!(line.starts_with("\rsecurity scan (scan) ..."));
    assert!(line.contains("10s"));
    assert!(line.contains("can take a minute or more"));
}

#[test]
fn every_tick_has_the_same_width_for_any_prefix_and_elapsed() {
    quickcheck::quickcheck(ticks_are_fixed_width as fn(String, u8) -> bool);
    fn ticks_are_fixed_width(prefix: String, secs: u8) -> bool {
        let line = live_line(&prefix, u64::from(secs));
        line.starts_with("\r") && line.len() == live_width(&prefix) + 1
    }
}

#[test]
fn the_elapsed_seconds_are_always_visible() {
    quickcheck::quickcheck(elapsed_is_reported as fn(String, u8) -> bool);
    fn elapsed_is_reported(prefix: String, secs: u8) -> bool {
        live_line(&prefix, u64::from(secs)).contains(&format!("{secs}s"))
    }
}

#[cfg(unix)]
#[test]
fn a_fast_scan_never_takes_a_heartbeat_tick() {
    let _guard = lock();
    let root = std::env::temp_dir().join(format!("tj-hb-fast-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let gate = scan_gate(&root, "fast", "#!/bin/sh\n");
    let scan = crate::gates::logic::stream::run(&gate, false).unwrap();
    assert!(!scan.heartbeat, "a sub-tick scan must not redraw");
    assert_eq!(scan.code, 0);
    let _ = std::fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn a_slow_scan_redraws_until_it_finishes() {
    let _guard = lock();
    let root = std::env::temp_dir().join(format!("tj-hb-slow-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let seconds = TICK.as_secs() + 1;
    let gate = scan_gate(&root, "slow", &format!("#!/bin/sh\nsleep {seconds}\n"));
    let scan = crate::gates::logic::stream::run(&gate, false).unwrap();
    assert!(scan.heartbeat, "a scan past the first tick must redraw");
    assert_eq!(scan.code, 0);
    let _ = std::fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn narrating_scans_never_redraw_themselves() {
    let _guard = lock();
    let root = std::env::temp_dir().join(format!("tj-hb-narrate-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let seconds = TICK.as_secs() + 1;
    let gate = scan_gate(&root, "loud", &format!("#!/bin/sh\nsleep {seconds}\n"));
    let scan = crate::gates::logic::stream::run(&gate, true).unwrap();
    assert!(!scan.heartbeat, "the narrated view streams, never redraws");
    let _ = std::fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn heartbeat_stops_promptly_when_the_scan_ends() {
    let pump = Heartbeat::start("security scan (pump) ...");
    let deadline = std::time::Instant::now() + TICK + TICK;
    while !pump.fired() && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert!(pump.fired(), "a first tick must arrive within two ticks");
    let started = std::time::Instant::now();
    pump.stop();
    assert!(
        started.elapsed() < TICK,
        "stop must not wait for the next tick"
    );
}

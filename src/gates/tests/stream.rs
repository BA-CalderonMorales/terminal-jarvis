use crate::gates::logic::stream::tee;
use std::io;

/// A scripted reader that replays exact outcomes, so the retry-vs-break
/// decision in `tee` (EINTR must retry; other errors must stop) is
/// witnessed deterministically instead of racing a real pipe.
struct Scripted {
    steps: Vec<io::Result<Vec<u8>>>,
    cursor: usize,
}

impl io::Read for Scripted {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.cursor >= self.steps.len() {
            return Ok(0);
        }
        let step = self.steps[self.cursor].as_ref().map(|text| {
            let take = text.len().min(buffer.len());
            buffer[..take].copy_from_slice(&text[..take]);
            take
        });
        match step {
            Ok(take) => {
                self.cursor += 1;
                Ok(take)
            }
            Err(error) => {
                self.cursor += 1;
                Err(io::Error::new(error.kind(), "scripted"))
            }
        }
    }
}

fn interrupted() -> io::Result<Vec<u8>> {
    Err(io::Error::new(io::ErrorKind::Interrupted, "eintr"))
}

fn other() -> io::Result<Vec<u8>> {
    Err(io::Error::other("boom"))
}

#[test]
fn tee_retries_an_interrupted_read_and_keeps_the_payload() {
    let mut reader = Scripted {
        steps: vec![interrupted(), Ok(b"kept".to_vec())],
        cursor: 0,
    };
    assert_eq!(tee(&mut reader, false), "kept");
}

#[test]
fn tee_stops_on_a_real_error_and_forgets_nothing_read() {
    let mut reader = Scripted {
        steps: vec![Ok(b"seen".to_vec()), other(), Ok(b"ghost".to_vec())],
        cursor: 0,
    };
    assert_eq!(tee(&mut reader, false), "seen");
}

#[cfg(unix)]
#[test]
fn a_hung_scan_with_a_pipe_holding_descendant_returns_bounded() {
    let _guard = crate::gates::tests_util::lock();
    let previous = std::env::var_os("TERMINAL_JARVIS_GATE_TIMEOUT_SECS");
    std::env::set_var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS", "1");
    let root = std::env::temp_dir().join(format!("tj-hang-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let gate =
        crate::gates::tests_util::scan_gate(&root, "hang", "#!/bin/sh\nsleep 60 &\nsleep 60\n");
    let started = std::time::Instant::now();
    let scan = crate::gates::logic::stream::run(&gate, false).unwrap();
    assert!(started.elapsed() < std::time::Duration::from_secs(15));
    assert_ne!(scan.code, 0);
    assert!(scan.output.contains("timed out after 1s and was killed"));
    assert!(matches!(
        crate::gates::verdict_for(&gate.name, scan.code, &scan.output),
        crate::gates::Verdict::Interrupted { .. }
    ));
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS", value),
        None => std::env::remove_var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS"),
    }
    let _ = std::fs::remove_dir_all(root);
}

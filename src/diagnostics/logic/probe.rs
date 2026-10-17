use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const PROBE_TIMEOUT: Duration = Duration::from_secs(3);
const PROBE_POLL: Duration = Duration::from_millis(50);

pub fn version(command: &str, args: &[String]) -> Option<String> {
    let Ok(mut child) = Command::new(command)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return None;
    };
    let started = Instant::now();
    let finished = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if started.elapsed() < PROBE_TIMEOUT => std::thread::sleep(PROBE_POLL),
            Ok(None) => break None,
            Err(_) => return None,
        }
    };
    if finished.is_none() {
        let _ = child.kill();
        let _ = child.wait();
        return None;
    }
    let mut output = child.stdout.take()?;
    let mut text = String::new();
    output.read_to_string(&mut text).ok()?;
    let lines = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let choice = lines
        .iter()
        .find(|line| line.bytes().any(|byte| byte.is_ascii_digit()))
        .or_else(|| lines.first())
        .copied()?;
    Some(choice.chars().take(48).collect())
}

#[cfg(test)]
#[path = "../tests/probe.rs"]
mod tests;

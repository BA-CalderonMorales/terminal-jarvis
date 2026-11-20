use std::time::{Duration, Instant};

const PROBE_TIMEOUT: Duration = Duration::from_secs(3);
const PROBE_POLL: Duration = Duration::from_millis(50);

pub fn version(command: &str, args: &[String]) -> Option<String> {
    let mut child = crate::runtime::spawn_program(command, args).ok()?;
    let started = Instant::now();
    let mut text = String::new();
    loop {
        if started.elapsed() >= PROBE_TIMEOUT {
            return None;
        }
        match child.next(PROBE_POLL) {
            crate::runtime::LiveEvent::Line(crate::runtime::LiveLine::Out(line)) => {
                if text.len() + line.len() + 1 > 64 * 1024 {
                    return None;
                }
                text.push_str(&line);
                text.push('\n');
            }
            crate::runtime::LiveEvent::Line(crate::runtime::LiveLine::Err(_)) => {}
            crate::runtime::LiveEvent::Idle => {}
            crate::runtime::LiveEvent::Done => break,
        }
    }
    if child.wait() != 0 {
        return None;
    }
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

#[cfg(all(test, unix))]
#[path = "../tests/probe_limits.rs"]
mod limits;

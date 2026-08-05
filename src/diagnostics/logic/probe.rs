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
mod tests {
    use super::*;

    fn run(script: &str) -> Option<String> {
        let script = script.to_string();
        version("sh", &["-c".to_string(), script])
    }

    #[test]
    fn prefers_the_digit_bearing_line() {
        assert_eq!(
            run("printf 'Warning: no daemon\\nclient version is 0.30.11\\n'").as_deref(),
            Some("client version is 0.30.11")
        );
    }

    #[test]
    fn falls_back_to_the_first_non_empty_line_without_digits() {
        assert_eq!(
            run("printf '\\nMistral Vibe CLI\\nusage: open with--version\\n'").as_deref(),
            Some("Mistral Vibe CLI")
        );
    }

    #[test]
    fn empty_or_unstartable_probes_report_none() {
        assert_eq!(run("exit 0"), None);
        assert_eq!(version("definitely-missing-binary-xyz", &[]), None);
    }

    #[test]
    fn long_lines_are_truncated() {
        let long = "x".repeat(200);
        assert_eq!(
            run(&format!("printf '{long}\\n'")).map(|v| v.len()),
            Some(48)
        );
    }
}

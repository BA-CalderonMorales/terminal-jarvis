use super::converse_live;
use crate::contracts::Harness;
use std::time::{Duration, Instant};

pub(super) fn speak_turn(
    turn: &mut converse_live::Turn<'_>,
    harnesses: &[Harness],
    name: &str,
    prompt: &str,
    allow_dangerous: bool,
) -> Result<String, String> {
    let home = turn.state_home.to_path_buf();
    let mut since_paint = Instant::now();
    let mut running = crate::cli::headless_stream(
        harnesses,
        name,
        prompt,
        &home,
        allow_dangerous,
        &mut |line| turn.push_row(line, &mut since_paint),
    )?;
    let _raw = crate::tui::term::enable_raw();
    crate::tui::input::spawn_watcher();
    let mut reply = String::new();
    loop {
        match running.next(Duration::from_millis(180)) {
            crate::runtime::LiveEvent::Line(crate::runtime::LiveLine::Out(line)) => {
                if reply.len() + line.len() > 256 * 1024 {
                    return Err("reply exceeded 256 KiB; conversation stopped".into());
                }
                reply.push_str(&line);
                reply.push('\n');
            }
            crate::runtime::LiveEvent::Line(crate::runtime::LiveLine::Err(line)) => {
                for row in line.split('\n') {
                    turn.push_row(row, &mut since_paint);
                }
            }
            crate::runtime::LiveEvent::Idle => turn.repaint(&mut since_paint),
            crate::runtime::LiveEvent::Done => break,
        }
        if turn.drain_keys(&mut since_paint) {
            return Err("cancelled".into());
        }
    }
    turn.push_flush();
    let code = running.wait();
    if code == 0 {
        Ok(reply.trim_end().to_string())
    } else {
        Err(format!("exit {code}"))
    }
}

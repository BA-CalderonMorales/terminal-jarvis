//! Owns a PTY session through timeout and unwinding.
use std::process::{Child, ExitStatus};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

pub struct SessionChild {
    child: Child,
    pub stopped: Arc<AtomicBool>,
    pub deadline: Instant,
}

impl SessionChild {
    pub fn new(child: Child) -> Self {
        Self {
            child,
            stopped: Arc::new(AtomicBool::new(false)),
            deadline: Instant::now() + Duration::from_secs(120),
        }
    }

    pub fn wait(&mut self) -> ExitStatus {
        loop {
            if let Some(status) = self.child.try_wait().expect("PTY child status") {
                self.stopped.store(true, Ordering::Release);
                return status;
            }
            assert!(
                Instant::now() < self.deadline,
                "PTY scenario exceeded 120 seconds"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

impl Drop for SessionChild {
    fn drop(&mut self) {
        if !self.stopped.load(Ordering::Acquire) {
            super::pty_tree::stop_descendants(self.child.id());
        }
        unsafe {
            kill(-(self.child.id() as i32), 9);
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.stopped.store(true, Ordering::Release);
    }
}

unsafe extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn panic_and_deadline_reap_the_owned_child() {
        for timeout in [false, true] {
            let child = std::process::Command::new("sleep")
                .arg("60")
                .spawn()
                .unwrap();
            let pid = child.id();
            let result = std::panic::catch_unwind(|| {
                let mut owned = SessionChild::new(child);
                if timeout {
                    owned.deadline = Instant::now();
                    owned.wait();
                } else {
                    panic!("fixture failed");
                }
            });
            assert!(result.is_err());
            assert_eq!(unsafe { kill(pid as i32, 0) }, -1, "child survived cleanup");
        }
    }
}

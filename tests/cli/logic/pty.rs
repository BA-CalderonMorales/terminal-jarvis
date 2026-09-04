use super::pty_io::{drain, wait_until};
use std::ffi::CStr;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::fd::FromRawFd;
use std::os::unix::process::CommandExt;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};

unsafe extern "C" {
    fn posix_openpt(flags: i32) -> i32;
    fn grantpt(fd: i32) -> i32;
    fn unlockpt(fd: i32) -> i32;
    fn ptsname(fd: i32) -> *mut std::ffi::c_char;
    fn setsid() -> i32;
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct Winsize {
    row: u16,
    col: u16,
    xpixel: u16,
    ypixel: u16,
}

#[cfg(target_os = "linux")]
const TIOCSCTTY: u64 = 0x540E;
#[cfg(not(target_os = "linux"))]
const TIOCSCTTY: u64 = 0x2000_745A;

pub fn run_pty(command: Command) -> (ExitStatus, Vec<u8>) {
    run_pty_input(command, &[])
}
pub fn run_pty_input(command: Command, input: &[u8]) -> (ExitStatus, Vec<u8>) {
    run_pty_probe(command, &[(input, None)])
}
pub fn run_pty_probe(
    mut command: Command,
    stages: &[(&[u8], Option<&[u8]>)],
) -> (ExitStatus, Vec<u8>) {
    let (master, slave) = open_pair();
    unsafe {
        command.pre_exec(|| {
            setsid();
            ioctl(0, TIOCSCTTY, 0);
            Ok(())
        });
    }
    command
        .stdin(Stdio::from(slave.try_clone().unwrap()))
        .stdout(Stdio::from(slave.try_clone().unwrap()))
        .stderr(Stdio::from(slave));
    let mut child = command.spawn().expect("CLI starts in pseudo-terminal");
    drop(command);
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let mut writer = master.try_clone().expect("pseudo-terminal writer clones");
    let reader_bytes = bytes.clone();
    let reader = std::thread::spawn(move || drain(master, reader_bytes));
    for (input, after) in stages {
        if let Some(marker) = after {
            wait_until(&bytes, marker);
        }
        let _ = writer.write_all(input);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    drop(writer);
    let status = child.wait().expect("CLI exits in pseudo-terminal");
    reader.join().expect("pseudo-terminal reader joins");
    let final_bytes = bytes.lock().unwrap().clone();
    (status, final_bytes)
}

fn open_pair() -> (File, File) {
    let fd = unsafe { posix_openpt(2) };
    assert!(fd >= 0, "posix_openpt failed");
    assert_eq!(unsafe { grantpt(fd) }, 0, "grantpt failed");
    assert_eq!(unsafe { unlockpt(fd) }, 0, "unlockpt failed");
    // A real size so the viewport boots in the pty; without this the child
    // sees a 0x0 window and every acceptance test silently runs chat mode.
    let size = Winsize {
        row: 24,
        col: 80,
        ..Default::default()
    };
    #[cfg(target_os = "linux")]
    const TIOCSWINSZ: u64 = 0x5413;
    #[cfg(not(target_os = "linux"))]
    const TIOCSWINSZ: u64 = 0x4008_7474;
    unsafe { ioctl(fd, TIOCSWINSZ, &size as *const Winsize) };
    let path = unsafe { CStr::from_ptr(ptsname(fd)) }.to_string_lossy();
    let slave = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path.as_ref())
        .expect("pseudo-terminal slave opens");
    (unsafe { File::from_raw_fd(fd) }, slave)
}

use std::ffi::CStr;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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

fn drain(mut master: File, bytes: Arc<Mutex<Vec<u8>>>) {
    let mut buffer = [0_u8; 4096];
    loop {
        match master.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => bytes.lock().unwrap().extend_from_slice(&buffer[..size]),
            Err(error) if error.raw_os_error() == Some(5) => break,
            Err(error) => panic!("pseudo-terminal read failed: {error}"),
        }
    }
}

fn wait_until(bytes: &Arc<Mutex<Vec<u8>>>, marker: &[u8]) {
    for _ in 0..=500 {
        let seen = bytes.lock().unwrap().clone();
        if seen.windows(marker.len()).any(|window| window == marker) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!("timed out waiting for {:?} in: {:?}", marker, bytes);
}

fn open_pair() -> (File, File) {
    let fd = unsafe { posix_openpt(2) };
    assert!(fd >= 0, "posix_openpt failed");
    assert_eq!(unsafe { grantpt(fd) }, 0, "grantpt failed");
    assert_eq!(unsafe { unlockpt(fd) }, 0, "unlockpt failed");
    let path = unsafe { CStr::from_ptr(ptsname(fd)) }.to_string_lossy();
    let slave = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path.as_ref())
        .expect("pseudo-terminal slave opens");
    (unsafe { File::from_raw_fd(fd) }, slave)
}

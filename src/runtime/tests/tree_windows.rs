use super::*;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use std::time::{Duration, Instant};

#[test]
fn descendant_helper() {
    if std::env::var_os("TJ_JOB_HELPER").is_none() {
        return;
    }
    let mut child = Command::new("cmd")
        .args(["/c", "ping -n 60 127.0.0.1 >nul"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    println!("DESCENDANT={}", child.id());
    std::thread::sleep(Duration::from_secs(60));
    let _ = child.wait();
}

#[test]
fn dropping_the_job_terminates_the_child_and_its_descendant() {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([
            "--exact",
            "runtime::logic::tree_windows::tests::descendant_helper",
            "--nocapture",
        ])
        .env("TJ_JOB_HELPER", "1")
        .stdout(Stdio::piped());
    let (mut child, job) = spawn(&mut command).unwrap();
    let pipe = child.stdout.take().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(pipe).lines().map_while(Result::ok) {
            if let Some(pid) = line.strip_prefix("DESCENDANT=") {
                let _ = tx.send(pid.parse::<u32>().unwrap());
                break;
            }
        }
    });
    let pid = match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(pid) => pid,
        Err(error) => {
            drop(job);
            let _ = child.wait();
            panic!("helper failed: {error}");
        }
    };
    let process = unsafe { OpenProcess(0x1000, 0, pid) };
    assert!(!process.is_null());
    drop(job);
    child.wait().unwrap();
    let deadline = Instant::now() + Duration::from_secs(3);
    let mut code = 259; // STILL_ACTIVE
    while code == 259 && Instant::now() < deadline {
        assert_ne!(unsafe { GetExitCodeProcess(process, &mut code) }, 0);
        std::thread::sleep(Duration::from_millis(20));
    }
    unsafe {
        CloseHandle(process);
    }
    assert_ne!(code, 259, "descendant survived job close");
}

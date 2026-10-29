//! Reap detached fixture descendants before terminating their PTY owner.
pub(super) fn stop_descendants(root: u32) {
    #[cfg(target_os = "linux")]
    unsafe {
        kill(root as i32, 19);
    }
    #[cfg(not(target_os = "linux"))]
    unsafe {
        kill(root as i32, 17);
    }
    if let Ok(output) = std::process::Command::new("ps")
        .args(["-axo", "pid=,ppid="])
        .output()
    {
        let pairs: Vec<(u32, u32)> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| {
                let mut words = line.split_whitespace();
                Some((words.next()?.parse().ok()?, words.next()?.parse().ok()?))
            })
            .collect();
        let mut owned = vec![root];
        let mut cursor = 0;
        while cursor < owned.len() {
            let parent = owned[cursor];
            for &(pid, ppid) in &pairs {
                if ppid == parent && !owned.contains(&pid) {
                    owned.push(pid);
                }
            }
            cursor += 1;
        }
        for pid in owned.into_iter().rev() {
            unsafe {
                kill(pid as i32, 9);
            }
        }
    }
}

unsafe extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
}

//! Start suspended, assign to a kill-on-close job, then resume its thread.
//! Assignment precedes execution, so descendants cannot escape that race.
use super::windows_ffi::*;
use std::os::windows::{io::AsRawHandle, process::CommandExt};
use std::{
    io,
    mem::size_of,
    process::{Child, Command},
};
pub struct Tree(Handle);
pub fn spawn(command: &mut Command) -> io::Result<(Child, Tree)> {
    let tree = Tree(unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) });
    if tree.0.is_null() {
        return Err(io::Error::last_os_error());
    }
    let mut limits = ExtendedLimits::default();
    limits.basic.flags = 0x2000; // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
    if unsafe {
        SetInformationJobObject(
            tree.0,
            9,
            (&limits as *const ExtendedLimits).cast(),
            size_of::<ExtendedLimits>() as u32,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    let mut child = command.creation_flags(4).spawn()?; // CREATE_SUSPENDED
    let assigned = unsafe { AssignProcessToJobObject(tree.0, child.as_raw_handle()) } != 0;
    let result = if assigned {
        resume(child.id())
    } else {
        Err(io::Error::last_os_error())
    };
    if let Err(error) = result {
        tree.kill();
        let _ = child.kill();
        let _ = child.wait();
        return Err(error);
    }
    Ok((child, tree))
}
fn resume(pid: u32) -> io::Result<()> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(4, 0) }; // TH32CS_SNAPTHREAD
    if snapshot as isize == -1 {
        return Err(io::Error::last_os_error());
    }
    let mut entry = ThreadEntry {
        size: size_of::<ThreadEntry>() as u32,
        ..Default::default()
    };
    let mut found = unsafe { Thread32First(snapshot, &mut entry) } != 0;
    let mut result = Err(io::Error::new(
        io::ErrorKind::NotFound,
        "suspended child thread missing",
    ));
    while found {
        if entry.process_id == pid {
            let thread = unsafe { OpenThread(2, 0, entry.thread_id) }; // THREAD_SUSPEND_RESUME
            result = if thread.is_null() {
                Err(io::Error::last_os_error())
            } else {
                let resumed = unsafe { ResumeThread(thread) };
                let result = if resumed == u32::MAX {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                };
                unsafe {
                    CloseHandle(thread);
                }
                result
            };
            break;
        }
        found = unsafe { Thread32Next(snapshot, &mut entry) } != 0;
    }
    unsafe {
        CloseHandle(snapshot);
    }
    result
}
impl Tree {
    pub fn kill(&self) {
        unsafe {
            TerminateJobObject(self.0, 130);
        }
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
}
#[cfg(test)]
#[path = "../tests/tree_windows.rs"]
mod tests;

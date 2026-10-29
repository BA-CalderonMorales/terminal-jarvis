//! Win32 process-tree ownership. Layouts follow the Windows SDK.
use std::ffi::c_void;
pub type Handle = *mut c_void;
#[repr(C)]
#[derive(Default)]
pub struct BasicLimits {
    pub process_time: i64,
    pub job_time: i64,
    pub flags: u32,
    pub min_working_set: usize,
    pub max_working_set: usize,
    pub active_processes: u32,
    pub affinity: usize,
    pub priority: u32,
    pub scheduling: u32,
}
#[repr(C)]
#[derive(Default)]
pub struct ExtendedLimits {
    pub basic: BasicLimits,
    pub io: [u64; 6],
    pub process_memory: usize,
    pub job_memory: usize,
    pub peak_process: usize,
    pub peak_job: usize,
}
#[repr(C)]
#[derive(Default)]
pub struct ThreadEntry {
    pub size: u32,
    pub usage: u32,
    pub thread_id: u32,
    pub process_id: u32,
    pub priority: i32,
    pub delta: i32,
    pub flags: u32,
}
#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn CreateJobObjectW(attributes: *const c_void, name: *const u16) -> Handle;
    pub fn SetInformationJobObject(job: Handle, class: i32, info: *const c_void, size: u32) -> i32;
    pub fn AssignProcessToJobObject(job: Handle, process: Handle) -> i32;
    pub fn TerminateJobObject(job: Handle, code: u32) -> i32;
    pub fn CloseHandle(handle: Handle) -> i32;
    pub fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> Handle;
    pub fn Thread32First(snapshot: Handle, entry: *mut ThreadEntry) -> i32;
    pub fn Thread32Next(snapshot: Handle, entry: *mut ThreadEntry) -> i32;
    #[cfg(test)]
    pub fn OpenProcess(access: u32, inherit: i32, pid: u32) -> Handle;
    #[cfg(test)]
    pub fn GetExitCodeProcess(process: Handle, code: *mut u32) -> i32;
    pub fn OpenThread(access: u32, inherit: i32, tid: u32) -> Handle;
    pub fn ResumeThread(thread: Handle) -> u32;
}

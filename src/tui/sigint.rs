//! Sigint: scoped SIGINT suppression around child-spawning actions. Children
//! share the tui's process group, so Ctrl+C aimed at a running agent would
//! also kill the shell. While a guarded action runs, the tui ignores SIGINT
//! in its own process, and the default disposition returns once the action
//! ends. Children are not left silenced: the spawn site (runtime runner)
//! installs a pre-exec reset of SIGINT to SIG_DFL, so an agent still dies on
//! Ctrl+C while the shell survives.
//! Std-only: the `signal` symbol is declared directly, no libc crate.

pub fn guarded<T>(body: impl FnOnce() -> T) -> T {
    #[cfg(unix)]
    {
        set_sigint(SIG_IGN);
    }
    let result = body();
    #[cfg(unix)]
    {
        set_sigint(SIG_DFL);
    }
    result
}

#[cfg(unix)]
extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

#[cfg(unix)]
const SIGINT: i32 = 2;
const SIG_IGN: usize = 1;
const SIG_DFL: usize = 0;

#[cfg(unix)]
fn set_sigint(handler: usize) {
    unsafe {
        signal(SIGINT, handler);
    }
}

//! Size: the terminal geometry, with a debug override for headless
//! capture environments whose ptys ignore TIOCSWINSZ.

/// (cols, rows) from the tty, or None when stdout is not a sized terminal.
pub fn size() -> Option<(usize, usize)> {
    // Debug hook: a fixed "cols x rows" for headless TUI frame capture in
    // environments whose ptys ignore TIOCSWINSZ. Real terminals are
    // unaffected -- the override only fires when the variable is set.
    if let Ok(explicit) = std::env::var("TJ_DEBUG_SIZE") {
        let mut parts = explicit.splitn(2, 'x');
        let cols = parts.next().and_then(|c| c.parse::<usize>().ok());
        let rows = parts.next().and_then(|r| r.parse::<usize>().ok());
        if let (Some(cols), Some(rows)) = (cols, rows) {
            return Some((cols, rows));
        }
    }
    geometry()
}

#[cfg(unix)]
fn geometry() -> Option<(usize, usize)> {
    #[repr(C)]
    struct Winsize {
        row: u16,
        col: u16,
        xpixel: u16,
        ypixel: u16,
    }
    unsafe extern "C" {
        fn ioctl(fd: i32, request: u64, ...) -> i32;
    }
    #[cfg(target_os = "linux")]
    const TIOCGWINSZ: u64 = 0x5413;
    #[cfg(not(target_os = "linux"))]
    const TIOCGWINSZ: u64 = 0x4008_7468;
    let mut size = Winsize {
        row: 0,
        col: 0,
        xpixel: 0,
        ypixel: 0,
    };
    let ok = unsafe { ioctl(1, TIOCGWINSZ, &mut size) } == 0;
    (ok && size.col > 0 && size.row > 0).then_some((size.col as usize, size.row as usize))
}

#[cfg(not(unix))]
fn geometry() -> Option<(usize, usize)> {
    None
}

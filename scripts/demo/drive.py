#!/usr/bin/env python3
"""Drive the tui on a pty, so a screen recording never depends on synthetic key
delivery or on which window holds focus: this process owns the child's input,
and the child's output is copied straight to the terminal foot gave us.

The sequence mirrors scripts/demo/tui.tape. The recorder places the window and
then touches TJ_DEMO_GO; only *then* is the tui started, because a tui that
boots while the window is still tiled measures the tiled geometry and composes
frames the placed window then clips. It touches TJ_DEMO_DONE once the last step
has settled, so the recorder can stop at a known point.

`--timing` prints the offsets the recorder needs to trim the capture.

usage: drive.py <repo> <tui-binary> | drive.py --timing
"""
import fcntl
import os
import pty
import select
import signal
import struct
import sys
import tempfile
import termios
import time

# (settle before the step, keystrokes). These mirror scripts/demo/tui.tape's
# Sleeps exactly, so the two candidates read at the same pace; the keystrokes
# are typed a little slower than a paste so the command line is legible.
STEPS = [
    (1.5, None),  # 1. boot: the first-boot primer
    (0.0, "list\n"),  # 2. the fleet picker
    (2.5, "status\n"),  # 3. readiness dashboard
    (2.5, "show codex\n"),  # 4. harness details
    (2.5, "codex\n"),  # 5. the switch: the header leaves "ACTIVE none"
    (2.5, "plan codex headless\n"),  # 6. a preview, not a run
    (2.5, "run codex -- --version\n"),  # 7. the guard refuses, by design
    (3.0, "/theme solarized\n"),  # 8. a named theme
    (2.5, "help\n"),  # 9. the command surface
    (2.5, "\x1b"),  # 10. Esc for NORMAL
    (1.0, "j"),  #      j/k scroll the help table
    (0.6, "j"),
    (0.6, "k"),
    (0.6, "i"),  #      i returns to Insert
    (0.8, "home\n"),  # 11. leave the frame settled on the primer
    (3.0, None),
]
PER_CHAR = 0.05
BOOT = 2.5
CHILD_PID = None


def terminal_size():
    size = os.get_terminal_size(1)
    return (size.columns, size.lines)


def apply_size(fd, size):
    fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", size[1], size[0], 0, 0))


def tty_raw(fd):
    """foot's pty must not translate or echo: the tui emits \r\n itself."""
    try:
        attrs = termios.tcgetattr(fd)
    except termios.error:
        return
    attrs[0] &= ~(
        termios.BRKINT | termios.ICRNL | termios.INPCK | termios.ISTRIP | termios.IXON
    )
    attrs[1] &= ~termios.OPOST
    attrs[3] &= ~(termios.ECHO | termios.ICANON | termios.IEXTEN | termios.ISIG)
    termios.tcsetattr(fd, termios.TCSANOW, attrs)


def settle(want=2, pause=0.2, limit=6.0):
    """The recorder floats and resizes the window after launch; wait until our
    own terminal size holds still so the child is born at the final geometry."""
    last = None
    same = 0
    waited = 0.0
    while waited < limit and same < want:
        current = terminal_size()
        same = same + 1 if current == last else 0
        last = current
        time.sleep(pause)
        waited += pause
    return last or terminal_size()


def schedule():
    """Absolute (time, bytes) events, and how long the whole flow runs."""
    events = []
    t = 0.0
    for pause, keys in STEPS:
        t += pause
        for char in keys or "":
            events.append((t, char.encode()))
            t += PER_CHAR
    return events, t + STEPS[-1][0]


def wait_for(path, timeout):
    deadline = time.monotonic() + timeout
    while not os.path.exists(path):
        if time.monotonic() > deadline:
            return False
        time.sleep(0.05)
    return True


def pump(fd, events, done_path):
    events = list(events)
    start = time.monotonic()
    while True:
        now = time.monotonic() - start
        while events and events[0][0] <= now:
            _, data = events.pop(0)
            os.write(fd, data)
        if not events and not os.path.exists(done_path):
            open(done_path, "w").close()
        ready, _, _ = select.select([fd], [], [], 0.01)
        if not ready:
            continue
        try:
            data = os.read(fd, 65536)
        except OSError:
            return
        if not data:
            return
        os.write(1, data)


def main():
    global CHILD_PID
    if len(sys.argv) > 1 and sys.argv[1] == "--timing":
        _, flow = schedule()
        print(f"BOOT {BOOT + 0.5:.2f}")
        print(f"FLOW {flow:.2f}")
        return 0
    repo, binary = sys.argv[1], sys.argv[2]
    go = os.environ.get("TJ_DEMO_GO", "/tmp/tj-demo-go")
    done = os.environ.get("TJ_DEMO_DONE", "/tmp/tj-demo-done")
    tty_raw(1)
    if not wait_for(go, 180):
        os.write(2, b"drive: the recorder never signalled go\n")
        return 1
    size = settle()
    pid, fd = pty.fork()
    if pid == 0:
        apply_size(0, size)
        os.chdir(repo)
        os.environ["TERMINAL_JARVIS_HOME"] = tempfile.mkdtemp(prefix="tj-demo-")
        os.execv(binary, [binary, "tui"])
    CHILD_PID = pid
    time.sleep(BOOT)
    events, _ = schedule()
    pump(fd, events, done)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

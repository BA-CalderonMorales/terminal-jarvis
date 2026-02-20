package repl

import "os"

// attachControllingTTY rebinds stdio to /dev/tty when available.
// This prevents immediate EOF in wrapped environments where stdin is not
// connected to the interactive terminal.
func attachControllingTTY() (restore func()) {
	if os.Getenv("JARVIS_NO_TTY_ATTACH") == "1" {
		return nil
	}

	tty, err := os.OpenFile("/dev/tty", os.O_RDWR, 0)
	if err != nil {
		return nil
	}

	oldIn := os.Stdin
	oldOut := os.Stdout
	oldErr := os.Stderr

	os.Stdin = tty
	os.Stdout = tty
	os.Stderr = tty

	return func() {
		os.Stdin = oldIn
		os.Stdout = oldOut
		os.Stderr = oldErr
		_ = tty.Close()
	}
}

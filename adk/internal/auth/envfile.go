package auth

import (
	"os"
	"os/exec"
	"strings"
)

// writeEnvKeyToFile writes or replaces KEY=value in the .env file.
// Creates the file if it does not exist.
func writeEnvKeyToFile(envPath, key, value string) {
	raw, _ := os.ReadFile(envPath)
	lines := strings.Split(string(raw), "\n")

	found := false
	for i, line := range lines {
		stripped := strings.TrimLeft(line, "# \t")
		if strings.HasPrefix(stripped, key+"=") {
			lines[i] = key + "=" + value
			found = true
			break
		}
	}
	if !found {
		lines = append(lines, key+"="+value)
	}

	// Remove trailing blank lines, then re-join with a final newline.
	for len(lines) > 0 && strings.TrimSpace(lines[len(lines)-1]) == "" {
		lines = lines[:len(lines)-1]
	}

	_ = os.WriteFile(envPath, []byte(strings.Join(lines, "\n")+"\n"), 0600)
}

// clearEnvKeysFromFile removes KEY=value entries from the .env file.
// Missing files are ignored.
func clearEnvKeysFromFile(envPath string, keys ...string) {
	if len(keys) == 0 {
		return
	}

	raw, err := os.ReadFile(envPath)
	if err != nil {
		return
	}

	toRemove := make(map[string]struct{}, len(keys))
	for _, k := range keys {
		toRemove[k] = struct{}{}
	}

	lines := strings.Split(string(raw), "\n")
	kept := make([]string, 0, len(lines))

	for _, line := range lines {
		stripped := strings.TrimLeft(line, "# \t")
		shouldRemove := false
		for key := range toRemove {
			if strings.HasPrefix(stripped, key+"=") {
				shouldRemove = true
				break
			}
		}
		if !shouldRemove {
			kept = append(kept, line)
		}
	}

	for len(kept) > 0 && strings.TrimSpace(kept[len(kept)-1]) == "" {
		kept = kept[:len(kept)-1]
	}

	_ = os.WriteFile(envPath, []byte(strings.Join(kept, "\n")+"\n"), 0600)
}

// openBrowser tries to open url in the default browser.
// Returns false in headless environments.
func openBrowser(rawURL string) bool {
	cmds := [][]string{
		{"xdg-open", rawURL},
		{"open", rawURL},
		{"cmd", "/c", "start", rawURL},
	}
	for _, c := range cmds {
		if path, err := exec.LookPath(c[0]); err == nil {
			cmd := exec.Command(path, c[1:]...)
			if err := cmd.Start(); err == nil {
				return true
			}
		}
	}
	return false
}

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

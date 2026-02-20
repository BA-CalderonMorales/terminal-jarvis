package auth

import (
	"bufio"
	"io"
	"strings"
)

type TextPrompt func(string) (string, error)

// readLine reads one logical line from stdin and accepts both '\n' and '\r'
// as submit keys so prompts work in raw and cooked terminal modes.
func readLine(reader *bufio.Reader) (string, error) {
	var b strings.Builder
	for {
		r, _, err := reader.ReadRune()
		if err != nil {
			if err == io.EOF {
				return strings.TrimSpace(b.String()), nil
			}
			return "", err
		}
		if r == '\n' || r == '\r' {
			return strings.TrimSpace(b.String()), nil
		}
		b.WriteRune(r)
	}
}

func readInput(reader *bufio.Reader, promptFn TextPrompt, prompt string) (string, error) {
	if promptFn != nil {
		s, err := promptFn(prompt)
		if err != nil {
			return "", err
		}
		return strings.TrimSpace(s), nil
	}
	return readLine(reader)
}

package repl

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/tools"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
)

type toolAlias struct {
	alias string
	tool  string
}

var nonAlphaNum = regexp.MustCompile(`[^a-z0-9]+`)

var launchAliases = []toolAlias{
	{alias: "claude code", tool: "claude"},
	{alias: "claude", tool: "claude"},
	{alias: "gemini", tool: "gemini"},
	{alias: "codex", tool: "codex"},
	{alias: "aider", tool: "aider"},
	{alias: "goose", tool: "goose"},
	{alias: "amp", tool: "amp"},
	{alias: "open code", tool: "opencode"},
	{alias: "opencode", tool: "opencode"},
	{alias: "llxprt", tool: "llxprt"},
	{alias: "qwen code", tool: "qwen"},
	{alias: "qwen", tool: "qwen"},
	{alias: "cursor agent", tool: "cursor-agent"},
	{alias: "copilot cli", tool: "copilot"},
	{alias: "copilot", tool: "copilot"},
	{alias: "crush", tool: "crush"},
	{alias: "ollama", tool: "ollama"},
	{alias: "vibe", tool: "vibe"},
	{alias: "forge", tool: "forge"},
	{alias: "droid", tool: "droid"},
	{alias: "kilocode", tool: "kilocode"},
	{alias: "nanocoder", tool: "nanocoder"},
	{alias: "letta", tool: "letta"},
	{alias: "eca", tool: "eca"},
	{alias: "jules", tool: "jules"},
	{alias: "pi", tool: "pi"},
}

func maybeHandleDirectLaunchIntent(input string) bool {
	norm := normalizeIntent(input)
	if norm == "" {
		return false
	}
	if !hasLaunchVerb(norm) {
		return false
	}

	toolName, ok := findToolAlias(norm)
	if !ok {
		return false
	}

	fmt.Printf("\n   %sLaunching %s...%s\n\n", ui.Cyan, toolName, ui.Reset)
	result := tools.Launch(toolName)
	fmt.Printf("   %s%s%s\n\n", ui.LightB, result, ui.Reset)
	return true
}

func normalizeIntent(s string) string {
	s = strings.ToLower(strings.TrimSpace(s))
	s = nonAlphaNum.ReplaceAllString(s, " ")
	return strings.Join(strings.Fields(s), " ")
}

func hasLaunchVerb(norm string) bool {
	verbs := []string{
		"launch", "run", "open", "start", "execute", "boot", "fire up",
	}
	padded := " " + norm + " "
	for _, v := range verbs {
		if strings.Contains(padded, " "+v+" ") {
			return true
		}
	}
	return false
}

func findToolAlias(norm string) (string, bool) {
	padded := " " + norm + " "
	for _, a := range launchAliases {
		if strings.Contains(padded, " "+a.alias+" ") {
			return a.tool, true
		}
	}
	return "", false
}

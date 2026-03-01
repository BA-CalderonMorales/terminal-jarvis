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
var launchVerbs = []string{"launch", "run", "open", "start", "execute", "boot", "fire up"}

var launchableTools = []string{
	"claude", "gemini", "qwen", "opencode", "codex", "aider", "amp", "copilot", "goose",
	"crush", "llxprt", "ollama", "vibe", "droid", "forge", "cursor-agent", "jules",
	"kilocode", "letta", "nanocoder", "pi", "code", "eca",
}

var inDevelopmentAliases = map[string]string{
	"kimi": "Kimi integration is currently in development and is not launchable yet.",
}

var launchAliases = []toolAlias{
	{alias: "claude code", tool: "claude"},
	{alias: "claude", tool: "claude"},
	{alias: "gemini", tool: "gemini"},
	{alias: "codex", tool: "codex"},
	{alias: "code", tool: "code"},
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
	{alias: "kilo code", tool: "kilocode"},
	{alias: "kilo", tool: "kilocode"},
	{alias: "kilocode", tool: "kilocode"},
	{alias: "nanocoder", tool: "nanocoder"},
	{alias: "letta", tool: "letta"},
	{alias: "editor code assistant", tool: "eca"},
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
	if ok {
		fmt.Printf("\n   %sLaunching %s...%s\n\n", ui.Cyan, toolName, ui.Reset)
		result := tools.Launch(toolName)
		fmt.Printf("   %s%s%s\n\n", ui.LightB, result, ui.Reset)
		return true
	}

	target := extractLaunchTarget(norm)
	if target == "" {
		fmt.Printf("\n   %sPlease specify a tool to launch.%s\n", ui.Cyan, ui.Reset)
		fmt.Printf("   Available tools: %s\n\n", strings.Join(launchableTools, ", "))
		return true
	}

	for alias, devMsg := range inDevelopmentAliases {
		if target == alias || strings.HasPrefix(target, alias+" ") {
			fmt.Printf("\n   %s%s%s\n", ui.Cyan, devMsg, ui.Reset)
			fmt.Printf("   Available tools: %s\n\n", strings.Join(launchableTools, ", "))
			return true
		}
	}

	if suggestion, ok := suggestTool(target); ok {
		fmt.Printf("\n   %sI don't have a tool called \"%s\" available.%s\n", ui.Cyan, target, ui.Reset)
		fmt.Printf("   Did you mean %s%s%s?\n", ui.LightB, suggestion, ui.Reset)
		fmt.Printf("   Available tools: %s\n\n", strings.Join(launchableTools, ", "))
		return true
	}

	fmt.Printf("\n   %sI don't have a tool called \"%s\" available.%s\n", ui.Cyan, target, ui.Reset)
	fmt.Printf("   Available tools: %s\n\n", strings.Join(launchableTools, ", "))
	return true
}

func normalizeIntent(s string) string {
	s = strings.ToLower(strings.TrimSpace(s))
	s = nonAlphaNum.ReplaceAllString(s, " ")
	return strings.Join(strings.Fields(s), " ")
}

func hasLaunchVerb(norm string) bool {
	padded := " " + norm + " "
	for _, v := range launchVerbs {
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

func extractLaunchTarget(norm string) string {
	padded := " " + norm + " "
	for _, verb := range launchVerbs {
		token := " " + verb + " "
		if idx := strings.Index(padded, token); idx >= 0 {
			target := strings.TrimSpace(padded[idx+len(token):])
			if target != "" {
				return target
			}
		}
	}
	return ""
}

func suggestTool(target string) (string, bool) {
	for _, a := range launchAliases {
		if strings.Contains(target, a.alias) || strings.Contains(a.alias, target) {
			return a.tool, true
		}
	}
	return "", false
}

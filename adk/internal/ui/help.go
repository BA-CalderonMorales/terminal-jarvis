package ui

import (
	"fmt"
	"time"
)

// PrintHelp prints the slash-command reference.
func PrintHelp() {
	fmt.Println()
	fmt.Printf("   %sCommands:%s\n", Cyan, Reset)
	fmt.Printf("   %s/tools%s               list all AI coding tools\n", Cyan, Reset)
	fmt.Printf("   %s/install <tool>%s      install a tool\n", Cyan, Reset)
	fmt.Printf("   %s/status%s              tool health dashboard\n", Cyan, Reset)
	fmt.Printf("   %s/auth [tool]%s         authentication help\n", Cyan, Reset)
	fmt.Printf("   %s/setup%s               interactive provider setup wizard\n", Cyan, Reset)
	fmt.Printf("   %s/logout [provider]%s   clear provider credentials (gemini/openrouter/ollama/all)\n", Cyan, Reset)
	fmt.Printf("   %s/config%s              show current config\n", Cyan, Reset)
	fmt.Printf("   %s/update [tool]%s       update one or all tools\n", Cyan, Reset)
	fmt.Printf("   %s/help%s                show this help\n", Cyan, Reset)
	fmt.Printf("   %s/exit%s                exit\n", Cyan, Reset)
	fmt.Println()
	fmt.Printf("   %sArrow keys for history  |  plain English also works%s\n", LightB, Reset)
	fmt.Println()
	fmt.Printf("   %sExamples:%s\n", BoldW, Reset)
	fmt.Printf("   %swhich tools are installed?%s\n", LightB, Reset)
	fmt.Printf("   %slaunch claude%s\n", LightB, Reset)
	fmt.Printf("   %show do I set up auth for gemini?%s\n", LightB, Reset)
	fmt.Println()
}

// PrintResponse prints the LLM reply with optional thinking section and response time.
func PrintResponse(text string, responseTimeMs int64) {
	thinking, response := separateThinking(text)

	if thinking != "" {
		fmt.Printf("   %s**thinking**%s\n", ThinkG, Reset)
		fmt.Printf("   %s%s%s\n\n", ThinkG, thinking, Reset)
	}
	if response != "" {
		// Display response time in header like agent-harness: "14:32:05 (6.2s)"
		responseTime := time.Duration(responseTimeMs) * time.Millisecond
		timestamp := time.Now().Format("15:04:05")
		fmt.Printf("   %s%s%s %s(%.1fs)%s\n", Dim, timestamp, Reset, Dim, responseTime.Seconds(), Reset)
		fmt.Printf("   %s%s%s\n\n", LightB, response, Reset)
	}
}

// separateThinking does a lightweight split of internal model monologue from
// the actual response. Returns ("", fullText) when no reasoning prefix is found.
func separateThinking(text string) (thinking, response string) {
	// Simple heuristic: if text starts with common thinking phrases, split at
	// first blank line. For now keep it simple — the Go binary doesn't use ADK
	// multi-step reasoning; LLM replies are typically direct.
	return "", text
}

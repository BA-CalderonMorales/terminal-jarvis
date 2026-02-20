package auth

import (
	"fmt"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
)

// SetupOllama prints Ollama setup instructions (no key to collect).
func SetupOllama() {
	fmt.Println()
	fmt.Printf("   %s► %sOllama Setup%s\n", ui.Cyan, ui.BoldW, ui.Reset)
	fmt.Printf("   %sRuns entirely on your machine -- no API key required.%s\n", ui.Dim, ui.Reset)
	fmt.Println()
	fmt.Printf("   %s1. Install Ollama:%s\n", ui.LightB, ui.Reset)
	fmt.Printf("   %s   https://ollama.com/download%s\n", ui.Cyan, ui.Reset)
	fmt.Println()
	fmt.Printf("   %s2. Pull a model (llama3.2 is a good starting point):%s\n", ui.LightB, ui.Reset)
	fmt.Printf("   %s   ollama pull llama3.2%s\n", ui.LightB, ui.Reset)
	fmt.Println()
	fmt.Printf("   %s3. Start Ollama and continue here -- no restart required.%s\n", ui.LightB, ui.Reset)
	fmt.Println()
	fmt.Printf("   %s(No changes to .env required)%s\n\n", ui.Dim, ui.Reset)
}

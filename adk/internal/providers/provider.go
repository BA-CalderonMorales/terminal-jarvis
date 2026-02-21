// Package providers implements LLM provider detection and API clients.
//
// Priority chain (mirrors Python providers.py):
//
//	JARVIS_MODEL env var → GOOGLE_API_KEY → OPENROUTER_API_KEY → Ollama
//
// Each provider implements the Provider interface. The chat engine in
// internal/chat uses the chain and falls back on timeout or auth failure.
package providers

import (
	"context"
	"encoding/json"
)

// Message represents a single turn in the conversation history.
type Message struct {
	Role       string // "user", "assistant", "tool"
	Content    string
	ToolCallID string     // non-empty when Role == "tool"
	ToolName   string     // non-empty when Role == "tool"
	ToolCalls  []ToolCall // non-empty when Role == "assistant" and responding with tool calls
}

// ToolCall is a request from the LLM to invoke a tool.
type ToolCall struct {
	ID   string
	Name string
	Args map[string]json.RawMessage
}

// Response is the LLM's reply to a chat turn.
type Response struct {
	Text     string    // final text reply
	ToolCall *ToolCall // non-nil when the model wants to call a tool
}

// ToolDef describes a callable tool for function-calling capable models.
type ToolDef struct {
	Name        string
	Description string
	// Parameters is an OpenAI-compatible JSON Schema object.
	Parameters map[string]interface{}
}

// Provider is the interface every LLM backend must satisfy.
type Provider interface {
	// Chat sends the conversation history and returns the next response.
	// tools may be nil for models that don't support function calling.
	Chat(ctx context.Context, messages []Message, tools []ToolDef) (Response, error)
	// Label returns a human-readable name shown in the home screen.
	Label() string
}

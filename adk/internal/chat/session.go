// Package chat manages conversation history and LLM dispatch.
package chat

import (
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
)

// Session holds the in-memory conversation history for a single user session.
type Session struct {
	Messages []providers.Message
}

// NewSession creates an empty Session with an optional system prompt.
func NewSession(systemPrompt string) *Session {
	s := &Session{}
	if systemPrompt != "" {
		s.Messages = append(s.Messages, providers.Message{
			Role:    "user",
			Content: systemPrompt,
		})
		s.Messages = append(s.Messages, providers.Message{
			Role:    "assistant",
			Content: "Understood. I am Terminal Jarvis, your AI coding tools assistant.",
		})
	}
	return s
}

// AddUser appends a user message.
func (s *Session) AddUser(content string) {
	s.Messages = append(s.Messages, providers.Message{Role: "user", Content: content})
}

// AddAssistant appends an assistant message.
func (s *Session) AddAssistant(content string) {
	s.Messages = append(s.Messages, providers.Message{Role: "assistant", Content: content})
}

// AddToolResult appends a tool result message.
func (s *Session) AddToolResult(toolCallID, toolName, result string) {
	s.Messages = append(s.Messages, providers.Message{
		Role:       "tool",
		Content:    result,
		ToolCallID: toolCallID,
		ToolName:   toolName,
	})
}

// SystemPrompt is the persona injected at session start.
const SystemPrompt = `You are Terminal Jarvis, an AI assistant that helps users manage AI coding tools.

Use the provided tools when the user asks you to do something. Keep replies concise.
Do NOT narrate what you are about to do -- just call the tool and report the result.`

package chat

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/tools"
)

// maxToolLoops caps the number of tool-call rounds to prevent infinite loops.
const maxToolLoops = 5

// Send adds the user message to the session, calls the provider, handles any
// tool calls in a loop, then returns the final text reply and response time.
func Send(ctx context.Context, session *Session, provider providers.Provider, userText string) (string, int64, error) {
	session.AddUser(userText)

	toolSpecs := tools.SpecList()
	var totalResponseTime int64

	for i := 0; i < maxToolLoops; i++ {
		start := time.Now()
		resp, err := provider.Chat(ctx, session.Messages, toolSpecs)
		elapsed := time.Since(start).Milliseconds()
		totalResponseTime += elapsed

		if err != nil {
			return "", totalResponseTime, err
		}

		// Plain text reply -- done.
		if resp.ToolCall == nil {
			session.AddAssistantWithTiming(resp.Text, totalResponseTime)
			return resp.Text, totalResponseTime, nil
		}

		// Tool call: execute and add result to history.
		tc := resp.ToolCall
		strArgs := decodeArgs(tc.Args)
		result := tools.Dispatch(tc.Name, strArgs)

		// Record the assistant's tool-call message with structured metadata.
		session.AddAssistantToolCall(*tc)
		session.AddToolResult(tc.ID, tc.Name, result)
	}

	return "", totalResponseTime, fmt.Errorf("tool call loop exceeded %d iterations", maxToolLoops)
}

// decodeArgs converts json.RawMessage values to plain strings for tool dispatch.
func decodeArgs(raw map[string]json.RawMessage) map[string]string {
	out := make(map[string]string, len(raw))
	for k, v := range raw {
		var s string
		if err := json.Unmarshal(v, &s); err == nil {
			out[k] = s
		} else {
			out[k] = string(v)
		}
	}
	return out
}

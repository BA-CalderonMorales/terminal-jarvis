package tools

import "github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"

// Executor is a function that the LLM can call by name.
type Executor func(args map[string]string) string

// Definition pairs the LLM tool spec with its Go implementation.
type Definition struct {
	Spec    providers.ToolDef
	Execute Executor
}

// strArg extracts a string argument from a decoded args map, returning "" if absent.
func strArg(args map[string]string, key string) string {
	return args[key]
}

// All is the complete list of tools exposed to the LLM agent.
var All = []Definition{
	{
		Spec: providers.ToolDef{
			Name:        "list_tools",
			Description: "List all available AI coding tools and their installation status.",
			Parameters:  map[string]interface{}{"type": "object", "properties": map[string]interface{}{}},
		},
		Execute: func(_ map[string]string) string { return Run("list") },
	},
	{
		Spec: providers.ToolDef{
			Name:        "get_tool_info",
			Description: "Get detailed information about a specific AI coding tool.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"tool_name": map[string]interface{}{
						"type":        "string",
						"description": "Name of the tool (e.g. claude, gemini, aider, goose).",
					},
				},
				"required": []string{"tool_name"},
			},
		},
		Execute: func(args map[string]string) string { return Run("info", strArg(args, "tool_name")) },
	},
	{
		Spec: providers.ToolDef{
			Name:        "launch_tool",
			Description: "Launch an AI coding tool interactively. Control returns when the user exits the tool.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"tool_name": map[string]interface{}{
						"type":        "string",
						"description": "Name of the tool to launch (e.g. claude, gemini, aider).",
					},
				},
				"required": []string{"tool_name"},
			},
		},
		Execute: func(args map[string]string) string { return Launch(strArg(args, "tool_name")) },
	},
	{
		Spec: providers.ToolDef{
			Name:        "install_tool",
			Description: "Install an AI coding tool.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"tool_name": map[string]interface{}{
						"type":        "string",
						"description": "Name of the tool to install (e.g. aider, goose, llxprt).",
					},
				},
				"required": []string{"tool_name"},
			},
		},
		Execute: func(args map[string]string) string { return Run("install", strArg(args, "tool_name")) },
	},
	{
		Spec: providers.ToolDef{
			Name:        "update_tool",
			Description: "Update one or all AI coding tools. Leave tool_name empty to update all.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"tool_name": map[string]interface{}{
						"type":        "string",
						"description": "Name of the tool to update. Omit to update all tools.",
					},
				},
			},
		},
		Execute: func(args map[string]string) string {
			if name := strArg(args, "tool_name"); name != "" {
				return Run("update", name)
			}
			return Run("update")
		},
	},
	{
		Spec: providers.ToolDef{
			Name:        "show_status",
			Description: "Show the health dashboard for all AI coding tools.",
			Parameters:  map[string]interface{}{"type": "object", "properties": map[string]interface{}{}},
		},
		Execute: func(_ map[string]string) string { return Run("status") },
	},
	{
		Spec: providers.ToolDef{
			Name:        "get_auth_help",
			Description: "Show authentication setup instructions for a specific AI coding tool.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"tool_name": map[string]interface{}{
						"type":        "string",
						"description": "Name of the tool to get auth help for (e.g. claude, gemini).",
					},
				},
				"required": []string{"tool_name"},
			},
		},
		Execute: func(args map[string]string) string { return Run("auth", "help", strArg(args, "tool_name")) },
	},
	{
		Spec: providers.ToolDef{
			Name:        "show_config",
			Description: "Show the current Terminal Jarvis configuration.",
			Parameters:  map[string]interface{}{"type": "object", "properties": map[string]interface{}{}},
		},
		Execute: func(_ map[string]string) string { return Run("config", "show") },
	},
	{
		Spec: providers.ToolDef{
			Name:        "clear_cache",
			Description: "Clear the version cache to force fresh tool detection.",
			Parameters:  map[string]interface{}{"type": "object", "properties": map[string]interface{}{}},
		},
		Execute: func(_ map[string]string) string { return Run("cache", "clear") },
	},
}

// SpecList returns just the ToolDef specs for registering with the LLM.
func SpecList() []providers.ToolDef {
	specs := make([]providers.ToolDef, len(All))
	for i, d := range All {
		specs[i] = d.Spec
	}
	return specs
}

// Dispatch looks up and executes a tool by name.
// Returns an error string if the tool is not found.
func Dispatch(name string, args map[string]string) string {
	for _, d := range All {
		if d.Spec.Name == name {
			return d.Execute(args)
		}
	}
	return "unknown tool: " + name
}

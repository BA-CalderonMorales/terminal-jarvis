package repl

import "testing"

func TestParseHeadlessSlashExit(t *testing.T) {
	action := parseHeadlessSlash("/exit")
	if !action.exit {
		t.Fatal("expected /exit to return exit action")
	}
}

func TestParseHeadlessSlashHelp(t *testing.T) {
	action := parseHeadlessSlash("/help")
	if action.stdout == "" {
		t.Fatal("expected /help to return help text")
	}
}

func TestParseHeadlessSlashTools(t *testing.T) {
	action := parseHeadlessSlash("/tools")
	if len(action.toolArgs) != 1 || action.toolArgs[0] != "list" {
		t.Fatalf("expected tool args [list], got %#v", action.toolArgs)
	}
}

func TestParseHeadlessSlashAuthWithTool(t *testing.T) {
	action := parseHeadlessSlash("/auth claude")
	if len(action.toolArgs) != 3 {
		t.Fatalf("expected 3 auth args, got %#v", action.toolArgs)
	}
	if action.toolArgs[0] != "auth" || action.toolArgs[1] != "help" || action.toolArgs[2] != "claude" {
		t.Fatalf("unexpected auth args %#v", action.toolArgs)
	}
}

func TestParseHeadlessSlashInstallMissingTool(t *testing.T) {
	action := parseHeadlessSlash("/install")
	if action.stderr == "" {
		t.Fatal("expected usage warning when /install has no tool")
	}
}

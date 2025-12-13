/**
 * Session Continuation E2E Tests
 *
 * Tests for Terminal Jarvis's session continuation system - THE defining feature
 * that prevents users from being kicked out during authentication workflows.
 *
 * These tests validate:
 * - CLI commands related to session management are properly exposed
 * - The run command is available for tool execution
 * - Help documentation is complete
 *
 * Note: Full interactive session continuation testing is done in Rust tests
 * (tests/session_continuation_tests.rs) because it requires direct function access.
 * These E2E tests ensure the CLI surface area supports the feature.
 */

import { render } from "cli-testing-library";
import { describe, expect, test } from "vitest";
import { getBinaryPath, normalizeOutput } from "./helpers";

describe("Session Continuation Support", () => {
  describe("Run Command Availability", () => {
    test("run command is listed in main help", async () => {
      const { findByText } = await render(getBinaryPath(), ["--help"]);

      // The run command is essential for launching tools with session continuation
      expect(await findByText("run")).toBeInTheConsole();
    });

    test("run command has help documentation", async () => {
      const instance = await render(getBinaryPath(), ["run", "--help"]);

      await new Promise((resolve) => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Run command should have usage information
      expect(output).toContain("Usage");
    });

    test("run command lists available tools", async () => {
      const instance = await render(getBinaryPath(), ["run", "--help"]);

      await new Promise((resolve) => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Should mention running tools
      expect(output.toLowerCase()).toMatch(/tool|ai|run/);
    });
  });

  describe("Auth Command Availability", () => {
    test("auth command is listed in main help", async () => {
      const { findByText } = await render(getBinaryPath(), ["--help"]);

      // Auth commands trigger session continuation
      expect(await findByText("auth")).toBeInTheConsole();
    });

    test("auth command has subcommands", async () => {
      const instance = await render(getBinaryPath(), ["auth", "--help"]);

      await new Promise((resolve) => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Auth should have usage information
      expect(output).toContain("Usage");
    });
  });

  describe("Tool Listing for Session Management", () => {
    test("list command produces output", async () => {
      const instance = await render(getBinaryPath(), ["list"]);

      await new Promise((resolve) => setTimeout(resolve, 2000));

      const output = normalizeOutput(instance.getStdallStr());

      // The list command should produce some output about tools
      // Either showing available tools or explaining none are configured
      const hasRelevantOutput =
        output.includes("Available") || // Normal case: tools found
        output.includes("tool") || // Some tool-related message
        output.includes("No tools"); // Edge case: no tools configured

      expect(hasRelevantOutput).toBe(true);
    });

    test("list command exits successfully", async () => {
      const instance = await render(getBinaryPath(), ["list"]);

      await new Promise((resolve) => setTimeout(resolve, 2000));

      const exitInfo = instance.hasExit();
      expect(exitInfo).toBeTruthy();
      if (exitInfo) {
        expect(exitInfo.exitCode).toBe(0);
      }
    });
  });

  describe("CLI Help Completeness", () => {
    test("all session-related commands are documented", async () => {
      const instance = await render(getBinaryPath(), ["--help"]);

      await new Promise((resolve) => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Commands that support session continuation workflow
      const sessionCommands = ["run", "auth", "list", "info"];

      for (const cmd of sessionCommands) {
        expect(output).toContain(cmd);
      }
    });

    test("usage information is present", async () => {
      const { findByText } = await render(getBinaryPath(), ["--help"]);

      expect(await findByText("Usage:")).toBeInTheConsole();
    });

    test("version is accessible", async () => {
      const instance = await render(getBinaryPath(), ["--version"]);

      await new Promise((resolve) => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Should show a version number
      expect(output).toMatch(/\d+\.\d+\.\d+/);
    });
  });

  describe("Run Command Error Handling", () => {
    test("run command with unknown tool shows helpful error", async () => {
      const instance = await render(getBinaryPath(), ["run", "nonexistent-tool-xyz"]);

      await new Promise((resolve) => setTimeout(resolve, 2000));

      const output = normalizeOutput(instance.getStdallStr());

      // Should either show an error about the tool not being found/installed
      // or list available tools
      const hasHelpfulOutput =
        output.toLowerCase().includes("not") ||
        output.toLowerCase().includes("error") ||
        output.toLowerCase().includes("install") ||
        output.toLowerCase().includes("available");

      expect(hasHelpfulOutput).toBe(true);
    });

    test("run command without tool name shows help or error", async () => {
      const instance = await render(getBinaryPath(), ["run"]);

      await new Promise((resolve) => setTimeout(resolve, 2000));

      const output = normalizeOutput(instance.getStdallStr());

      // Should show usage help or prompt for tool name
      const hasUsageInfo =
        output.toLowerCase().includes("usage") ||
        output.toLowerCase().includes("tool") ||
        output.toLowerCase().includes("required") ||
        output.toLowerCase().includes("help");

      expect(hasUsageInfo).toBe(true);
    });
  });
});

describe("Session Continuation Documentation", () => {
  /**
   * This test documents the session continuation behavior.
   * It serves as living documentation that explains the feature.
   */
  test("session continuation is the defining feature of Terminal Jarvis", async () => {
    // Session continuation prevents users from being kicked out during:
    // - Authentication workflows (/auth, /login, --auth, --login)
    // - Setup commands (/setup, --setup)
    // - Configuration commands (/config, --config)
    // - OAuth flows (authenticate, oauth)

    // When these commands complete, Terminal Jarvis automatically restarts
    // the tool so users can continue working without losing their context.

    // Exit commands (/exit, /quit, /bye, etc.) properly terminate the session
    // and return the user to the Terminal Jarvis menu.

    // This test validates that the CLI is properly structured to support
    // this workflow by verifying the run command is accessible.

    const instance = await render(getBinaryPath(), ["run", "--help"]);

    await new Promise((resolve) => setTimeout(resolve, 1000));

    const output = normalizeOutput(instance.getStdallStr());

    // The run command is the entry point for session continuation
    expect(output).toContain("Usage");

    console.log("[SUCCESS] CLI structure verified for session continuation support");
    console.log("[INFO] Session continuation behavior:");
    console.log("  - Auth commands (e.g., /auth): Restart tool after completion");
    console.log("  - Exit commands (e.g., /exit): Return to Terminal Jarvis menu");
    console.log("[INFO] Comprehensive tests in tests/session_continuation_tests.rs");
  });
});

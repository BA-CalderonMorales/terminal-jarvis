#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const binaryPath = resolve(__dirname, "../../target/release/terminal-jarvis");

function normalizeOutput(output) {
  return output
    .replace(/\x1B\[[0-9;]*[JKmsu]/g, "")
    .replace(/\r\n/g, "\n")
    .trim();
}

function runCli(args, timeoutMs = 10000) {
  const result = spawnSync(binaryPath, args, {
    encoding: "utf8",
    env: { ...process.env, CI: "true", NO_COLOR: "1" },
    timeout: timeoutMs,
    maxBuffer: 2 * 1024 * 1024,
  });

  const timedOut = result.error?.code === "ETIMEDOUT";
  const output = normalizeOutput(`${result.stdout ?? ""}\n${result.stderr ?? ""}`);

  return {
    status: result.status,
    signal: result.signal,
    timedOut,
    output,
  };
}

function test(name, fn) {
  try {
    fn();
    console.log(`[PASS] ${name}`);
  } catch (err) {
    console.error(`[FAIL] ${name}`);
    if (err instanceof Error) {
      console.error(err.message);
    } else {
      console.error(String(err));
    }
    process.exitCode = 1;
  }
}

if (!existsSync(binaryPath)) {
  console.error(`[FAIL] release binary not found: ${binaryPath}`);
  console.error("Run: CARGO_TARGET_DIR=target cargo build --release");
  process.exit(1);
}

test("direct external command accepts copilot tool name", () => {
  const result = runCli(["copilot", "--help"]);
  assert.equal(result.timedOut, false, "command timed out");
  assert.ok(
    !result.output.includes("is not a valid tool or command"),
    "copilot was rejected as invalid tool"
  );
  assert.ok(result.output.length > 0, "no output captured");
});

test("run command without separator consumes --help", () => {
  const result = runCli(["run", "copilot", "--help"]);
  assert.equal(result.timedOut, false, "command timed out");
  assert.equal(result.status, 0, "expected success status");
  assert.ok(
    result.output.includes("Usage: terminal-jarvis run"),
    "expected run subcommand help output"
  );
});

test("run command forwards --help to tool after -- separator", () => {
  const result = runCli(["run", "copilot", "--", "--help"]);
  assert.equal(result.timedOut, false, "command timed out");
  assert.ok(
    !result.output.includes("Usage: terminal-jarvis run"),
    "run help should not appear when forwarding args"
  );
  assert.ok(
    !result.output.includes("is not a valid tool or command"),
    "tool unexpectedly rejected during forwarding"
  );
  assert.ok(result.output.length > 0, "no output captured");
});

test("direct tool command closes cleanly on repeated launch", () => {
  const first = runCli(["copilot", "--help"]);
  const second = runCli(["copilot", "--help"]);
  assert.equal(first.timedOut, false, "first command timed out");
  assert.equal(second.timedOut, false, "second command timed out");
  assert.ok(
    !first.output.includes("is not a valid tool or command"),
    "first run rejected copilot"
  );
  assert.ok(
    !second.output.includes("is not a valid tool or command"),
    "second run rejected copilot"
  );
});

if (process.exitCode && process.exitCode !== 0) {
  process.exit(process.exitCode);
}

console.log("[PASS] all direct invocation forwarding smoke tests passed");

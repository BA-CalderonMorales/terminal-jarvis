#!/usr/bin/env node

import { spawn } from "child_process";
import { existsSync } from "fs";
import { arch, platform } from "os";
import { join } from "path";

function getBundledBinaryPath(): string {
  const currentPlatform = platform();
  const currentArch = arch();

  // Map Node.js platform/arch to our binary naming convention
  let binaryName = "terminal-jarvis";

  if (currentPlatform === "linux") {
    binaryName += "-linux";
  } else if (currentPlatform === "darwin") {
    binaryName += "-macos";
  } else if (currentPlatform === "win32") {
    binaryName += "-windows.exe";
  } else {
    binaryName += "-linux"; // fallback
  }

  if (currentPlatform !== "win32") {
    if (currentArch === "x64" || currentArch === "x86_64") {
      binaryName += "-x64";
    } else if (currentArch === "arm64") {
      binaryName += "-arm64";
    } else {
      binaryName += "-x64"; // fallback
    }
  }

  return join(__dirname, "..", "bin", binaryName);
}

// Try to find the Rust binary in common locations
const possibleBinaries = [
  join(__dirname, "..", "bin", "terminal-jarvis"), // Bundled binary (generic)
  join(__dirname, "..", "bin", "terminal-jarvis-linux-x64"), // Bundled binary (platform-specific)
  join(__dirname, "..", "..", "..", "target", "debug", "terminal-jarvis"), // Local debug build
  join(__dirname, "..", "..", "..", "target", "release", "terminal-jarvis"), // Local release build
  join(process.env.HOME || "", ".cargo", "bin", "terminal-jarvis"), // Cargo home
  "/usr/local/bin/terminal-jarvis", // System install
];

async function findRustBinary(): Promise<string | null> {
  // First check local build and other possible locations
  for (const path of possibleBinaries) {
    if (existsSync(path)) {
      return path;
    }
  }

  // Then check if terminal-jarvis is available in PATH
  return new Promise((resolve) => {
    const child = spawn("which", ["terminal-jarvis"], { stdio: "pipe" });
    let output = "";

    child.stdout.on("data", (data) => {
      output += data.toString();
    });

    child.on("close", (code) => {
      if (code === 0 && output.trim()) {
        const binaryPath = output.trim();
        // Simple check: if it's not in node_modules and exists, assume it's the Rust binary
        if (
          !binaryPath.includes("node_modules") &&
          !binaryPath.includes(".npm") &&
          existsSync(binaryPath)
        ) {
          resolve(binaryPath);
        } else {
          resolve(null);
        }
      } else {
        resolve(null);
      }
    });

    child.on("error", () => resolve(null));
  });
}

async function main() {
  const rustBinary = await findRustBinary();

  if (rustBinary) {
    // Run the Rust binary with all arguments
    const child = spawn(rustBinary, process.argv.slice(2), {
      stdio: "inherit",
      cwd: process.cwd(),
    });

    // Add a reasonable timeout to prevent indefinite hanging
    const timeout = setTimeout(() => {
      console.log("\n[TIMEOUT] T.JARVIS process timed out after 60 seconds");
      console.log("          This usually happens with long-running interactive tools.");
      console.log("");
      console.log("Tip:");
      console.log("  - The tool might still be running in the background");
      console.log("  - Use Ctrl+C next time to stop manually");
      console.log("  - Some AI tools take time to initialize");
      console.log("");
      child.kill("SIGTERM");
      process.exit(1);
    }, 60000); // 60 second timeout

    child.on("close", (code) => {
      clearTimeout(timeout);
      process.exit(code || 0);
    });

    child.on("error", (err) => {
      clearTimeout(timeout);
      console.error("\n[ERROR] T.JARVIS SYSTEM: Binary execution failed");
      console.error("   " + err.message);
      console.error("");

      // Provide specific error context
      if (err.message.includes("ENOENT")) {
        console.error("[DIAGNOSIS] Binary not found or not executable");
        console.error(
          "           The T.JARVIS binary appears to be missing or has incorrect permissions.",
        );
      } else if (err.message.includes("EACCES")) {
        console.error("[DIAGNOSIS] Permission denied");
        console.error("           The binary exists but cannot be executed.");
        console.error("           SOLUTION: chmod +x " + rustBinary);
      } else if (err.message.includes("spawn")) {
        console.error("[DIAGNOSIS] Process spawn failure");
        console.error("           This indicates a system-level execution issue.");
      }
      console.error("");
      showFallbackMessage();
    });
  } else {
    showFallbackMessage();
  }
}

function showFallbackMessage() {
  console.log("Terminal Jarvis v0.0.76");
  console.log("");
  console.log("[ERROR] T.JARVIS SYSTEM: Could not locate or execute the core binary.");
  console.log("");
  console.log("Debug: Searched for binary in:");
  console.log("  - Bundled binary (bin/terminal-jarvis)");
  console.log("  - Local builds (target/debug, target/release)");
  console.log("  - ~/.cargo/bin/terminal-jarvis");
  console.log("  - /usr/local/bin/terminal-jarvis");
  console.log("  - PATH (excluding node_modules)");
  console.log("");
  console.log("[WARNING] This should not happen! The binary should be bundled with this package.");
  console.log("");
  console.log("Quick Fix (try these in order):");
  console.log("  1. npm uninstall -g terminal-jarvis");
  console.log("  2. npm cache clean --force");
  console.log("  3. npm install -g terminal-jarvis");
  console.log("");
  console.log("Detailed Troubleshooting:");
  console.log("  - Check binary exists: ls -la $(npm root -g)/terminal-jarvis/bin/");
  console.log("  - Fix permissions: chmod +x $(npm root -g)/terminal-jarvis/bin/*");
  console.log("  - Check npm global: npm list -g terminal-jarvis");
  console.log("  - Verify Node.js: node --version (need 16+)");
  console.log("");
  console.log("Alternative Installation Methods:");
  console.log(
    "  - From source: cargo install --git https://github.com/BA-CalderonMorales/terminal-jarvis",
  );
  console.log(
    "  - Direct download: Check releases at github.com/BA-CalderonMorales/terminal-jarvis",
  );
  console.log("");
  console.log("Need Help?");
  console.log("  - Report issue: https://github.com/BA-CalderonMorales/terminal-jarvis/issues");
  console.log(
    "  - Include output of: npm --version && node --version && npm list -g terminal-jarvis",
  );
  process.exit(1);
}

main().catch((err) => {
  console.error("Unexpected error:", err);
  process.exit(1);
});

#!/usr/bin/env node

import { spawn } from 'child_process';
import { existsSync } from 'fs';
import { join } from 'path';

// Try to find the Rust binary in common locations
const possibleBinaries = [
    join(__dirname, '..', '..', '..', 'target', 'debug', 'terminal-jarvis'),    // Local debug build
    join(__dirname, '..', '..', '..', 'target', 'release', 'terminal-jarvis'),  // Local release build
    join(process.env.HOME || '', '.cargo', 'bin', 'terminal-jarvis'),  // Cargo home
    '/usr/local/bin/terminal-jarvis',  // System install
];

async function findRustBinary(): Promise<string | null> {
    // First check local build and other possible locations
    for (const path of possibleBinaries) {
        if (existsSync(path)) {
            return path;
        }
    }
    
    // Then check if terminal-jarvis is available in PATH and verify it's the Rust binary
    return new Promise((resolve) => {
        const child = spawn('which', ['terminal-jarvis'], { stdio: 'pipe' });
        let output = '';
        
        child.stdout.on('data', (data) => {
            output += data.toString();
        });
        
        child.on('close', (code) => {
            if (code === 0 && output.trim()) {
                const binaryPath = output.trim();
                // Test if this is actually the Rust binary by running it with --version
                const testChild = spawn(binaryPath, ['--version'], { stdio: 'pipe' });
                let versionOutput = '';
                
                testChild.stdout.on('data', (data) => {
                    versionOutput += data.toString();
                });
                
                testChild.on('close', (testCode) => {
                    // If it responds to --version and mentions terminal-jarvis, it's likely the Rust binary
                    if (testCode === 0 && versionOutput.toLowerCase().includes('terminal-jarvis')) {
                        resolve(binaryPath);
                    } else {
                        resolve(null);
                    }
                });
                
                testChild.on('error', () => resolve(null));
            } else {
                resolve(null);
            }
        });
        
        child.on('error', () => resolve(null));
    });
}

async function main() {
    const rustBinary = await findRustBinary();
    
    if (rustBinary) {
        // Run the Rust binary with all arguments
        const child = spawn(rustBinary, process.argv.slice(2), {
            stdio: 'inherit',
            cwd: process.cwd()
        });
        
        child.on('close', (code) => {
            process.exit(code || 0);
        });
        
        child.on('error', (err) => {
            console.error('Error running terminal-jarvis binary:', err.message);
            showFallbackMessage();
        });
    } else {
        showFallbackMessage();
    }
}

function showFallbackMessage() {
    console.log("🤖 Terminal Jarvis v0.0.8");
    console.log("");
    console.log("⚠️  The full T.JARVIS interactive interface requires the Rust binary.");
    console.log("");
    console.log("🔍 Debug: Searched for binary in:");
    console.log("  • Local builds (target/debug, target/release)");
    console.log("  • ~/.cargo/bin/terminal-jarvis");
    console.log("  • /usr/local/bin/terminal-jarvis");
    console.log("  • PATH (verified with --version check)");
    console.log("");
    console.log("🚀 Install for full functionality:");
    console.log("  cargo install --git https://github.com/BA-CalderonMorales/terminal-jarvis");
    console.log("");
    console.log("📦 Or install Rust first:");
    console.log("  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
    console.log("  source ~/.cargo/env");
    console.log("  cargo install --git https://github.com/BA-CalderonMorales/terminal-jarvis");
    console.log("");
    console.log("✨ Features you'll get with the full version:");
    console.log("  • Interactive T.JARVIS interface with ASCII art");
    console.log("  • One-click tool installation (claude, gemini, qwen, opencode)");
    console.log("  • Smart tool detection and management");
    console.log("  • Real-time status checking");
    console.log("  • Responsive terminal design");
    console.log("");
    console.log("💡 This NPM package serves as a convenient installer and launcher.");
    process.exit(1);
}

main().catch((err) => {
    console.error('Unexpected error:', err);
    process.exit(1);
});

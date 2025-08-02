#!/usr/bin/env node

import { spawn } from 'child_process';
import { existsSync } from 'fs';
import { join } from 'path';
import { platform, arch } from 'os';

function getBundledBinaryPath(): string {
    const currentPlatform = platform();
    const currentArch = arch();
    
    // Map Node.js platform/arch to our binary naming convention
    let binaryName = 'terminal-jarvis';
    
    if (currentPlatform === 'linux') {
        binaryName += '-linux';
    } else if (currentPlatform === 'darwin') {
        binaryName += '-macos';
    } else if (currentPlatform === 'win32') {
        binaryName += '-windows.exe';
    } else {
        binaryName += '-linux'; // fallback
    }
    
    if (currentPlatform !== 'win32') {
        if (currentArch === 'x64' || currentArch === 'x86_64') {
            binaryName += '-x64';
        } else if (currentArch === 'arm64') {
            binaryName += '-arm64';
        } else {
            binaryName += '-x64'; // fallback
        }
    }
    
    return join(__dirname, '..', 'bin', binaryName);
}

// Try to find the Rust binary in common locations
const possibleBinaries = [
    join(__dirname, '..', 'bin', 'terminal-jarvis'),  // Bundled binary (generic)
    join(__dirname, '..', 'bin', 'terminal-jarvis-linux-x64'),  // Bundled binary (platform-specific)
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
    
    // Then check if terminal-jarvis is available in PATH
    return new Promise((resolve) => {
        const child = spawn('which', ['terminal-jarvis'], { stdio: 'pipe' });
        let output = '';
        
        child.stdout.on('data', (data) => {
            output += data.toString();
        });
        
        child.on('close', (code) => {
            if (code === 0 && output.trim()) {
                const binaryPath = output.trim();
                // Simple check: if it's not in node_modules and exists, assume it's the Rust binary
                if (!binaryPath.includes('node_modules') && !binaryPath.includes('.npm') && existsSync(binaryPath)) {
                    resolve(binaryPath);
                } else {
                    resolve(null);
                }
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
        
        // Add a reasonable timeout to prevent indefinite hanging
        const timeout = setTimeout(() => {
            console.log('\n⚠️  Process timed out after 60 seconds');
            child.kill('SIGTERM');
            process.exit(1);
        }, 60000); // 60 second timeout
        
        child.on('close', (code) => {
            clearTimeout(timeout);
            process.exit(code || 0);
        });
        
        child.on('error', (err) => {
            clearTimeout(timeout);
            console.error('Error running terminal-jarvis binary:', err.message);
            showFallbackMessage();
        });
    } else {
        showFallbackMessage();
    }
}

function showFallbackMessage() {
    console.log("🤖 Terminal Jarvis v0.0.16");
    console.log("");
    console.log("❌ Error: Could not find or execute the T.JARVIS binary.");
    console.log("");
    console.log("🔍 Debug: Searched for binary in:");
    console.log("  • Bundled binary (bin/terminal-jarvis)");
    console.log("  • Local builds (target/debug, target/release)");
    console.log("  • ~/.cargo/bin/terminal-jarvis");
    console.log("  • /usr/local/bin/terminal-jarvis");
    console.log("  • PATH (excluding node_modules)");
    console.log("");
    console.log("� This should not happen! The binary should be bundled with this package.");
    console.log("");
    console.log("🔧 Troubleshooting:");
    console.log("  • Try reinstalling: npm uninstall -g terminal-jarvis && npm install -g terminal-jarvis");
    console.log("  • Check if binary exists: ls -la $(npm root -g)/terminal-jarvis/bin/");
    console.log("  • Verify executable permissions: chmod +x $(npm root -g)/terminal-jarvis/bin/terminal-jarvis");
    console.log("");
    console.log("� Alternative - Install directly from source:");
    console.log("  cargo install --git https://github.com/BA-CalderonMorales/terminal-jarvis");
    console.log("");
    console.log("� If this persists, please report this issue:");
    console.log("  https://github.com/BA-CalderonMorales/terminal-jarvis/issues");
    process.exit(1);
}

main().catch((err) => {
    console.error('Unexpected error:', err);
    process.exit(1);
});

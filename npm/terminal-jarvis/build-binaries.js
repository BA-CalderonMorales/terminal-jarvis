#!/usr/bin/env node

const { execSync } = require('child_process');
const { existsSync, mkdirSync, copyFileSync } = require('fs');
const { join } = require('path');

// Create bin directory if it doesn't exist
const binDir = join(__dirname, 'bin');
if (!existsSync(binDir)) {
    mkdirSync(binDir, { recursive: true });
}

// Build release binary
console.log('🔨 Building Rust binary in release mode...');
try {
    execSync('cargo build --release', { 
        cwd: join(__dirname, '..', '..'),
        stdio: 'inherit'
    });
} catch (error) {
    console.error('❌ Failed to build Rust binary:', error.message);
    process.exit(1);
}

// Copy binary with platform-specific name
const sourceBinary = join(__dirname, '..', '..', 'target', 'release', 'terminal-jarvis');
const targetBinary = join(binDir, 'terminal-jarvis-linux-x64');

if (existsSync(sourceBinary)) {
    copyFileSync(sourceBinary, targetBinary);
    console.log(`✅ Binary copied to: ${targetBinary}`);
    
    // Make it executable
    execSync(`chmod +x "${targetBinary}"`);
    console.log('✅ Binary made executable');
} else {
    console.error('❌ Source binary not found:', sourceBinary);
    process.exit(1);
}

console.log('🎉 Build complete! Binary bundled with NPM package.');

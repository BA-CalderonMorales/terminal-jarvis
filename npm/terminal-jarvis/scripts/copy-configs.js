#!/usr/bin/env node

const { existsSync, mkdirSync, readdirSync, copyFileSync, statSync } = require('fs');
const { join, resolve } = require('path');

function ensureDir(dir) {
    if (!existsSync(dir)) {
        mkdirSync(dir, { recursive: true });
    }
}

function copyDirRecursive(src, dest, filterFn) {
    ensureDir(dest);
    const entries = readdirSync(src, { withFileTypes: true });
    for (const entry of entries) {
        const srcPath = join(src, entry.name);
        const destPath = join(dest, entry.name);
        if (entry.isDirectory()) {
            copyDirRecursive(srcPath, destPath, filterFn);
        } else if (!filterFn || filterFn(srcPath)) {
            copyFileSync(srcPath, destPath);
        }
    }
}

try {
    // Source directory: repo config/tools
    const srcDir = resolve(__dirname, '..', '..', '..', 'config', 'tools');
    // Destination inside npm package: npm/terminal-jarvis/config/tools
    const destDir = resolve(__dirname, '..', 'config', 'tools');

    // Validate source exists
    if (!existsSync(srcDir) || !statSync(srcDir).isDirectory()) {
        console.warn('[WARN] Source tools config directory not found:', srcDir);
        process.exit(0); // don\'t fail build; just warn
    }

    ensureDir(destDir);

    // Copy only .toml files and keep folder structure flat
    copyDirRecursive(srcDir, destDir, (p) => p.endsWith('.toml'));

    console.log('[OK] Tool configs copied to', destDir);
} catch (err) {
    console.error('[ERROR] Failed to copy tool configs:', err && err.message ? err.message : err);
    process.exit(1);
}

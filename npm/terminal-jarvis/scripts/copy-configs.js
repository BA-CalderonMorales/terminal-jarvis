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
    const toolsSrcDir = resolve(__dirname, '..', '..', '..', 'config', 'tools');
    // Destination inside npm package: npm/terminal-jarvis/config/tools
    const toolsDestDir = resolve(__dirname, '..', 'config', 'tools');

    // Validate source exists
    if (!existsSync(toolsSrcDir) || !statSync(toolsSrcDir).isDirectory()) {
        console.warn('[WARN] Source tools config directory not found:', toolsSrcDir);
        process.exit(0); // don\'t fail build; just warn
    }

    ensureDir(toolsDestDir);

    // Copy only .toml files and keep folder structure flat
    copyDirRecursive(toolsSrcDir, toolsDestDir, (p) => p.endsWith('.toml'));

    console.log('[OK] Tool configs copied to', toolsDestDir);

    // Source directory: repo config/evals
    const evalsSrcDir = resolve(__dirname, '..', '..', '..', 'config', 'evals');
    // Destination inside npm package: npm/terminal-jarvis/config/evals
    const evalsDestDir = resolve(__dirname, '..', 'config', 'evals');

    // Validate source exists
    if (!existsSync(evalsSrcDir) || !statSync(evalsSrcDir).isDirectory()) {
        console.warn('[WARN] Source evals config directory not found:', evalsSrcDir);
        process.exit(0); // don\'t fail build; just warn
    }

    ensureDir(evalsDestDir);

    // Copy entire evals directory structure including evaluations subdirectory
    copyDirRecursive(evalsSrcDir, evalsDestDir, (p) => p.endsWith('.toml'));

    console.log('[OK] Evals configs copied to', evalsDestDir);
} catch (err) {
    console.error('[ERROR] Failed to copy configs:', err && err.message ? err.message : err);
    process.exit(1);
}

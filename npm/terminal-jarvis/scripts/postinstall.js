#!/usr/bin/env node
/*
 Enhanced postinstall script for terminal-jarvis
 - Detect OS and architecture
 - Download platform-specific binary from GitHub releases with retry logic
 - Create dynamic launcher script
 - Verify installation

 Version Hint (used by CI for consistency checks):
 Terminal Jarvis v0.0.70
*/

const fs = require('fs');
const os = require('os');
const path = require('path');
const https = require('https');
const { spawnSync } = require('child_process');
const { createWriteStream } = require('fs');

const pkg = require('../package.json');

// Configuration
const DOWNLOAD_RETRIES = 3;
const DOWNLOAD_TIMEOUT = 30000; // 30 seconds
const GITHUB_REPO = 'BA-CalderonMorales/terminal-jarvis';

function log(msg) {
    console.log(`[terminal-jarvis] ${msg}`);
}

function warn(msg) {
    console.warn(`[terminal-jarvis] Warning: ${msg}`);
}

function error(msg) {
    console.error(`[terminal-jarvis] Error: ${msg}`);
}

/**
 * Detect platform and return download information
 */
function getPlatformInfo() {
    const platform = os.platform();
    const arch = os.arch();

    // Map to GitHub release file names
    if (platform === 'darwin' && (arch === 'x64' || arch === 'arm64')) {
        return {
            name: 'macOS',
            file: 'terminal-jarvis-mac.tar.gz',
            isWindows: false
        };
    }

    if (platform === 'linux' && (arch === 'x64' || arch === 'arm64')) {
        return {
            name: 'Linux',
            file: 'terminal-jarvis-linux.tar.gz',
            isWindows: false
        };
    }

    if (platform === 'win32' && (arch === 'x64' || arch === 'arm64')) {
        return {
            name: 'Windows',
            file: 'terminal-jarvis-windows.tar.gz',
            isWindows: true
        };
    }

    return null;
}

/**
 * Construct GitHub release asset URL
 */
function getAssetUrl(version, fileName) {
    const tag = `v${version}`;
    return `https://github.com/${GITHUB_REPO}/releases/download/${tag}/${fileName}`;
}

/**
 * Download file with retry logic and timeout
 */
async function download(url, dest, retries = DOWNLOAD_RETRIES) {
    await fs.promises.mkdir(path.dirname(dest), { recursive: true });

    for (let attempt = 1; attempt <= retries; attempt++) {
        try {
            log(`Download attempt ${attempt}/${retries}: ${url}`);

            await new Promise((resolve, reject) => {
                const timeout = setTimeout(() => {
                    reject(new Error(`Download timeout after ${DOWNLOAD_TIMEOUT}ms`));
                }, DOWNLOAD_TIMEOUT);

                https.get(url, (res) => {
                    // Handle redirects
                    if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
                        clearTimeout(timeout);
                        return download(res.headers.location, dest, 1).then(resolve).catch(reject);
                    }

                    if (res.statusCode !== 200) {
                        clearTimeout(timeout);
                        return reject(new Error(`HTTP ${res.statusCode}: ${res.statusMessage}`));
                    }

                    const fileStream = createWriteStream(dest);
                    res.pipe(fileStream);

                    fileStream.on('finish', () => {
                        clearTimeout(timeout);
                        fileStream.close(() => resolve(dest));
                    });

                    fileStream.on('error', (err) => {
                        clearTimeout(timeout);
                        reject(err);
                    });
                }).on('error', (err) => {
                    clearTimeout(timeout);
                    reject(err);
                });
            });

            // Success
            return dest;

        } catch (err) {
            if (attempt === retries) {
                throw new Error(`Failed after ${retries} attempts: ${err.message}`);
            }
            warn(`Attempt ${attempt} failed: ${err.message}. Retrying...`);
            // Wait before retry (exponential backoff)
            await new Promise(resolve => setTimeout(resolve, 1000 * attempt));
        }
    }
}

/**
 * Extract tar.gz archive
 */
async function extractTarGz(archivePath, extractDir) {
    await fs.promises.mkdir(extractDir, { recursive: true });

    const res = spawnSync('tar', ['-xzf', archivePath, '-C', extractDir], { stdio: 'inherit' });
    if (res.status !== 0) {
        throw new Error('Failed to extract archive with tar');
    }
}

/**
 * Verify prerequisites
 */
function checkPrerequisites() {
    const missing = [];

    const tarCheck = spawnSync('tar', ['--version'], { stdio: 'ignore' });
    if (tarCheck.status !== 0) {
        missing.push('tar');
    }

    if (missing.length > 0) {
        warn(`Missing required system tool(s): ${missing.join(', ')}`);
        const platform = os.platform();

        if (missing.includes('tar')) {
            if (platform === 'linux') {
                warn('Install tar using your package manager:');
                console.log('  Debian/Ubuntu: sudo apt-get update && sudo apt-get install -y tar');
                console.log('  Fedora/RHEL:   sudo dnf install -y tar');
                console.log('  Arch Linux:    sudo pacman -S tar');
            } else if (platform === 'darwin') {
                warn('tar should be available on macOS by default.');
                console.log('  Install via:   xcode-select --install');
            } else if (platform === 'win32') {
                warn('tar should be available on Windows 10+ by default.');
            }
        }

        return false;
    }

    return true;
}

/**
 * Main installation workflow
 */
(async () => {
    try {
        // Detect platform
        const platformInfo = getPlatformInfo();
        if (!platformInfo) {
            warn(`Unsupported platform: ${os.platform()}/${os.arch()}`);
            warn('Skipping binary download. You can still install via cargo:');
            console.log('  cargo install terminal-jarvis');
            process.exit(0);
        }

        log(`Detected ${platformInfo.name} (${os.arch()})`);

        // Check prerequisites
        if (!checkPrerequisites()) {
            warn('Skipping installation due to missing prerequisites.');
            warn('You can retry by running: node node_modules/terminal-jarvis/scripts/postinstall.js');
            return;
        }

        // Setup paths
        const version = pkg.version;
        const assetFile = platformInfo.file;
        const url = getAssetUrl(version, assetFile);

        const pkgRoot = path.join(__dirname, '..');
        const binDir = path.join(pkgRoot, 'bin');
        const downloadDir = path.join(pkgRoot, 'downloads');
        const archivePath = path.join(downloadDir, assetFile);

        // Download binary
        log(`Downloading v${version} from GitHub releases...`);
        await download(url, archivePath);
        log('[SUCCESS] Download complete');

        // Extract archive
        log('Extracting archive...');
        await extractTarGz(archivePath, downloadDir);
        log('[SUCCESS] Extraction complete');

        // Move binary to bin directory
        const extractedBin = path.join(downloadDir, 'terminal-jarvis');
        const binaryDest = path.join(binDir, platformInfo.isWindows ? 'terminal-jarvis.exe' : 'terminal-jarvis-bin');

        await fs.promises.mkdir(binDir, { recursive: true });
        await fs.promises.copyFile(extractedBin, binaryDest);
        if (!platformInfo.isWindows) {
            await fs.promises.chmod(binaryDest, 0o755);
        }
        log(`[SUCCESS] Binary installed: ${binaryDest}`);

        // Note: The launcher script (bin/terminal-jarvis) is committed to the repository
        // We only download and install the binary here
        const launcherPath = path.join(binDir, 'terminal-jarvis');

        // Cleanup downloads
        try {
            await fs.promises.rm(downloadDir, { recursive: true, force: true });
            log('Cleaned up temporary files');
        } catch (err) {
            // Non-critical
        }

        // Verify installation
        log('Verifying installation...');
        const verifyPath = platformInfo.isWindows ? binaryDest : launcherPath;
        const verifyRes = spawnSync(verifyPath, ['--version'], { stdio: 'pipe', encoding: 'utf8' });

        if (verifyRes.error) {
            warn(`Verification warning: ${verifyRes.error.message}`);
        } else if (verifyRes.status === 0) {
            log('[SUCCESS] Installation verified');
            log('');
            log('Terminal Jarvis is ready!');
            log('Run: npx terminal-jarvis --help');
        }

    } catch (err) {
        error(`Installation failed: ${err.message}`);
        warn('Fallback options:');
        console.log('  1. Install via cargo: cargo install terminal-jarvis');
        console.log('  2. Install via Homebrew: brew install ba-calderonmorales/terminal-jarvis/terminal-jarvis');
        console.log('  3. Download manually from: https://github.com/BA-CalderonMorales/terminal-jarvis/releases');

        // Don't fail npm install completely
        process.exit(0);
    }
})();

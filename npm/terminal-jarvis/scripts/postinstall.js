#!/usr/bin/env node
/*
 Postinstall script for terminal-jarvis
 - Detect OS and architecture
 - Determine GitHub release asset URL for the current version
 - Download archive
 - Extract executable to bin/terminal-jarvis
 - Make it executable
 - Verify by running `--version`
 
 Version Hint (used by CI for consistency checks):
 Terminal Jarvis v0.0.68
*/

const fs = require('fs');
const os = require('os');
const path = require('path');
const https = require('https');
const { spawnSync } = require('child_process');
const { createWriteStream } = require('fs');
const { pipeline } = require('stream');
const { promisify } = require('util');
// No external tar dependency; we'll shell out to system 'tar'

const pipe = promisify(pipeline);

const pkg = require('../package.json');

function log(msg) {

    console.log(`[terminal-jarvis] ${msg}`);

}

function warn(msg) {

    console.warn(`[terminal-jarvis] Warning: ${msg}`);

}

function error(msg) {

    console.error(`[terminal-jarvis] Error: ${msg}`);

}

function getPlatformTriple() {

    // Map Node.js platform/arch to our release naming
    const platform = os.platform();
    const arch = os.arch();

    // Supported combos
    if (platform === 'darwin' && (arch === 'x64' || arch === 'arm64')) {

        return { name: 'mac', file: 'terminal-jarvis-mac.tar.gz' };

    }

    if (platform === 'linux' && (arch === 'x64' || arch === 'arm64')) {

        return { name: 'linux', file: 'terminal-jarvis-linux.tar.gz' };

    }

    return null;
}

function githubAssetUrl(version, fileName) {

    const tag = `v${version}`;

    return `https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/${tag}/${fileName}`;

}

async function download(url, dest) {

    await fs.promises.mkdir(path.dirname(dest), { recursive: true });

    return new Promise((resolve, reject) => {

        https.get(url, (res) => {

            if (res.statusCode && res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {

                // Redirect
                return download(res.headers.location, dest).then(resolve).catch(reject);

            }

            if (res.statusCode !== 200) {

                return reject(new Error(`Request failed. Status code: ${res.statusCode}`));

            }

            const fileStream = createWriteStream(dest);
            res.pipe(fileStream);
            fileStream.on('finish', () => fileStream.close(() => resolve(dest)));
            fileStream.on('error', reject);

        }).on('error', reject);

    });

}

async function extractTarGz(archivePath, extractDir) {

    await fs.promises.mkdir(extractDir, { recursive: true });

    const res = spawnSync('tar', ['-xzf', archivePath, '-C', extractDir], { stdio: 'inherit' });
    if (res.status !== 0) {
        throw new Error('Failed to extract archive with tar');
    }

}

(async () => {

    try {

        const triple = getPlatformTriple();

        if (!triple) {

            warn(`Unsupported platform/architecture (${os.platform()}/${os.arch()}). Skipping binary download.`);

            process.exit(0);

        }

        // Pre-flight: ensure required system tools exist
        const missing = [];
        const tarCheck = spawnSync('tar', ['--version'], { stdio: 'ignore' });
        if (tarCheck.status !== 0) {
            missing.push('tar');
        }

        if (missing.length > 0) {
            warn(`Missing required system tool(s): ${missing.join(', ')}`);
            const plt = os.platform();
            if (missing.includes('tar')) {
                if (plt === 'linux') {
                    warn('Install tar using your package manager:');
                    console.log('  - Debian/Ubuntu: sudo apt-get update && sudo apt-get install -y tar');
                    console.log('  - Fedora/CentOS/RHEL: sudo dnf install -y tar  (or: sudo yum install -y tar)');
                    console.log('  - Arch: sudo pacman -S tar');
                } else if (plt === 'darwin') {
                    warn('tar should be available on macOS by default. If missing:');
                    console.log('  - Install Xcode Command Line Tools: xcode-select --install');
                    console.log('  - Or install via Homebrew: brew install gnu-tar');
                } else {
                    warn('Please install a tar utility compatible with .tar.gz archives.');
                }
            }
            warn('Skipping binary download due to missing prerequisites. You can retry after installing the required tools by reinstalling or running:');
            console.log('  node node_modules/terminal-jarvis/scripts/postinstall.js');
            return; // Exit gracefully; do not fail npm install
        }

        const version = pkg.version;
        const assetFile = triple.file;
        const url = githubAssetUrl(version, assetFile);

        const pkgRoot = path.join(__dirname, '..');
        const binDir = path.join(pkgRoot, 'bin');
        const binPath = path.join(binDir, 'terminal-jarvis');
        const downloadDir = path.join(pkgRoot, 'downloads');
        const archivePath = path.join(downloadDir, assetFile);

        log(`Detected ${triple.name}. Downloading binary for v${version}...`);
        log(`GET ${url}`);

        await download(url, archivePath);

        log('Extracting archive...');
        await extractTarGz(archivePath, downloadDir);

        // The archives should contain a single executable named "terminal-jarvis"
        const extractedBin = path.join(downloadDir, 'terminal-jarvis');
        await fs.promises.mkdir(binDir, { recursive: true });
        await fs.promises.copyFile(extractedBin, binPath);
        await fs.promises.chmod(binPath, 0o755);

        // Create a small JS launcher for Windows (future) or ensure bin entry points to the POSIX binary
        log('Verifying installation...');
        const res = spawnSync(binPath, ['--version'], { stdio: 'inherit' });
        if (res.error) {

            warn(`Failed to execute downloaded binary: ${res.error.message}`);

        }

        log('Installation complete. You can now run: npx terminal-jarvis');

    } catch (e) {

        warn(`Postinstall failed: ${e.message}`);
        warn('Falling back; you may need to install manually or ensure network access to GitHub releases.');

    }

})();

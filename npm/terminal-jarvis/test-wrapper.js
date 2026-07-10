const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");
const wrapper = require("./bin/terminal-jarvis");

function tempDir() {
  return fs.mkdtempSync(path.join(os.tmpdir(), "terminal-jarvis-wrapper-"));
}

function writeExecutable(file, content) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, content);
  fs.chmodSync(file, 0o755);
}

test("cached binary validation rejects text files", () => {
  const file = path.join(tempDir(), "terminal-jarvis");
  fs.writeFileSync(file, "not a release binary\n");
  fs.chmodSync(file, 0o755);
  assert.equal(wrapper.cachedBinaryUsable(file), false);
});

test("cached binary validation accepts Windows PE files", () => {
  const file = path.join(tempDir(), "terminal-jarvis.exe");
  fs.writeFileSync(file, Buffer.from([0x4d, 0x5a, 0x90, 0x00]));
  fs.chmodSync(file, 0o755);
  assert.equal(wrapper.cachedBinaryUsable(file), true);
});

test("cached release validation requires binary and catalog", () => {
  const root = tempDir();
  const binary = path.join(root, "bin", "terminal-jarvis");
  const catalog = path.join(root, "harnesses");
  fs.mkdirSync(path.dirname(binary), { recursive: true });
  fs.writeFileSync(binary, Buffer.from([0x7f, 0x45, 0x4c, 0x46]));
  assert.equal(wrapper.cachedReleaseUsable(binary, catalog), false);
  fs.mkdirSync(catalog);
  assert.equal(wrapper.cachedReleaseUsable(binary, catalog), true);
});

test("child environment pins wrapper cache and release catalog", () => {
  const cache = path.join(tempDir(), "cache");
  const catalog = path.join(tempDir(), "harnesses");
  const env = wrapper.childEnv({
    binary: "/tmp/terminal-jarvis",
    source: "github-release-cache",
    releaseUrl: "https://example.invalid/release.tgz",
    cache,
    catalog,
  });
  assert.equal(env.TERMINAL_JARVIS_CACHE, cache);
  assert.equal(env.TERMINAL_JARVIS_CATALOG, catalog);
  assert.equal(env.TERMINAL_JARVIS_DISTRIBUTION, "github-release-cache");
});

test("download errors name the release URL and override knob", () => {
  const error = wrapper.downloadFailure("https://example.invalid/release.tgz", new Error("ENOTFOUND"));
  assert.match(error.message, /failed to download Terminal Jarvis release/);
  assert.match(error.message, /https:\/\/example\.invalid\/release\.tgz/);
  assert.match(error.message, /TERMINAL_JARVIS_RELEASE_BASE/);
  assert.match(error.message, /bin[\/\\]README\.txt/);
});

test("platform mapping names published release assets", () => {
  assert.equal(wrapper.platformNameFor("linux", "x64"), "linux-x64-gnu");
  assert.equal(wrapper.platformNameFor("linux", "arm64"), "linux-arm64-gnu");
  assert.equal(wrapper.platformNameFor("darwin", "x64"), "macos-x64");
  assert.equal(wrapper.platformNameFor("darwin", "arm64"), "macos-arm64");
  assert.equal(wrapper.platformNameFor("win32", "x64"), "win32-x64");
});

test("archive names use a Windows-native zip and tar elsewhere", () => {
  assert.match(wrapper.archiveName("win32", "x64"), /win32-x64\.zip$/);
  assert.match(wrapper.archiveName("linux", "x64"), /linux-x64-gnu\.tar\.gz$/);
});

test("PowerShell extraction safely quotes Windows paths", () => {
  const command = wrapper.powershellExtractCommand(
    "C:\\Users\\O'Brien\\release.zip",
    "C:\\Users\\O'Brien\\cache"
  );
  assert.equal(
    command,
    "Expand-Archive -LiteralPath 'C:\\Users\\O''Brien\\release.zip' " +
      "-DestinationPath 'C:\\Users\\O''Brien\\cache' -Force"
  );
});

test("cache roots use each platform's conventional user location", () => {
  assert.equal(
    wrapper.cacheRootFor("win32", { LOCALAPPDATA: "C:\\Users\\me\\AppData\\Local" }, "C:\\Users\\me"),
    path.join("C:\\Users\\me\\AppData\\Local", "terminal-jarvis", "npm")
  );
  assert.equal(
    wrapper.cacheRootFor("linux", {}, "/home/me"),
    path.join("/home/me", ".cache", "terminal-jarvis", "npm")
  );
});

test("executable name matches host conventions", () => {
  assert.equal(wrapper.executableName("linux"), "terminal-jarvis");
  assert.equal(wrapper.executableName("darwin"), "terminal-jarvis");
  assert.equal(wrapper.executableName("win32"), "terminal-jarvis.exe");
});

test("unsupported platform text names supported release assets", () => {
  assert.throws(
    () => wrapper.platformNameFor("freebsd", "x64"),
    /linux-x64-gnu.*linux-arm64-gnu.*macos-x64.*macos-arm64.*win32-x64.*README\.txt/
  );
});

test("npm pack ships wrapper guidance without native binaries", () => {
  const result = spawnSync("npm", ["pack", "--dry-run", "--json", "--loglevel", "error"], {
    cwd: __dirname,
    encoding: "utf8",
  });
  assert.equal(result.status, 0, result.stderr);
  const [pack] = JSON.parse(result.stdout);
  const files = pack.files.map((file) => file.path);
  assert.ok(files.includes("bin/README.txt"));
  assert.ok(files.includes("bin/terminal-jarvis"));
  assert.ok(!files.some((file) => file.endsWith("terminal-jarvis-bin")));
  assert.ok(!files.some((file) => file.startsWith("harnesses/")));
});

test("wrapper forwards --version to the resolved binary", () => {
  const fake = path.join(tempDir(), "terminal-jarvis");
  writeExecutable(fake, "#!/usr/bin/env node\nconsole.log('fake ' + process.argv.slice(2).join(' '));\n");
  const result = spawnSync(process.execPath, [path.join(__dirname, "bin", "terminal-jarvis"), "--version"], {
    env: { ...process.env, TERMINAL_JARVIS_BIN: fake },
    encoding: "utf8",
  });
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout.trim(), "fake --version");
});

test("path diagnostic reports stale binary before npm shim", () => {
  const root = tempDir();
  const stale = path.join(root, "cargo", "terminal-jarvis");
  const shim = path.join(root, "npm", "terminal-jarvis");
  writeExecutable(stale, "#!/bin/sh\n");
  writeExecutable(shim, "#!/bin/sh\n");
  const diagnostic = wrapper.pathShadowDiagnostic({
    pathValue: `${path.dirname(stale)}${path.delimiter}${path.dirname(shim)}`,
    expectedPaths: [shim],
  });
  assert.ok(diagnostic.includes(stale));
  assert.ok(diagnostic.includes(shim));
  assert.match(diagnostic, /before the npm shim/);
});

test("global postinstall warns when a stale binary shadows the npm shim", () => {
  const root = tempDir();
  const prefix = path.join(root, "node");
  const stale = path.join(root, "cargo", "terminal-jarvis");
  const shim = path.join(prefix, "bin", "terminal-jarvis");
  writeExecutable(stale, "#!/bin/sh\n");
  writeExecutable(shim, "#!/bin/sh\n");
  const result = spawnSync(process.execPath, [path.join(__dirname, "postinstall.js")], {
    cwd: __dirname,
    env: {
      ...process.env,
      npm_config_global: "true",
      npm_config_prefix: prefix,
      PATH: `${path.dirname(stale)}${path.delimiter}${path.dirname(shim)}`,
    },
    encoding: "utf8",
  });
  assert.equal(result.status, 0, result.stderr);
  assert.ok(result.stderr.includes(stale));
  assert.ok(result.stderr.includes(shim));
  assert.ok(!result.stderr.includes("refusing to complete a global install"));
});

test("global postinstall warns when the npm shim is missing from PATH", () => {
  const root = tempDir();
  const prefix = path.join(root, "node");
  const result = spawnSync(process.execPath, [path.join(__dirname, "postinstall.js")], {
    cwd: __dirname,
    env: {
      ...process.env,
      npm_config_global: "true",
      npm_config_prefix: prefix,
      PATH: path.join(root, "not-on-path"),
    },
    encoding: "utf8",
  });
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stderr, /npm shim .* is not reachable on PATH/);
  assert.match(result.stderr, /Add .*node(?:[\\/]+bin)? to PATH/);
});

test("global postinstall path diagnostic supports explicit skip", () => {
  const status = wrapper.postinstallPathStatus({
    npm_config_global: "true",
    TERMINAL_JARVIS_SKIP_PATH_DIAGNOSTIC: "1",
  });
  assert.equal(status.kind, "ok");
  assert.equal(status.diagnostic, "");
});

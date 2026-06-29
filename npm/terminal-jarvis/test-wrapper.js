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
});

test("platform mapping names published release assets", () => {
  assert.equal(wrapper.platformNameFor("linux", "x64"), "linux-x64-gnu");
  assert.equal(wrapper.platformNameFor("darwin", "x64"), "macos-x64");
  assert.equal(wrapper.platformNameFor("darwin", "arm64"), "macos-arm64");
});

test("unsupported platform text gives Windows install guidance", () => {
  assert.throws(
    () => wrapper.platformNameFor("win32", "x64"),
    /Native Windows npm installs are not supported.*use WSL on Windows/
  );
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

test("global postinstall fails when PATH shadows the npm shim", () => {
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
  assert.equal(result.status, 1);
  assert.ok(result.stderr.includes(stale));
  assert.ok(result.stderr.includes(shim));
});

test("global postinstall path diagnostic supports explicit skip", () => {
  const status = wrapper.postinstallPathStatus({
    npm_config_global: "true",
    TERMINAL_JARVIS_SKIP_PATH_DIAGNOSTIC: "1",
  });
  assert.equal(status.kind, "ok");
  assert.equal(status.diagnostic, "");
});

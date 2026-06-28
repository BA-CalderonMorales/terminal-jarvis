const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");
const wrapper = require("./bin/terminal-jarvis");

function tempDir() {
  return fs.mkdtempSync(path.join(os.tmpdir(), "terminal-jarvis-wrapper-"));
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

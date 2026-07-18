const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");
const pkg = require("./package.json");
const wrapper = require("./bin/terminal-jarvis");
const cacheIntegrity = require("./bin/cache-integrity");

function tempDir() {
  return fs.mkdtempSync(path.join(os.tmpdir(), "terminal-jarvis-wrapper-"));
}

function writeExecutable(file, content) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, content);
  fs.chmodSync(file, 0o755);
}

function nativeBinary(format, arch) {
  if (format === "elf") {
    const binary = Buffer.alloc(64);
    Buffer.from([0x7f, 0x45, 0x4c, 0x46]).copy(binary);
    binary.writeUInt16LE(arch === "arm64" ? 183 : 62, 18);
    return binary;
  }
  if (format === "pe") {
    const binary = Buffer.alloc(128);
    binary.write("MZ", 0, "ascii");
    binary.writeUInt32LE(64, 60);
    Buffer.from([0x50, 0x45, 0x00, 0x00]).copy(binary, 64);
    binary.writeUInt16LE(arch === "arm64" ? 0xaa64 : 0x8664, 68);
    return binary;
  }
  const binary = Buffer.alloc(32);
  Buffer.from([0xcf, 0xfa, 0xed, 0xfe]).copy(binary);
  binary.writeUInt32LE(arch === "arm64" ? 0x0100000c : 0x01000007, 4);
  return binary;
}

function fakeReleaseOptions(cache, overrides = {}) {
  const arch = overrides.arch || "x64";
  const target = arch === "arm64" ? "linux-arm64-gnu" : "linux-x64-gnu";
  return {
    platform: "linux",
    arch,
    runtime: { android: false, libc: "gnu" },
    cache,
    env: overrides.env || {},
    releaseBase: overrides.releaseBase || `https://example.invalid/v${pkg.version}`,
    fetchFile: overrides.fetchFile || (async (_url, destination) => {
      fs.writeFileSync(destination, overrides.archiveBytes || "fixture archive");
    }),
    verifyChecksum: overrides.verifyChecksum || (async () => {}),
    unpack: overrides.unpack || ((_archive, extractionRoot) => {
      const bundle = path.join(extractionRoot, `${pkg.name}-${pkg.version}-${target}`);
      const binary = path.join(bundle, "bin", "terminal-jarvis");
      fs.mkdirSync(path.dirname(binary), { recursive: true });
      fs.writeFileSync(binary, nativeBinary("elf", overrides.binaryArch || arch));
      for (const directory of ["harnesses", "gates"]) {
        fs.mkdirSync(path.join(bundle, directory));
        fs.writeFileSync(path.join(bundle, directory, "index.toml"), "name='fixture'\n");
      }
    }),
  };
}

test("binary identity rejects text and distinguishes native architectures", () => {
  const root = tempDir();
  for (const [name, bytes, expected] of [
    ["text", Buffer.from("not a binary"), { format: "unknown", arch: "unknown" }],
    ["elf-x64", nativeBinary("elf", "x64"), { format: "elf", arch: "x64" }],
    ["elf-arm64", nativeBinary("elf", "arm64"), { format: "elf", arch: "arm64" }],
    ["pe-x64", nativeBinary("pe", "x64"), { format: "pe", arch: "x64" }],
    ["mach-arm64", nativeBinary("mach-o", "arm64"), { format: "mach-o", arch: "arm64" }],
  ]) {
    const file = path.join(root, name);
    fs.writeFileSync(file, bytes);
    assert.deepEqual(cacheIntegrity.binaryIdentity(file), expected);
  }
});

test("child environment pins wrapper cache and release catalog", () => {
  const cache = path.join(tempDir(), "cache");
  const catalog = path.join(tempDir(), "harnesses");
  const gates = path.join(tempDir(), "gates");
  const env = wrapper.childEnv({
    binary: "/tmp/terminal-jarvis",
    source: "github-release-cache",
    releaseUrl: "https://example.invalid/release.tgz",
    cache,
    catalog,
    gates,
    checksum: "cache-integrity-verified",
  });
  assert.equal(env.TERMINAL_JARVIS_CACHE, cache);
  assert.equal(env.TERMINAL_JARVIS_CATALOG, catalog);
  assert.equal(env.TERMINAL_JARVIS_GATES, gates);
  assert.equal(env.TERMINAL_JARVIS_CHECKSUM, "cache-integrity-verified");
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
  assert.equal(wrapper.platformNameFor("linux", "x64", { libc: "gnu" }), "linux-x64-gnu");
  assert.equal(wrapper.platformNameFor("linux", "arm64", { libc: "gnu" }), "linux-arm64-gnu");
  assert.equal(wrapper.platformNameFor("darwin", "x64"), "macos-x64");
  assert.equal(wrapper.platformNameFor("darwin", "arm64"), "macos-arm64");
  assert.equal(wrapper.platformNameFor("win32", "x64"), "win32-x64");
});

test("archive names use a Windows-native zip and tar elsewhere", () => {
  assert.match(wrapper.archiveName("win32", "x64"), /win32-x64\.zip$/);
  assert.match(wrapper.archiveName("linux", "x64", { libc: "gnu" }), /linux-x64-gnu\.tar\.gz$/);
});

test("Linux selection rejects musl, Alpine, and unidentified libc", () => {
  for (const libc of ["musl", "unknown", ""]) {
    assert.throws(
      () => wrapper.platformNameFor("linux", "x64", { libc }),
      /Only GNU libc.*musl\/Alpine.*linux-x64-gnu.*linux-arm64-gnu/
    );
  }
});

test("unclaimed architectures reject with the complete supported target set", () => {
  const cases = [
    ["linux", "ia32", { libc: "gnu" }],
    ["darwin", "ia32", {}],
    ["win32", "arm64", {}],
  ];
  for (const [platform, arch, runtime] of cases) {
    assert.throws(
      () => wrapper.platformNameFor(platform, arch, runtime),
      /linux-x64-gnu.*linux-arm64-gnu.*macos-x64.*macos-arm64.*win32-x64/
    );
  }
});

test("musl rejects before any release fetch", async () => {
  let fetched = false;
  await assert.rejects(
    wrapper.downloadRelease({
      platform: "linux",
      arch: "x64",
      runtime: { android: false, libc: "musl" },
      fetchFile: async () => { fetched = true; },
    }),
    /Only GNU libc.*musl\/Alpine.*linux-x64-gnu/
  );
  assert.equal(fetched, false);
});

test("Android and Termux reject before selecting a GNU asset", async () => {
  assert.throws(
    () => wrapper.platformNameFor("android", "arm64", { android: true }),
    /Android\/Termux.*linux-x64-gnu.*win32-x64/
  );
  let fetched = false;
  await assert.rejects(
    wrapper.downloadRelease({
      platform: "linux",
      arch: "arm64",
      runtime: { android: true, libc: "gnu" },
      fetchFile: async () => { fetched = true; },
    }),
    /Android\/Termux.*linux-arm64-gnu/
  );
  assert.equal(fetched, false);
});

test("runtime detection accepts glibc WSL without claiming verification", () => {
  const report = { getReport: () => ({ header: { glibcVersionRuntime: "2.39" } }) };
  const runtime = wrapper.runtimePlatformDetails("linux", { WSL_DISTRO_NAME: "Ubuntu" }, report);
  const asset = wrapper.platformNameFor("linux", "x64", runtime);
  assert.deepEqual(runtime, { android: false, libc: "gnu" });
  assert.equal(asset, "linux-x64-gnu");
  assert.doesNotMatch(asset, /verified/i);
});

test("runtime detection fails closed without glibc and recognizes Termux", () => {
  const report = { getReport: () => ({ header: {} }) };
  assert.deepEqual(
    wrapper.runtimePlatformDetails("linux", {}, report),
    { android: false, libc: "unknown" }
  );
  assert.deepEqual(
    wrapper.runtimePlatformDetails("linux", { TERMUX_VERSION: "0.119" }, report),
    { android: true, libc: "" }
  );
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

test("checksum validation is local, strict, and deterministic", async () => {
  const archive = path.join(tempDir(), "release.tar.gz");
  fs.writeFileSync(archive, "candidate archive bytes");
  const digest = crypto.createHash("sha256").update(fs.readFileSync(archive)).digest("hex");
  const checksumUrl = "https://example.invalid/release.tar.gz.sha256";
  await wrapper.verifyChecksum(checksumUrl, archive, async () => `${digest}  release.tar.gz\n`);
  await assert.rejects(
    wrapper.verifyChecksum(checksumUrl, archive, async () => "0".repeat(64)),
    /checksum mismatch/
  );
  await assert.rejects(
    wrapper.verifyChecksum(checksumUrl, archive, async () => "not-a-sha256"),
    /invalid SHA-256 checksum.*example\.invalid/
  );
});

test("extraction selects native tools and reports launch and exit failures", () => {
  const calls = [];
  const run = (command, args, options) => {
    calls.push({ command, args, options });
    return { status: 0 };
  };
  wrapper.unpack("/tmp/release.tar.gz", "/tmp/cache", "linux", run);
  wrapper.unpack("C:\\release.zip", "C:\\cache", "win32", run);
  assert.deepEqual(calls[0], {
    command: "tar",
    args: ["-xzf", "/tmp/release.tar.gz", "-C", "/tmp/cache"],
    options: { stdio: "inherit" },
  });
  assert.equal(calls[1].command, "powershell.exe");
  assert.ok(calls[1].args.includes("-NonInteractive"));
  assert.throws(
    () => wrapper.unpack(
      "release.tar.gz",
      "cache",
      "linux",
      () => ({ status: null, error: new Error("ENOENT") })
    ),
    /failed to start tar extraction: ENOENT/
  );
  assert.throws(
    () => wrapper.unpack("release.zip", "cache", "win32", () => ({ status: 7 })),
    /zip extraction failed/
  );
});

test("corrupt cache is replaced once and the recovered release is reused", async () => {
  const cache = tempDir();
  const target = "linux-x64-gnu";
  const root = path.join(cache, pkg.version, target);
  const bundle = path.join(root, `${pkg.name}-${pkg.version}-${target}`);
  const binary = path.join(bundle, "bin", "terminal-jarvis");
  const catalog = path.join(bundle, "harnesses");
  const gates = path.join(bundle, "gates");
  const stale = path.join(root, "stale-partial-download");
  fs.mkdirSync(path.dirname(binary), { recursive: true });
  fs.writeFileSync(binary, "corrupt\n");
  fs.writeFileSync(stale, "partial\n");
  let fetches = 0;
  let checksums = 0;
  let extractions = 0;
  const options = {
    platform: "linux",
    arch: "x64",
    runtime: { android: false, libc: "gnu" },
    cache,
    env: {},
    releaseBase: `https://example.invalid/v${pkg.version}`,
    fetchFile: async (_url, destination) => {
      fetches += 1;
      fs.writeFileSync(destination, "fixture archive");
    },
    verifyChecksum: async () => { checksums += 1; },
    unpack: (_archive, extractionRoot) => {
      extractions += 1;
      const stagedBundle = path.join(extractionRoot, `${pkg.name}-${pkg.version}-${target}`);
      const stagedBinary = path.join(stagedBundle, "bin", "terminal-jarvis");
      fs.mkdirSync(path.dirname(stagedBinary), { recursive: true });
      fs.writeFileSync(stagedBinary, nativeBinary("elf", "x64"));
      fs.mkdirSync(path.join(stagedBundle, "harnesses"));
      fs.writeFileSync(path.join(stagedBundle, "harnesses", "index.toml"), "name='fixture'\n");
      fs.mkdirSync(path.join(stagedBundle, "gates"));
      fs.writeFileSync(path.join(stagedBundle, "gates", "index.toml"), "name='fixture'\n");
    },
  };
  const recovered = await wrapper.downloadRelease(options);
  assert.equal(recovered.source, "github-release");
  assert.equal(recovered.gates, gates);
  assert.equal(recovered.checksum, "cache-integrity-verified");
  assert.equal(fs.existsSync(path.join(root, cacheIntegrity.METADATA_FILE)), true);
  assert.equal(fs.existsSync(stale), false);
  assert.deepEqual([fetches, checksums, extractions], [1, 1, 1]);
  const reused = await wrapper.downloadRelease({
    ...options,
    fetchFile: async () => assert.fail("cache reuse must not fetch"),
    verifyChecksum: async () => assert.fail("cache reuse must not recheck"),
    unpack: () => assert.fail("cache reuse must not extract"),
  });
  assert.equal(reused.source, "github-release-cache");
  assert.equal(reused.checksum, "cache-integrity-verified");
});

test("cache metadata binds target archive binary catalog and gates", async () => {
  const cache = tempDir();
  await wrapper.downloadRelease(fakeReleaseOptions(cache));
  const root = path.join(cache, pkg.version, "linux-x64-gnu");
  const metadata = JSON.parse(
    fs.readFileSync(path.join(root, cacheIntegrity.METADATA_FILE), "utf8")
  );
  assert.equal(metadata.schema_version, 1);
  assert.deepEqual(metadata.package, { name: pkg.name, version: pkg.version });
  assert.deepEqual(metadata.target, { id: "linux-x64-gnu", platform: "linux", arch: "x64" });
  assert.match(metadata.release.archive, /linux-x64-gnu\.tar\.gz$/);
  for (const field of [metadata.release, ...Object.values(metadata.payload)]) {
    assert.match(field.sha256, /^[a-f0-9]{64}$/);
  }
  assert.deepEqual(metadata.payload.binary, {
    ...metadata.payload.binary,
    format: "elf",
    arch: "x64",
  });
});

test("valid read-only cache is reused without repair writes", async () => {
  const cache = tempDir();
  const options = fakeReleaseOptions(cache);
  const installed = await wrapper.downloadRelease(options);
  const root = path.join(cache, pkg.version, "linux-x64-gnu");
  const metadata = path.join(root, cacheIntegrity.METADATA_FILE);
  const modified = fs.statSync(metadata).mtimeMs;
  for (const directory of [installed.catalog, installed.gates, path.dirname(installed.binary), root]) {
    fs.chmodSync(directory, 0o555);
  }
  fs.chmodSync(metadata, 0o444);
  fs.chmodSync(installed.binary, 0o555);
  const reused = await wrapper.downloadRelease({
    ...options,
    env: { TERMINAL_JARVIS_NO_DOWNLOAD: "1" },
    fetchFile: async () => assert.fail("read-only cache reuse must not fetch"),
    verifyChecksum: async () => assert.fail("read-only cache reuse must not recheck"),
    unpack: () => assert.fail("read-only cache reuse must not extract"),
  });
  assert.equal(reused.source, "github-release-cache");
  assert.equal(fs.statSync(metadata).mtimeMs, modified);
});

test("wrong-architecture payload rejects before cache promotion", async () => {
  const cache = tempDir();
  const root = path.join(cache, pkg.version, "linux-x64-gnu");
  await assert.rejects(
    wrapper.downloadRelease(fakeReleaseOptions(cache, { binaryArch: "arm64" })),
    /release binary target is not usable/
  );
  assert.equal(fs.existsSync(root), false);
  assert.deepEqual(fs.readdirSync(path.dirname(root)).filter((name) => name.includes(".stage-")), []);
});

test("tampered archive binary catalog gates or metadata trigger staged repair", async () => {
  for (const component of ["archive", "binary", "catalog", "gates", "metadata"]) {
    const cache = tempDir();
    const options = fakeReleaseOptions(cache);
    const installed = await wrapper.downloadRelease(options);
    const root = path.join(cache, pkg.version, "linux-x64-gnu");
    const paths = {
      archive: path.join(root, wrapper.archiveName("linux", "x64", { libc: "gnu" })),
      binary: installed.binary,
      catalog: path.join(installed.catalog, "index.toml"),
      gates: path.join(installed.gates, "index.toml"),
      metadata: path.join(root, cacheIntegrity.METADATA_FILE),
    };
    if (component === "metadata") fs.chmodSync(paths[component], 0o644);
    fs.appendFileSync(paths[component], "tampered");
    let fetches = 0;
    const repaired = await wrapper.downloadRelease({
      ...options,
      fetchFile: async (_url, destination) => {
        fetches += 1;
        fs.writeFileSync(destination, "fixture archive");
      },
    });
    assert.equal(fetches, 1, component);
    assert.equal(repaired.source, "github-release", component);
    assert.equal(
      fs.readFileSync(path.join(repaired.catalog, "index.toml"), "utf8"),
      "name='fixture'\n"
    );
  }
});

test("failed staged repair preserves the prior invalid cache", async () => {
  const cache = tempDir();
  const root = path.join(cache, pkg.version, "linux-x64-gnu");
  fs.mkdirSync(root, { recursive: true });
  const marker = path.join(root, "prior-invalid-cache");
  fs.writeFileSync(marker, "preserve until replacement is verified\n");
  await assert.rejects(
    wrapper.downloadRelease(fakeReleaseOptions(cache, {
      verifyChecksum: async () => { throw new Error("checksum mismatch"); },
    })),
    /checksum mismatch/
  );
  assert.equal(fs.readFileSync(marker, "utf8"), "preserve until replacement is verified\n");
  assert.deepEqual(fs.readdirSync(path.dirname(root)).filter((name) => name.includes(".stage-")), []);
});

test("release source identity change invalidates the old cache", async () => {
  const cache = tempDir();
  await wrapper.downloadRelease(fakeReleaseOptions(cache));
  let fetches = 0;
  const changed = await wrapper.downloadRelease(fakeReleaseOptions(cache, {
    releaseBase: `https://mirror.example.invalid/v${pkg.version}`,
    fetchFile: async (_url, destination) => {
      fetches += 1;
      fs.writeFileSync(destination, "fixture archive");
    },
  }));
  assert.equal(fetches, 1);
  assert.match(changed.releaseUrl, /mirror\.example\.invalid/);
});

test("failed cache recovery leaves no staged partial release", async () => {
  const cache = tempDir();
  const target = "linux-x64-gnu";
  const root = path.join(cache, pkg.version, target);
  await assert.rejects(
    wrapper.downloadRelease({
      platform: "linux",
      arch: "x64",
      runtime: { libc: "gnu" },
      cache,
      env: {},
      releaseBase: `https://example.invalid/v${pkg.version}`,
      fetchFile: async (_url, destination) => fs.writeFileSync(destination, "partial"),
      verifyChecksum: async () => { throw new Error("checksum mismatch"); },
    }),
    /failed to download.*checksum mismatch/
  );
  assert.equal(fs.existsSync(root), false);
  assert.deepEqual(fs.readdirSync(path.dirname(root)).filter((name) => name.includes(".stage-")), []);
});

test("incomplete extraction is rejected and removed", async () => {
  const cache = tempDir();
  const root = path.join(cache, pkg.version, "linux-x64-gnu");
  await assert.rejects(
    wrapper.downloadRelease({
      platform: "linux",
      arch: "x64",
      runtime: { libc: "gnu" },
      cache,
      env: {},
      releaseBase: `https://example.invalid/v${pkg.version}`,
      fetchFile: async (_url, destination) => fs.writeFileSync(destination, "archive"),
      verifyChecksum: async () => {},
      unpack: () => {},
    }),
    /release archive missing.*bin.*terminal-jarvis/
  );
  assert.equal(fs.existsSync(root), false);
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

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const SCHEMA_VERSION = 1;
const METADATA_FILE = "cache-identity-v1.json";

function sha256File(file) {
  return crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
}

function digestTree(root) {
  const hash = crypto.createHash("sha256");
  const walk = (directory, prefix = "") => {
    for (const name of fs.readdirSync(directory).sort()) {
      const file = path.join(directory, name);
      const relative = path.posix.join(prefix, name);
      const stat = fs.lstatSync(file);
      if (stat.isSymbolicLink()) throw new Error(`cached payload contains symlink: ${relative}`);
      if (stat.isDirectory()) {
        hash.update(`d\0${relative}\0`);
        walk(file, relative);
      } else if (stat.isFile()) {
        hash.update(`f\0${relative}\0${stat.size}\0`);
        hash.update(fs.readFileSync(file));
        hash.update("\0");
      } else {
        throw new Error(`cached payload contains unsupported entry: ${relative}`);
      }
    }
  };
  walk(root);
  return hash.digest("hex");
}

function binaryIdentity(file) {
  const bytes = fs.readFileSync(file);
  if (bytes.length >= 20 && bytes.subarray(0, 4).equals(Buffer.from([0x7f, 0x45, 0x4c, 0x46]))) {
    const machine = bytes.readUInt16LE(18);
    return { format: "elf", arch: machine === 62 ? "x64" : machine === 183 ? "arm64" : "unknown" };
  }
  if (bytes.length >= 68 && bytes.subarray(0, 2).toString("ascii") === "MZ") {
    const offset = bytes.readUInt32LE(60);
    if (offset + 6 <= bytes.length && bytes.subarray(offset, offset + 4).toString("hex") === "50450000") {
      const machine = bytes.readUInt16LE(offset + 4);
      return { format: "pe", arch: machine === 0x8664 ? "x64" : machine === 0xaa64 ? "arm64" : "unknown" };
    }
  }
  if (bytes.length >= 8 && bytes.subarray(0, 4).toString("hex") === "cffaedfe") {
    const cpu = bytes.readUInt32LE(4);
    return { format: "mach-o", arch: cpu === 0x01000007 ? "x64" : cpu === 0x0100000c ? "arm64" : "unknown" };
  }
  return { format: "unknown", arch: "unknown" };
}

function binaryUsable(file, expectedArch, hostPlatform) {
  const stat = fs.statSync(file);
  const executable = hostPlatform === "win32" || (stat.mode & 0o111) !== 0;
  const identity = binaryIdentity(file);
  return stat.isFile() && executable && identity.arch === expectedArch && identity.format !== "unknown";
}

function payloadPaths(root, identity) {
  const bundle = path.join(root, `${identity.name}-${identity.version}-${identity.target}`);
  return {
    archive: path.join(root, identity.archive),
    binary: path.join(bundle, "bin", identity.binaryName),
    catalog: path.join(bundle, "harnesses"),
    gates: path.join(bundle, "gates"),
    metadata: path.join(root, METADATA_FILE),
  };
}

function createMetadata(root, identity) {
  const files = payloadPaths(root, identity);
  if (!fs.existsSync(files.archive)) throw new Error(`release archive missing: ${files.archive}`);
  if (!fs.existsSync(files.binary)) throw new Error(`release archive missing binary: ${files.binary}`);
  if (!binaryUsable(files.binary, identity.arch, identity.platform)) {
    throw new Error(`release binary target is not usable: ${files.binary}`);
  }
  for (const [label, directory] of [["catalog", files.catalog], ["gates", files.gates]]) {
    if (!fs.statSync(directory).isDirectory()) throw new Error(`release archive missing ${label}: ${directory}`);
  }
  const binary = binaryIdentity(files.binary);
  return {
    schema_version: SCHEMA_VERSION,
    package: { name: identity.name, version: identity.version },
    target: { id: identity.target, platform: identity.platform, arch: identity.arch },
    release: { url: identity.releaseUrl, archive: identity.archive, sha256: sha256File(files.archive) },
    payload: {
      binary: { path: path.relative(root, files.binary), format: binary.format, arch: binary.arch, sha256: sha256File(files.binary) },
      catalog: { path: path.relative(root, files.catalog), sha256: digestTree(files.catalog) },
      gates: { path: path.relative(root, files.gates), sha256: digestTree(files.gates) },
    },
  };
}

function writeMetadata(root, metadata) {
  const destination = path.join(root, METADATA_FILE);
  const temporary = `${destination}.tmp-${process.pid}`;
  fs.writeFileSync(temporary, `${JSON.stringify(metadata, null, 2)}\n`, { mode: 0o444 });
  fs.renameSync(temporary, destination);
}

function inspect(root, identity) {
  try {
    const files = payloadPaths(root, identity);
    const recorded = JSON.parse(fs.readFileSync(files.metadata, "utf8"));
    const expected = createMetadata(root, identity);
    if (JSON.stringify(recorded) !== JSON.stringify(expected)) return null;
    return { ...files, metadata: recorded };
  } catch {
    return null;
  }
}

function stagePath(root) {
  return `${root}.stage-${process.pid}-${crypto.randomBytes(6).toString("hex")}`;
}

function promote(stage, root) {
  const backup = `${root}.backup-${process.pid}-${crypto.randomBytes(6).toString("hex")}`;
  const hadRoot = fs.existsSync(root);
  if (hadRoot) fs.renameSync(root, backup);
  try {
    fs.renameSync(stage, root);
  } catch (error) {
    if (hadRoot && fs.existsSync(backup)) fs.renameSync(backup, root);
    throw error;
  }
  if (hadRoot) fs.rmSync(backup, { recursive: true, force: true });
}

module.exports = {
  METADATA_FILE,
  binaryIdentity,
  createMetadata,
  digestTree,
  inspect,
  payloadPaths,
  promote,
  sha256File,
  stagePath,
  writeMetadata,
};

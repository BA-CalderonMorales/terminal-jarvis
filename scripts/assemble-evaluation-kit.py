#!/usr/bin/env python3
"""Assemble the deterministic, nonpublishing Terminal Jarvis evaluation kit."""
import argparse
import hashlib
import json
import os
import shutil
import stat
import sys
import tempfile
import zipfile
from pathlib import Path

PLATFORMS = [
    "linux-x64-gnu", "linux-arm64-gnu", "macos-x64", "macos-arm64", "win32-x64"
]
EXECUTABLES = {platform: "terminal-jarvis" for platform in PLATFORMS}
EXECUTABLES["win32-x64"] = "terminal-jarvis.exe"
EPOCH_FLOOR = 315532800


def fail(message):
    raise SystemExit(f"assemble-evaluation-kit: {message}")


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def tree_digest(root):
    value = hashlib.sha256()
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        relative = path.relative_to(root).as_posix().encode()
        value.update(relative + b"\0" + bytes.fromhex(digest(path)))
    return value.hexdigest()


def copy_tree(source, destination):
    shutil.copytree(source, destination)
    for path in destination.rglob("*"):
        os.utime(path, (ARGS.epoch, ARGS.epoch), follow_symlinks=False)
        if path.is_file():
            path.chmod(0o644)


def write_json(path, value):
    path.write_text(json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n")
    os.utime(path, (ARGS.epoch, ARGS.epoch))
    path.chmod(0o644)


def component(path, root, kind, target=None):
    return {
        "path": path.relative_to(root).as_posix(), "kind": kind,
        "target": target, "size": path.stat().st_size, "sha256": digest(path)
    }


def copy_payloads(root):
    evidence = []
    expected_catalog = tree_digest(ARGS.repository / "harnesses")
    expected_gates = tree_digest(ARGS.repository / "gates")
    for platform in PLATFORMS:
        source = ARGS.candidates / platform
        if not source.is_dir():
            source = ARGS.candidates / f"candidate-{platform}"
        record_path = source / "native-evidence-v1.json"
        if not record_path.is_file():
            fail(f"missing native evidence for {platform}")
        record = json.loads(record_path.read_text())
        binary_name = EXECUTABLES[platform]
        binary = source / binary_name
        required = {
            "schema_version": 1, "target": platform, "version": ARGS.version,
            "ref": ARGS.ref, "binary_sha256": digest(binary),
            "catalog_sha256": expected_catalog, "gates_sha256": expected_gates,
            "native_fixture": "pass", "public_launcher": "pending",
        }
        for key, value in required.items():
            if record.get(key) != value:
                fail(f"{platform} evidence {key} mismatch")
        destination = root / "payloads" / platform / binary_name
        destination.parent.mkdir(parents=True)
        shutil.copyfile(binary, destination)
        destination.chmod(0o755)
        os.utime(destination, (ARGS.epoch, ARGS.epoch))
        evidence.append(record)
    return evidence


def assemble(stage):
    root = stage / f"terminal-jarvis-{ARGS.version}-evaluation-kit"
    root.mkdir()
    evidence = copy_payloads(root)
    copy_tree(ARGS.repository / "harnesses", root / "catalogs" / "harnesses")
    copy_tree(ARGS.repository / "gates", root / "catalogs" / "gates")
    for name in ["run.sh", "run.ps1", "run.cmd", "unsupported-transcript.txt", "EVALUATION.md"]:
        source = ARGS.repository / "evaluation" / name
        destination = root / name
        shutil.copyfile(source, destination)
        destination.chmod(0o755 if name in {"run.sh", "run.cmd"} else 0o644)
        os.utime(destination, (ARGS.epoch, ARGS.epoch))
    shutil.copyfile(ARGS.repository / "LICENSE", root / "LICENSE")
    os.utime(root / "LICENSE", (ARGS.epoch, ARGS.epoch))
    write_json(root / "native-evidence-v1.json", evidence)
    write_json(root / "provenance-v1.json", {
        "schema_version": 1, "repository": ARGS.repository_url, "ref": ARGS.ref,
        "source_date_epoch": ARGS.epoch, "builder": "candidate workflow",
        "publication": "not-authorized", "attestation": "prepared-not-published",
        "workflow": ".github/workflows/candidate-v0.1.13.yml",
        "actions": ["actions/checkout@v7", "actions/setup-node@v6",
                    "actions/upload-artifact@v7", "actions/download-artifact@v8",
                    "dtolnay/rust-toolchain@stable"]
    })
    packages = [{"SPDXID": "SPDXRef-" + p["target"], "name": "terminal-jarvis",
                 "versionInfo": ARGS.version, "downloadLocation": "NOASSERTION",
                 "checksums": [{"algorithm": "SHA256", "checksumValue": p["binary_sha256"]}]}
                for p in evidence]
    write_json(root / "component-inventory.spdx.json", {
        "spdxVersion": "SPDX-2.3", "dataLicense": "CC0-1.0",
        "SPDXID": "SPDXRef-DOCUMENT", "name": f"terminal-jarvis-{ARGS.version}-evaluation-kit",
        "documentNamespace": f"https://github.com/bashfulrobot/terminal-jarvis/evaluation/{ARGS.ref}",
        "creationInfo": {"created": ARGS.created, "creators": ["Tool: assemble-evaluation-kit.py"]},
        "packages": packages
    })
    inventory = []
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        kind = "payload" if "payloads" in path.parts else "support"
        inventory.append(component(path, root, kind))
    manifest = {
        "schema_version": 1, "name": "terminal-jarvis-evaluation-kit",
        "version": ARGS.version, "ref": ARGS.ref, "source_date_epoch": ARGS.epoch,
        "platforms": PLATFORMS, "components": inventory,
        "catalog_sha256": tree_digest(ARGS.repository / "harnesses"),
        "gates_sha256": tree_digest(ARGS.repository / "gates"),
    }
    write_json(root / "manifest-v1.json", manifest)
    files = sorted(item for item in root.rglob("*") if item.is_file())
    sums = "".join(f"{digest(path)}  {path.relative_to(root).as_posix()}\n" for path in files)
    (root / "SHA256SUMS").write_text(sums)
    os.utime(root / "SHA256SUMS", (ARGS.epoch, ARGS.epoch))
    return root


def create_zip(root, output):
    timestamp = __import__("time").gmtime(max(ARGS.epoch, EPOCH_FLOOR))[:6]
    with zipfile.ZipFile(output, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for path in sorted(item for item in root.rglob("*") if item.is_file()):
            relative = (Path(root.name) / path.relative_to(root)).as_posix()
            info = zipfile.ZipInfo(relative, timestamp)
            info.create_system = 3
            mode = 0o755 if os.access(path, os.X_OK) else 0o644
            info.external_attr = (stat.S_IFREG | mode) << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, path.read_bytes(), compress_type=zipfile.ZIP_DEFLATED,
                             compresslevel=9)


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidates", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--repository", type=Path, default=Path.cwd())
    parser.add_argument("--repository-url", required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--ref", required=True)
    parser.add_argument("--epoch", type=int, required=True)
    parser.add_argument("--created", required=True)
    return parser.parse_args()


ARGS = parse_args()
ARGS.repository = ARGS.repository.resolve()
ARGS.candidates = ARGS.candidates.resolve()
if ARGS.version != "0.1.13" or len(ARGS.ref) != 40 or ARGS.epoch < 0:
    fail("version must be 0.1.13, ref must be a full SHA, and epoch must be nonnegative")
configured = ARGS.repository.joinpath("scripts/release.toml").read_text()
if 'release_platforms = ["' + '", "'.join(PLATFORMS) + '"]' not in configured:
    fail("scripts/release.toml target denominator differs from the evaluation contract")
ARGS.out_dir.mkdir(parents=True, exist_ok=True)
output = ARGS.out_dir / f"terminal-jarvis-{ARGS.version}-evaluation-kit.zip"
with tempfile.TemporaryDirectory() as temporary:
    create_zip(assemble(Path(temporary)), output)
print(f"{digest(output)}  {output.name}")

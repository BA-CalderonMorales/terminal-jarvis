#!/usr/bin/env python3
"""Verify evaluation-kit shape, checksums, and zero-host boundaries."""
import argparse
import hashlib
import json
import os
import tempfile
import zipfile
from pathlib import Path, PurePosixPath

PLATFORMS = {"linux-x64-gnu", "linux-arm64-gnu", "macos-x64", "macos-arm64", "win32-x64"}
ALLOWED_EXECUTABLES = {"run.sh", "run.cmd"} | {
    f"payloads/{target}/terminal-jarvis" for target in PLATFORMS - {"win32-x64"}
} | {"payloads/win32-x64/terminal-jarvis.exe"}


def fail(message):
    raise SystemExit(f"verify-evaluation-kit: {message}")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


parser = argparse.ArgumentParser()
parser.add_argument("archive", type=Path)
args = parser.parse_args()
if args.archive.name != "terminal-jarvis-0.1.13-evaluation-kit.zip":
    fail("archive name differs from the v0.1.13 contract")
with tempfile.TemporaryDirectory() as temporary:
    root = Path(temporary)
    with zipfile.ZipFile(args.archive) as archive:
        names = archive.namelist()
        modes = {info.filename: (info.external_attr >> 16) & 0o777 for info in archive.infolist()}
        for info in archive.infolist():
            path = PurePosixPath(info.filename)
            if path.is_absolute() or ".." in path.parts or info.is_dir():
                fail(f"unsafe or unexpected entry: {info.filename}")
        archive.extractall(root)
    tops = list(root.iterdir())
    if len(tops) != 1 or not tops[0].is_dir():
        fail("archive must contain exactly one root directory")
    kit = tops[0]
    manifest = json.loads((kit / "manifest-v1.json").read_text())
    if manifest.get("version") != "0.1.13" or set(manifest.get("platforms", [])) != PLATFORMS:
        fail("manifest version or platform denominator mismatch")
    for record in manifest.get("components", []):
        path = kit / record["path"]
        if not path.is_file() or sha(path) != record["sha256"] or path.stat().st_size != record["size"]:
            fail(f"manifest component mismatch: {record['path']}")
    expected = {}
    for line in (kit / "SHA256SUMS").read_text().splitlines():
        digest, name = line.split("  ", 1)
        expected[name] = digest
    for name, digest in expected.items():
        if sha(kit / name) != digest:
            fail(f"SHA256SUMS mismatch: {name}")
    actual_exec = set()
    suspicious_suffixes = {".bat", ".com", ".dll", ".dylib", ".so"}
    for path in (item for item in kit.rglob("*") if item.is_file()):
        relative = path.relative_to(kit).as_posix()
        mode = modes[f"{kit.name}/{relative}"] & 0o111
        if mode or path.suffix.lower() in suspicious_suffixes | {".exe"}:
            actual_exec.add(relative)
    if actual_exec != ALLOWED_EXECUTABLES:
        fail(f"unexpected executable set: {sorted(actual_exec ^ ALLOWED_EXECUTABLES)}")
    seed = os.environ.get("TJ_EVALUATION_SECRET_SENTINEL", "TJ_EVALUATION_SECRET_SENTINEL_9f1d")
    if seed.encode() in args.archive.read_bytes():
        fail("seeded secret found in archive")
print(f"verify-evaluation-kit: ok ({len(names)} files, five payloads)")

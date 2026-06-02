#!/usr/bin/env python3
"""Verify dependency versions that close known Grype/GHSA alerts."""

from __future__ import annotations

import argparse
import json
import re
import sys
import tomllib
from pathlib import Path


GO_MINIMUMS = {
    "google.golang.org/grpc": (1, 81, 1),
    "go.opentelemetry.io/otel": (1, 43, 0),
    "go.opentelemetry.io/otel/sdk": (1, 43, 0),
}

CARGO_MINIMUMS = {
    "rpassword": (7, 5, 2),
}

OPTIONAL_CARGO_MINIMUMS = {
    "rustls-webpki": (0, 103, 12),
}

GO_MODULE_RE = re.compile(r"^\s*([^\s]+)\s+v([0-9][^\s]*)")


def format_version(version: tuple[int, ...]) -> str:
    return ".".join(map(str, version))


def parse_version(version: str) -> tuple[int, ...]:
    version = version.removesuffix("/go.mod")
    core = version.split("-", 1)[0]
    return tuple(int(part) for part in core.split("."))


def check_go_mod(root: Path) -> list[str]:
    modules: dict[str, list[tuple[int, ...]]] = {}
    for path in (root / "adk" / "go.mod", root / "adk" / "go.sum"):
        if not path.exists():
            return [f"{path.relative_to(root)} is missing"]
        for line in path.read_text(encoding="utf-8").splitlines():
            match = GO_MODULE_RE.match(line)
            if not match:
                continue
            module = match.group(1)
            version = parse_version(match.group(2))
            modules.setdefault(module, []).append(version)

    errors = []
    for module, minimum in GO_MINIMUMS.items():
        versions = modules.get(module, [])
        if not versions:
            errors.append(f"{module} is missing from adk/go.mod and adk/go.sum")
        for actual in versions:
            if actual < minimum:
                errors.append(
                    f"{module} is {format_version(actual)}, expected >= {format_version(minimum)}"
                )

    return errors


def check_cargo_lock(root: Path) -> list[str]:
    cargo_lock = root / "Cargo.lock"
    if not cargo_lock.exists():
        return ["Cargo.lock is missing"]
    data = tomllib.loads(cargo_lock.read_text(encoding="utf-8"))
    packages = data.get("package", [])

    errors = []
    by_name: dict[str, list[str]] = {}
    for package in packages:
        by_name.setdefault(package["name"], []).append(package["version"])

    for package, minimum in CARGO_MINIMUMS.items():
        versions = by_name.get(package, [])
        if not versions:
            errors.append(f"{package} is missing from Cargo.lock")
            continue
        for version in versions:
            actual = parse_version(version)
            if actual < minimum:
                errors.append(f"{package} is {version}, expected >= {format_version(minimum)}")

    for package, minimum in OPTIONAL_CARGO_MINIMUMS.items():
        for version in by_name.get(package, []):
            actual = parse_version(version)
            if actual < minimum:
                errors.append(f"{package} is {version}, expected >= {format_version(minimum)}")

    if "libsql-sqlite3-parser" in by_name:
        errors.append("libsql-sqlite3-parser is present in Cargo.lock")

    return errors


def check_e2e_lock(root: Path) -> list[str]:
    lockfile = root / "e2e" / "package-lock.json"
    if not lockfile.exists():
        return ["e2e/package-lock.json is missing"]
    data = json.loads(lockfile.read_text(encoding="utf-8"))
    packages = data.get("packages", {})

    errors = []
    if "node_modules/rollup" in packages:
        errors.append("rollup is present in e2e/package-lock.json")

    rollup_optionals = sorted(name for name in packages if name.startswith("node_modules/@rollup/"))
    if rollup_optionals:
        errors.append("rollup optional packages are present: " + ", ".join(rollup_optionals))

    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".", help="repository root")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    errors = check_go_mod(root) + check_cargo_lock(root) + check_e2e_lock(root)
    if errors:
        for error in errors:
            print(f"[FAIL] {error}", file=sys.stderr)
        return 1

    print("[PASS] Known dependency alerts are guarded")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

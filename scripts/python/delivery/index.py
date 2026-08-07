#!/usr/bin/env python3
"""Public entrypoint to the python delivery domain."""
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
DISPATCH = {
    "check-matrix": "check-delivery-matrix.py",
}


def main():
    if len(sys.argv) < 2:
        print("usage: python3 scripts/python/delivery/index.py <check-matrix> [candidates-dir]", file=sys.stderr)
        return 2
    name, args = sys.argv[1], sys.argv[2:]
    target = DISPATCH.get(name)
    if target is None:
        print(f"delivery python index: unknown command {name}", file=sys.stderr)
        return 2
    return subprocess.call([sys.executable, str(HERE / "logic" / target), *args])


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""Public entrypoint to the python release domain."""

import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from logic.cli import USAGE, run  # noqa: E402


def main() -> int:
    if len(sys.argv) < 2 or sys.argv[1] != "changelog":
        print(USAGE, file=sys.stderr)
        return 2
    return run(sys.argv[2:])


if __name__ == "__main__":
    sys.exit(main())

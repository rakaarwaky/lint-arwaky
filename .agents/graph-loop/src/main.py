#!/usr/bin/env python3
"""Graph Loop CLI — single entry point for all modules.

Usage (from anywhere):
    python3 .agents/graph-loop/src/main.py engine start
    python3 .agents/graph-loop/src/main.py engine status
    python3 .agents/graph-loop/src/main.py dashboard full
    python3 .agents/graph-loop/src/main.py health report
"""
from __future__ import annotations

import sys
from pathlib import Path

# bootstrap: allow running as a plain script (no package install needed)
_HERE = Path(__file__).resolve().parent
if str(_HERE.parent) not in sys.path:
    sys.path.insert(0, str(_HERE.parent))


def usage() -> int:
    print("""Graph Loop Pipeline (v2.0 / Python)

Usage: main.py <module> [command] [args]

Modules:
  engine      start | once | recover | status
  dashboard   full | status | nodes | features | activity | metrics
  health      report | check-engine | check-agents | check-state |
              check-disk | check-logs | check-locks
  tui         [--refresh <seconds>]
""")
    return 1


def main() -> int:
    if len(sys.argv) < 2:
        return usage()
    module, rest = sys.argv[1], sys.argv[2:]
    if module == "engine":
        from src.engine import cli
    elif module == "dashboard":
        from src.dashboard import cli
    elif module == "health":
        from src.health import cli
    elif module == "tui":
        from src.tui import cli
    else:
        return usage()
    return cli(rest)


if __name__ == "__main__":
    sys.exit(main())

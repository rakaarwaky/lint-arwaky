#!/usr/bin/env python3
"""Static audit of `crates/` against ARCHITECTURE.md (AES rule set).

Stub: the real checker lands in a later ticket. Until then this prints a
baseline marker and exits 0 so T00 can record the before-state files.

Usage: python3 scripts/audit_architecture_compliance.py
"""
from __future__ import annotations

import sys


def main() -> int:
    print("ARCHITECTURE COMPLIANCE BASELINE — stub")
    return 0


if __name__ == "__main__":
    sys.exit(main())

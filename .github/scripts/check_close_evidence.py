#!/usr/bin/env python3
"""Check that PRs declaring 'Closes #N' include regression evidence.

Exit codes:
  0 — PR has no 'Closes #' declarations, or evidence is present (pass)
  1 — PR declares 'Closes #' but lacks evidence markers (fail)
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

# Patterns that indicate the PR body contains evidence of verified fixes.
EVIDENCE_PATTERNS = [
    re.compile(r"regression_\w+", re.IGNORECASE),      # regression test file/name reference
    re.compile(r"\bVerification:\b", re.MULTILINE),     # explicit Verification section
    re.compile(r"\bTests?\s*(pass|passed)\b", re.IGNORECASE),
    re.compile(r"cargo\s+(test|nextest)\b"),
    re.compile(r"regression.*test", re.IGNORECASE),
]


def extract_closed_issues(pr_body: str) -> list[int]:
    """Extract all issue numbers declared as 'Closes #N' in the PR body."""
    return [int(n) for n in re.findall(r"Closes\s+#(\d+)", pr_body, re.IGNORECASE)]


def has_evidence(pr_body: str, diff_files: list[str] | None = None) -> bool:
    """Return True if the PR body (or diff files) contain evidence markers."""
    if any(p.search(pr_body) for p in EVIDENCE_PATTERNS):
        return True
    # A diff touching test files is also acceptable evidence.
    if diff_files:
        test_markers = ("regression_", "test.rs", "test.py", "test.ts", "tests/")
        for f in diff_files:
            if any(m in f.lower() for m in test_markers):
                return True
    return False


def main() -> int:
    """Entry point: read PR body and diff file list from env or args.

    Usage:
      check_close_evidence.py <pr_body_file> [diff_files_csv]

    Environment variables (used by GitHub Actions):
      PR_BODY   — path to a file containing the PR body (or literal body text)
      PR_DIFF   — comma-separated list of changed file paths in the diff
    """
    pr_body = ""
    diff_files: list[str] | None = None

    if len(sys.argv) >= 2:
        # Argument mode: first arg is a file containing the PR body.
        body_path = Path(sys.argv[1])
        pr_body = body_path.read_text() if body_path.exists() else ""
        if len(sys.argv) >= 3:
            diff_files = [f.strip() for f in sys.argv[2].split(",") if f.strip()]
    else:
        # Env-var mode (GitHub Actions):
        import os
        pr_body = os.environ.get("PR_BODY", "")
        diff_files = os.environ.get("PR_DIFF", "").split(",") or None

    if not pr_body.strip():
        print("check_close_evidence: no PR body provided — pass")
        return 0

    declared = extract_closed_issues(pr_body)
    if not declared:
        print("check_close_evidence: no 'Closes #' declarations found — pass")
        return 0

    print(f"check_close_evidence: declared 'Closes' for issues {declared}")

    if has_evidence(pr_body, diff_files):
        print("check_close_evidence: regression/verification evidence found — pass")
        return 0

    print(
        "check_close_evidence: FAIL — 'Closes #' declared but no regression "
        "evidence found.\n"
        "The PR must include one of:\n"
        "  • A regression test (regression_<short-name>.rs) in the diff\n"
        "  • A 'Verification:' section in the PR body with test output\n"
        "  • A reference to passing regression tests\n"
        "See CONTRIBUTING.md § Issue Closure Policy for the template."
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())

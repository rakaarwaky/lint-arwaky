#!/usr/bin/env python3
"""Check that PRs declaring 'Closes #N' include regression evidence.

Exit codes:
  0 — PR has no 'Closes #' declarations, or evidence is present (pass)
  1 — PR declares 'Closes #' but lacks evidence markers (fail)
"""

from __future__ import annotations

import os
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

# A diff touching one of these paths counts as evidence on its own.
TEST_FILE_MARKERS = ("regression_", "test.rs", "test.py", "test.ts", "tests/")

FAILURE_NOTICE = """check_close_evidence: FAIL — 'Closes #' declared but no regression \
evidence found.
The PR must include one of:
  • A regression test (regression_<short-name>.rs) in the diff
  • A 'Verification:' section in the PR body with test output
  • A reference to passing regression tests
See CONTRIBUTING.md § Issue Closure Policy for the template."""


def extract_closed_issues(pr_body: str) -> list[int]:
    """Extract all issue numbers declared as 'Closes #N' in the PR body."""
    return [int(n) for n in re.findall(r"Closes\s+#(\d+)", pr_body, re.IGNORECASE)]


def body_has_evidence(pr_body: str) -> bool:
    """Return True if the PR body names a verification step or a test run."""
    return any(pattern.search(pr_body) for pattern in EVIDENCE_PATTERNS)


def diff_has_evidence(diff_files: list[str]) -> bool:
    """Return True if the diff touches a test file."""
    return any(marker in path.lower() for path in diff_files for marker in TEST_FILE_MARKERS)


def has_evidence(pr_body: str, diff_files: list[str] | None = None) -> bool:
    """Return True if the PR body (or diff files) contain evidence markers."""
    if body_has_evidence(pr_body):
        return True
    return bool(diff_files) and diff_has_evidence(diff_files)


def split_csv(raw: str) -> list[str]:
    """Split a comma-separated argument into a list of non-empty entries."""
    return [entry.strip() for entry in raw.split(",") if entry.strip()]


def read_from_args() -> tuple[str, list[str] | None]:
    """Read the PR body from argv[1] and the diff file list from argv[2]."""
    body_path = Path(sys.argv[1])
    body = body_path.read_text() if body_path.exists() else ""
    files = split_csv(sys.argv[2]) if len(sys.argv) >= 3 else None
    return body, files


def read_from_env() -> tuple[str, list[str] | None]:
    """Read the PR body and diff file list from the GitHub Actions env."""
    return os.environ.get("PR_BODY", ""), split_csv(os.environ.get("PR_DIFF", "")) or None


def main() -> int:
    """Entry point: read PR body and diff file list from env or args.

    Usage:
      check_close_evidence.py <pr_body_file> [diff_files_csv]

    Environment variables (used by GitHub Actions):
      PR_BODY   — path to a file containing the PR body (or literal body text)
      PR_DIFF   — comma-separated list of changed file paths in the diff
    """
    pr_body, diff_files = read_from_args() if len(sys.argv) >= 2 else read_from_env()

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

    print(FAILURE_NOTICE)
    return 1


if __name__ == "__main__":
    sys.exit(main())
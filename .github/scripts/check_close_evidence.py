#!/usr/bin/env python3
"""Check that PRs declaring a closing keyword include regression evidence.

Exit codes:
  0 — PR has no closing declarations, or evidence is present (pass)
  1 — PR closes an issue but lacks evidence markers (fail)
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

# GitHub closing keywords. All of them close the issue on merge, so all of
# them must trigger the gate — matching only `Closes` let a PR use the
# template's `Fixes #N` and skip the check entirely.
CLOSING_KEYWORDS = r"(?:Closes|Fixes|Resolves)\s+#(\d+)"

# A `Verification:` heading. The trailing `\b` that used to sit after the
# colon could only match when a NON-word character followed, so the template's
# own `Verification:\n` heading was rejected while `Verification:x` passed.
VERIFICATION_HEADING = re.compile(
    r"^[ \t]*(?:#{1,6}[ \t]*)?(?:\*\*)?Verification(?:\*\*)?[ \t]*:",
    re.MULTILINE,
)

# A reported passing run. Matching the bare command name let "cargo test
# failed" satisfy the gate, so a pass word is required in the same sentence.
PASSING_RUN = re.compile(
    r"\btests?\s+(?:pass|passed|passes|green)\b"
    r"|\bcargo\s+(?:test|nextest)\b[^\n]*\b(?:pass|passed|passes|ok|green)\b",
    re.IGNORECASE,
)

# A named regression test.
REGRESSION_REF = re.compile(r"regression[_\s-]\w+", re.IGNORECASE)

EVIDENCE_PATTERNS = (VERIFICATION_HEADING, PASSING_RUN, REGRESSION_REF)

# Only a regression test counts as diff-side evidence. A generic `tests/`
# match let an unrelated unit-test edit satisfy the gate.
REGRESSION_TEST_FILE = re.compile(r"regression", re.IGNORECASE)

FAILURE_NOTICE = """check_close_evidence: FAIL — a closing keyword was used but no \
regression evidence found.
The PR must include one of:
  • A regression test (a file whose name contains 'regression') in the diff
  • A 'Verification:' section in the PR body reporting a passing run
  • A reference to passing regression tests
See CONTRIBUTING.md § Issue Closure Policy for the template."""


def extract_closed_issues(pr_body: str) -> list[int]:
    """Extract every issue number closed by a supported closing keyword."""
    return [int(n) for n in re.findall(CLOSING_KEYWORDS, pr_body, re.IGNORECASE)]


def body_has_evidence(pr_body: str) -> bool:
    """Return True if the PR body names a verification step or a passing run."""
    return any(pattern.search(pr_body) for pattern in EVIDENCE_PATTERNS)


def diff_has_evidence(diff_files: list[str]) -> bool:
    """Return True if the diff adds or edits a regression test file."""
    return any(REGRESSION_TEST_FILE.search(os.path.basename(path)) for path in diff_files)


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
        print("check_close_evidence: no closing keyword declarations found — pass")
        return 0

    print(f"check_close_evidence: declared closing keywords for issues {declared}")

    if has_evidence(pr_body, diff_files):
        print("check_close_evidence: regression/verification evidence found — pass")
        return 0

    print(FAILURE_NOTICE)
    return 1


if __name__ == "__main__":
    sys.exit(main())
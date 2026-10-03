#!/usr/bin/env python3
"""Check that PRs using a closing keyword include regression evidence.

Exit codes:
  0 — PR closes nothing, or evidence is present (pass)
  1 — PR closes an issue but lacks evidence markers (fail)
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

# Any of GitHub's closing keywords closes the issue on merge, so all of them
# gate. Matching only "Closes" let a PR use the template's "Fixes #N" and skip
# this check entirely.
CLOSING_KEYWORD = re.compile(r"(?:Closes|Fixes|Resolves)\s+#\d+", re.IGNORECASE)

# Evidence that the fix was verified, as one alternation so the regex engine
# does the branching instead of this file's control flow.
#
#   • a "Verification:" heading, optionally a "###" heading or bold. A trailing
#     \b only matches when a NON-word character follows, which rejected the
#     template's own "Verification:\n..." heading while accepting
#     "Verification:x" — so the boundary is spelled out here.
#   • a reported passing run. A bare "cargo test" mention is not enough;
#     "cargo test failed" must not satisfy the gate.
#   • a named regression test.
EVIDENCE = re.compile(
    r"^[ \t]*(?:#{1,6}[ \t]*)?(?:\*\*)?Verification(?:\*\*)?[ \t]*:"
    r"|\btests?\s+(?:pass|passed|passes|green)\b"
    r"|\bcargo\s+(?:test|nextest)\b[^\n]*\b(?:pass|passed|passes|ok|green)\b"
    r"|\bregression[_\s-]\w+",
    re.IGNORECASE | re.MULTILINE,
)

# Diff-side evidence. Only a regression test counts: matching any path under
# tests/ let an unrelated unit-test edit satisfy the gate.
REGRESSION_FILE_MARKER = "regression"

FAILURE_NOTICE = """check_close_evidence: FAIL — a closing keyword was used but no \
regression evidence found.
The PR must include one of:
  • A regression test (a file whose name contains 'regression') in the diff
  • A 'Verification:' section in the PR body reporting a passing run
  • A reference to passing regression tests
See CONTRIBUTING.md § Issue Closure Policy for the template."""


def read_body() -> str:
    """Return the PR body from the argv path, else from `PR_BODY`.

    A body-file argument that does not exist fails the job: a missing input
    must never read as "no PR body provided".
    """
    if len(sys.argv) <= 1:
        return os.environ.get("PR_BODY", "")
    body_arg = Path(sys.argv[1])
    if not body_arg.is_file():
        print(f"check_close_evidence: body file not found: {body_arg} — fail")
        raise SystemExit(1)
    return body_arg.read_text()


def main() -> int:
    """Entry point.

    Usage:
      check_close_evidence.py <pr_body_file> [diff_files_csv]

    Environment variables (used by GitHub Actions):
      PR_BODY — the PR body text
      PR_DIFF — comma-separated list of changed file paths in the diff
    """
    pr_body = read_body()
    diff_files = sys.argv[2] if len(sys.argv) > 2 else os.environ.get("PR_DIFF", "")
    closes_issue = bool(pr_body.strip()) and CLOSING_KEYWORD.search(pr_body) is not None
    has_evidence = EVIDENCE.search(pr_body) is not None or REGRESSION_FILE_MARKER in diff_files
    fails = closes_issue and not has_evidence
    print(FAILURE_NOTICE if fails else "check_close_evidence: pass")
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
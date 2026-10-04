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
#   • a "Verification:" heading, optionally a "###" heading or bold.
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

# Diff-side evidence. Only a regression test counts.
REGRESSION_FILE_MARKER = "regression"

FAILURE_NOTICE = """check_close_evidence: FAIL — a closing keyword was used but no \
regression evidence found.
The PR must include one of:
  • A regression test (a file whose name contains 'regression') in the diff
  • A 'Verification:' section in the PR body reporting a passing run
  • A reference to passing regression tests
See CONTRIBUTING.md § Issue Closure Policy for the template."""


def main() -> int:
    """Entry point: 0 when the gate passes, 1 when it fails."""
    argv = sys.argv[1:]
    body_arg = argv[0] if argv else None
    diff_files = (argv[1] if len(argv) > 1 else os.environ.get("PR_DIFF", "")).lower()
    if body_arg is not None and not Path(body_arg).is_file():
        print(f"check_close_evidence: body file not found: {body_arg} — fail")
        return 1
    pr_body = Path(body_arg).read_text() if body_arg is not None else os.environ.get("PR_BODY", "")
    fails = (
        bool(pr_body.strip())
        and CLOSING_KEYWORD.search(pr_body) is not None
        and EVIDENCE.search(pr_body) is None
        and REGRESSION_FILE_MARKER not in diff_files
    )
    print(FAILURE_NOTICE if fails else "check_close_evidence: pass")
    return int(fails)


if __name__ == "__main__":
    sys.exit(main())

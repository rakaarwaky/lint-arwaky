# UAT Evidence

This directory holds the replayable evidence for the persona acceptance matrix in
[`TEST.md` §4](../../TEST.md). A scenario is `Verified` only when its transcript lives here (or a CI
run URL is recorded in the same row) and the commit it names is reachable from the release tag.

## Layout

```text
docs/uat/
├── README.md
└── <version>/
    ├── UAT-001.log
    ├── UAT-002.log
    └── ...
```

## What a transcript must contain

1. Tester (name), date, OS and architecture, tool version (`lint-arwaky-cli version`).
2. The exact commands run, in order, copied from the shell.
3. The captured stdout and stderr, including exit codes (`echo "exit=$?"`).
4. The commit under test, and the CI run URL if the scenario was executed in CI.
5. For negative scenarios, proof that nothing was written (a `git status` or checksum before/after).

## Status values

| Value | Meaning |
|-------|---------|
| `Verified` | Transcript present, replayable by a second person, commit reachable from the tag |
| `Failed` | Transcript present and the stated business outcome did not hold |
| `Not run` | No evidence for this release candidate — blocks release |

`Not run` is the default. Never carry a `Verified` row forward from an earlier release without
re-running it; that is exactly how the withdrawn `8d4342a` record became untrustworthy.

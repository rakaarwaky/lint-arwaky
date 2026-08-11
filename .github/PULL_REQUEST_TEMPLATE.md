## Summary

<!-- High-level description of the change and the problem it solves. -->

## Changes

<!-- Bullet list of what changed. -->

## How to verify

<!-- Steps to test / verify. -->

## Checklist

- [ ] Conventional commit title (e.g. `feat:`, `fix:`, `refactor:`, `chore:`)
- [ ] `bash scripts/gates.sh` passes:
  - [ ] rustfmt
  - [ ] clippy (`-D warnings`)
  - [ ] self-lint (`check .` == 0 violations)
  - [ ] workspace tests
  - [ ] false negatives (workspaces-bad: >= 24 unique AES codes per language)
  - [ ] false positives (workspaces-good == 0)
- [ ] PR description describes the change (no bare file paths)
- [ ] No out-of-scope changes

## Related issues

<!-- Closes #N, Fixes #N -->

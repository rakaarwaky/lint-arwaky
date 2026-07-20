# FRD — auto-fix

## Feature Goal
The auto-fix crate provides an automatic correction mechanism for AES rule violations that can be fixed mechanically. It consumes linting results and applies deterministic corrections to affected source files, reducing the manual workload for developers when fixing errors that can be detected and corrected deterministically.

## Requirements & Scope
- In scope:
  - AES Fix: Unused Import Correction — automatically remove import lines not referenced in the file (Rust, Python, JavaScript, TypeScript).
  - AES Fix: File Naming Correction — rename files that violate the snake_case convention to the correct format (all supported languages).
  - AES Fix: Bypass Warning Correction — add or fix invalid bypass comments (noqa, type: ignore) to the correct format, or remove them along with the code fix (Python ruff/mypy, JavaScript/TypeScript eslint).
  - AES Fix: Code Format Correction — apply automatic formatting using rustfmt, prettier, or the built-in formatter (Rust, JavaScript/TypeScript).
  - Idempotent, deterministic fixes that do not break code functionality.
- Out of scope:
  - Detecting violations (handled by the rule crates).
  - Subjective or non-mechanical refactors.
  - Changing public APIs or program semantics.

## Success Indicators
- [ ] Fix accuracy — applied fixes never break code functionality.
- [ ] Coverage — the target percentage of automatically fixable violations is reached.
- [ ] Idempotency — running auto-fix repeatedly on the same file produces no further changes.
- [ ] Rule conformance — the crate itself complies with AES rules and passes linting checks when complete.

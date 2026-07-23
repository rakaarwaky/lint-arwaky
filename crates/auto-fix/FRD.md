# FRD — auto-fix

## Feature Goal

The auto-fix crate provides an automatic correction mechanism for AES rule violations that can be fixed mechanically. It consumes linting results and applies deterministic corrections to affected source files, reducing the manual workload for developers when fixing errors that can be detected and corrected deterministically.

## Requirements & Scope

- AES Fix: Unused Import Correction
  - Requirement: Automatically remove import lines that are not referenced in the file.
  - Scope: Rust, Python, JavaScript, and TypeScript.
- AES Fix: File Naming Correction
  - Requirement: Automatically rename files that violate the snake_case convention to the correct format.
  - Scope: All supported languages.
- AES Fix: Bypass Warning Correction
  - Requirement: Add or fix invalid bypass comments (such as noqa, type: ignore) to the correct format, or remove them along with the code fix.
  - Scope: Python (ruff, mypy) and JavaScript/TypeScript (eslint).
- AES Fix: Code Format Correction
  - Requirement: Apply automatic formatting using rustfmt, prettier, or the built-in formatter.
  - Scope: Rust, JavaScript/TypeScript.
- Fixes must be idempotent and deterministic, and must not break code functionality.

## Success Indicators

- [ ] Fix accuracy — applied fixes never break code functionality.
- [ ] Coverage — the target percentage of automatically fixable violations is reached.
- [ ] Idempotency — running auto-fix repeatedly on the same file produces no further changes.
- [ ] Rule conformance — the crate itself complies with AES rules and passes linting checks when complete.

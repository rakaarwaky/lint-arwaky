# Feature Requirement Document (FRD) - Auto Fix

See [ARCHITECTURE.md](../../../ARCHITECTURE.md) for layer rules and [README.md](../../../README.md) for project context.

## 1. Feature Goal

The primary purpose of the `auto-fix` module is to provide an automatic correction mechanism for AES rule violations that can be fixed mechanically. This module takes linting results and applies automatic corrections to the affected source files, reducing the manual workload for developers when fixing errors that can be detected and corrected deterministically.

## 2. Requirements & Scope

The `auto-fix` module is responsible for applying automatic corrections based on the following specifications:

### Rules Specifications

- **AES Fix: Unused Import Correction**
  - **Requirement**: Automatically remove import lines that are not referenced in the file.
  - **Scope**: Rust, Python, JavaScript, and TypeScript.

- **AES Fix: File Naming Correction**
  - **Requirement**: Automatically rename files that violate the snake_case convention to the correct format.
  - **Scope**: All supported languages.

- **AES Fix: Bypass Warning Correction**
  - **Requirement**: Add or fix invalid bypass comments (such as `noqa`, `type: ignore`) to the correct format, or remove them along with the code fix.
  - **Scope**: Python (ruff, mypy) and JavaScript/TypeScript (eslint).

- **AES Fix: Code Format Correction**
  - **Requirement**: Apply automatic formatting using rustfmt, prettier, or the built-in formatter.
  - **Scope**: Rust, JavaScript/TypeScript.

### Inputs

- A list of linting results (`Vec<LintResult>`) containing fixable violations.
- Project configuration (`ArchitectureConfig`) to determine the applicable rules.

### Outputs

- Source files that have been corrected.
- A change report containing the number of fixes applied per category.

---

## 3. Success Indicators

The success of the `auto-fix` module is measured by:

- **Fix Accuracy**: Applied fixes do not break code functionality.
- **Coverage Rules**: The percentage of automatically fixable violations reaches the target.
- **Idempotency**: Running auto-fix repeatedly on the same file does not produce additional changes.
- **Rule Conformance**: When complete, the module itself must comply with AES rules and pass linting checks.

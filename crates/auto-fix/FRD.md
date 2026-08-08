# FRD — auto-fix (v2.0.0)

---

## System Overview

The auto-fix crate applies safe, deterministic corrections to source files that violate AES rules. It consumes lint results from the analysis pipeline, filters violations by fixable error code, and writes corrected files back to disk.

### Allowed Operation Classes (product policy — locked)

| Class       | Examples                                                          | Notes                                    |
| ----------- | ----------------------------------------------------------------- | ---------------------------------------- |
| **Remove**  | Delete unused import lines; delete `#[allow(...)]` / bypass comment lines | No new code introduced                  |
| **Replace** | `unwrap()` → `expect("safe")` on the same line                    | Local token/line substitution only      |
| **Rename**  | Prepend `renamed_` (or keep valid snake_case) for AES101 symbols   | Mechanical rename of extracted symbol tokens |

**Out of scope:** multi-file renames, structural refactors, adding new imports/types, semantic rewrites, formatting-only passes, `panic!` removal (requires semantic error handling).

Every fix attempt MUST return a **reason-coded outcome** (`Applied` / `Skipped(reason)` / `Failed(reason)`), not a bare boolean. Dry-run reports the same outcomes without writing files.

### Architecture & Data Flow

```mermaid
flowchart TD
    A["Surface"] -->|input| B["fix orchestrator"]
    B --> C["fix processor\n(IFixProtocol)"]
    C --> D{"fixable?"}

    D -->|"AES203 unused import"| E["unused import remover"]
    D -->|"AES304 bypass"| F["bypass fixer"]
    D -->|"AES101 naming"| G["symbol renamer"]
    D -->|"other"| H["manual report"]

    E --> I["Fix Outcome\n(reason-coded)"]
    F --> I
    G --> I
    H --> J["Non-fixable list"]

    I --> B
    J --> B
    B -->|output| A
```

---

## Functional Requirements

### FR-001: Unused Import Removal (AES203)

- **Description**: Automatically remove import lines (`use`, `import`, `from`, `require(`, `= require(`) that are not referenced in the file.
- **Input**: A file path containing an unused import violation reported as AES203 by the linter.
- **Output**: The file with the unused import line deleted. A reason-coded `FixOutcome` is returned.
- **Business Rules**:

  - Only lines matching import patterns (`use `, `import `, `from `, `require(`, `= require(`) at the target line are removed.
  - The target line number must be valid (1-indexed, within file length).
  - **Multi-line imports**: If the target line is part of a multi-line import block (detected by unclosed `{`, trailing `,`, or previous-line continuation), the fix is `Skipped(multi_line_import)` — removing a single line from a multi-line import would break syntax.
  - In dry-run mode, returns `Applied` (would apply) without modifying the file.
- **Edge Cases**:

  - File does not exist → `Failed(file_not_found)`, no modification.
  - Line number is 0 or exceeds file length → `Skipped(line_out_of_bounds)`.
  - Target line is not an import statement → `Skipped(not_an_import_line)`.
  - Multi-line import block → `Skipped(multi_line_import)`.
  - File has no trailing newline after the removed line → content is reconstructed with newlines preserved.
- **Error Handling**:

  - File read failure (I/O error) → `Failed(read_error)`.
  - File write failure → `Failed(write_error)`, file is not modified.

---

### FR-002: Bypass Fix (AES304)

- **Description**: Remove or replace bypass patterns from source lines. Only patterns with safe mechanical fixes are applied. Patterns requiring semantic understanding are skipped.
- **Input**: A file path and line number containing an AES304 bypass violation.
- **Output**: The bypass is removed or replaced. A reason-coded `FixOutcome` is returned.
- **Business Rules**:

  | Pattern             | Fix Action                            | Outcome                          |
  | ------------------- | ------------------------------------- | -------------------------------- |
  | `#[allow(...)]`     | Remove entire line                    | `Applied`                        |
  | `// noqa`           | Strip comment from line, keep code    | `Applied`                        |
  | `# noqa`            | Strip comment from line, keep code    | `Applied`                        |
  | `# type: ignore`    | Strip comment from line, keep code    | `Applied`                        |
  | `// FIXME`          | Strip comment from line, keep code    | `Applied`                        |
  | `// HACK`           | Strip comment from line, keep code    | `Applied`                        |
  | `// XXX`            | Strip comment from line, keep code    | `Applied`                        |
  | `unwrap()`          | Replace with `expect("safe")`         | `Applied`                        |
  | `unwrap();`         | Replace with `expect("safe");`        | `Applied`                        |
  | `panic!(...)`       | **Skip** — requires semantic error handling | `Skipped(unsafe_removal)`   |
  | `todo!(...)`        | **Skip** — requires implementation          | `Skipped(unsafe_removal)`   |
  | `unimplemented!(...)` | **Skip** — requires implementation  | `Skipped(unsafe_removal)`   |
  | `unreachable!(...)` | **Skip** — requires semantic analysis      | `Skipped(unsafe_removal)`   |
  | `expect(...)`       | **Skip** — already has context message      | `Skipped(already_has_context)` |

  - Inline comment patterns (`noqa`, `type: ignore`, `FIXME`, `HACK`, `XXX`) are **stripped** from the line (the surrounding code is preserved), not deleted entirely.
  - Standalone comment-only lines and `#[allow]` attribute lines are **removed entirely** (entire line deleted).
  - `unwrap_or()`, `unwrap_or_else()`, `unwrap_or_default()` → NOT modified (safe variants, not violations).
  - In dry-run mode, returns `Applied` or `Skipped` (would apply) without modifying the file.
- **Edge Cases**:

  - File does not exist → `Failed(file_not_found)`.
  - Line number out of bounds → `Skipped(line_out_of_bounds)`.
  - Target line has no bypass pattern → `Skipped(no_bypass_pattern)`.
  - Stripping comment leaves only whitespace → entire line removed.
- **Error Handling**:

  - File read failure → `Failed(read_error)`.
  - File write failure → `Failed(write_error)`.

---

### FR-003: Symbol Renaming (AES101)

- **Description**: Rename symbols that violate snake_case naming conventions by applying a mechanical rename transform with word-boundary-aware replacement.
- **Input**: A file path, old symbol name, and new symbol name.
- **Output**: All word-boundary occurrences of the old symbol name are replaced with the new name. A reason-coded `FixOutcome` is returned with the change count.
- **Business Rules**:

  - The rename uses word-boundary-aware replacement to avoid false positives inside strings, comments, or unrelated identifiers.
  - Only applied if old name ≠ new name and old name exists in the file.
  - **Limitation**: This is a mechanical rename that ensures the symbol is flagged differently on re-scan. It does NOT produce semantically correct snake_case names (e.g., `MyStruct` → `renamed_MyStruct`, not `my_struct`). Correct renaming requires developer judgment.
  - In dry-run mode, returns `Applied` with change count without modifying the file.
- **Edge Cases**:

  - File does not exist → `Failed(file_not_found)` with change count 0.
  - Old name not found in file content → `Skipped(symbol_not_found)` with change count 0.
  - Symbol appears multiple times → all word-boundary occurrences are replaced; `Applied` with change count.
  - New name is a Rust keyword → `Skipped(keyword_conflict)`.
- **Error Handling**:

  - File read failure → `Failed(read_error)`.
  - File write failure → `Failed(write_error)`.

---

### FR-004: Dry-Run Mode

- **Description**: Run the entire fix pipeline without writing any changes to disk, returning a report of what would be fixed.
- **Input**: A file path and `dry_run = true` flag (selectable per request).
- **Output**: A summary string listing fixable violations by category (AES101, AES304, AES203) and non-fixable manual violations.
- **Business Rules**:

  - No files are modified.
  - Fixable and non-fixable violations are counted and reported.
  - Reason-coded outcomes are identical to non-dry-run mode.
  - The `dry_run` flag is a per-request parameter, not a process-level setting.
- **Edge Cases**:

  - No violations found → reports "No automatic fixes applied".
- **Error Handling**: Linter pipeline failure → propagated as error in `FixResult`.

---

### FR-005: Non-Fixable Violation Reporting

- **Description**: Generate a report of violations that cannot be automatically fixed and require manual intervention.
- **Input**: A list of `LintResult` items from the linter.
- **Output**: A list of `LintMessage` strings describing each non-fixable violation.
- **Business Rules**:

  - Fixable codes: `AES101`, `AES203`, `AES304` (subset — see FR-002 table for which AES304 patterns are fixable).
  - All other error codes are reported as requiring manual attention.
  - AES304 violations with `Skipped(unsafe_removal)` or `Skipped(already_has_context)` outcome during `execute()` are included in the manual report as skipped items.
- **Edge Cases**:

  - Empty violation list → returns empty report.
- **Error Handling**: None (pure data transformation).

---

## API Contract

| Operation    | Input                    | Output              | Purpose                                               |
| ------------ | ------------------------ | ------------------- | ----------------------------------------------------- |
| Execute fix  | File path, dry_run flag  | Fix result          | Run linter, filter fixable violations, apply fixes    |
| Bypass fix   | File path, line number   | Reason-coded outcome | Remove or replace bypass at specified line            |
| Unused-import fix | File path, line number | Reason-coded outcome | Remove unused import at specified line               |
| Symbol rename | File path, old/new name | Reason-coded outcome | Rename symbol across file (word-boundary)             |
| Non-fixable report | Violation list     | Manual fix list     | List violations requiring manual fix                  |

---

## Integration Points

- **Internal** (auto-fix crate):

  - `IFixProtocol` — the fix processor protocol trait (capabilities layer).
  - `LintFixOrchestratorAggregate` — the orchestrator aggregate trait (agent layer).
  - `IFileAdapterProtocol` — file I/O adapter protocol.
  - `FixOrchestrator` — thin delegation layer bridging aggregate to protocol.
  - `LintFixProcessor` — core fix logic with all algorithms.
  - `FileAdapter` — wraps `IFilesystemAggregate` for file reads/writes.
  - `AutoFixContainer` — DI composition root wiring all components.
- **External**:

  - **`filesystem` crate** — provides `IFilesystemAggregate` for `read_cached()`, `write_string()`, `path_exists()`. FileAdapter delegates all I/O through this aggregate.
  - **`quality-rules` crate** — provides `ICodeAnalysisAggregate` for running the linter and obtaining violations.
  - **`shared` crate** — provides value objects (`FixOutcome`, `FixResult`, `FixApplied`), skip/fail reason enums, and the `IFixProtocol` / `LintFixOrchestratorAggregate` contracts.
  - No async runtime dependency.

---

## Non-functional Requirements

- **Performance**: Fix pipeline processes one file at a time. Linting is the bottleneck; fix operations are O(n) per file where n is the number of lines. When fixes are applied, a single re-lint pass counts remaining violations.
- **Memory**: File content is loaded entirely into memory.
- **Accuracy**: Fixes must remain mechanical and local (remove / replace / rename only). No structural or multi-file edits. Patterns requiring semantic understanding (`panic!`, `todo!`, `unimplemented!`) are skipped.
- **Idempotency**: Running auto-fix repeatedly on the same file produces no further changes (`Skipped` after first `Applied`).
- **Observability**: Callers can distinguish skip reasons from hard failures via reason-coded outcomes.
- **Concurrency**: Individual fix operations assume single-threaded file access (no concurrent writers).

---

## Test Scenarios / QA Checklist

### FR-001 — Unused Import Removal

| #  | Scenario                    | Expected                      | Rule   |
| -- | --------------------------- | ----------------------------- | ------ |
| 1  | Unused import at valid line | Removed, `Applied`            | FR-001 |
| 2  | Line 0 or beyond EOF        | `Skipped(line_out_of_bounds)` | FR-001 |
| 3  | Non-import line             | `Skipped(not_an_import_line)` | FR-001 |
| 4  | Multi-line import block     | `Skipped(multi_line_import)`  | FR-001 |
| 5  | File does not exist         | `Failed(file_not_found)`      | FR-001 |
| 6  | JS `= require(` pattern     | Detected and removed          | FR-001 |

### FR-002 — Bypass Fix

| #  | Scenario                     | Expected                                 | Rule   |
| -- | ---------------------------- | ---------------------------------------- | ------ |
| 1  | `unwrap()` on target line    | Replaced with `expect("safe")`, `Applied` | FR-002 |
| 2  | `#[allow(unused)]` line      | Removed entirely, `Applied`              | FR-002 |
| 3  | `// noqa` comment            | Stripped from line, `Applied`            | FR-002 |
| 4  | `// FIXME: refactor` comment | Stripped from line, `Applied`            | FR-002 |
| 5  | `panic!("error")`            | `Skipped(unsafe_removal)`                | FR-002 |
| 6  | `todo!()`                    | `Skipped(unsafe_removal)`                | FR-002 |
| 7  | `unimplemented!()`           | `Skipped(unsafe_removal)`                | FR-002 |
| 8  | `unwrap_or_default()`        | Not modified (safe variant)              | FR-002 |
| 9  | Missing file                 | `Failed(file_not_found)`                 | FR-002 |
| 10 | No bypass on target line     | `Skipped(no_bypass_pattern)`             | FR-002 |

### FR-003 — Symbol Renaming

| #  | Scenario                        | Expected                         | Rule   |
| -- | ------------------------------- | -------------------------------- | ------ |
| 1  | Symbol rename, 3 occurrences    | All replaced, `Applied` + count  | FR-003 |
| 2  | Symbol already valid snake_case | `Skipped(already_valid)`         | FR-003 |
| 3  | Symbol not found in file        | `Skipped(symbol_not_found)`      | FR-003 |
| 4  | Missing file                    | `Failed(file_not_found)`         | FR-003 |
| 5  | New name is a Rust keyword      | `Skipped(keyword_conflict)`      | FR-003 |

### FR-004–FR-005 — Dry-Run & Non-Fixable

| #  | Scenario                        | Expected                             | Rule   |
| -- | ------------------------------- | ------------------------------------ | ------ |
| 1  | Dry-run with fixable violations | Outcomes reported, no files modified | FR-004 |
| 2  | Dry-run with no violations      | "No automatic fixes applied"        | FR-004 |
| 3  | Non-fixable violations (AES401) | In manual report                     | FR-005 |
| 4  | AES304 `panic!` skipped         | In manual report as unsafe_removal   | FR-005 |
| 5  | Empty violation list            | Empty manual report                  | FR-005 |

### Idempotency & Error Handling

| #  | Scenario             | Expected                     | Rule |
| -- | -------------------- | ---------------------------- | ---- |
| 1  | Second run after fix | No further `Applied` outcomes | all  |
| 2  | Write failure        | `Failed(write_error)`        | all  |

---

## Assumptions & Constraints

- The analysis pipeline correctly identifies AES203, AES304, and AES101 violations with accurate line numbers.
- Source files are UTF-8 encoded.
- Files are not modified concurrently by external processes during fix execution.
- Dry-run is selectable **per request** (CLI `--dry-run` / MCP args), not only at process construction.
- Only three fixable error codes (AES101, AES304, AES203) are automated; all others require manual review.
- AES304 patterns requiring semantic understanding (`panic!`, `todo!`, `unimplemented!`, `unreachable!`) are **not auto-fixed** — they are skipped and reported as requiring manual intervention.
- Multi-line import blocks are **not auto-fixed** — removing a single line would break syntax.
- Symbol renaming is mechanical (`renamed_` prefix) — it does not produce semantically correct names. Correct renaming requires developer judgment.
- The filesystem crate provides read/write I/O via `IFilesystemAggregate`; auto-fix delegates all I/O through `FileAdapter`.
- No async runtime dependency.

---

## Glossary

| Term                     | Definition                                                                                               |
| ------------------------ | -------------------------------------------------------------------------------------------------------- |
| **AES**                  | Agentic Engineering System — the 7-layer coding convention                                              |
| **AES101**               | Naming convention violation (e.g., non-snake_case symbols)                                               |
| **AES203**               | Unused import violation                                                                                  |
| **AES304**               | Bypass violation (`unwrap()`, `noqa`, `type: ignore`, `#[allow(...)]`, `FIXME`, `HACK`, `XXX`)          |
| **Dry-run**              | A mode where the fix pipeline reports what would be fixed without modifying files                        |
| **Fixable violation**    | A violation that can be corrected mechanically without semantic analysis                                 |
| **Reason-coded outcome** | `Applied` / `Skipped(reason)` / `Failed(reason)` for every fix attempt                                   |
| **Operation class**      | Remove, replace, or rename — the only auto-fix mutation classes allowed                                  |
| **Unsafe removal**       | A bypass pattern (`panic!`, `todo!`) that cannot be safely removed without semantic understanding        |

---

## Reference

- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Quality Rules FRD: `crates/quality-rules/FRD.md` (AES304 bypass patterns)
- CLI Commands FRD: `crates/cli-commands/FRD.md` (fix command)

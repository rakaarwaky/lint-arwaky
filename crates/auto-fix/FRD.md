# FRD — auto-fix

## System Overview

The auto-fix crate applies safe, deterministic corrections to source files that violate AES rules. It consumes lint results from the analysis crate, filters violations by fixable error code, and writes corrected files back to disk. Only removal operations are automated — no code is added or modified. The crate follows the AES 7-layer architecture: the fix processor (capabilities) implements the fix protocol, the fix orchestrator (agent) delegates to the protocol, and the auto-fix container (root) wires dependencies.

## Functional Requirements

### FR-001: Unused Import Removal (AES203)

- **Description**: Automatically remove import lines (`use`, `import`, `from`, `require(`) that are not referenced in the file.
- **Input**: A file path containing an unused import violation reported as AES203 by the linter.
- **Output**: The file with the unused import line deleted. A `FixApplied` event is emitted.
- **Business Rules**:
  - Only lines matching import patterns (`use `, `import `, `from `, `require(`) at the target line are removed.
  - The target line number must be valid (1-indexed, within file length).
  - In dry-run mode, returns `true` without modifying the file.
- **Edge Cases**:
  - File does not exist: returns `false`, no modification.
  - Line number is 0 or exceeds file length: returns `false`.
  - Target line is not an import statement: returns `false`.
  - File has no trailing newline after the removed line: content is reconstructed with newlines preserved.
- **Error Handling**:
  - File read failure (I/O error): returns `false`.
  - File write failure: returns `false`, file is not modified.

### FR-002: Bypass Comment Removal (AES304)

- **Description**: Remove or replace invalid bypass comments (`#[allow(...)]`, `unwrap()`, `noqa`, `type: ignore`, `panic!`) from source lines.
- **Input**: A file path and line number containing an AES304 bypass violation.
- **Output**: The bypass comment is removed or `unwrap()` is replaced with `expect("safe")`. A `FixApplied` event is emitted.
- **Business Rules**:
  - Lines starting with `#[allow(`, `//`, or `#` (attribute/comment lines) are fully removed.
  - Lines containing `unwrap()` or ending with `unwrap();` are replaced: `unwrap()` → `expect("safe")`, `unwrap();` → `expect("safe");`.
  - Other bypass patterns (`noqa`, `type: ignore`, `panic!`) trigger removal only if they match the target line.
  - In dry-run mode, returns `true` without modifying the file.
- **Edge Cases**:
  - File does not exist: returns `false`.
  - Line number out of bounds: returns `false`.
  - Target line has no bypass pattern: returns `false`.
- **Error Handling**:
  - File read failure: returns `false`.
  - File write failure: returns `false`.

### FR-003: Symbol Renaming (AES101)

- **Description**: Rename symbols that violate snake_case naming conventions by applying a simple rename transform.
- **Input**: A file path and naming violation message containing the symbol to rename.
- **Output**: All occurrences of the old symbol name are replaced with the new name. A `FixApplied` event is emitted with the change count.
- **Business Rules**:
  - The symbol name is extracted from the violation message (token containing `_` with length > 3).
  - Rename logic: if the name already contains `_` and has ≥3 parts, it is kept as-is (already snake_case). Otherwise, `renamed_` prefix is prepended.
  - Only applied if old name ≠ new name.
- **Edge Cases**:
  - File does not exist: returns 0 changes.
  - Old name not found in file content: returns 0 changes.
  - Symbol appears multiple times: all occurrences are replaced.
- **Error Handling**:
  - File read failure: returns 0.
  - File write failure: returns 0.

### FR-004: Dry-Run Mode

- **Description**: Run the entire fix pipeline without writing any changes to disk, returning a report of what would be fixed.
- **Input**: A file path and `dry_run = true` flag.
- **Output**: A summary string listing fixable violations by category (AES101, AES304, AES203) and non-fixable manual violations.
- **Business Rules**:
  - No files are modified.
  - Fixable and non-fixable violations are counted and reported.
- **Edge Cases**:
  - No violations found: reports "No automatic fixes applied".
- **Error Handling**:
  - Linter pipeline failure: propagated as error in `FixResult`.

### FR-005: Non-Fixable Violation Reporting

- **Description**: Generate a report of violations that cannot be automatically fixed and require manual intervention.
- **Input**: A list of `LintResult` items from the linter.
- **Output**: A list of `LintMessage` strings describing each non-fixable violation (AES101, AES304, AES203 are fixable; all others are not).
- **Business Rules**:
  - Only violations with codes containing `AES101`, `AES304`, or `AES203` are considered fixable.
  - All other error codes are reported as requiring manual attention.
- **Edge Cases**:
  - Empty violation list: returns empty report.
- **Error Handling**:
  - None (pure data transformation).

## API Contract

| Function                                         | Input                  | Output                        | Description                                                        |
| ------------------------------------------------ | ---------------------- | ----------------------------- | ------------------------------------------------------------------ |
| The fix processor's execute method               | file path              | fix result                    | Run linter, filter fixable violations, apply fixes, return summary |
| The fix processor's bypass comment fix method    | file path, line number | bool                          | Remove or replace bypass comment at specified line                 |
| The fix processor's unused import fix method     | file path, line number | bool                          | Remove unused import at specified line                             |
| The fix processor's non-fixable reporting method | violation list         | lint message list             | List violations requiring manual fix                               |
| The fix orchestrator's run fix method            | file path              | fix result                    | Delegate to the fix protocol's execute method                      |
| The fix orchestrator's manual report method      | violation list         | string list                   | Delegate to non-fixable reporting                                  |
| The file adapter's read file method              | file path              | optional content              | Read file content                                                  |
| The file adapter's write file method             | file path, content     | bool                          | Write content to file                                              |
| The file adapter's path existence check          | file path              | bool                          | Check if file exists                                               |
| The auto-fix container's orchestrator factory    | dry run flag           | shared orchestrator reference | Wire and return orchestrator                                       |

## Integration Points

- **Internal**:
  - The analysis crate: consumed via the analysis aggregate to run linting and obtain violations.
  - The shared crate: VOs (`FilePath`, `LintResult`, `FixResult`, `ErrorCode`, `Count`), contracts (the fix protocol, the file adapter protocol, the fix orchestrator aggregate), events (fix applied events), utilities (the file handler utility, the symbol renaming utility).
- **External**:
  - Filesystem: reads and writes source files via the file handler utility.

## Non-functional Requirements (Detailed)

- **Performance**: Fix pipeline processes one file at a time. Linting is the bottleneck; fix operations are O(n) per file where n is the number of lines.
- **Memory**: File content is loaded entirely into memory. Large files (>10MB) may cause high memory usage.
- **Accuracy**: Fixes must never break code functionality. Only deterministic, safe removals are applied.
- **Idempotency**: Running auto-fix repeatedly on the same file produces no further changes.
- **Concurrency**: The fix processor uses shared references for the linter; individual fix operations are not synchronized (single-threaded file access assumed).

## Test Scenarios / QA Checklist

- [ ] Unused import at valid line is removed and file compiles after fix.
- [ ] Unused import at line 0 or beyond EOF returns `false` without modification.
- [ ] Unused import on a non-import line returns `false`.
- [ ] Bypass comment `unwrap()` is replaced with `expect("safe")`.
- [ ] Bypass comment `#[allow(unused)]` line is removed entirely.
- [ ] Bypass comment on non-existent file returns `false`.
- [ ] Symbol rename replaces all occurrences in file.
- [ ] Symbol rename on non-existent file returns 0.
- [ ] Dry-run mode returns report without modifying any files.
- [ ] Non-fixable violations (e.g., AES401) appear in manual report.
- [ ] Empty violation list produces empty manual report.
- [ ] Running fix twice on same file produces no further changes (idempotency).
- [ ] File write failure is handled gracefully (returns `false`).

## Assumptions & Constraints

- The linter (the analysis aggregate) correctly identifies AES203, AES304, and AES101 violations with accurate line numbers.
- Source files are UTF-8 encoded.
- Files are not modified concurrently by external processes during fix execution.
- The `dry_run` flag is set at construction time and cannot be changed per invocation.
- Only the three fixable error codes (AES101, AES304, AES203) are supported; all others require manual review.

## Glossary

- **AES**: Arwaky Engineering Standards — the architecture rules enforced by lint-arwaky.
- **AES101**: Naming convention violation (e.g., non-snake_case symbols).
- **AES203**: Unused import violation.
- **AES304**: Bypass comment violation (`unwrap()`, `noqa`, `type: ignore`, `#[allow(...)]`, `panic!`).
- **Dry-run**: A mode where the fix pipeline reports what would be fixed without modifying files.
- **Fixable violation**: A violation that can be corrected mechanically without semantic analysis.
- **Fix protocol**: The capabilities-layer protocol for applying fixes.

## Reference

- PRD: [PRD.md](../../PRD.md)

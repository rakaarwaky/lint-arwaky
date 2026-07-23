# FRD — code-analysis

## System Overview

The code-analysis crate enforces general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, duplicate blocks, and bypass annotations while guaranteeing zero tolerance for warning/error suppressions.

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    CodeAnalysisOrchestrator                               │
│  (agent layer — collects files, runs checks, formats reports)            │
├──────────┬──────────┬──────────┬──────────┬──────────┬─────────────────┤
│ ArchLine │ Mandatory│ Bypass   │ Dead     │ Code     │ Cargo.toml      │
│ Checker  │ Def      │ Checker  │ Inher.   │ Dupl.    │ Bypass          │
│ AES301/  │ Checker  │ AES304   │ Checker  │ Analyzer │ Checker         │
│ AES302   │ AES303   │          │ AES303   │ AES305   │ AES304          │
├──────────┴──────────┴──────────┴──────────┴──────────┴─────────────────┤
│ Shared utilities: file reader, bypass detector, language mapper,         │
│ column index, compliance score, code duplication detector                │
│ Config: ArchitectureConfig → per-rule thresholds, forbidden patterns     │
└──────────────────────────────────────────────────────────────────────────┘
```

## Functional Requirements

### FR-001: Maximum File Line Count (AES301)

- **Description**: Source files must not exceed the maximum allowed line count to prevent bloated, unmaintainable files.
- **Input**: Source file path + content
- **Output**: AES301 diagnostic if line count exceeds maximum
- **Business Rules**:
  - Max line count is read from `LayerDefinition.code_analysis.max_lines.value`
  - Default max: 1000 lines (configurable per rule in YAML)
  - Applies to: Rust, Python, TypeScript, JavaScript source files
  - Barrel files (`mod.rs`, `__init__.py`) are skipped
  - Files in the layer's exception list are skipped
  - If no `LayerDefinition` is provided, the check is skipped
- **Edge Cases**:
  - Files with long comments or docstrings still count all lines including blank lines
  - Generated code — no special exclusion; the rule applies uniformly
  - Files at exactly max_lines — passes (comparison is strict `>`)
- **Error Handling**: Emit AES301 with actual line count and the configured maximum

### FR-002: Minimum File Line Count (AES302)

- **Description**: Source files must have minimum length to avoid empty placeholders and stub files.
- **Input**: Source file path + content
- **Output**: AES302 diagnostic if line count is below minimum
- **Business Rules**:
  - Min line count is read from `LayerDefinition.code_analysis.min_lines.value`
  - Default min: 10 lines
  - Applies to: Rust, Python, TypeScript, JavaScript source files
  - Barrel files and exception files are skipped
  - Files at exactly min_lines — passes (comparison is strict `<`)
- **Edge Cases**:
  - Config files or entry points — skipped via exception list
  - Files with only comments and no code — still counted by line number
- **Error Handling**: Emit AES302 with actual line count and the configured minimum

### FR-003: Mandatory Definitions (AES303)

- **Description**: Source files must declare at least one primary symbol (struct, enum, trait, class, interface, type) to prevent empty placeholder files.
- **Input**: Source file path + content
- **Output**: AES303 diagnostic if no definition found
- **Business Rules**:
  - Rust: `struct`, `enum`, `trait`, `type` declarations (including visibility modifiers `pub`, `pub(crate)`, etc.)
  - Python: `class` declarations
  - TypeScript/JavaScript: `class`, `interface`, `type` declarations (including `export`, `export default`, `abstract`, `declare` prefixes)
  - Files skipped: `__init__.py`, `main.py`, `py.typed`, `mod.rs`, `lib.rs`, `main.rs`, `*_constant.rs`, `*_constant.py`
  - If `mandatory_class_definition` is disabled in the layer config, skip
  - Files in exception list are skipped
  - Dead inheritance sub-check: unit structs (`struct Foo;`) without following `impl` block, empty Python classes (`class Foo: pass`), empty JS/TS classes (`class Foo {}`) are flagged as AES303
  - `#[cfg(test)]` blocks are skipped during dead inheritance scanning
- **Edge Cases**:
  - Empty `impl` blocks — not a primary symbol, does not satisfy the requirement
  - Unit structs followed by `impl` block — not flagged (intentional placeholder)
  - Tuple structs (`struct Foo(i32)`) — not flagged as unit struct
- **Error Handling**: Emit AES303 with the expected symbol types for the language

### FR-004: Bypass Detection (AES304)

- **Description**: Detects and flags any attempt to suppress warnings/errors, panic, or use unsafe fallbacks in production code.
- **Input**: Source file path + content
- **Output**: AES304 diagnostic for each bypass found (may emit multiple per file)
- **Business Rules**:
  - **Rust forbidden tokens** (word-boundary matching): `unwrap()`, `expect()`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`
  - **Rust attribute bypasses**: `#[allow(...)]`, `#[warn(...)]`, `#[deny(...)]` (any `#[<level>(...)]`)
  - **Comment bypasses** (non-word patterns): `type: ignore`, `noqa`, `@ts-ignore`, `@ts-expect-error`, `eslint-disable`, `lint-disable`, `FIXME`, `HACK`, `XXX`
  - **Safe variants NOT flagged**: `unwrap_or()`, `unwrap_or_else()`, `unwrap_or_default()` — verified by byte-level suffix parsing
  - **Language-scoped patterns**: Python `raise NotImplementedError` / `assert false`; JS/TS `throw new Error(...)` / `throw new TypeError(...)`, etc.
  - **Cargo.toml bypass**: `workspace.lints.clippy` or `lints.clippy` sections with `level = "allow"` values
  - `#[cfg(test)]` blocks and `static Lazy<Regex>` multiline initializations are skipped
  - Patterns are read from `ArchitectureConfig.code_analysis.forbidden_bypass.values` (YAML-configurable)
  - Fallback default pattern list applied if config is empty
- **Edge Cases**:
  - Bypass tokens inside string literals or char literals — not flagged (byte-level position check)
  - Nested attributes or conditional compilation — `#[cfg(test)]` blocks are fully skipped
  - Bypass tokens in comments — non-word patterns (like `FIXME`, `HACK`) ARE flagged; word tokens (like `unwrap`) are NOT flagged when in comments only
  - `unwrap()` in test code inside `#[cfg(test)]` — skipped
- **Error Handling**: Emit AES304 with the violation kind (BypassComment, UnwrapExpect, Panic, Todo, Unimplemented) and line number

### FR-005: Duplicate Code Detection (AES305)

- **Description**: Compares code blocks across all workspace files and flags files with excessive content overlap.
- **Input**: All workspace source files (pre-read entries of path + content)
- **Output**: AES305 diagnostic for files exceeding duplication threshold
- **Business Rules**:
  - Algorithm: sliding window hash-based comparison with normalized lines (trim, alphanumeric-only)
  - Window size (`min_lines`): read from AES305 rule config, default 10 lines
  - Threshold: read from AES305 rule config `duplication_threshold`, default 50%
  - A file's shared-window percentage is calculated against all other files
  - One violation per file that exceeds the threshold (not per duplicate block)
  - Ignored paths from config are excluded from scanning
  - Pre-read entries avoid double I/O (file content read once in orchestrator)
- **Edge Cases**:
  - Files shorter than `min_lines` — skipped (no windows to compare)
  - All files identical — each file gets one violation
  - Generated code or boilerplate — no special exclusion
  - Single file in workspace — no violations (no other files to compare)
- **Error Handling**: Emit AES305 with the shared percentage, total windows, and list of similar files (up to 5)

### FR-006: File Read Error Diagnostics (AES000)

- **Description**: Emit diagnostic when a file cannot be read or exceeds the maximum lintable size.
- **Input**: File path
- **Output**: AES000 or AES301 (LOW severity) diagnostic
- **Business Rules**:
  - Max file size: 2 MiB — files exceeding this emit AES301 with LOW severity and skip
  - Read errors (permissions, I/O) emit AES000 with the error reason
  - Cargo.toml read errors also emit AES000
- **Edge Cases**:
  - Binary files — may fail to read or produce garbled content
  - Symlinks to non-existent targets — read error
  - Permission-denied files — AES000 with reason
- **Error Handling**: Emit AES000/AES301 with error message; file is skipped for further checks

## Data Model / Entity Relationship

```
CodeAnalysisRuleVO {
    rule_code: String            (e.g., "AES301")
    max_lines: Option<u32>       (file too large threshold)
    min_lines: Option<u32>       (file too short / duplication window)
    threshold_pct: f64           (duplication threshold percentage)
}

AesCodeAnalysisViolation (enum)
  ├── FileTooLarge { reason }
  ├── FileTooShort { reason }
  ├── MandatoryClassDefinition { reason }
  ├── DeadInheritance { reason }
  ├── BypassComment { reason }
  ├── UnwrapExpect { reason }
  ├── Panic { reason }
  ├── Todo { reason }
  ├── Unimplemented { reason }
  └── CodeDuplication { reason: Option<LintMessage> }

Diagnostic / LintResult {
    file: FilePath
    line: u32
    code: String                 (e.g., "AES301")
    severity: Severity           (CRITICAL | HIGH | MEDIUM | LOW)
    message: LintMessage
}

Language (enum) — Python | JavaScript | TypeScript | Rust | Unknown
ViolationKind (enum) — BypassComment | UnwrapExpect | Panic | Todo | Unimplemented
```

## API Contract

| Function | Input | Output | Description |
| --- | --- | --- | --- |
| `ArchLineChecker::check_line_counts()` | file, definition, content, violations | Mutates violations | Check AES301 (max) and AES302 (min) line counts |
| `MandatoryDefinitionChecker::check_mandatory_class_definition()` | file, definition, content, violations | Mutates violations | Check AES303 — file must declare at least one primary symbol |
| `MandatoryDefinitionChecker::check_dead_inheritance()` | file, content, violations | Mutates violations | Check AES303 — detect empty unit structs, empty classes |
| `BypassChecker::check_bypass_comments()` | file, content, violations | Mutates violations | Check AES304 — detect forbidden tokens, attributes, and comment bypasses |
| `BypassChecker::check_cargo_toml()` | content, violations | Mutates violations | Check AES304 — detect Cargo.toml clippy allow bypass |
| `CodeDuplicationAnalyzer::check_file_similarity_entries()` | entries, min_lines, threshold_pct | Vec<(String, Violation)> | Check AES305 — file-level similarity analysis |
| `CodeAnalysisOrchestrator::run_all_checks()` | config, files, root_dir | Vec<LintResult> | Run all AES301–305 checks on workspace files |
| `CodeAnalysisOrchestrator::format_report()` | results, project_root | String | Format compliance report |
| `has_critical()` | results | bool | Check if any CRITICAL severity violations exist |
| `compute_score()` | results | f64 | Calculate compliance score |

## Integration Points

- **Internal**:
  - `shared::config_system` — reads `ArchitectureConfig` YAML for per-rule thresholds, forbidden bypass patterns, ignored paths
  - `shared::taxonomy_definition_vo` — `LayerDefinition` for min/max lines, mandatory class toggle, exception lists
  - `shared::code_analysis::utility_file_reader` — reads files with 2 MiB size limit
  - `shared::code_analysis::utility_bypass_detector` — word-boundary matching, string/char position checks, cfg(test) skip logic
  - `shared::code_analysis::utility_language_mapper` — detects source language from file extension
  - `shared::code_analysis::utility_code_duplication_detector` — window normalization, hash-based dedup
  - `shared::common::utility_compliance_score` — compliance score calculation
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Analyze 1000 source files in < 3 seconds (single-pass checks, hash-based duplication)
- Memory: O(n) where n = total file content across workspace; pre-read entries avoid re-reading
- Accuracy: Zero false positives for valid code; bypass detection uses word-boundary and string-position awareness to avoid false matches

## Test Scenarios / QA Checklist

| # | Input | Expected Output | Rule |
|---|-------|-----------------|------|
| 1 | File with 1500 lines, max_lines=1000 | AES301 — file exceeds max line count | AES301 |
| 2 | File with 5 lines, min_lines=10 | AES302 — file below min line count | AES302 |
| 3 | File with 50 lines, no struct/enum/trait/class/interface | AES303 — mandatory definition missing | AES303 |
| 4 | `struct Foo;` with no following `impl` block | AES303 — dead inheritance (unit struct) | AES303 |
| 5 | `struct Foo;` followed by `impl Foo { ... }` | No violation (intentional placeholder) | AES303 pass |
| 6 | `class Foo: pass` (Python) | AES303 — dead inheritance (empty class) | AES303 |
| 7 | `class Foo {}` (TypeScript) | AES303 — dead inheritance (empty class) | AES303 |
| 8 | `let x = foo.unwrap();` | AES304 — unwrap detected | AES304 |
| 9 | `let x = foo.unwrap_or(default);` | No violation (safe variant) | AES304 pass |
| 10 | `#[allow(dead_code)]` | AES304 — allow attribute bypass | AES304 |
| 11 | `// TODO: fix this later` | AES304 — bypass comment detected | AES304 |
| 12 | `unwrap()` inside `#[cfg(test)]` block | No violation (test block skipped) | AES304 pass |
| 13 | `unwrap()` inside a string literal | No violation (inside string) | AES304 pass |
| 14 | `warnings = "allow"` in Cargo.toml `[lints.clippy]` | AES304 — Cargo.toml clippy allow bypass | AES304 |
| 15 | File with 70% content shared across 2+ files | AES305 — duplication exceeds 50% threshold | AES305 |
| 16 | File with 30% content shared across 2+ files | No violation (below 50% threshold) | AES305 pass |
| 17 | Single file in workspace | No duplication violation (no files to compare) | AES305 pass |
| 18 | File exceeding 2 MiB | AES301 LOW — file skipped (exceeds lintable size) | AES000 |
| 19 | File with read permission denied | AES000 — file skipped with error reason | AES000 |
| 20 | `mod.rs` or `__init__.py` | No violation (barrel file exception) | exception |

## Assumptions & Constraints

- Rules are configurable via YAML (ArchitectureConfig); default thresholds apply when config values are absent
- File reading uses a 2 MiB size limit to prevent memory exhaustion
- Duplicate detection uses hash-based window comparison (not AST-level)
- Bypass detection is language-aware (Rust, Python, JavaScript, TypeScript each have language-specific patterns)
- The orchestrator pre-reads all files once and passes content to checkers to avoid redundant I/O
- `#[cfg(test)]` blocks are universally skipped for bypass detection (unwrap/panic is normal in tests)

## Glossary

- **AES**: Agentic Engineering System — the 7-layer architecture framework
- **Bypass**: Any attempt to suppress, ignore, or work around warnings/errors (e.g., `unwrap()`, `#[allow(...)]`, `noqa`)
- **Diagnostic**: Violation report with file location, rule code, severity, and message
- **Dead inheritance**: Empty or stub definitions (unit structs, empty classes) that provide no real implementation
- **Primary symbol**: A meaningful type declaration (struct, enum, trait, class, interface, type alias)
- **Window**: A contiguous block of N lines used for duplication comparison
- **Severity levels**: CRITICAL (bypasses), HIGH (line count, naming), MEDIUM (dead inheritance), LOW (file read errors)

## Reference

- PRD: [PRD.md](../../PRD.md)

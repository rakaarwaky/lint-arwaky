# FRD — code-analysis

## Feature Goal
The code-analysis crate enforces general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, and duplicate blocks, while guaranteeing zero tolerance for warning/error bypasses (suppression comments or crash-prone operations like unwrap/expect).

## Requirements & Scope

### AES301 — Maximum File Line Count
- **Requirement:** Files must not exceed the maximum allowed line count (configured via YAML, defaults to 500–800 lines depending on language/layer) to ensure single-responsibility cohesion.
- **Applies to:** Rust, Python, TypeScript, JavaScript
- **Default:** 1000 lines (configurable per rule)

### AES302 — Minimum File Line Count
- **Requirement:** Scanned files must have a minimum length (at least 10 lines) to avoid empty or trivial placeholder components.
- **Applies to:** Rust, Python, TypeScript, JavaScript
- **Default:** `min_lines: 10` (resolved from CON-01 — FRD intent was 10, not 0)

### AES303 — Mandatory Definitions and Empty Traits
- **Requirement 1:** Source files must declare at least one primary symbol (struct, enum, trait, type, class, interface) to prevent dead files.
- **Requirement 2:** Enforces that traits/structures are not empty placeholders (no dead inheritance or empty impl blocks).
- **Primary symbols by language:**
  | Language   | Primary symbols                                                |
  |------------|---------------------------------------------------------------|
  | Rust       | `struct`, `enum`, `trait`, `type`                             |
  | Python     | `class`, `def`, `async def`                                   |
  | TypeScript | `class`, `interface`, `type`, `enum`, `function`              |
  | JavaScript | `class`, `function`, `async function`                         |
- **Empty definition detection:**
  - Rust: unit structs (`struct Foo;`) are flagged
  - JS/TS: empty class/interface bodies (`class Foo {}`) are flagged
  - Detected via word-boundary token matching (no substring FP)

### AES304 — Bypass Detection and Panic Avoidance
- **Requirement 1:** Detects and flags any attempt to suppress warnings or errors using comment bypasses (such as `noqa`, `type: ignore`, `eslint-disable`, `eslint-disable-next-line`).
- **Requirement 2:** Flags compiler/runtime bypasses like `#[allow(...)]` or `#[warn(...)]` (7 prefix variants including inner attributes and clippy-specific).
- **Requirement 3:** Flags fatal operations that bypass clean error handling and can crash the application: `.unwrap()`, `.unwrap_err()`, `.expect()`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`.
- **Safe unwrap variants NOT flagged:** `.unwrap_or()`, `.unwrap_or_else()`, `.unwrap_or_default()` (detected via position-aware byte scanning)
- **Default forbidden_bypass patterns:**
  ```
  unwrap, expect, panic, todo, unimplemented, unreachable,
  noqa, type: ignore, @ts-ignore, @ts-expect-error,
  eslint-disable, eslint-disable-next-line
  ```
- **Inner attribute prefixes detected (7):**
  `#[allow(`, `#[expect(`, `#[warn(`, `#![allow(`, `#![expect(`, `#![warn(`, `#[clippy::allow(`

### AES305 — Duplicate Code Block Detection
- **Requirement:** Compares code blocks across files and flags identical/highly similar code segments exceeding a configurable token/line threshold to enforce DRY (Don't Repeat Yourself) principles.
- **Algorithm:** Window-based hashing with normalized lines (whitespace-agnostic). Uses per-file hash cache for O(n) second pass.
- **Defaults:** `min_dup_lines: 5`, `threshold_pct: 50.0` (resolved from CON-04 — keep window percentage, no AST dependency)

### AES000 — File Read Error/Skip Diagnostics
- **Requirement:** When a file cannot be read or exceeds the size limit, emit a diagnostic (AES000) instead of silently skipping.
- **Max file size:** 2 MiB (enforced via `read_lintable_file` utility)

## Architecture Decisions

| ID | Decision | Rationale | Status |
|----|----------|-----------|--------|
| CON-01 | AES302 default = 10 (not 0) | Match FRD intent | ✅ Applied in `CodeAnalysisRuleVO::default()` |
| CON-04 | AES305 keep window-% algorithm | Fast, no new deps, catches ~80% of duplication | ✅ Applied |
| CON-05 | Clippy self-violation: fix code, no exception | The linter must pass its own rules | ✅ Applied |

## Traceability Matrix

| FRD Req | Rule Code | Module(s) | Verifier |
|---------|-----------|-----------|----------|
| Max line count | AES301 | `mandatory_definition_checker.rs` | `check_max_line_count()` |
| Min line count | AES302 | `mandatory_definition_checker.rs` | `check_min_line_count()` |
| Mandatory definitions | AES303 | `mandatory_definition_checker.rs`, `utility_mandatory.rs` | `check_mandatory_definitions()` |
| Bypass detection | AES304 | `bypass_checker.rs`, `utility_bypass.rs` | `check_forbidden_bypass()` |
| Duplicate blocks | AES305 | `duplication_analyzer.rs`, `utility_duplication.rs` | `handle_duplicates()` |
| File read errors | AES000 | `orchestrator.rs`, `utility_file_reader.rs` | `read_lintable_file()` -> AES000 diagnostic |
| Column tracking | — | `utility_column.rs`, `taxonomy_result_vo.rs` | `compute_column()`, `byte_offset_to_column()` |
| Global container DI | — | `orchestrator.rs`, `root_*_container.rs` | `new_with_config()` / `new_with_container()` |

## Success Indicators
- [x] Prevention of suppression hacks — absolute blockage of sneaky bypass comments (`type: ignore`, `noqa`) and raw panic/unwraps, forcing clean error propagation.
- [x] Code size discipline — strict enforcement of LOC limits with configurable rules (AES301, AES302).
- [x] DRY codebase — high-performance window-based duplication detection using per-file hash caching (AES305).
- [x] Granular location info — reporting of line and column numbers for unwraps, panics, and duplicate lines.
- [x] Compliance — workspace self-check passes (`cargo test --workspace`).
- [x] Clippy clean — zero warnings on code-analysis crate.
- [x] Architecture compliance — no agent-layer I/O (delegated to `utility_file_reader`), no utility-layer I/O, no global mutable state.

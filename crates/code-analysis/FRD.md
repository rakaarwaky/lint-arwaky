# Feature Requirement Document (FRD) - Code Analysis

See [RULES_AES.md](../../.agents/rules/RULES_AES.md) for AES301-AES305 details and [ARCHITECTURE.md](../../../ARCHITECTURE.md) for layer rules.

## 1. Feature Goal

The goal of the `code-analysis` module is to enforce general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, and duplicate blocks, while guaranteeing zero tolerance for warning/error bypasses (such as compiler suppression comments or crash-prone operations like unwrap/expect).

## 2. Requirements & Scope

The `code-analysis` module audits source code metrics and constructs checks based on the following specifications:

### Rules Specifications

- **AES301: Maximum File Line Count**
  - **Requirement**: Files must not exceed the maximum allowed line count (configured via YAML, defaults to 500-800 lines depending on language/layer) to ensure single-responsibility cohesion.

- **AES302: Minimum File Line Count**
  - **Requirement**: Scanned files must have a minimum length (e.g., at least 10 lines) to avoid empty or trivial placeholder components.

- **AES303: Mandatory Definitions and Empty Traits**
  - **Requirement 1**: Source files must declare at least one primary symbol (e.g., struct, enum, class, or interface/trait) to prevent dead files.
  - **Requirement 2**: Enforces that traits/structures are not empty placeholders (no dead inheritance or empty impl blocks).

- **AES304: Bypass Detection and Panic Avoidance**
  - **Requirement 1**: Detects and flags any attempt to suppress warnings or errors using comment bypasses (such as `noqa`, `type: ignore`, `eslint-disable`, `eslint-disable-next-line`).
  - **Requirement 2**: Flags compiler/runtime bypasses like `#[allow(...)]` or `#[warn(...)]`.
  - **Requirement 3**: Flags fatal operations that bypass clean error handling and can crash the application, such as `.unwrap()`, `.expect()`, and `panic!`.

- **AES305: Duplicate Code Block Detection**
  - **Requirement**: Compares code blocks across files and flags identical/highly similar code segments exceeding a configurable token/line threshold to enforce DRY (Don't Repeat Yourself) principles.

---

## 3. Success Indicators

The success of the `code-analysis` module is measured by:

- **Prevention of Suppression Hacks**: Absolute blockage of sneaky bypass comments (`type: ignore`, `noqa`) and raw panic/unwraps, forcing clean error propagation.
- **Code Size Discipline**: Strict enforcement of LOC (Lines of Code) limits with configurable rules.
- **DRY Codebase**: High-performance detection of structural duplication without lagging execution speeds.
- **Granular Location Info**: Reporting of line and column numbers for unwraps, panics, and duplicate lines.
- **Compliance**: Full verification of the workspace's self-check results.

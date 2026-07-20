# FRD — code-analysis

## Feature Goal
The code-analysis crate enforces general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, and duplicate blocks, while guaranteeing zero tolerance for warning/error bypasses (suppression comments or crash-prone operations like unwrap/expect).

## Requirements & Scope
- AES301 Maximum File Line Count
  - Requirement: Files must not exceed the maximum allowed line count (configured via YAML, defaults to 500–800 lines depending on language/layer) to ensure single-responsibility cohesion.
- AES302 Minimum File Line Count
  - Requirement: Scanned files must have a minimum length (e.g., at least 10 lines) to avoid empty or trivial placeholder components.
- AES303 Mandatory Definitions and Empty Traits
  - Requirement 1: Source files must declare at least one primary symbol (e.g., struct, enum, class, or interface/trait) to prevent dead files.
  - Requirement 2: Enforces that traits/structures are not empty placeholders (no dead inheritance or empty impl blocks).
- AES304 Bypass Detection and Panic Avoidance
  - Requirement 1: Detects and flags any attempt to suppress warnings or errors using comment bypasses (such as noqa, type: ignore, eslint-disable, eslint-disable-next-line).
  - Requirement 2: Flags compiler/runtime bypasses like #[allow(...)] or #[warn(...)].
  - Requirement 3: Flags fatal operations that bypass clean error handling and can crash the application, such as .unwrap(), .expect(), and panic!.
- AES305 Duplicate Code Block Detection
  - Requirement: Compares code blocks across files and flags identical/highly similar code segments exceeding a configurable token/line threshold to enforce DRY (Don't Repeat Yourself) principles.

## Success Indicators
- [ ] Prevention of suppression hacks — absolute blockage of sneaky bypass comments (type: ignore, noqa) and raw panic/unwraps, forcing clean error propagation.
- [ ] Code size discipline — strict enforcement of LOC (Lines of Code) limits with configurable rules.
- [ ] DRY codebase — high-performance detection of structural duplication without lagging execution speeds.
- [ ] Granular location info — reporting of line and column numbers for unwraps, panics, and duplicate lines.
- [ ] Compliance — when complete, the workspace self-check passes fully.

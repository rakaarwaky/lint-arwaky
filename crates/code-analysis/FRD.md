# FRD — code-analysis

## Feature Goal
The code-analysis crate enforces general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, and duplicate blocks, while guaranteeing zero tolerance for warning/error bypasses (suppression comments or crash-prone operations like unwrap/expect).

## Requirements & Scope
- AES301 Maximum File Line Count — files must not exceed the configured maximum (YAML, defaults 500–800 lines by language/layer) to ensure single-responsibility cohesion.
- AES302 Minimum File Line Count — scanned files must meet a minimum length (e.g. at least 10 lines) to avoid empty or trivial placeholder components.
- AES303 Mandatory Definitions and Empty Traits — each source file declares at least one primary symbol (struct/enum/class/interface); traits/structs are not empty placeholders.
- AES304 Bypass Detection and Panic Avoidance — flag suppression comments (noqa, type: ignore, eslint-disable) and compiler/runtime bypasses (#[allow], #[warn], .unwrap(), .expect(), panic!).
- AES305 Duplicate Code Block Detection — flag identical/highly similar code blocks across files above a configurable token/line threshold (DRY).

## Success Indicators
- [ ] Prevention of suppression hacks — absolute blockage of bypass comments and raw panics/unwraps, forcing clean error propagation.
- [ ] Code size discipline — strict, configurable enforcement of LOC limits.
- [ ] DRY codebase — high-performance duplication detection without lagging execution.
- [ ] Granular location info — line and column reporting for unwraps, panics, and duplicate lines.
- [ ] Compliance — the workspace self-check passes fully when complete.

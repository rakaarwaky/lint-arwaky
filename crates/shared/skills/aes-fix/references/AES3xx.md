# AES3xx — Quality Checks

## What the rule checks

Quality checks live in the `quality-rules` crate. In the codebase these checks emit **AES301–AES305** (the `xx` in this file's name is the family bucket; the actual rule codes are AES301–AES305):

- **AES301** — File too large (exceeds `code_analysis.max_lines`).
- **AES302** — File too short (below `code_analysis.min_lines`).
- **AES303** — Mandatory definition: a file must declare a struct/enum/trait/class/interface/type (scoped out of `utility` and `surface` layers).
- **AES304** — Bypass: bypass annotations, panics, and fallback calls (e.g. `unwrap_or_default`, `""`, `.unwrap()`).
- **AES305** — Code duplication: excessive duplicated non-import windows across files.

Checkers: `capabilities_line_checker` (AES301/AES302), `capabilities_mandatory_definition_checker` (AES303), `capabilities_check_bypass_checker` (AES304), `capabilities_code_duplication_analyzer` (AES305), orchestrated by `agent_quality_orchestrator` via `root_quality_rules_container`.

## Common violation causes

- A file exceeds the configured maximum line count (AES301).
- A file has fewer lines than the configured minimum (AES302).
- A file holds only functions with no type definition (AES303).
- A file silently discards errors with `unwrap_or_default`, `.unwrap()`, `""`, or panic annotations (AES304).
- Two or more files share large non-import windows of duplicated code (AES305).

## How to fix (concrete steps)

1. Identify the violated rule code (AES301–AES305) from the scan report.
2. For AES301: split the file into smaller, focused modules; move helpers to `*_utility`.
3. For AES302: expand the component or merge it into a related module so it meets the minimum.
4. For AES303: group the functions into a struct/class or implement an interface/trait; utility and surface files are exempt (no definition required).
5. For AES304: replace silent discards with proper error propagation; wrap errors into taxonomy VOs or return `Result`.
6. For AES305: extract the shared logic into a single utility module and import it from both locations.
7. Re-run `lint-arwaky-cli scan .` and confirm the quality finding is gone.

## Example

Before (AES304 — silently discarded error):
```rust
let _ = checker.check(file).unwrap_or_default();
```

After:
```rust
let report = checker.check(file)?;
return ResponseVo::Accepted(report.into());
```

## Verify

```bash
lint-arwaky-cli scan .
# Confirm no AES301–AES305 finding remains for the affected file.
```

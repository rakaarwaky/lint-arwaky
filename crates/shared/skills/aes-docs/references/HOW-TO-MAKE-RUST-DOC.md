# HOW TO MAKE RUST DOC COMMENTS

> **Purpose**: Document every public Rust item so `cargo doc` readers
> know *what* and *why* without opening the body.
>
> **Audience**: Engineers and agents reading rustdoc output.
>
> **Scope**: Module, struct/enum/trait, function, and method docs on
> the crate's public surface.
>
> **Location**: On the item itself (`///` or `/** … */` immediately
> above the `pub` item).
>
> **Length**: Summary line; `# Arguments` / `# Returns` / `# Errors`
> / `# Example` only when the signature does not already say it.

---

## Rules

Six rules. Each one prevents a specific failure mode.

1. **Convert `//` to `///`.** Plain comments never appear in rustdoc;
 a public item documented only with `//` is undocumented.
2. **Doc comments explain *what* and *why*, never *how*.** The body
 is the *how*; a second copy goes stale.
3. **Summary line first.** One sentence before any `#` section, so
 list views stay useful.
4. **Explain *why* for logic over 10 lines**, next to the branch or
 invariant — not a step-by-step restatement of the lines.
5. **`# Example` for non-obvious usage.** Examples must compile
 (`cargo test --doc`). Prefer a short doctest over prose.
6. **Types on every parameter and return in the prose only when the
 signature is unclear.** Prefer the signature; document units,
 edge cases, and failure modes.

---

## Workflow

1. **Create file** — module docstring at top of source file.
2. **Write Purpose** — one-line module job.
3. **Write Audience** — who reads this.
4. **Write Scope** — what the module covers.
5. **Verify** → `aa check docs` passes; doc present.

## Template

Copy, fill, delete nothing.

```rust
/// Orchestrates <name-feature>.
///
/// Execution order:
/// 1. Load rules  2. Scan paths  3. Report violations  4. Apply fixes
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
}

/// Check whether *path* violates this rule.
///
/// # Arguments
///
/// * `path` - File path to check
///
/// # Returns
///
/// `true` if the path matches the rule
///
/// # Errors
///
/// Returns `Err` if `path` is empty
///
/// # Example
///
/// ```
/// let rule = ImportRuleVO::new("*.test.ts", "Test file");
/// assert!(rule.check("foo.test.ts"));
/// ```
pub fn check(&self, path: &str) -> Result<bool, Error> {
    // ...
}
```

---

## Section Contract

Every public item is required to carry the rows that apply. Each
exists for one reason.

| Section                                            | Why it belongs here                                                            |
| -------------------------------------------------- | ------------------------------------------------------------------------------ |
| Module doc (`//!` or `//!` file header) (required) | Orients the reader before any type.                                            |
| Item summary (required)                            | One sentence: what the item is or does. Watch for restating the identifier.    |
| Structure notes (rec)                              | Execution order or invariants for types that orchestrate. Skip for plain data. |
| `# Arguments` (rec)                                | Documents parameters whose meaning is not in the type.                         |
| `# Returns` / `# Errors` (rec)                     | The contract callers depend on. Required when the fn returns `Result`.         |
| `# Example` (rec)                                  | Proves the API is usable as documented. Skip only for trivial accessors.       |

---

## Verify

```bash
cargo doc --no-deps
cargo test --doc
# Checks: rustdoc builds; doctests compile; no public /// missing on touched pub items.
# Manual: no "how" narration; summary first; no status words.
```

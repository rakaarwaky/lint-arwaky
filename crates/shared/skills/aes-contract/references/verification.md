# Verification

## Machine checks

Primary gate:

```bash
lint-arwaky-cli scan <contract-dir>   # AES101/102, AES201–205, AES402, role → must be 0
```

Fallback gate only — language compile:

- Python: `python -c "import <shared_package>.contract_<concept>_<suffix>"`
- Rust: `cargo check -p <crate-name>`
- TypeScript: `npx tsc --noEmit`

A pass means naming, layer imports, primitives, and roles are clean. Per-seam protocol shape (one file per feature, one trait per seam, many methods per trait) and aggregate cardinality (one `execute()` only) are **manual** — the linter does not count methods.

## Human checks

A machine pass does not mean the contract is right. Per-seam protocol shape and aggregate cardinality still need a reader (HOW-TO § Rules).

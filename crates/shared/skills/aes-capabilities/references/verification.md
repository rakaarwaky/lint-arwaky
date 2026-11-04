# Verification

## Primary gate

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
```

A pass means naming, imports, primitives, and roles are clean.

## Fallback gate only

Language compile — run only when the HOW-TO lists it:

- Python: `python -c "import <module>"`
- Rust: `cargo check -p <crate-name>`
- TypeScript: `npx tsc --noEmit`

## Human checks

- Layer purpose and structural order (3-block) still need a reader — see HOW-TO § Rules.
- "Only what this layer may do" is not fully machine-checked.

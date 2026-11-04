# Verification

## Machine checks

Primary gate:

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
```

## Fallback gate only — language compile

- Python: `python -c "import <package>"`
- Rust: `cargo check -p <crate-name>`
- TypeScript: `npx tsc --noEmit`

A pass means naming, imports, primitives, and roles are clean. Structural judgement (suffix choice, construction rules, "pure literals only") is **manual** — see HOW-TO § Rules.

## Human checks

A machine pass does not mean the file is right. Layer purpose, structural order, and "only what this layer may do" still need a reader (HOW-TO § Rules).

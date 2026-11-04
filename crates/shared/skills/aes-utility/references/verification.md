# Verification

## Machine checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
```

A pass means naming, imports, primitives, and roles are clean.

## Fallback gate only

- Language compile (`python -c "import ..."`, `cargo check`, `npx tsc --noEmit`) — fallback only when the linter is unavailable.

## Human checks

Structural judgement (tier choice, block order, helper-vs-utility, "orchestration only") is manual — read the language HOW-TO § Rules. A machine pass does not mean the file is right; layer purpose and "only what this layer may do" still need a reader.

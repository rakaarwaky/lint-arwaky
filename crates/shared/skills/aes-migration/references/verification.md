# Verification

## Machine checks

```bash
# Phase 0 baseline
lint-arwaky-cli scan .

# Per-phase (replace <dir> with the layer directory)
lint-arwaky-cli scan <taxonomy-dir>
lint-arwaky-cli scan <contract-dir>
lint-arwaky-cli scan <utility-dir>
lint-arwaky-cli scan <capabilities-dir>
lint-arwaky-cli scan <agent-dir>
lint-arwaky-cli scan <surface-dir>
lint-arwaky-cli scan <root-dir>

# Final gate
lint-arwaky-cli scan .   # must be 0
```

## Fallback gate only — language compile

- Python: `python -m compileall -q <modules>`
- Rust: `cargo check --workspace`
- TypeScript: `npx tsc --noEmit`

A pass means naming, layer imports, primitives, and roles are clean. Structural judgement (3-block order, orchestration purity, DI wiring) is **manual** — see the language playbook § Rules.

## Human checks

A machine pass does not mean the migration is right. Per-seam protocol shape (one file per feature, one trait per seam, many methods per trait) and aggregate cardinality (exactly one `execute()`) still need a reader (layer HOW-TOs).

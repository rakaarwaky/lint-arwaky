# Verification

## Machine checks

Primary AES gate — must exit 0 with no findings:

```bash
lint-arwaky-cli scan <target-path> --format json
echo "exit=$?"   # 0 pass · 1 violations · 2 config/parse error
```

Family isolation while iterating:

```bash
lint-arwaky-cli import <path> --filter AES201
lint-arwaky-cli role <path> --filter AES403
lint-arwaky-cli orphan <path> --format json
```

## Fallback gate only — language verify pipeline

- Python: `python -m py_compile` / Ruff / Mypy
- Rust: `cargo check` / clippy / nextest
- TypeScript: `npx tsc --noEmit` / ESLint / vitest

## Human checks

- [ ] Pre-flight build/import/typecheck passed **before** trusting the scan.
- [ ] CRITICAL (AES201/205/304) fixed before cosmetics.
- [ ] Every fix applied from the routing HOW-TO row (or auto-fix set), not a hand-rolled edit.
- [ ] Barrel/`mod.rs`/`index.ts` updated after every AES101 rename.
- [ ] Re-scan reports **0** AES violations (exit 0).
- [ ] Language verify pipeline also green.

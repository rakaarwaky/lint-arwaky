# Pre-flight checklist

- [ ] `lint-arwaky-cli scan .` baseline captured.
- [ ] Language playbook loaded ([python](python-migration.md) / [rust](rust-migration.md) / [typescript](typescript-migration.md)).
- [ ] Each layer skill consulted: `aes-taxonomy`, `aes-contract`, `aes-utility`, `aes-capabilities`, `aes-agent`, `aes-surface`, `aes-root`.
- [ ] Every touched layer's `Verify` block executed.
- [ ] Protocol = one file per feature, one trait per seam, each rich; aggregate = exactly one `execute()` method (manual).
- [ ] Files registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the playbook lists it.

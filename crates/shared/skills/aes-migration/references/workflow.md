# Workflow

1. **Load the language playbook**: Python → [references/python-migration.md](python-migration.md), Rust → [references/rust-migration.md](rust-migration.md), TypeScript → [references/typescript-migration.md](typescript-migration.md).
2. **Phase 0 — Audit**: Run `lint-arwaky-cli scan .`; categorize violations by layer; choose strategy.
3. **Phase 1 — Taxonomy**: Extract VOs, errors, constants; register in shared barrel.
4. **Phase 2 — Contract**: Create `I<Concept>Protocol` (one file per feature, one trait per capability seam) and `I<Concept>Aggregate` (exactly one `execute()` method); register.
5. **Phase 3 — Utility**: Extract stateless helpers; move to shared; register.
6. **Phase 4 — Capabilities**: Implement protocols; follow 3-block structure; verify AES403.
7. **Phase 5 — Agent**: Implement aggregates; inject via constructors; verify AES405.
8. **Phase 6 — Surface**: Delegate to agent aggregate; verify AES201 purpose (call, not impl).
9. **Phase 7 — Root**: Wire containers; bootstrap entry; register in barrel/mod/index.
10. **Phase 8 — Verify**: Full scan → 0 violations; language compile clean; tests green.

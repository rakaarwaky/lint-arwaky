# Workflow

1. **Create file** — add the module-level doc comment at the top of the source file.
2. **Write Purpose** — one line stating the module's job.
3. **Write Audience** — who reads this (engineer, agent, downstream consumer).
4. **Write Scope** — what the module covers and does not own.
5. **Verify** — `lint-arwaky-cli docs .` passes; the doc is present; language doc build is clean:
   - Rust: `cargo doc --no-deps` and `cargo test --doc`
   - Python: `python -c "import <package>"`
   - TypeScript: `tsc --noEmit`

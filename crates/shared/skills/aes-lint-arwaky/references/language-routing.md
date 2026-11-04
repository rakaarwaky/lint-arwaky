# Language routing

Read the HOW-TO for the active language before fixing. Do not restate flags, subcommands, or the fix table here — they live in the HOW-TOs.

| Language | HOW-TO | Use when |
| -------- | ------ | -------- |
| All (shared CLI, MCP, exit codes, AES101 naming) | [references/lint-commands.md](lint-commands.md) | Invoking scan/fix/ci, aliases, flags, MCP tools, layer matrix |
| Python | [references/lint-python.md](lint-python.md) | `modules/`, `py_compile` / Ruff / Mypy verify, `__init__.py` barrel |
| Rust | [references/lint-rust.md](lint-rust.md) | `crates/`, `cargo check` / clippy / nextest verify, `mod.rs` barrel |
| TypeScript | [references/lint-typescript.md](lint-typescript.md) | `packages/`, `tsc` / ESLint / vitest verify, `index.ts` barrel |
| Fix routing (every AES code) | [references/lint-routing.md](lint-routing.md) | Mapping a finding to its fix + owning `aes-*` skill |

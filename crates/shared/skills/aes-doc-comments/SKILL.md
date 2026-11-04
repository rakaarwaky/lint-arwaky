---
name: aes-doc-comments
description: Adds docstrings, doc comments, and language-native API docs for public items. Use when documenting Python, Rust, or TypeScript public surface.
when-to-use: "Use when adding or auditing docstrings, doc comments, rustdoc, JSDoc/TSDoc, or type-hint doc blocks on a Python, Rust, or TypeScript public surface."
argument-hint: "<layer-path>"
---

# aes-doc-comments

Ensure every public code item carries doc comments in its language's native form, so `cargo doc`, `python -c "import <package>"`, and `tsc` readers know *what* and *why* without opening the body. The **language** decides the comment syntax and section layout.

## Global rules

- **Module-level docstring/comment is mandatory**: first statement of every public module; one sentence on the module's job; no changelog.
- **Doc comments explain *what* and *why*, never *how***: restating the next three lines of code wastes the reader's time.
- **Never restate the signature in prose**: parameter names match the signature; do not repeat types already in the annotation when the annotation says it better.
- **Public classes and functions always get doc comments**; private helpers (`_name` in Python, non-`pub` items in Rust) may omit them when the name is enough.
- **`Args` / `Returns` on every public function** whose parameters or return value are not obvious from the type hints alone.
- **No status claims**: doc comments never say implemented / shipped / N% done — that lives in `BACKLOG.md` (`status-in-spec`).
- **Register**: module is reachable through the shared barrel (`__init__.py` / `mod.rs` / `index.ts`).
- **Verify**: `lint-arwaky-cli docs .` passes; language doc build is fallback only.
- **Related**: aes-docs (spec/status docs), aes-taxonomy (domain VOs), aes-contract (protocol declarations) — each carries its own HOW-TO.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-doc.md](references/python-doc.md) | Python PEP 257 docstrings — rules, template, section contract, verify |
| Language HOW-TOs | [references/rust-doc.md](references/rust-doc.md) | Rust `///` rustdoc sections — rules, template, doctest policy, verify |
| Language HOW-TOs | [references/typescript-doc.md](references/typescript-doc.md) | TypeScript JSDoc/TSDoc — rules, template, section contract, verify |
| Workflow | [references/workflow.md](references/workflow.md) | Step-by-step: create module doc, fill sections, verify |
| Templates | [references/templates.md](references/templates.md) | Copy source for each language's doc comment shape |
| Verification | [references/verification.md](references/verification.md) | Primary gate + fallback language doc build |

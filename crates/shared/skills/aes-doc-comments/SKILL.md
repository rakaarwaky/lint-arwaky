---
name: aes-doc-comments
description: "Adds docstrings, doc comments, and language-native API docs for public items. Use when documenting Python, Rust, or TypeScript public surface."
when-to-use: "add, audit, or fix doc comments / docstrings / rustdoc / JSDoc on public modules, classes, traits, interfaces, functions, or methods; verify doc comments with lint-arwaky-cli docs."
argument-hint: '<python | rust | typescript> [file path]'
metadata:
  tags:
    - python
    - rust
    - typescript
    - docs
    - docstring
    - doc-comments
    - jsdoc
    - rustdoc
    - pep257
  related_skills:
    - aes-docs
    - aes-taxonomy
    - aes-contract
---

# aes-doc-comments

Add doc comments in each language's native form — PEP 257 docstrings, `///` rustdoc sections, JSDoc/TSDoc — to every public module, class, trait, interface, function, and method, so `cargo doc`, `python -c "import <package>"`, and `tsc` readers know *what* and *why* without opening the body. The audience is the engineer or agent reading generated API docs; the **language** decides the comment syntax and section layout. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Module-level doc comment is mandatory: first statement of every public module; one sentence on the module's job; no changelog.
- Doc comments explain *what* and *why*, never *how* — restating the next three lines of code wastes the reader's time.
- Never restate the signature in prose: parameter names match the signature; types already in the annotation stay in the annotation.
- Public classes and functions always get doc comments; private helpers (`_name` in Python, non-`pub` items in Rust) may omit them when the name is enough.
- `Args` / `Returns` on every public function whose parameters or return value are not obvious from the type hints alone.
- No status claims: doc comments never say implemented / shipped / N% done — that lives in `BACKLOG.md` (status-in-spec).
- Verify: `lint-arwaky-cli docs .` passes; language doc build clean — `cargo doc --no-deps` and `cargo test --doc` for Rust, `python -c "import <package>"` for Python, `tsc` for TypeScript.
- One fact, one home: the language HOW-TO carries the syntax and section layout — read it, do not restate here. Cite the gate, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-doc.md](references/python-doc.md) | Python docstring HOW-TO — PEP 257 rules, template, section contract, verify |
| [references/rust-doc.md](references/rust-doc.md) | Rust rustdoc HOW-TO — `///` rules, sections, doctest policy, verify |
| [references/typescript-doc.md](references/typescript-doc.md) | TypeScript JSDoc/TSDoc HOW-TO — rules, template, section contract, verify |
| Related skills | aes-docs, aes-taxonomy, aes-contract |

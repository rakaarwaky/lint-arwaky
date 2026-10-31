---
name: aes-doc-comments
description: Adds docstrings, doc comments, and language-native API docs for public items. Use when documenting Python, Rust, or TypeScript public surface.
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
  triggers:
    - add docstring
    - add doc comments
    - document public api
    - add rustdoc
    - add jsdoc
    - add type hints
    - pep257
---
# aes-doc-comments

> **Purpose**: Ensure every public code item carries doc comments in its language's native form, so `cargo doc`, `python -c "import <package>"`, and `tsc` readers know *what* and *why* without opening the body.
> **Audience**: Engineers and agents reading generated API docs.
> **Scope**: Python, Rust, and TypeScript modules; module/class/struct/enum/trait/function/method doc comments on the public surface.

**Code surface (doc comments)** — Rules / Template / Section Contract / Verify:

| Language   | Focus                          | Template                                                           |
| ---------- | ------------------------------ | ------------------------------------------------------------------ |
| Python     | PEP 257 docstrings             | [references/HOW-TO-MAKE-PYTHON-DOC.md](references/HOW-TO-MAKE-PYTHON-DOC.md) |
| Rust       | `///` + rustdoc sections       | [references/HOW-TO-MAKE-RUST-DOC.md](references/HOW-TO-MAKE-RUST-DOC.md) |
| TypeScript | JSDoc / TSDoc                  | [references/HOW-TO-MAKE-TYPESCRIPT-DOC.md](references/HOW-TO-MAKE-TYPESCRIPT-DOC.md) |

## Rules

Six rules. Each one prevents a specific failure mode.

1. **Module-level docstring/comment is mandatory.** First statement of every public module. One sentence on the module's job; no changelog.
2. **Doc comments explain *what* and *why*, never *how*.** Restating the next three lines of code wastes the reader's time.
3. **Never restate the signature in prose.** Parameter names match the signature; do not repeat types already in the annotation when the annotation says it better.
4. **Public classes and functions always get doc comments.** Private helpers (`_name` in Python, non-`pub` items in Rust) may omit them when the name is enough.
5. **`Args` / `Returns` on every public function** whose parameters or return value are not obvious from the type hints alone.
6. **No status claims.** Doc comments never say implemented / shipped / N% done — that lives in `BACKLOG.md` (`status-in-spec`).

## Workflow

1. **Create file** — module doc comment at top of source file.
2. **Write Purpose** — one-line module job.
3. **Write Audience** — who reads this.
4. **Write Scope** — what the module covers.
5. **Verify** → `lint-arwaky-cli docs` passes; doc present; `cargo doc --no-deps` and `cargo test --doc` clean for Rust; `python -c "import <package>"` clean for Python; `tsc` clean for TypeScript.

## Templates

Copy, fill, delete nothing — see [`references/HOW-TO-MAKE-PYTHON-DOC.md`](references/HOW-TO-MAKE-PYTHON-DOC.md), [`references/HOW-TO-MAKE-RUST-DOC.md`](references/HOW-TO-MAKE-RUST-DOC.md), and [`references/HOW-TO-MAKE-TYPESCRIPT-DOC.md`](references/HOW-TO-MAKE-TYPESCRIPT-DOC.md).

## Verification

### Machine Checks

```bash
lint-arwaky-cli docs .                 # invariant audit of every document (strict; every finding gates)
cargo doc --no-deps                    # Rust rustdoc builds; no public /// missing
cargo test --doc                       # Rust doctests compile
```

### Human Checks

- No "how" narration in doc comments; the body shows how.
- Summary line first, before any `#` section (Rust) or body text (Python).
- No status words (`implemented`, `shipped`, `N% done`).

---

## Related Skills

- `aes-docs`
- `aes-taxonomy`
- `aes-contract`

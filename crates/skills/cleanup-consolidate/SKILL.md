---
name: cleanup-consolidate
description: "Removes dead code and merges duplicates. Use when cleaning Python, Rust, TS packages."
metadata:
  tags:
    - python
    - rust
    - typescript
    - cleanup
    - consolidation
    - bloat
    - stubs
    - thin-wrappers
    - dead-code
    - orphan
    - unused-files
    - merge
    - deduplication
    - single-file
    - single-struct
    - aes
    - ruff
    - vulture
    - black
    - eslint
    - prettier
    - knip
  triggers:
    - "cleanup python"
    - "clean bloat python"
    - "fix formatting python"
    - "remove unused imports python"
    - "remove stubs python"
    - "remove thin wrappers python"
    - "find unused files python"
    - "find dead code python"
    - "remove dead code python"
    - "cleanup module python"
    - "pep8 python"
    - "consolidate python"
    - "merge files python"
    - "combine modules python"
    - "cleanup rust"
    - "clean bloat rust"
    - "remove stubs rust"
    - "remove thin wrappers rust"
    - "find unused files rust"
    - "find dead code rust"
    - "remove dead code rust"
    - "cleanup crate rust"
    - "merge two files into one"
    - "combine two impl files"
    - "consolidate files"
    - "merge capabilities files"
    - "merge agent files"
    - "merge overlap rust"
    - "deduplicate modules rust"
    - "cleanup typescript"
    - "clean bloat typescript"
    - "fix formatting typescript"
    - "remove unused imports typescript"
    - "remove stubs typescript"
    - "remove thin wrappers typescript"
    - "find unused files typescript"
    - "find dead code typescript"
    - "remove dead code typescript"
    - "cleanup package typescript"
    - "remove unused exports typescript"
    - "consolidate typescript"
    - "merge files typescript"
    - "combine modules typescript"
  dependencies: []
  related:
    - add-docs
    - fix-bypass
    - lint-arwaky
    - create-capabilities
    - create-agent
---
# cleanup-consolidate

Unified codebase cleanup combining **dead code removal** and **file consolidation**. First find and remove dead code, unused files, stubs, thin wrappers, and duplicates. Then detect overlapping files that share the same domain and merge them into single cohesive modules. The result is a cleaner codebase with fewer files, less bloat, and maximum signal-to-noise ratio.

**CRITICAL: Two-Phase Approach** — Phase 1 removes dead code. Phase 2 merges overlapping files. Never skip Phase 1 — consolidating files with dead code wastes effort.

## Language Routing

Read the matching reference and follow it end to end. It is self-contained: rules, detection patterns, exact tool commands, report example, checklists, quick-reference commands, common mistakes, decision flowchart.

| Language   | Unit          | Primary detection tooling                             | Export surface you MUST update      | Reference                          |
| ---------- | ------------- | ----------------------------------------------------- | ----------------------------------- | ---------------------------------- |
| Python     | package       | `ruff`, `vulture`, `black`, `mypy`/`pyright`, `pytest` | `__init__.py`, `__all__`, `pyproject.toml` | [references/python.md](references/python.md) |
| Rust       | crate         | `cargo clippy`, `cargo-udeps`, `cargo check --all-features` | `mod.rs`, trait definitions, `lib.rs`      | [references/rust.md](references/rust.md)     |
| TypeScript | package       | `knip`, `tsc --noEmit`, `eslint`, `prettier`          | barrel `index.ts`, `package.json` `exports`/`main`/`types` | [references/typescript.md](references/typescript.md) |

## Shared Rules

- **Never remove real logic** — only remove code not relevant to FRD scope
- **Always snapshot before cleanup** — git commit or stash before any deletion
- **Always run the language's checks after changes** — verify no breakage
- **File with 0 inbound imports/references AND not an entry point** = likely unused (verify with multi-pattern check)
- **File with only re-exports** = evaluate whether the re-export adds value
- **Single Responsibility** (consolidation): each file should have ONE clear purpose
- **Related classes/functions belong in the same file** (consolidation)

Language-specific invariants (Rust: *always update the trait when removing impl methods*, *one struct per file*; Python: *respect `# noqa` / `# type: ignore` / `# pragma: no cover` / `if TYPE_CHECKING:`*; TypeScript: *respect `@ts-ignore` / `eslint-disable` / side-effect imports / path aliases*) and the full **Exceptions — NEVER Remove Without Explicit Approval** tables live in each reference.

## When to Use

After refactoring modules · before committing changes · when the user asks to clean bloat from a package/crate/module · after merging branches (accumulated dead code) · before release (final bloat + format pass) · when cleaning accumulated commented-out code · when onboarding new developers (reduce noise) · for files with scattered responsibilities · for multiple small files that belong together · after a refactor that split code across files.

## The Fundamental Question

> **"Why does this function/class/type/file need to exist?"**

Answer "because it was always there", "because it might be useful someday", or "because it handles edge cases we don't have" → **REMOVE**.
Answer "because it's required by the FRD" or "because it's called by an FRD-required method" → **KEEP**.
Any other answer → consult that language's full KEEP table (decorators, dynamic imports, feature gates, ambient types, contracts, build scripts, `__all__`/barrel consumers…) in the reference. If unclear → flag for manual review, never auto-remove.

For consolidation: **"Do these files serve the same purpose or share the same domain?"** → If yes, merge into 1 file.

## Phase 1 — Dead Code Cleanup (identical steps in all three references)

1. **1.1 Safety Snapshot** — commit, then branch; non-negotiable.
2. **1.2 Read Requirements** — understand MVP scope; list entry points, public API surface, framework registrations, optional-dependency features.
3. **1.3 Primary Detection (tooling)** — run the language's linter/dead-code tools FIRST; they understand language semantics. Capture reports to files; do not auto-fix formatting yet.
4. **1.4 Secondary Detection (file-level scan)** — orphan script: 0 inbound imports across every import style, dynamic loading, test and build references.
5. **1.5 Detect Function-Level Bloat** — stubs, thin wrappers, duplicate definitions, commented-out code, suppressed items, decorator/registered code.
6. **1.6 Analyze and Categorize** — KEEP / REMOVE / INVESTIGATE per the Fundamental Question.
7. **1.7 Report Phase 1** — see report format below.
8. **1.8 Get Approval for Phase 1** — no deletions before approval.
9. **1.9 Execute Phase 1 Cleanup** — auto-fixable safe tool fixes first, then approved manual removals, then update the export surface.
10. **1.10 Verify Phase 1** — language checks below.

## Phase 2 — File Consolidation

Detect same-purpose / same-domain / split-functionality file groups → pick the target file (the one holding the most logic) → move the source files' unique members into it → update **all** references (imports, exports, tests, wiring) → delete the source file(s) → verify compilation and behavior. Rust merges follow *one struct per file* (merge fields, keep each trait impl, prefix shared method names); Python/TypeScript merge re-export surfaces too (`__init__.py`/`__all__`, barrel `index.ts`/`package.json`).

## Report Format (both phases)

```
## Cleanup Report: <unit>
### Summary            # files scanned, items analyzed, counts per category found
### Per-File Findings  # per file: | Item | Type | Lines | Verdict | Reason |
### Items Requiring Manual Review   # ambiguous / INVESTIGATE categories
### Formatting Fixes (auto-applied) # format-only changes needing no approval
```

Every finding carries a Verdict (KEEP / REMOVE / DELETE / INVESTIGATE) and a one-line Reason. Each reference contains a fully worked example table for its language.

## Verification Gates

After any removal or merge, in this order: static/lint check clean → format check clean → type/compile check clean → full test suite passes → the package/crate still imports or resolves at its entry point → downstream consumers still build → no stale references to deleted names (grep the old names). Exact commands per language are in the references, as are the **Verification Checklist** (Phase 1 / Phase 2 / Final) and **Common Mistakes** tables.

## Rollback & Commit

Rollback via the snapshot: restore a single file from history, or reset the cleanup branch. Commit only after all gates pass, with a descriptive message itemising counts removed (stubs, thin wrappers, duplicates, unused files, unused imports, commented-out code) and files merged — per-language commit-message templates are in the references.

## Dry-Run Mode

When the user requests `--dry-run` or says "just show me what you'd remove": run Phase 1 steps 1.1–1.6 and Phase 2 overlap detection, generate the full report, **do NOT execute any deletions, edits, or format changes**, present the report, and wait for explicit approval. This is the **default mode** for first-time runs on a package/crate.

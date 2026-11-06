---
name: aes-layer
description: "AES layer scaffolding for agent, capabilities, contract, root, surface, taxonomy, utility across Python, Rust, TypeScript, plus test-suite HOW-TOs under reference/test/. Create a new layer file or improve/audit an existing one."
when-to-use: "Create a new AES layer file or improve/audit an existing one. Pass a folder path (a crate/module/package dir) or a single file path; the layer (agent/capabilities/contract/root/surface/taxonomy/utility) and language (python/rust/typescript) are auto-detected from the path and routed to reference/<layer>/<language>.md. Without --dry-run the task runs autonomously without asking for permission; with --dry-run it generates a report only and makes no changes."
argument-hint: '<path> [create|improve] [--dry-run]'
user-invocable: true
---

# aes-layer

Scaffold AES layer files — agent orchestrators, capabilities, contracts, composition roots, surface commands/UI, taxonomy VOs, and stateless utilities — across Python, Rust, and TypeScript. Each layer owns one domain: agent implements exactly one aggregate; capabilities implement one protocol; contract defines protocol ABCs; root wires containers and entries; surface maps input events to VOs; taxonomy holds shared VOs/constants; utility holds pure helpers. The layer chain reads contract → capabilities → agent, consumed by surface/root. One layer file implements one protocol/aggregate pair where applicable. Invariants read by more than one layer live in Global Rules below. The layer also provisions test-suite HOW-TOs under `reference/test/` (python/rust/typescript), moved here from the former `aes-testing-suite` skill.

## Global rules

- Argument shape: `<path> [create|improve] [--dry-run]`. `<path>` is a folder (crate/module/package) or a single file; missing mode defaults to improve.
- Language auto-detection: resolve the path, detect python (.py) / rust (.rs) / typescript (.ts/.tsx/.js/.jsx/.mts/.cts) from files present; on mixed or ambiguous input, list the detected languages and proceed with the dominant one, naming it in the report.
- Routing: dispatch to `reference/<layer>/<language>.md` based on the detected layer and language; that language HOW-TO owns the workflow steps.
- Mode create: scaffold a new file from the language HOW-TO template section.
- Mode improve: audit the named file/folder against the language HOW-TO rules, list findings, apply fixes.
- Safety model: default (no `--dry-run`) = autonomous, no permission prompts, execute directly. `--dry-run` = read-only: produce the findings report and STOP before any write; the report is the deliverable. This bullet is the single source of truth for mode/safety semantics.
- Report: every run ends with a report block — files touched (dry-run: files that WOULD be touched), language detected, mode, findings/fixes.
- Verify: `lint-arwaky-cli scan <layer-path>` → 0 violations. Language compile is fallback only.
- On-disk verify: a parallel agent or `git reset` can revert files without notice — after applying any edit, re-read the touched region (or grep the new content) before declaring the run done.
- One fact, one home: the language HOW-TO carries its layer's suffix, imports, and structure — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [reference/agent/python.md](reference/agent/python.md) | Python agent HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/agent/rust.md](reference/agent/rust.md) | Rust agent HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/agent/typescript.md](reference/agent/typescript.md) | TypeScript agent HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/capabilities/python.md](reference/capabilities/python.md) | Python capabilities HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/capabilities/rust.md](reference/capabilities/rust.md) | Rust capabilities HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/capabilities/typescript.md](reference/capabilities/typescript.md) | TypeScript capabilities HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/contract/python.md](reference/contract/python.md) | Python contract HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/contract/rust.md](reference/contract/rust.md) | Rust contract HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/contract/typescript.md](reference/contract/typescript.md) | TypeScript contract HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/root/python.md](reference/root/python.md) | Python root HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/root/rust.md](reference/root/rust.md) | Rust root HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/root/typescript.md](reference/root/typescript.md) | TypeScript root HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/surface/python.md](reference/surface/python.md) | Python surface HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/surface/rust.md](reference/surface/rust.md) | Rust surface HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/surface/typescript.md](reference/surface/typescript.md) | TypeScript surface HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/taxonomy/python.md](reference/taxonomy/python.md) | Python taxonomy HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/taxonomy/rust.md](reference/taxonomy/rust.md) | Rust taxonomy HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/taxonomy/typescript.md](reference/taxonomy/typescript.md) | TypeScript taxonomy HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/utility/python.md](reference/utility/python.md) | Python utility HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/utility/rust.md](reference/utility/rust.md) | Rust utility HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/utility/typescript.md](reference/utility/typescript.md) | TypeScript utility HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [reference/test/python.md](reference/test/python.md) | Python test-suite HOW-TO — prefixes, run commands, coverage, verify |
| [reference/test/rust.md](reference/test/rust.md) | Rust test-suite HOW-TO — prefixes, nextest, criterion, verify |
| [reference/test/typescript.md](reference/test/typescript.md) | TypeScript test-suite HOW-TO — prefixes, vitest, coverage, verify |

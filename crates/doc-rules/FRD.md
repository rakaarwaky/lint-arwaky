# Doc Rules — FRD

## Feature Overview

The doc-rules crate enforces the AES document-invariant contract across the workspace. It audits every recognized `.md` document against seven rules (AES601–AES607): FR-ID format and field completeness with FR/protocol class parity, section structure shape, spec purity, crosslink integrity, document heading structure, template body-shape conformance with required-document presence, and PR-ID to module-folder ownership. AES606 is answered by four capability seams — table-shape conformance, required-document presence, member-document presence, and template-body placeholder conformance — so this document declares ten requirements against ten protocol classes. Findings carry a machine-readable `violation_type` so consumers can route on them without parsing prose.

Each rule is one capability behind one protocol seam; AES606 carries four seams, so the nine requirements below map one-for-one onto the nine capability seams the agent coordinates. The agent walks the document chain once, hands every capability the same audit context, and merges what they report; the filesystem walk itself lives in the shared utility layer, because an agent may not perform I/O. No external tool is involved.

**Path scoping.** The audit request carries one path, and that path is the **audit root**, not a filter. This differs from `scan`/`check`, which index the whole workspace and then narrow the findings to the target: `docs <path>` discovers the document chain starting at `<path>` and audits only what it finds there. Two consequences follow for a sub-directory run. First, root-level documents outside `<path>` are not audited at all. Second, crosslink and master-document invariants (AES604) resolve against `<path>` — a crate directory has no sibling `PRD.md`/`ROADMAP.md`, so the invariants that look for them behave as they would in a workspace that has none. A whole-workspace audit is the intended default; a sub-directory run is a narrower, self-contained audit, not a filtered view of the full one.

## Functional Requirements

### FR-DOC-001: FR Format, PR-ID Family, Protocol-Class Parity, and BACKLOG Qualifier Alignment (AES601)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-002: Section Structure (AES602)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-003: Spec Purity (AES603)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-004: Crosslinks and State-Vocabulary Placement (AES604)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-005: Document Heading Structure (AES605)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-006: Template Body-Shape Conformance (AES606)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-007: Required-Document Presence (AES606)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-008: Member-Folder Document Presence (AES606)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-009: Template Body & Placeholder Conformance (AES606)

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-DOC-010: PR-ID Module-Folder Ownership (AES607)

| Aspect | Detail|
|--------|-------|
| Description | Every `PR-<PROJECT>-NNN` item declared in the root PRD's `Project Requirements` must own exactly one top-level module folder under `crates\|modules\|packages` whose name matches the PR-ID's project identifier (`PROJECT`). A PRD whose PR-IDs all share one project identifier is satisfied when the layout root exists and is non-empty (that project owns the whole folder set). |
| Business Rules | The root PRD is the only document that declares PR-IDs in `Project Requirements`. Ownership is keyed on the project identifier segment (`PR-LA-001` → `LA`), not on strict cardinality-1 per PR-ID. One identifier may own the whole layout. A PRD declaring no PR-IDs stays silent. A root without a workspace-root marker (`Cargo.toml`, `pyproject.toml`, `package.json`) stays silent. A workspace with no `crates\|modules\|packages` layout root stays silent. |
| Edge Cases | No matching folder for the identifier fires one finding per PR-ID, anchored to its `Project Requirements` heading line, stamped on the PRD document with code `AES607` and violation type `pr_folder_not_owned`. `shared`/`kernel` folders are skipped as kernel dirs. `exempt_members` does not apply — this is a root-PRD-level check, not a per-member check. |
| Capability | `PrFolderOwnershipChecker` implements `IPrFolderOwnershipProtocol` (`audit_pr_folder_ownership`). |
| Protocol | `IPrFolderOwnershipProtocol` in `shared/doc_rules`. |
| Method | `audit_pr_folder_ownership(context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` carrying the document chain rooted at the workspace root. |
| Output | `Vec<DocFinding>` — one finding per PR-ID whose project identifier owns no top-level module folder under `crates\|modules\|packages`. |
| Error | Not applicable; audit-only violation, no auto-fix. |
| Event | `lint-arwaky-cli docs .` (doc-audit pass); not run by `scan`/`check`. |
| Test | Capability test suite covers: clean-matching-folder, missing-folder fires one finding, multi-project membership, single-project owns whole non-empty layout, empty layout fires each PR-ID, non-root silent, no PR-IDs silent, no layout root silent. |
| Metric | Zero `AES607` / `pr_folder_not_owned` findings from `lint-arwaky-cli docs .` on a conforming project. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| — | — |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|

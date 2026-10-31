# ARCHITECTURE Rules

**Purpose:** Document the layered architecture — what each layer is, what it may depend on, and what it must not touch — so both humans and agents modify the system safely.

**Audience:** Engineers, Tech Lead, AI agents that read or generate structural code.

**Scope:** The architectural rules that govern the workspace. Exactly one per project root.

**Location:** Project root (or at `README.md` level when the repo has a single crate/package).

**Length:** 50–500 lines (flat budget shared by every document type).

## Rules

1. **Write rules, not descriptions.** Every section must say what an agent or engineer is allowed to do or not do. A paragraph that reads like marketing text instead of an enforcement rule is the bug in this file.
2. **Dependencies are explicit.** Every layer states exactly what it may import from, and exactly what it may not. Vague "follows best practices" lines are replaced with named-layer restrictions (`Layer X depends only on Layer Y`).
3. **No per-file exceptions without a reason.** When a layer has a special case, state the exception as its own subsection and name the failure mode it prevents.
4. **One diagram per page.** ASCII trees are acceptable. Embedding large PNG or SVG images violates the length budget and breaks copy-paste for agents.
5. **Do not restate code.** If a pattern is visible in the source already, the architecture file explains *why* the pattern exists, not the implementation.
6. **Name the naming convention.** File names must communicate three parts — layer, concern, role — joined by underscores, followed by the language extension: `layer_concern_role.<ext>`.
7. **No implementation detail.** Concrete class names, API signatures, and library choices belong in `FRD.md`. This file owns the boundary rules; FRD owns the per-feature mechanics.
8. **Nothing about the current build.** Whether a layer shipped or not is a backlog claim with evidence, not an architecture sentence.

## Rule Inventory Overview

Carries the pinned AES rule inventory table (first-cell AES-rows: code, group, name, severity, ran-by) plus the group-coverage counts, so a reader can navigate the full inventory in-file.

| Code | Group | Name | Severity | Ran by |
|------|-------|------|----------|--------|

## Group 1: Naming

Groups the naming rules (AES101–AES103) with their fires-on and exception conditions.

## Group 2: Layer & Import Boundary

Groups the import-boundary rules (AES201–AES205).

## Group 3: File & Content Quality

Groups the quality rules (AES301–AES305).

## Group 4: Role Violations

Groups the role rules (AES401–AES406).

## Group 5: Orphan Code

Groups the orphan-code rules (AES501–AES506).

## Group 6: Document Invariants

Groups the document-invariant rules (AES601–AES607).

## Group 7: Folder Structure

Groups the folder-structure rules (AES701–AES706).

## Rule Source of Truth & Parity

States that emitted rule codes are the source of truth and names the parity test that keeps the inventory table in sync.

## Section Contract

| Section | Why it belongs here |
|---------|---------------------|
| Purpose | Names the failure modes the architecture prevents. Watch for descriptive prose instead of rules. |
| Workspace Organization | Defines terms so engineers and agents share a vocabulary. Watch for unstated implicit terms. |
| Naming Convention | Makes file names self-documenting. Watch for missing exceptions list. |
| Vertical Slicing Layout | Shows where each file lands; prevents the `surface/` folder mistake. Watch for missing shared layout. |
| Taxonomy Layer | The foundation — if it drifts, everything above drifts. Watch for missing dependency rule. |
| Contract Layer | The stability boundary. Watch for missing "no implementation" guard. |
| Utility Layer | Shared mechanics that keep Capabilities clean. Watch for business logic leaking into Utility. |
| Capabilities Layer | The behavior implementation. Watch for inter-capability imports. |
| Agent Layer | The orchestrator — sequence, not business. Watch for business calculation here. |
| Surface Layer | The outer boundary. Watch for orchestration or business logic leaking into Surface. |
| Root Layer | The composition entry. Watch for business logic in the container. |

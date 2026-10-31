# Calculator Architecture

## Overview

<One paragraph. Name the failure modes this architecture prevents: tangled imports, undiscoverable behavior, agent misrouting. Keep it to what the boundary rules guard.>

## Rule Index

| Code | Group | Name | Severity |
| ---- | ----- | ---- | -------- |
| AES101 | Naming | Naming Convention | HIGH |
| AES102 | Naming | Suffix/Prefix Rules | HIGH |
| AES103 | Naming | Test File Prefix | HIGH |
| AES201 | Import | Forbidden Import | CRITICAL |
| AES202 | Import | Mandatory Import | HIGH |
| AES203 | Import | Unused Import | MEDIUM |
| AES204 | Import | Dummy Import | HIGH |
| AES205 | Import | Circular Import | CRITICAL |
| AES301 | Quality | File Maximum Limit | HIGH |
| AES302 | Quality | File Minimum Limit | HIGH |
| AES303 | Quality | Mandatory Definition | HIGH/MEDIUM |
| AES304 | Quality | Bypass Comment | CRITICAL |
| AES305 | Quality | Duplication Code | MEDIUM |
| AES401 | Role | Taxonomy Role | HIGH |
| AES402 | Role | Contract Role | HIGH |
| AES403 | Role | Capabilities Role | HIGH/MEDIUM/LOW |
| AES404 | Role | Utility Role | MEDIUM |
| AES405 | Role | Agent Role | HIGH/MEDIUM/LOW |
| AES406 | Role | Surface Role | HIGH |
| AES501 | Orphan | Taxonomy Orphan | LOW |
| AES502 | Orphan | Contract Orphan | MEDIUM |
| AES503 | Orphan | Capabilities Orphan | MEDIUM |
| AES504 | Orphan | Utility Orphan | MEDIUM |
| AES505 | Orphan | Agent Orphan | HIGH |
| AES506 | Orphan | Surface Orphan | HIGH/MEDIUM/LOW |
| AES601 | Doc | AGENTS Document | HIGH |
| AES602 | Doc | README Document | HIGH |
| AES603 | Doc | ARCHITECTURE Document | HIGH |
| AES604 | Doc | CONTRIBUTING Document | HIGH |
| AES605 | Doc | PRD Document | HIGH |
| AES606 | Doc | ROADMAP Document | HIGH |
| AES607 | Doc | BACKLOG Document | HIGH |
| AES608 | Doc | DATA Document | HIGH |
| AES609 | Doc | FRD Document | HIGH |
| AES610 | Doc | DESIGN Document | HIGH |
| AES701 | Structure | Shared Folder Purity | MEDIUM |
| AES702 | Structure | Feature Folder Health | MEDIUM |
| AES703 | Structure | Surface Folder Purity | MEDIUM |
| AES704 | Structure | Test Suite Coverage | MEDIUM |
| AES705 | Structure | Member-Root Placement | MEDIUM |
| AES706 | Structure | PR-LA Folder Mapping | MEDIUM |

---

## Group 1: Naming (AES101–AES103)

Group 1 enforces file naming conventions to ensure clarity and consistency across the codebase.

### Naming Format

| Component | Position | Example |
| --------- | -------- | ------- |
| Layer | Prefix | `capabilities` |
| Concern | Middle | `user` |
| Role | Suffix | `checker` |
| Extension | End | `.rs` / `.py` / `.ts` |

**Pattern:** `layer_concern_role.rs|py|ts`

### AES101 — Naming Convention

### AES102 — Suffix/Prefix Rules

### AES103 — Test File Prefix

---

## Group 2: Layer & Import Boundary (AES201–AES205)

Group 2 enforces layer dependency boundaries to maintain architectural integrity. The Import Matrix below is the canonical source for all allowed/mandatory/forbidden dependency rules.

### AES201 — Forbidden Import

### AES202 — Mandatory Import

### AES203 — Unused Import

### AES204 — Dummy Import

### AES205 — Circular Import

---

## Group 3: File & Content Quality (AES301–AES305)

Group 3 enforces file size limits and content quality standards to maintain code readability.

### AES301 / AES302 — File Line-Count Limits

### AES303 — Mandatory Definition

### AES304 — Bypass Comment

### AES305 — Duplication Code

---

## Group 4: Role Violations (AES401–AES406)

Group 4 enforces role-specific constraints to ensure each layer maintains its intended responsibility. The layer reference tables below summarize each layer's purpose and rules; the per-rule cards that follow encode the violations.

### Layer Reference

#### Taxonomy (AES401)

#### Contract (AES402)

#### Capabilities (AES403)

#### Utility (AES404)

#### Agent (AES405)

#### Surface (AES406)

#### Root

### AES401 — Taxonomy Role

### AES402 — Contract Role

### AES403 — Capabilities Role

### AES404 — Utility Role

### AES405 — Agent Role

### AES406 — Surface Role

---

## Group 5: Orphan Code (AES501–AES506)

Group 5 detects orphaned code that is not wired into the system's composition layer. A file is orphaned when no other layer imports or references it; the composition root's container/entry files are the wiring point.

### Orphan Rules

### AES506 Category / Severity / Consumer

---

## Group 6: Document Invariants (AES601–AES610)

Group 6 enforces document structure, cross-reference integrity, and template conformance across the ten recognized spec documents.

### Shared invariants (apply to every AES6xx rule below)

### Document × Invariant

---

## Group 7: Folder Structure (AES701–AES706)

Group 7 enforces folder structure invariants to maintain vertical slicing and layer purity.

### Terms

### Vertical Slicing Layout

AI agents frequently make this mistake: do NOT create `surface/`, `taxonomy/`, `contract/`, `capabilities/`, `utility/`, `agent/` folders. The correct structure groups files by feature, with layers as filenames, not directories.

### General Layout

```text
project-root/
├── crates|packages|modules/
│   ├── shared/
│   ├── <feature-a>/
│   ├── <feature-b>/
│   ├── <surface-a>/
│   └── <surface-b>/
├── Cargo.toml / package.json / pyproject.toml
```

### Member Types

### AES701 — Shared Folder Purity

### AES702 — Feature Folder Health

### AES703 — Surface Folder Purity

### AES704 — Test Suite Coverage

### AES705 — Member-Root Placement

### AES706 — PR Folder Mapping
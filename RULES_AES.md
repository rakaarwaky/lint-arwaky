# AES Rules — lint-arwaky

| Field | Value |
| --- | --- |
| Summary | 38 rules across 7 groups: Naming 3 · Import 5 · Quality 5 · Role 6 · Orphan 6 · Doc 7 · Structure 6 |
| Source of truth | The rule codes emitted by the crates under `crates/` — this file is the readable mirror, verified by the `doc_codes_match_emitted_inventory` parity test |
| Architecture spec | [ARCHITECTURE.md](ARCHITECTURE.md) — the 7-layer contract this file does not repeat |
| Full rule detail | Full rule detail lives in ARCHITECTURE.md §14–20. This file is the readable mirror, verified by the parity test. |
| Run everything | `lint-arwaky-cli check .` (= `scan`; code + structure + doc groups 1–7) |
| Run doc group only | `lint-arwaky-cli docs .` (group 6; `docs scaffold` writes templates) |
| Config switches | `lint_arwaky.config.yaml` → `architecture.rules.<CODE>.enabled` / `exceptions` / `ignored_rules` (absent entry = enabled) |
| Format | Every heading is followed by exactly one table; severities are as emitted by the checkers |

## How to read this file

| Element | Meaning |
| --- | --- |
| First cell of a rule row | The rule code exactly as the CLI reports it (JSON `code`) |
| Severity | Severity the checker emits; where sub-checks differ, the per-sub-check column in `#### Sub-checks` wins |
| Violation types | `violation_type` field (snake_case) for groups 6–7; SCREAMING_SNAKE `violation_name` for groups 1–5 where populated |
| Fix | One-line remediation |
| Ran by | `check`/`scan` = full pipeline; `docs` = doc audit; both for group 6 |
| Exceptions | Config `exceptions` list entries the checker skips (barrel files, config exemptions) |
| How findings show | Group 6/7 findings render with their `violation_type` in the JSON report; per-finding `why`/`fix` rendering notes live in `ARCHITECTURE.md` / the HOW-TO skill references, never in this file |
| Sub-checks | Group-table count of the rule's `#### Sub-checks` facet rows (or `13 scope rows` for the AES201 matrix); `—` when the rule renders no sub-check facet |

## Rule inventory

| Code | Group | Name | Severity | Ran by |
| --- | --- | --- | --- | --- |
| AES101 | Naming | Naming Convention | HIGH | check/scan, naming |
| AES102 | Naming | Suffix/Prefix Rules | HIGH | check/scan, naming |
| AES103 | Naming | Test File Prefix | HIGH | check/scan, naming |
| AES201 | Import | Forbidden Import | CRITICAL | check/scan, import |
| AES202 | Import | Mandatory Import | HIGH | check/scan, import |
| AES203 | Import | Unused Import | MEDIUM | check/scan, import |
| AES204 | Import | Dummy Import | HIGH | check/scan, import |
| AES205 | Import | Circular Import | CRITICAL | check/scan, import |
| AES301 | Quality | File Maximum Limit | HIGH | check/scan, quality |
| AES302 | Quality | File Minimum Limit | HIGH | check/scan, quality |
| AES303 | Quality | Mandatory Definition | HIGH (missing) / MEDIUM (empty) | check/scan, quality |
| AES304 | Quality | Bypass Comment | CRITICAL | check/scan, quality |
| AES305 | Quality | Duplication Code | MEDIUM | check/scan, quality |
| AES401 | Role | Taxonomy Role | HIGH | check/scan, role |
| AES402 | Role | Contract Role | HIGH | check/scan, role |
| AES403 | Role | Capabilities Role | HIGH / MEDIUM / LOW per sub-check | check/scan, role |
| AES404 | Role | Utility Role | MEDIUM | check/scan, role |
| AES405 | Role | Agent Role | HIGH / MEDIUM / LOW per sub-check | check/scan, role |
| AES406 | Role | Surface Role | HIGH (one sub-check MEDIUM) | check/scan, role |
| AES501 | Orphan | Taxonomy Orphan | LOW | check/scan, orphan |
| AES502 | Orphan | Contract Orphan | MEDIUM | check/scan, orphan |
| AES503 | Orphan | Capabilities Orphan | MEDIUM | check/scan, orphan |
| AES504 | Orphan | Utility Orphan | MEDIUM | check/scan, orphan |
| AES505 | Orphan | Agent Orphan | HIGH | check/scan, orphan |
| AES506 | Orphan | Surface Orphan | HIGH smart / MEDIUM utility / LOW passive | check/scan, orphan |
| AES601 | Doc | FR Format | HIGH | check/scan, docs |
| AES602 | Doc | Section Structure | HIGH | check/scan, docs |
| AES603 | Doc | Spec Purity | HIGH | check/scan, docs |
| AES604 | Doc | Crosslinks | HIGH | check/scan, docs |
| AES605 | Doc | Doc Heading Structure | HIGH | check/scan, docs |
| AES606 | Doc | Doc Template Conformance | HIGH | check/scan, docs |
| AES607 | Doc | PR Folder Ownership | HIGH | check/scan, docs |
| AES701 | Structure | Shared Folder Purity | MEDIUM (report) | check/scan |
| AES702 | Structure | Feature Folder Health | MEDIUM (report) | check/scan |
| AES703 | Structure | Surface Folder Purity | MEDIUM (report) | check/scan |
| AES704 | Structure | Test Suite Coverage | MEDIUM (report) | check/scan |
| AES705 | Structure | Member-Root Placement | MEDIUM (report) | check/scan |
| AES706 | Structure | PR-LA Folder Mapping | MEDIUM (report) | check/scan |

## Group coverage and counts

| Group | Code range | Count | Command scope |
| --- | --- | --- | --- |
| 1 Naming | AES101–AES103 | 3 | check/scan, `naming` |
| 2 Import | AES201–AES205 | 5 | check/scan, `import` |
| 3 Quality | AES301–AES305 | 5 | check/scan, `quality` |
| 4 Role | AES401–AES406 | 6 | check/scan, `role` |
| 5 Orphan | AES501–AES506 | 6 | check/scan, `orphan` |
| 6 Doc | AES601–AES607 | 7 | check/scan **and** `docs` |
| 7 Structure | AES701–AES706 | 6 | check/scan (severity normalized to MEDIUM at report) |
| Total | AES101–AES706 | 38 | — |

## Group 1: Naming

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES101 | Naming Convention | HIGH | 1 | `NAMING_CONVENTION` | check/scan, naming |
| AES102 | Suffix/Prefix Rules | HIGH | 3 | `SUFFIX_PREFIX` | check/scan, naming |
| AES103 | Test File Prefix | HIGH | 3 | `TEST_FILE_PREFIX` | check/scan, naming |

### AES101 — Naming Convention

| Code | Name |
| --- | --- |
| AES101 | Naming Convention |

### AES102 — Suffix/Prefix Rules

| Code | Name |
| --- | --- |
| AES102 | Suffix/Prefix Rules |

### AES103 — Test File Prefix

| Code | Name |
| --- | --- |
| AES103 | Test File Prefix |

## Group 2: Layer & Import Boundary

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES201 | Forbidden Import | CRITICAL | 13 scope rows | `FORBIDDEN_IMPORT` | check/scan, import |
| AES202 | Mandatory Import | HIGH | — | `MANDATORY_IMPORT` | check/scan, import |
| AES203 | Unused Import | MEDIUM | — | `UNUSED_IMPORT` | check/scan, import |
| AES204 | Dummy Import | HIGH | — | `DUMMY_IMPORT` | check/scan, import |
| AES205 | Circular Import | CRITICAL | — | `CIRCULAR_IMPORT` | check/scan, import |

### AES201 — Forbidden Import

| Code | Name |
| --- | --- |
| AES201 | Forbidden Import |

### AES202 — Mandatory Import

| Code | Name |
| --- | --- |
| AES202 | Mandatory Import |

### AES203 — Unused Import

| Code | Name |
| --- | --- |
| AES203 | Unused Import |

### AES204 — Dummy Import

| Code | Name |
| --- | --- |
| AES204 | Dummy Import |

### AES205 — Circular Import

| Code | Name |
| --- | --- |
| AES205 | Circular Import |

## Group 3: File & Content Quality

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES301 | File Maximum Limit | HIGH | — | `FILE_TOO_LARGE` | check/scan, quality |
| AES302 | File Minimum Limit | HIGH | — | `FILE_TOO_SHORT` | check/scan, quality |
| AES303 | Mandatory Definition | HIGH (missing) / MEDIUM (empty) | 2 | `MANDATORY_DEFINITION`, `DEAD_INHERITANCE` | check/scan, quality |
| AES304 | Bypass Comment | CRITICAL | — | `BYPASS_COMMENT`, `UNWRAP_EXPECT`, `PANIC`, `UNIMPLEMENTED`, `TODO` | check/scan, quality |
| AES305 | Duplication Code | MEDIUM | 1 | — | check/scan, quality |

### AES301 — File Maximum Limit

| Code | Name |
| --- | --- |
| AES301 | File Maximum Limit |

### AES302 — File Minimum Limit

| Code | Name |
| --- | --- |
| AES302 | File Minimum Limit |

### AES303 — Mandatory Definition

| Code | Name |
| --- | --- |
| AES303 | Mandatory Definition |

### AES304 — Bypass Comment

| Code | Name |
| --- | --- |
| AES304 | Bypass Comment |

### AES305 — Duplication Code

| Code | Name |
| --- | --- |
| AES305 | Duplication Code |

## Group 4: Role Violations

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES401 | Taxonomy Role | HIGH | — | `TAXONOMY_ROLE` | check/scan, role |
| AES402 | Contract Role | HIGH | — | `CONTRACT_ROLE` | check/scan, role |
| AES403 | Capabilities Role | HIGH / MEDIUM / LOW per sub-check | 8 | `CAPABILITY_ROLE` | check/scan, role |
| AES404 | Utility Role | MEDIUM | — | `UTILITY_ROLE` | check/scan, role |
| AES405 | Agent Role | HIGH / MEDIUM / LOW per sub-check | 11 | `AGENT_ROLE` | check/scan, role |
| AES406 | Surface Role | HIGH (one sub-check MEDIUM) | 4 | `SURFACE_ROLE` | check/scan, role |

### AES401 — Taxonomy Role

| Code | Name |
| --- | --- |
| AES401 | Taxonomy Role |

### AES402 — Contract Role

| Code | Name |
| --- | --- |
| AES402 | Contract Role |

### AES403 — Capabilities Role

| Code | Name |
| --- | --- |
| AES403 | Capabilities Role |

### AES404 — Utility Role

| Code | Name |
| --- | --- |
| AES404 | Utility Role |

### AES405 — Agent Role

| Code | Name |
| --- | --- |
| AES405 | Agent Role |

### AES406 — Surface Role

| Code | Name |
| --- | --- |
| AES406 | Surface Role |

## Group 5: Orphan Code

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES501 | Taxonomy Orphan | LOW | — | `TAXONOMY_ORPHAN` | check/scan, orphan |
| AES502 | Contract Orphan | MEDIUM | — | `CONTRACT_ORPHAN` | check/scan, orphan |
| AES503 | Capabilities Orphan | MEDIUM | — | `CAPABILITIES_ORPHAN` | check/scan, orphan |
| AES504 | Utility Orphan | MEDIUM | — | `UTILITY_ORPHAN` | check/scan, orphan |
| AES505 | Agent Orphan | HIGH | — | `AGENT_ORPHAN` | check/scan, orphan |
| AES506 | Surface Orphan | HIGH smart / MEDIUM utility / LOW passive | — | `SURFACE_ORPHAN` | check/scan, orphan |

### AES501 — Taxonomy Orphan

| Code | Name |
| --- | --- |
| AES501 | Taxonomy Orphan |

### AES502 — Contract Orphan

| Code | Name |
| --- | --- |
| AES502 | Contract Orphan |

### AES503 — Capabilities Orphan

| Code | Name |
| --- | --- |
| AES503 | Capabilities Orphan |

### AES504 — Utility Orphan

| Code | Name |
| --- | --- |
| AES504 | Utility Orphan |

### AES505 — Agent Orphan

| Code | Name |
| --- | --- |
| AES505 | Agent Orphan |

### AES506 — Surface Orphan

| Code | Name |
| --- | --- |
| AES506 | Surface Orphan |

## Group 6: Document Invariants

| Code | Name | Severity | Sub-checks | Violation types | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES601 | FR Format | HIGH | — | 14 | check/scan, docs |
| AES602 | Section Structure | HIGH | — | 1 | check/scan, docs |
| AES603 | Spec Purity | HIGH | — | 2 | check/scan, docs |
| AES604 | Crosslinks | HIGH | — | 11 | check/scan, docs |
| AES605 | Doc Heading Structure | HIGH | — | 5 | check/scan, docs |
| AES606 | Doc Template Conformance | HIGH | — | 11 | check/scan, docs |
| AES607 | PR Folder Ownership | HIGH | — | 1 | check/scan, docs |

### AES601 — FR Format

| Code | Name |
| --- | --- |
| AES601 | FR Format |

### AES602 — Section Structure

| Code | Name |
| --- | --- |
| AES602 | Section Structure |

### AES603 — Spec Purity

| Code | Name |
| --- | --- |
| AES603 | Spec Purity |

### AES604 — Crosslinks

| Code | Name |
| --- | --- |
| AES604 | Crosslinks |

### AES605 — Doc Heading Structure

| Code | Name |
| --- | --- |
| AES605 | Doc Heading Structure |

### AES606 — Doc Template Conformance

| Code | Name |
| --- | --- |
| AES606 | Doc Template Conformance |

### AES607 — PR Folder Ownership

| Code | Name |
| --- | --- |
| AES607 | PR Folder Ownership |

## Group 7: Folder Structure

| Code | Name | Severity | Sub-checks | Violation types | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES701 | Shared Folder Purity | MEDIUM (report) | — | 2 | check/scan |
| AES702 | Feature Folder Health | MEDIUM (report) | — | 7 | check/scan |
| AES703 | Surface Folder Purity | MEDIUM (report) | — | 3 | check/scan |
| AES704 | Test Suite Coverage | MEDIUM (report) | — | 3 | check/scan |
| AES705 | Member-Root Placement | MEDIUM (report) | — | 1 | check/scan |
| AES706 | PR-LA Folder Mapping | MEDIUM (report) | — | 3 | check/scan |

### AES701 — Shared Folder Purity

| Code | Name |
| --- | --- |
| AES701 | Shared Folder Purity |

### AES702 — Feature Folder Health

| Code | Name |
| --- | --- |
| AES702 | Feature Folder Health |

### AES703 — Surface Folder Purity

| Code | Name |
| --- | --- |
| AES703 | Surface Folder Purity |

### AES704 — Test Suite Coverage

| Code | Name |
| --- | --- |
| AES704 | Test Suite Coverage |

### AES705 — Member-Root Placement

| Code | Name |
| --- | --- |
| AES705 | Member-Root Placement |

### AES706 — PR-LA Folder Mapping

| Code | Name |
| --- | --- |
| AES706 | PR-LA Folder Mapping |

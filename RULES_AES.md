# AES Rules — lint-arwaky

| Field | Value |
| --- | --- |
| Summary | 38 rules across 7 groups |
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
| Name | Short rule name as emitted by the checker |
| Severity | Severity as emitted by the checker; normalized to MEDIUM at report for group 7 |
| Ran by | `check`/`scan` = full pipeline; `docs` = doc audit; both for group 6 |
| Fix / Exceptions / Sub-checks | Per-rule detail lives in ARCHITECTURE.md §14–20, not in this file |

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

| Code | Name |
| --- | --- |
| AES101 | Naming Convention |
| AES102 | Suffix/Prefix Rules |
| AES103 | Test File Prefix |

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

| Code | Name |
| --- | --- |
| AES201 | Forbidden Import |
| AES202 | Mandatory Import |
| AES203 | Unused Import |
| AES204 | Dummy Import |
| AES205 | Circular Import |

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

| Code | Name |
| --- | --- |
| AES301 | File Maximum Limit |
| AES302 | File Minimum Limit |
| AES303 | Mandatory Definition |
| AES304 | Bypass Comment |
| AES305 | Duplication Code |

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

| Code | Name |
| --- | --- |
| AES401 | Taxonomy Role |
| AES402 | Contract Role |
| AES403 | Capabilities Role |
| AES404 | Utility Role |
| AES405 | Agent Role |
| AES406 | Surface Role |

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

| Code | Name |
| --- | --- |
| AES501 | Taxonomy Orphan |
| AES502 | Contract Orphan |
| AES503 | Capabilities Orphan |
| AES504 | Utility Orphan |
| AES505 | Agent Orphan |
| AES506 | Surface Orphan |

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

| Code | Name |
| --- | --- |
| AES601 | FR Format |
| AES602 | Section Structure |
| AES603 | Spec Purity |
| AES604 | Crosslinks |
| AES605 | Doc Heading Structure |
| AES606 | Doc Template Conformance |
| AES607 | PR Folder Ownership |

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

| Code | Name |
| --- | --- |
| AES701 | Shared Folder Purity |
| AES702 | Feature Folder Health |
| AES703 | Surface Folder Purity |
| AES704 | Test Suite Coverage |
| AES705 | Member-Root Placement |
| AES706 | PR-LA Folder Mapping |

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

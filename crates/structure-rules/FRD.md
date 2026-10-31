# FRD — Structure Rules

## Feature Overview

The structure-rules crate enforces the 7-layer AES folder discipline by auditing the layout of every workspace. It checks that shared, feature, and surface folders obey their shape contracts — shared folders carry DATA.md + BACKLOG.md, feature folders carry FRD.md + BACKLOG.md, and surface folders carry DESIGN.md + BACKLOG.md — and that every feature folder carries a complete test suite. It returns findings as workspace-root-relative paths so a member-directory scan resolves them correctly.

## Functional Requirements

### FR-STR-001: Shared Folder Purity and Documentation

**Description:** The structure auditor shall enforce shared folder purity and documentation by verifying that the shared layer folder holds only permitted layers and carries its required documentation pair.

#### Business Logic
- The shared folder shall hold only layer contracts, utilities, and taxonomy files.
- The shared folder shall carry its documentation pair consisting of DATA.md and BACKLOG.md.
- A capability file detected inside the shared folder shall fire an individual AES701 finding.
- A missing documentation pair in the shared folder shall fire an AES701 finding.
- A shared folder containing only permitted layers and its documentation pair shall produce zero findings.

#### Edge Cases
- When a shared folder contains multiple forbidden capability files, the auditor shall report each forbidden file as an individual finding.
- When an empty shared folder is scanned, the auditor shall report the missing documentation pair and no purity findings.
- When member-root files are evaluated, they shall be classified before applying shared purity rules.
- When an unreadable directory is encountered, the auditor shall skip that subtree rather than terminating the audit.

#### Technical Specification
| Aspect | Detail |
|---|---|
| **Input** | Audit root path and discovered filesystem entries from StructureRequest::AuditAll |
| **Output** | StructureResponse::Findings containing AES701 findings |
| **Protocol** | `IStructureSharedPurityProtocol` |
| **Method** | `audit_shared` |
| **Event** | None; audit is synchronous |
| **Error** | Unreadable directory subtree skipped; findings emitted for missing documentation pair or foreign capability files |

#### Test
| Context | When Action | Then Observable Result |
|---|---|---|
| A shared folder holds a capability file | The auditor audits shared folder purity | An AES701 finding naming the foreign capability file is emitted |
| A shared folder lacks its documentation pair | The auditor audits shared folder purity | An AES701 finding reporting missing DATA.md and BACKLOG.md is emitted |
| A shared folder holds only permitted layers and its doc pair | The auditor audits shared folder purity | Zero AES701 findings are emitted |

#### Metric
| Target Metric | Value |
|---|---|
| Forbidden capability file detection accuracy | 100% |
| False positive rate on conforming shared trees | 0% |

### FR-STR-002: Feature Folder Health and Documentation

**Description:** The structure auditor shall validate feature folder health and documentation by ensuring every feature folder contains its required orchestrator, capabilities, root container, documentation pair, and shared mirror.

#### Business Logic
- Every feature folder shall hold an agent orchestrator, at least one capability, and a root container.
- Every feature folder shall carry its documentation pair consisting of FRD.md and BACKLOG.md.
- Every feature folder shall be mirrored as a shared layer subfolder in its member.
- A documentation pair folder without an orchestrator shall fire an AES702 finding.
- A feature folder lacking capabilities or root container shall fire an AES702 finding.
- An audit root that is itself a member directory holding the orchestrator shall fire an AES702 finding.

#### Edge Cases
- When a feature folder defines a language-extension root container, the auditor shall accept it as valid container composition.
- When capabilities exist without an orchestrator, the auditor shall fire an orchestrator-missing finding.
- When an orchestrator exists without capabilities, the auditor shall fire a capability-missing finding.
- When a feature lacks a shared layer mirror subfolder, the auditor shall emit an AES702 missing mirror finding.
- When an unreadable subtree is encountered, the auditor shall skip it and continue auditing.

#### Technical Specification
| Aspect | Detail |
|---|---|
| **Input** | Audit root path and discovered filesystem entries from StructureRequest::AuditAll |
| **Output** | StructureResponse::Findings containing AES702 findings |
| **Protocol** | `IStructureFeatureHealthProtocol` |
| **Method** | `audit_feature` |
| **Event** | None; audit is synchronous |
| **Error** | Unreadable directory subtree skipped; findings emitted for missing orchestrator, capabilities, root container, or doc pair |

#### Test
| Context | When Action | Then Observable Result |
|---|---|---|
| A feature folder lacks a root container | The auditor audits feature folder health | An AES702 finding reporting missing root container is emitted |
| A feature folder has capabilities without an orchestrator | The auditor audits feature folder health | An AES702 finding reporting missing orchestrator is emitted |
| A feature folder has its doc pair without an orchestrator | The auditor audits feature folder health | An AES702 finding reporting doc pair without orchestrator is emitted |
| A feature folder conforms to all health and doc rules | The auditor audits feature folder health | Zero AES702 findings are emitted |

#### Metric
| Target Metric | Value |
|---|---|
| Structural gap detection accuracy | 100% |
| Conforming feature false positive findings | 0 |

### FR-STR-003: Surface Folder Purity and Documentation

**Description:** The structure auditor shall enforce surface folder purity and documentation by verifying that only surface-layer files and the required documentation pair reside in surface folders.

#### Business Logic
- A surface folder shall contain only surface-layer files (render trees, passive handlers, surface commands/actions).
- A surface folder shall carry its documentation pair consisting of DESIGN.md and BACKLOG.md.
- Any non-surface file (such as a capability or agent orchestrator) located inside a surface folder shall fire an individual AES703 finding.
- A missing documentation pair in a surface folder shall fire an AES703 finding.
- Conforming surface trees shall produce zero findings.

#### Edge Cases
- When multiple foreign capability or agent files are located inside a surface folder, each foreign file shall fire its own individual finding.
- When an empty surface folder is scanned, the auditor shall emit a missing documentation pair finding.
- When an unreadable subtree is encountered, the auditor shall skip the unreadable subtree without failing the audit.

#### Technical Specification
| Aspect | Detail |
|---|---|
| **Input** | Audit root path and discovered filesystem entries from StructureRequest::AuditAll |
| **Output** | StructureResponse::Findings containing AES703 findings |
| **Protocol** | `IStructureSurfacePurityProtocol` |
| **Method** | `audit_surface` |
| **Event** | None; audit is synchronous |
| **Error** | Unreadable directory subtree skipped; findings emitted for foreign layers or missing documentation pair |

#### Test
| Context | When Action | Then Observable Result |
|---|---|---|
| A foreign capability file sits in a surface folder | The auditor audits surface folder purity | An AES703 finding naming the foreign file is emitted |
| A surface folder lacks its documentation pair | The auditor audits surface folder purity | An AES703 finding reporting missing DESIGN.md and BACKLOG.md is emitted |
| A surface folder contains only surface files and doc pair | The auditor audits surface folder purity | Zero AES703 findings are emitted |

#### Metric
| Target Metric | Value |
|---|---|
| Foreign layer detection accuracy in surface folders | 100% |
| Conforming surface false positive findings | 0 |

### FR-STR-004: Test Suite Category Coverage

**Description:** The structure auditor shall verify test suite category coverage by ensuring every feature folder owning source code contains required test directories and files across all defined testing categories.

#### Business Logic
- Every feature folder that owns source shall provide test coverage across required test categories: unit, contract, integration, e2e, acceptance, dogfood, and benches.
- The auditor shall discover categories dynamically against the discovered tree rather than a hardcoded list.
- An expected test category that is absent or empty shall fire an AES704 finding.
- Non-feature members or members without source code shall not be required to maintain feature test suites.

#### Edge Cases
- When an entire test category directory is missing, the auditor shall fire an AES704 coverage finding.
- When a test category directory exists but contains no test files, the auditor shall treat the category as missing and emit an AES704 finding.
- When optional or supplementary test categories exist beyond the required set, the auditor shall accept them without emitting findings.
- When an unreadable test directory is encountered, the auditor shall skip it without failing the overall audit.

#### Technical Specification
| Aspect | Detail |
|---|---|
| **Input** | Audit root path and discovered test directory entries from StructureRequest::AuditAll |
| **Output** | StructureResponse::Findings containing AES704 findings |
| **Protocol** | `IStructureTestSuiteProtocol` |
| **Method** | `audit_test_suite` |
| **Event** | None; audit is synchronous |
| **Error** | Unreadable test directory skipped; findings emitted for missing or empty test categories |

#### Test
| Context | When Action | Then Observable Result |
|---|---|---|
| A feature folder lacks an integration test category | The auditor audits test suite coverage | An AES704 finding for the missing integration test category is emitted |
| A feature folder contains an empty contract test directory | The auditor audits test suite coverage | An AES704 finding for the empty contract category is emitted |
| A feature folder provides all required test categories | The auditor audits test suite coverage | Zero AES704 findings are emitted |

#### Metric
| Target Metric | Value |
|---|---|
| Missing test category detection rate | 100% |
| Fully covered test suite false positive findings | 0 |

### FR-STR-005: Member-Root File Placement

**Description:** The structure auditor shall enforce member-root file placement by verifying that member directories place configuration, manifest, and documentation files at their root rather than nested in subdirectories.

#### Business Logic
- A member directory shall place its configuration, manifest, and root documentation directly at its root.
- A required root file located only inside a nested subdirectory shall not satisfy root placement and shall fire an AES705 finding.
- Root placement checks shall evaluate paths relative to the member root.
- Non-member directories shall be excluded from member-root file placement checks.

#### Edge Cases
- When a member directory contains configuration only in a nested subdirectory, the auditor shall emit an AES705 finding.
- When a member directory has its configuration and manifest at the root level, the auditor shall emit zero findings.
- When a non-member or unreadable directory is encountered, the auditor shall skip it.

#### Technical Specification
| Aspect | Detail |
|---|---|
| **Input** | Audit root path and discovered member directory entries from StructureRequest::AuditAll |
| **Output** | StructureResponse::Findings containing AES705 findings |
| **Protocol** | `IStructureMemberRootProtocol` |
| **Method** | `audit_member_root` |
| **Event** | None; audit is synchronous |
| **Error** | Non-member or unreadable directories skipped; findings emitted for misplaced root files |

#### Test
| Context | When Action | Then Observable Result |
|---|---|---|
| A member holds its configuration only in a nested subdirectory | The auditor audits member-root file placement | An AES705 finding reporting missing member-root configuration is emitted |
| A member holds required configuration and docs at its root | The auditor audits member-root file placement | Zero AES705 findings are emitted |

#### Metric
| Target Metric | Value |
|---|---|
| Misplaced member-root file detection rate | 100% |
| Conforming member-root false positive findings | 0 |

### FR-STR-006: PR-LA Folder Alignment

**Description:** The structure auditor shall enforce PR-LA folder alignment by verifying a strict 1:1 mapping between PR-LA identifiers, crate folder locations, and the feature requirement prefix in each folder's requirement document.

#### Business Logic
- Every PR-LA identifier declared in the root specification documents shall map 1:1 to a crate folder by positional order.
- Every crate folder at an assigned position shall carry an FRD.md document.
- The requirement identifiers in a crate folder's FRD.md shall carry a feature prefix derived from the crate folder name.
- A PR-LA heading title following the colon separator shall match the crate folder name.
- Root specification documents PRD.md and ROADMAP.md shall be evaluated together to verify mutual agreement with the folder list.

#### Edge Cases
- When a PR-LA identifier in a root document has no matching crate folder at its position, the auditor shall emit an AES706 finding.
- When a crate folder has no FRD.md document, the auditor shall emit an AES706 missing FRD finding.
- When requirement identifiers in an FRD use a prefix that does not match the crate folder, the auditor shall emit an AES706 prefix mismatch finding.
- When a heading title after the colon does not match the crate folder name, the auditor shall emit an AES706 title mismatch finding.
- When an unreadable specification document is encountered, the auditor shall skip checks for that document without failing the audit.

#### Technical Specification
| Aspect | Detail |
|---|---|
| **Input** | Audit filesystem entries and contents of PRD.md and ROADMAP.md from StructureRequest::AuditAll |
| **Output** | StructureResponse::Findings containing AES706 findings |
| **Protocol** | `IStructurePrLaProtocol` |
| **Method** | `audit_pr_la_folder` |
| **Event** | None; audit is synchronous |
| **Error** | Unreadable specification document skipped; findings emitted for missing crate folder, missing FRD, prefix mismatch, or title mismatch |

#### Test
| Context | When Action | Then Observable Result |
|---|---|---|
| A PR-LA identifier lacks a corresponding crate folder | The auditor audits PR-LA folder alignment | An AES706 finding for the unmapped PR-LA identifier is emitted |
| A crate folder's FRD uses a requirement prefix differing from its folder name | The auditor audits PR-LA folder alignment | An AES706 finding for the requirement prefix mismatch is emitted |
| A crate folder at an expected PR-LA position lacks FRD.md | The auditor audits PR-LA folder alignment | An AES706 finding for missing FRD is emitted |
| All PR-LA identifiers, crate folders, titles, and FR prefixes align | The auditor audits PR-LA folder alignment | Zero AES706 findings are emitted |

#### Metric
| Target Metric | Value |
|---|---|
| PR-LA folder misalignment detection accuracy | 100% |
| Conforming workspace false positive findings | 0 |

## Public Interface

| Aspect | Detail |
|---|---|
| Agent | `StructureOrchestrator` |
| Aggregate | `IStructureAggregate` |
| Method | `execute` |
| Request | `StructureRequest::AuditAll` |
| Response | `StructureResponse::Findings` |

## Reference

| Artifact | Purpose | Link |
|---|---|---|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

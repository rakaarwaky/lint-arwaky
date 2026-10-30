# FRD — Structure Rules (AES701–AES705)

## Feature Overview

The structure-rules crate enforces the 7-layer AES folder discipline by auditing the layout of every workspace. It checks that shared, feature, and surface folders obey their shape contracts — shared folders carry DATA.md + BACKLOG.md, feature folders carry FRD.md + BACKLOG.md, and surface folders carry DESIGN.md + BACKLOG.md — and that every feature folder carries a complete test suite. It returns findings as workspace-root-relative paths so a member-directory scan resolves them correctly.

## Functional Requirements

### FR-STR-001: Shared Folder Purity and Documentation (AES701)

#### Description

| No | Detail |
| --- | --- |
| 1 | Enforce that the shared/kernel folder holds only permitted layers and carries its doc pair. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES701 fires when a capability file sits inside shared (shared must hold only kernel contracts/utilities/taxonomy); fires when shared lacks its documentation pair; each forbidden file gets its own violation kind. |
| 2 | A shared tree holding only permitted layers stays silent. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Shared with only permitted layers produces zero findings; each forbidden file reported individually (not merged); missing doc pair fires once; empty shared root yields no findings; member-root files are classified before purity rules. |
| 2 | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | SharedPurityAuditor checks shared-folder purity and docs. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IStructureSharedPurityProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | Shared purity audit returning findings per forbidden file and missing doc pair. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Audit root plus tree entries; source is StructureRequest::AuditAll via IStructureAggregate::execute. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Findings in StructureResponse::Findings with AES701 codes. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable directories skip their subtree instead of failing the audit. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None; audit is synchronous. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | aes701_fires_when_shared_holds_a_capability_file, aes701_fires_when_shared_lacks_doc_pair, aes701_reports_each_forbidden_file_with_its_own_kind, and aes701_stays_silent_when_shared_holds_only_permitted_layers assert firing and silence. |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded forbidden-file fixture yields one finding per file; clean shared fixture yields zero AES701 findings. |
| 2 | — |

### FR-STR-002: Feature Folder Health and Documentation (AES702)

#### Description

| No | Detail |
| --- | --- |
| 1 | Validate feature folder health: orchestrator, capabilities, root container, and documentation conventions. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES702 fires when a feature folder lacks its orchestrator, capabilities, root container (language-extension containers accepted), or shared mirror; fires when a doc-pair folder has no orchestrator; fires when an audit root is itself a member directory; fires when the member holds the orchestrator instead of the feature; forbidden files inside a feature fire individually; missing doc pair fires once. |
| 2 | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Doc-pair folder without orchestrator fires; capabilities without orchestrator fires; orchestrator without capabilities fires; feature without root container fires; python feature accepts language-extension container; audit root being a member directory fires; member-holding-orchestrator fires; features with complete structure stay silent. |
| 2 | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | FeatureHealthAuditor checks feature folder structure and docs. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IStructureFeatureHealthProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | Feature health audit over the tree returning AES702 findings per structural gap. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Audit root plus entries; source is StructureRequest::AuditAll. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Findings in StructureResponse::Findings with AES702 codes. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable subtree skipped; root that is a member directory is reported, not mis-audited. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | aes702_fires_when_feature_lacks_root_container, aes702_fires_when_orchestrator_lacks_capabilities, aes702_fires_when_a_doc_pair_folder_has_no_orchestrator, and aes702_root_container_accepts_language_extension assert firing rules. |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Each seeded structural gap fires its own AES702 finding; a complete seeded feature yields zero findings. |
| 2 | — |

### FR-STR-003: Surface Folder Purity and Documentation (AES703)

#### Description

| No | Detail |
| --- | --- |
| 1 | Enforce surface folder purity: only surface-layer files plus its documentation pair live there. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES703 fires when non-surface files (other layers' code) sit in a surface folder; fires when the surface folder lacks its documentation pair; permitted surface files (render trees, passive handlers, orchestrators) stay silent. |
| 2 | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | A pure surface folder yields zero findings; a foreign capability/agent file inside surface fires individually; missing doc pair fires once; empty surface root yields nothing. |
| 2 | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | SurfacePurityAuditor checks surface-folder purity and docs. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IStructureSurfacePurityProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | Surface purity audit returning AES703 findings per violation. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Audit root plus entries; source is StructureRequest::AuditAll. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Findings in StructureResponse::Findings with AES703 codes. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable subtree skipped. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | Surface purity suite tests in the structure rust suite assert AES703 firing on foreign files and silence on pure surface trees. |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded foreign-file fixture yields one finding per file; pure surface fixture yields zero AES703 findings. |
| 2 | — |

### FR-STR-004: Test Suite Category Coverage (AES704)

#### Description

| No | Detail |
| --- | --- |
| 1 | Verify every test-suite category has its required test coverage layout. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES704 audits test directories per category (unit, contract, e2e, integration, acceptance, dogfood) and fires when a category expected for the tree is absent or empty; coverage checks run against the discovered tree, not a hard-coded list. |
| 2 | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | A tree missing an entire category fires a coverage finding; a category directory that exists but is empty is treated as missing; extra categories beyond the required set never fire; unreadable test dirs skip without failing the audit. |
| 2 | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | TestSuiteCoverageAuditor computes per-category coverage. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IStructureTestSuiteProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | Test-suite coverage audit returning AES704 findings for absent categories. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Audit root plus test-dir entries; source is StructureRequest::AuditAll. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Findings in StructureResponse::Findings with AES704 codes. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable test directories yield no findings for their category rather than an audit failure. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | Test-suite coverage suite tests in the structure rust tests assert AES704 on seeded missing-category trees and silence on fully covered trees. |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded tree missing one category fires exactly one finding; fully covered tree fires zero. |
| 2 | — |

### FR-STR-005: Member-Root File Placement (AES705)

#### Description

| No | Detail |
| --- | --- |
| 1 | Require member directories to place config and docs at their own root, not scattered inside. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES705 fires when a member directory (a real member module/package) lacks required root-level files such as its manifest/config or docs; root placement is checked relative to the member root so nested files do not satisfy it. |
| 2 | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Member with config only in a nested subdir fires (root placement not satisfied); member with root files passes; non-member directories are not checked; nested file does not count as root placement. |
| 2 | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | MemberRootPlacementAuditor enforces member-root file placement. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IStructureMemberRootProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | Member-root placement audit returning AES705 findings when required root files are absent. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Audit root plus entries; source is StructureRequest::AuditAll. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Findings in StructureResponse::Findings with AES705 codes. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Non-member or unreadable directories are skipped. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | Member-root suite tests in the structure rust tests assert AES705 on nested-only-file members and silence on properly rooted members. |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded nested-only member yields exactly one finding; properly rooted member yields zero. |
| 2 | — |

### FR-STR-006: PR-LA Folder Alignment (AES706)

#### Description

| No | Detail |
| --- | --- |
| 1 | Enforce the 1:1 mapping between PR-LA ids at each position, the crate folder that sits there, and the feature requirement prefix that folder's FRD carries. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES706 fires when a crate folder's requirement ids use a prefix that does not derive from the folder name; fires when a crate folder listed by position has no FRD.md; fires when a PR-LA id in the root spec documents has no crate folder at its position. |
| 2 | Fires when a PR-LA heading title after the colon does not name its folder. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | A PR-LA id beyond the known folder order fires once per document; requirement ids with the correct prefix stay silent; a documented folder with no FRD.md fires pr_la_missing_frd; a title that names its folder reports no mismatch; PRD.md and ROADMAP.md are read together so both must agree with the folder list. |
| 2 | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | PrLaFolderAuditor checks its PR-LA id to crate folder mapping. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IStructurePrLaProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | PR-LA folder audit returning AES706 findings per wrong prefix, missing FRD, dangling id, and misnamed title. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Audit entries plus the contents of PRD.md and ROADMAP.md; source is StructureRequest::AuditAll via IStructureAggregate::execute. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Findings in StructureResponse::Findings with AES706 codes. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | An unreadable spec document skips its checks instead of failing the audit. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None; audit is synchronous. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | conforming_workspace_reports_no_aes706_findings, pr_la_id_without_folder_fires_aes706, fr_id_wrong_prefix_fires_aes706, and missing_frd_fires_aes706 assert firing and silence. |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Each wrong prefix or dangling id yields one AES706 finding per document that carries it; a conforming workspace yields zero. |
| 2 | — |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | StructureOrchestrator — the agent-layer orchestrator behind the structural audit of the check command. |
| Aggregate | IStructureAggregate — the feature's single front door. |
| Method | execute(&self, request: StructureRequest) -> StructureResponse |
| Request | StructureRequest::AuditAll |
| Response | StructureResponse::Findings |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|

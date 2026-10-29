# FRD — structure-rules

## Feature Overview

The structure-rules crate enforces the 7-layer AES folder discipline by auditing the layout of every workspace. It checks that shared, feature, and surface folders obey their shape contracts — shared folders carry DATA.md + BACKLOG.md, feature folders carry FRD.md + BACKLOG.md, and surface folders carry DESIGN.md + BACKLOG.md — and that every feature folder carries a complete test suite. It returns findings as workspace-root-relative paths so a member-directory scan resolves them correctly.

## Functional Requirements

### FR-STR-001: Shared Folder Purity and Documentation (AES701)

| Aspect | Detail|
|--------|-------|
| Description | Enforce that the shared/kernel folder holds only permitted layers and carries its doc pair. |
| Business Rules | AES701 fires when a capability file sits inside shared (shared must hold only kernel contracts/utilities/taxonomy); fires when shared lacks its documentation pair; each forbidden file gets its own violation kind; a shared tree holding only permitted layers stays silent. |
| Edge Cases | Shared with only permitted layers produces zero findings; each forbidden file reported individually (not merged); missing doc pair fires once; empty shared root yields no findings; member-root files are classified before purity rules. |
| Capability | SharedPurityAuditor checks shared-folder purity and docs. |
| Protocol | IStructureSharedPurityProtocol. |
| Method | audit_shared(&self, request: StructureRequest) -> StructureResponse |
| Input | Audit root plus tree entries; source is StructureRequest::AuditAll via IStructureAggregate::execute. |
| Output | Findings in StructureResponse::Findings with AES701 codes. |
| Error | Unreadable directories skip their subtree instead of failing the audit. |
| Event | None; audit is synchronous. |
| Test | aes701_fires_when_shared_holds_a_capability_file, aes701_fires_when_shared_lacks_doc_pair, aes701_reports_each_forbidden_file_with_its_own_kind, and aes701_stays_silent_when_shared_holds_only_permitted_layers assert firing and silence. |
| Metric | Seeded forbidden-file fixture yields one finding per file; clean shared fixture yields zero AES701 findings. |

### FR-STR-002: Feature Folder Health and Documentation (AES702)

| Aspect | Detail|
|--------|-------|
| Description | Validate feature folder health: orchestrator, capabilities, root container, and documentation conventions. |
| Business Rules | AES702 fires when a feature folder lacks its orchestrator, capabilities, root container (language-extension containers accepted), or shared mirror; fires when a doc-pair folder has no orchestrator; fires when an audit root is itself a member directory; fires when the member holds the orchestrator instead of the feature; forbidden files inside a feature fire individually; missing doc pair fires once. |
| Edge Cases | Doc-pair folder without orchestrator fires; capabilities without orchestrator fires; orchestrator without capabilities fires; feature without root container fires; python feature accepts language-extension container; audit root being a member directory fires; member-holding-orchestrator fires; features with complete structure stay silent. |
| Capability | FeatureHealthAuditor checks feature folder structure and docs. |
| Protocol | IStructureFeatureHealthProtocol. |
| Method | audit_feature(&self, request: StructureRequest) -> StructureResponse |
| Input | Audit root plus entries; source is StructureRequest::AuditAll. |
| Output | Findings in StructureResponse::Findings with AES702 codes. |
| Error | Unreadable subtree skipped; root that is a member directory is reported, not mis-audited. |
| Event | None. |
| Test | aes702_fires_when_feature_lacks_root_container, aes702_fires_when_orchestrator_lacks_capabilities, aes702_fires_when_a_doc_pair_folder_has_no_orchestrator, and aes702_root_container_accepts_language_extension assert firing rules. |
| Metric | Each seeded structural gap fires its own AES702 finding; a complete seeded feature yields zero findings. |

### FR-STR-003: Surface Folder Purity and Documentation (AES703)

| Aspect | Detail|
|--------|-------|
| Description | Enforce surface folder purity: only surface-layer files plus its documentation pair live there. |
| Business Rules | AES703 fires when non-surface files (other layers' code) sit in a surface folder; fires when the surface folder lacks its documentation pair; permitted surface files (render trees, passive handlers, orchestrators) stay silent. |
| Edge Cases | A pure surface folder yields zero findings; a foreign capability/agent file inside surface fires individually; missing doc pair fires once; empty surface root yields nothing. |
| Capability | SurfacePurityAuditor checks surface-folder purity and docs. |
| Protocol | IStructureSurfacePurityProtocol. |
| Method | audit_surface(&self, request: StructureRequest) -> StructureResponse |
| Input | Audit root plus entries; source is StructureRequest::AuditAll. |
| Output | Findings in StructureResponse::Findings with AES703 codes. |
| Error | Unreadable subtree skipped. |
| Event | None. |
| Test | contract_structure_rules — aes703_fires_when_a_surface_folder_holds_a_capability, aes703_stays_silent_when_a_surface_crate_holds_a_container, and aes703_fires_when_a_surface_folder_lacks_design_md — assert firing and silence. |
| Metric | Seeded foreign-file fixture yields one finding per file; pure surface fixture yields zero AES703 findings. |

### FR-STR-004: Test Suite Category Coverage (AES704)

| Aspect | Detail|
|--------|-------|
| Description | Verify every test-suite category has its required test coverage layout. |
| Business Rules | AES704 audits test directories per category (unit, contract, e2e, integration, acceptance, dogfood) and fires when a category expected for the tree is absent or empty; coverage checks run against the discovered tree, not a hard-coded list. |
| Edge Cases | A tree missing an entire category fires a coverage finding; a category directory that exists but is empty is treated as missing; extra categories beyond the required set never fire; unreadable test dirs skip without failing the audit. |
| Capability | TestSuiteCoverageAuditor computes per-category coverage. |
| Protocol | IStructureTestSuiteProtocol. |
| Method | audit_test_suite(&self, request: StructureRequest) -> StructureResponse |
| Input | Audit root plus test-dir entries; source is StructureRequest::AuditAll. |
| Output | Findings in StructureResponse::Findings with AES704 codes. |
| Error | Unreadable test directories yield no findings for their category rather than an audit failure. |
| Event | None. |
| Test | e2e_structure_rules_flow — e2e_each_missing_test_category_is_named_and_located, e2e_a_missing_bench_directory_is_reported_once_not_per_category, e2e_a_support_prefix_does_not_satisfy_a_required_category, and e2e_a_folder_owning_no_feature_source_owes_no_test_suite — assert category reporting. |
| Metric | Seeded tree missing one category fires exactly one finding; fully covered tree fires zero. |

### FR-STR-005: Member-Root File Placement (AES705)

| Aspect | Detail|
|--------|-------|
| Description | Require member directories to place config and docs at their own root, not scattered inside. |
| Business Rules | AES705 fires when a member directory (a real member module/package) lacks required root-level files such as its manifest/config or docs; root placement is checked relative to the member root so nested files do not satisfy it. |
| Edge Cases | Member with config only in a nested subdir fires (root placement not satisfied); member with root files passes; non-member directories are not checked; nested file does not count as root placement. |
| Capability | MemberRootPlacementAuditor enforces member-root file placement. |
| Protocol | IStructureMemberRootProtocol. |
| Method | audit_member_root(&self, request: StructureRequest) -> StructureResponse |
| Input | Audit root plus entries; source is StructureRequest::AuditAll. |
| Output | Findings in StructureResponse::Findings with AES705 codes. |
| Error | Non-member or unreadable directories are skipped. |
| Event | None. |
| Test | contract_structure_rules — aes705_reports_a_capability_file_loose_at_a_member_root, aes705_allows_wiring_and_unclassified_files_at_a_member_root, and aes705_ignores_a_layer_file_inside_a_feature_folder — assert placement rules. |
| Metric | Seeded nested-only member yields exactly one finding; properly rooted member yields zero. |

### FR-STR-006: PR-LA Folder Alignment (AES706)

| Aspect | Detail|
|--------|-------|
| Description | Enforce the 1:1 mapping between PR-LA ids at each position, the crate folder that sits there, and the feature requirement prefix that folder's FRD carries. |
| Business Rules | AES706 fires when a crate folder's requirement ids use a prefix that does not derive from the folder name; fires when a crate folder listed by position has no FRD.md; fires when a PR-LA id in the root spec documents has no crate folder at its position; fires when a PR-LA heading title after the colon does not name its folder. |
| Edge Cases | A PR-LA id beyond the known folder order fires once per document; requirement ids with the correct prefix stay silent; a documented folder with no FRD.md fires pr_la_missing_frd; a title that names its folder reports no mismatch; PRD.md and ROADMAP.md are read together so both must agree with the folder list. |
| Capability | PrLaFolderAuditor checks its PR-LA id to crate folder mapping. |
| Protocol | IStructurePrLaProtocol. |
| Method | audit_pr_la_folder(&self, request: StructureRequest) -> StructureResponse |
| Input | Audit entries plus the contents of PRD.md and ROADMAP.md; source is StructureRequest::AuditAll via IStructureAggregate::execute. |
| Output | Findings in StructureResponse::Findings with AES706 codes. |
| Error | An unreadable spec document skips its checks instead of failing the audit. |
| Event | None; audit is synchronous. |
| Test | conforming_workspace_reports_no_aes706_findings, pr_la_id_without_folder_fires_aes706, fr_id_wrong_prefix_fires_aes706, and missing_frd_fires_aes706 assert firing and silence. |
| Metric | Each wrong prefix or dangling id yields one AES706 finding per document that carries it; a conforming workspace yields zero. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | StructureOrchestrator — the agent-layer orchestrator behind the structural audit of the check command. |
| Aggregate | IStructureAggregate — the feature's single front door. |
| Method | execute(&self, request: StructureRequest) -> StructureResponse |
| Request | StructureRequest::AuditAll |
| Response | StructureResponse::Findings |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

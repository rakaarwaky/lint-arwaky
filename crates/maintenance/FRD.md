# FRD — maintenance (v2.1.0)

---

## Feature Overview

The maintenance crate provides operational health and upkeep commands for the
lint-arwaky system: environment diagnostics, toolchain verification, adapter
health checking, cache cleanup, tool updates, binary self-update from
GitHub releases, security scanning, dependency reporting, and project
statistics. It is the ops-focused crate — it handles environment health,
not code quality analysis.

The crate follows the AES 7-layer architecture. Each of the eight business
capabilities below corresponds to one `_protocol` trait in `shared/maintenance`
and one implementation in `capabilities_maintenance_checker`.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["Surface"] -->|input| B["maintenance aggregate"]
    B --> C{"action"}

    C -->|"doctor / diagnose / health"| D["MaintenanceChecker"]
    C -->|"security / dependencies"| D
    C -->|"stats / clean / update"| D
    C -->|"self-update"| D

    D --> F["filesystem IO protocol"]
    F -->|subprocess| G["Tool Output"]
    G --> D
    D --> H["Maintenance Result"]

    H --> B
    B -->|output| A
```

---

## Functional Requirements

### FR-Maintenance-001: Environment Health Check (doctor + toolchain)

#### Description

| No | Detail |
| --- | --- |
| 1 | Report overall environment health including toolchain state for the doctor command. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | Doctor aggregates binary availability, toolchain diagnosis, and health status into one result; DiagnoseToolchain returns binary path plus non-empty ok/missing lists. |
| 2 | Doctor result includes health status; Cancel request aborts an in-flight operation gracefully. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Missing tools produce entries in the missing list while the overall doctor call still succeeds. |
| 2 | Cancel returns Cancelled rather than a partial error; empty environment yields a doctor result with all entries missing and no panic. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | DoctorChecker implements doctor and toolchain diagnosis. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IDoctorProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `doctor()` result plus toolchain diagnosis returning binary path and categorized tool lists. |

#### Input

| No | Detail |
| --- | --- |
| 1 | MaintenanceRequest::{Doctor, DiagnoseToolchain, Cancel}; source is the doctor surface command. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::{Doctor, Toolchain, Cancelled} carrying status and tool lists. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unavailable tools are reported in lists, not as top-level failures; cancellation surfaces as Cancelled. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None; doctor runs synchronously. |

#### Test

| No | Detail |
| --- | --- |
| 1 | maintenance_doctor_reports_health_status, diagnose_toolchain_has_binary_path, diagnose_toolchain_returns_non_empty_lists, and doctor_checker_serves_doctor_and_toolchain assert doctor output. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Diagnosis lists are non-empty on a seeded environment; doctor exit status reflects health on every run. |

### FR-Maintenance-002: Project Statistics (stats)

#### Description

| No | Detail |
| --- | --- |
| 1 | Compute project statistics (counts and language breakdown) for the stats command. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | Stats counts source files by language and totals for the current directory; counts are internally consistent (sum of per-language equals total). |
| 2 | Stats operate on the provided directory, not hard-coded paths. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Empty directory yields zero counts without error; nested directories counted; unreadable subtrees skipped. |
| 2 | Stats on current dir runs without panic. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | ProjectStatsChecker computes project statistics. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IProjectStatsProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `stats()` over the target directory returning counts and breakdown. |

#### Input

| No | Detail |
| --- | --- |
| 1 | Directory path; source is MaintenanceRequest::Stats. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::Stats with count fields. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable path reports zero/partial counts rather than failing. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |

#### Test

| No | Detail |
| --- | --- |
| 1 | maintenance_stats_counts_are_consistent and orchestrator_stats_on_current_dir assert count consistency and current-dir routing. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Sum of per-language counts equals the total on every seeded fixture run. |

### FR-Maintenance-003: Cache Cleanup (clean)

#### Description

| No | Detail |
| --- | --- |
| 1 | Remove cache artifacts to reclaim disk space for the clean command. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | Clean removes cache directories the tool owns (AST cache, report cache); it never touches source files or user configuration. |
| 2 | Repeat cleans are safe. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Missing cache directory is a successful no-op; clean does not panic on any seeded state. |
| 2 | Read-only cache entries are skipped with a reported warning rather than aborting the rest. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | CacheCleanupChecker removes owned cache artifacts. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ICacheCleanupProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `clean()` removing known cache roots and returning cleanup status. |

#### Input

| No | Detail |
| --- | --- |
| 1 | Cache root locations (default: configured cache dir); source is MaintenanceRequest::Clean. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::Clean with removal status. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Permission failures on individual entries are reported; overall call still completes. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |

#### Test

| No | Detail |
| --- | --- |
| 1 | orchestrator_clean_does_not_panic and cache_cleanup_checker_implements_cache_cleanup_protocol assert safety and contract. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Clean leaves zero owned cache files on the seeded fixture; source files remain untouched (byte-identical). |

### FR-Maintenance-004: Tool Update (update)

#### Description

| No | Detail |
| --- | --- |
| 1 | Check for and apply updates to bundled linter tools for the update command. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | ToolUpdateChecker probes each bundled tool for newer versions and reports/updates per tool; no-op when already current. |
| 2 | Update does not touch tools the app does not own. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Already-current tool yields no change; a tool that fails to probe is reported but does not block others. |
| 2 | Offline probe reports unavailable rather than hanging (timeout honored). |

#### Capability

| No | Detail |
| --- | --- |
| 1 | ToolUpdateChecker performs tool update checks and installs. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IToolUpdateProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `update()` probing and updating managed tools returning per-tool status. |

#### Input

| No | Detail |
| --- | --- |
| 1 | Tool list from configuration; source is MaintenanceRequest::Update. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::Update with per-tool update outcomes. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Per-tool probe or install failure is reported on that entry; overall response still returns. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |

#### Test

| No | Detail |
| --- | --- |
| 1 | tool_update_checker_implements_tool_update_protocol asserts the protocol contract. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Each probed tool returns a definite status (current, updated, or failed) on a seeded run. |

### FR-Maintenance-005: Security Scan

#### Description

| No | Detail |
| --- | --- |
| 1 | Scan the project for known security issues for the security scan command. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | SecurityScan runs advisory/security checks over the dependency set and reports findings with severity; scan complements cargo-audit results surfaced through external-lint. |
| 2 | Empty dependency set yields zero findings. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | No advisories found yields an empty clean result; unreadable manifest is reported as a scan error. |
| 2 | Network-dependent advisory fetch failing degrades to local-only results. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | SecurityScanChecker performs the security scan. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ISecurityScanProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `security_scan()` returning findings list and scan status. |

#### Input

| No | Detail |
| --- | --- |
| 1 | Project path; source is MaintenanceRequest::SecurityScan. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::Security with findings. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Manifest/lock unreadable yields a scoped error in the response; never panics. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |

#### Test

| No | Detail |
| --- | --- |
| 1 | e2e_security_scan_and_dependency_report and orchestrator_security_scan assert scan flow on seeded fixtures. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded vulnerable fixture reports at least one finding; clean fixture reports zero. |

### FR-Maintenance-006: Dependency Report

#### Description

| No | Detail |
| --- | --- |
| 1 | Produce a dependency report for the project's dependency graph. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | DependencyReport lists dependencies and their sources; works off the parsed manifest/lock data; consistent with the security scan's data source. |
| 2 | Missing dependency data yields an empty report rather than failure. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Partial graphs report what was parsed; report works even when security scan finds nothing. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | DependencyReportChecker aggregates dependency information. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IDependencyReportProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `dependency_report()` returning dependency listing. |

#### Input

| No | Detail |
| --- | --- |
| 1 | Project path; source is MaintenanceRequest::DependencyReport. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::Dependencies with the dependency listing. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable input yields empty listing plus error note. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |

#### Test

| No | Detail |
| --- | --- |
| 1 | orchestrator_dependency_report, dependency_report_checker_implements_dependency_report_protocol, and e2e_security_scan_and_dependency_report assert reporting. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded manifest fixtures produce a listing matching their dependency count. |

### FR-Maintenance-007: Adapter Health Check

#### Description

| No | Detail |
| --- | --- |
| 1 | Check that every configured linter adapter is installed and runnable. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | HealthCheck probes all ten default adapters across the four language groups and reports per-adapter availability; all languages are covered. |
| 2 | Missing adapters are reported, not fatal. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Missing adapter binary reports unavailable for that adapter while others still probe; empty adapter config yields an empty health map. |
| 2 | Probing continues after a failing adapter. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | AdapterHealthChecker probes adapter availability. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IAdapterHealthProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `health_check()` returning per-adapter availability (ten adapters, all four languages). |

#### Input

| No | Detail |
| --- | --- |
| 1 | Adapter configuration; source is MaintenanceRequest::HealthCheck. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::Health with availability per adapter. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Individual probe failure marks that adapter unhealthy; overall response completes. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |

#### Test

| No | Detail |
| --- | --- |
| 1 | health_check_returns_10_adapters, health_check_has_all_languages, maintenance_health_check_covers_all_languages, and orchestrator_health_check assert coverage. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Health check returns exactly ten adapter entries covering Rust, Python, JS, and Markdown groups. |

### FR-Maintenance-008: Self-Update (binary)

#### Description

| No | Detail |
| --- | --- |
| 1 | Update the lint-arwaky binary itself with integrity verification for the self-update command. |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | SelfUpdate downloads the target release artifact, verifies the checksum/signature when a target exists and always when no target exists, and replaces the binary atomically; corrupt downloads are rejected even when the filename matches. |
| 2 | Success or already-up-to-date is reported as success. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Already-up-to-date reports success; corrupt download rejected even when target name matches; no existing target triggers full verification. |
| 2 | Target exists skips redundant re-verify per policy but never skips replace-time check; interrupted download leaves the current binary intact. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | SelfUpdateChecker performs download, verify, and replace. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ISelfUpdateProtocol |

#### Method

| No | Detail |
| --- | --- |
| 1 | `self_update()` downloading, verifying, and replacing the binary. |

#### Input

| No | Detail |
| --- | --- |
| 1 | Release source and current binary path; source is MaintenanceRequest::SelfUpdate. |

#### Output

| No | Detail |
| --- | --- |
| 1 | MaintenanceResponse::SelfUpdate with update outcome. |

#### Error

| No | Detail |
| --- | --- |
| 1 | Corrupt or unverifiable download aborts before replace; caller sees failure with the current binary untouched. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |

#### Test

| No | Detail |
| --- | --- |
| 1 | self_update_checker_implements_self_update_protocol, self_update_rejects_corrupt_download_even_when_target_matches, self_update_verifies_download_when_no_target_exists, and self_update_success_or_already_up_to_date assert integrity and outcomes. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Corrupt payload never replaces the binary on a seeded run; valid payload updates atomically in one call. |

## Public Interface

| Aspect     | Detail |
|------------|--------|
| Agent      | MaintenanceCommandsOrchestrator — the agent-layer orchestrator behind doctor, stats, clean, update, security, dependencies, health, and self-update commands. |
| Aggregate  | IMaintenanceAggregate — the feature's single front door. |
| Method     | execute(&self, request: MaintenanceRequest) -> MaintenanceResponse |
| Request    | MaintenanceRequest::{Stats, Clean, Update, SelfUpdate, Doctor, Cancel, DiagnoseToolchain, HealthCheck, SecurityScan, DependencyReport} |
| Response   | MaintenanceResponse::{Stats, Clean, Update, SelfUpdate, Doctor, Cancelled, Toolchain, Health, Security, Dependencies} |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|

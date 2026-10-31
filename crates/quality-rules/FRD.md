# FRD — quality-rules

## Feature Overview

The quality-rules feature enforces general code quality, formatting limits, and clean-coding policies on the scanned codebase. It blocks bloated files, empty structures, duplicate blocks, and bypass annotations, and applies zero tolerance for warning and error suppressions.

It sits behind the `ICodeAnalysisAggregate` front door and is called by the Surface layer of the check command. It calls no external service itself: file discovery, raw content reads, and parsing are pre-fetched by the filesystem aggregate, and the quality-rules orchestrator performs zero I/O on the pre-fetched data.

## Functional Requirements

### FR-QualityRules-001: Maximum File Line Count (AES301)

**Description:** The quality-rules feature shall flag source files whose line count exceeds the configured maximum, and shall allow files at or below the limit.

#### Business Logic
- The AES301 check shall fire a `FileTooLarge` violation when a file's line count is above the configured maximum.
- The violation message shall contain both the observed line count and the limit for diagnosis.
- Files whose line count is exactly at the limit shall pass without a finding.

#### Edge Cases
- A file one line over the limit shall produce a `FileTooLarge` finding; a file at the limit shall produce none.
- An unreadable file shall yield no finding rather than failing the analysis.
- An empty scan set shall yield no findings.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path plus content; source is `CodeAnalysisRequest::RunAnalysis` via `ICodeAnalysisAggregate::execute`. |
| **Output** | A `FileTooLarge` violation inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `ILineCheckerProtocol` |
| **Method** | `check_line_counts` |
| **Event** | None; the analysis is synchronous. |
| **Error** | An unreadable file reports no finding for that path rather than failing the analysis. |
#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A source file exceeds the configured maximum | The quality-rules audit runs AES301 on that file | One `FileTooLarge` finding is produced and its message contains the observed line count and the limit |
| A source file sits exactly at the configured maximum | The quality-rules audit runs AES301 on that file | No finding is produced |
| The file content cannot be read | The quality-rules audit runs AES301 on that path | No finding is produced and the analysis continues |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per oversized file seeded into the scan | Exactly 1 |
| Findings for an at-limit file | 0 |

### FR-QualityRules-002: Minimum File Line Count (AES302)

**Description:** The quality-rules feature shall flag source files whose line count falls below the configured minimum, and shall allow files at or above the limit.

#### Business Logic
- The AES302 check shall fire a `FileTooShort` violation when a file's line count is under the configured minimum.
- The violation message shall contain both the observed line count and the limit for diagnosis.
- Files whose line count is exactly at the minimum shall pass without a finding.

#### Edge Cases
- A file one line under the minimum shall produce a `FileTooShort` finding; a file at the minimum shall produce none.
- An empty file counts as below the minimum and shall produce a finding.
- An unreadable file shall yield no finding rather than failing the analysis.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path plus content; source is `CodeAnalysisRequest::RunAnalysis`. |
| **Output** | A `FileTooShort` violation inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `ILineCheckerProtocol` |
| **Method** | `check_line_counts` |
| **Event** | None; the analysis is synchronous. |
| **Error** | An unreadable file reports no finding rather than failing the analysis. |
#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A source file is under the configured minimum | The quality-rules audit runs AES302 on that file | One `FileTooShort` finding is produced and its message contains the observed line count and the limit |
| A source file sits exactly at the configured minimum | The quality-rules audit runs AES302 on that file | No finding is produced |
| An empty file is scanned | The quality-rules audit runs AES302 on that file | One `FileTooShort` finding is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per undersized file seeded into the scan | Exactly 1 |
| Findings for an at-minimum file | 0 |

### FR-QualityRules-003: Mandatory Definitions & Dead Inheritance (AES303)

**Description:** The quality-rules feature shall require a mandatory top-level definition in every source file and shall flag declared types that are never given a body.

#### Business Logic
- The AES303 mandatory-definition check shall fire when a source file contains no definition of a recognized kind (struct, enum, trait, class, interface, or type).
- The AES303 dead-inheritance check shall fire a `DeadInheritance` violation for a struct or class declared without an impl block, and for empty class or interface variants, per language.
- A file that carries a definition and non-empty bodies shall pass without a finding.

#### Edge Cases
- A file without any struct or class shall produce a mandatory-definition finding.
- A struct declared without an impl block shall produce a `DeadInheritance` finding.
- An empty Python class, an empty JS class, and an empty JS interface shall each produce a mandatory-definition finding.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Parsed file entries (definitions, impls) plus file content; source is `CodeAnalysisRequest::RunAnalysis`. |
| **Output** | `MandatoryClassDefinition` and `DeadInheritance` violations inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `IMandatoryClassProtocol`, `IDeadInheritanceProtocol` |
| **Method** | `check_mandatory_class_definition`, `check_dead_inheritance` |
| **Event** | None; the analysis is synchronous. |
| **Error** | An unparseable file contributes no findings and the analysis continues across files. |
#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A file has no struct or class | The quality-rules audit runs AES303 on that file | One mandatory-definition finding is produced |
| A struct is declared without an impl block | The quality-rules audit runs AES303 on that file | One `DeadInheritance` finding is produced |
| An empty JS interface is scanned | The quality-rules audit runs AES303 on that file | One mandatory-definition finding is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per definition-miss fixture seeded into the scan | Exactly 1 |
| Findings per struct-without-impl fixture | Exactly 1 `DeadInheritance` |

### FR-QualityRules-004: Bypass Detection (AES304)

**Description:** The quality-rules feature shall detect bypass annotations and panic-style shortcuts that dodge quality checks, and shall count each occurrence as its own finding.

#### Business Logic
- The AES304 check shall fire one finding per occurrence for each bypass kind: `BypassComment` (suppression comments), `UnwrapExpect`, `Panic`, `Todo`, and dead-body shortcuts.
- Each finding shall carry its kind in the violation message.
- A file with no bypass markers shall produce no AES304 finding.

#### Edge Cases
- Multiple bypass sites in one file shall each fire a finding.
- A comment-only todo marker shall flag the `Todo` kind.
- An unreachable-body marker shall be counted as a dead-body shortcut finding.
- An unreadable file shall yield no finding rather than failing the analysis.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File content plus path; source is `CodeAnalysisRequest::RunAnalysis`. |
| **Output** | Violations of kind `BypassComment`, `UnwrapExpect`, `Panic`, `Todo`, or dead-body shortcuts inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `IBypassCheckerProtocol` |
| **Method** | `check_bypass_comments` |
| **Event** | None; the analysis is synchronous. |
| **Error** | An unreadable file contributes no findings; no analysis failure occurs. |
#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A file contains one panic-style call | The quality-rules audit runs AES304 on that file | One `Panic` finding is produced |
| A file contains two bypass suppression comments | The quality-rules audit runs AES304 on that file | Two `BypassComment` findings are produced |
| A clean file is scanned | The quality-rules audit runs AES304 on that file | No AES304 finding is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per bypass occurrence seeded into the scan | Exactly 1, of the matching kind |
| Findings for a clean file | 0 |

### FR-QualityRules-005: Duplicate Code Detection (AES305)

**Description:** The quality-rules feature shall detect excessive code duplication across the scanned file set, and shall group near-identical files into a single finding.

#### Business Logic
- The AES305 check shall fire a `CodeDuplication` violation when duplicate regions across scanned files exceed the configured duplication threshold.
- Identical files or near-clones shall group into a single finding that names the participating files.
- A file shall never be compared against itself.
- A set of fully distinct files shall produce no finding.

#### Edge Cases
- An exact duplicate pair shall produce one `CodeDuplication` finding.
- A fully distinct file set shall produce no finding.
- An unreadable file shall drop out of the comparison set rather than failing the run.
- An empty scan set shall produce no findings.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Set of file contents from the analysis request; source is `CodeAnalysisRequest::RunAnalysis`. |
| **Output** | A `CodeDuplication` violation inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `ICodeMetricAnalyzerProtocol` |
| **Method** | `handle_duplicates_entries` |
| **Event** | None; the analysis is synchronous. |
| **Error** | An unreadable file drops out of the comparison set rather than failing the run. |
#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A seeded clone pair exceeds the duplication threshold | The quality-rules audit runs AES305 on that set | One `CodeDuplication` finding is produced naming both participants |
| A set of fully distinct files is scanned | The quality-rules audit runs AES305 on that set | No `CodeDuplication` finding is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per seeded clone pair | Exactly 1 `CodeDuplication` |
| Findings for a distinct-file fixture set | 0 |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | `CodeAnalysisOrchestrator` — the agent-layer orchestrator behind the quality-rules stage of the check command. |
| Aggregate | `ICodeAnalysisAggregate` — the feature's single front door. |
| Method | `execute(&self, request: CodeAnalysisRequest) -> CodeAnalysisResponse` |
| Request | `CodeAnalysisRequest::{RunAnalysis, Name}` |
| Response | `CodeAnalysisResponse::{Analysis, Name}` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking and epics | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

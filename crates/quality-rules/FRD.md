# FRD — quality-rules

## Feature Overview

The quality-rules feature enforces general code quality, formatting limits, and clean-coding policies on the scanned codebase. It blocks bloated files, empty structures, duplicate blocks, and bypass annotations, and applies zero tolerance for warning and error suppressions.

It sits behind the `ICodeAnalysisAggregate` front door and is called by the Surface layer of the check command. It calls no external service itself: file discovery, raw content reads, and parsing are pre-fetched by the filesystem aggregate, and the quality-rules orchestrator performs zero I/O on the pre-fetched data.

### Developer Journey

**Persona:** a backend Rust developer working inside an AES-layered repository who must satisfy the quality limits, and a CI reviewer reading the quality-rules findings in the Self-Lint gate.

**Journey:**

| Step | Developer does | Sees |
|------|----------------|------|
| 1 | Runs `lint-arwaky-cli check .` (or `--group quality`) before pushing a branch | One block per finding: `[BADGE] CODE file:line  message` |
| 2 | Reads a quality-rules finding | The measured value, the configured limit, and one concrete fix action |
| 3 | Splits, merges, renames, or de-duplicates as the finding's fix says | Nothing |
| 4 | Re-runs the same command | The finding is gone |

**Entry command:**

```text
lint-arwaky-cli check .
```

The check command runs all 8 linter groups by default; restrict to the quality stage with `--group quality` (the quality-rules stage enforces AES301–AES305). The CI Self-Lint gate is the `check` command in the repo's `ci.yml`, and the `report` command provides the progress/summary view.

**First-run output (text formatter, `check` command):**

```text
  [HIGH] AES301 src/agent_orchestrator:file  File has 1420 lines, more than the maximum 1000 lines.
  [CRIT] AES304 src/order_validator:42  UNWRAP_EXPECT: forbidden `unwrap` call detected.
  [MED]  AES305 src/order_validator:file  82% of this file's code appears in another file (the limit is 75%). Also duplicated in src/payment_validator. Blocks: lines 12-40 duplicated in src/payment_validator:lines 10-38; lines 88-101 duplicated in src/payment_validator:lines 80-95
```

**Summary view (`report` command, when a baseline snapshot exists):**

```text
Baseline: 2026-11-09 12:00:00 UTC
  AES301: +1 finding (was 0, now 1)
  AES304: 0 findings (unchanged)
  AES305: +1 finding (was 0, now 1)
Total: +2 findings
```

**Worked example per code:**

- **AES301** — a 1420-line file exceeds the 1000-line maximum: one `[HIGH]` finding on the file (line marker `file`), message naming the observed count and the limit.
- **AES302** — an empty, unparseable, or stub file below the layer minimum: one `[HIGH]` finding on the file; the empty-file variant reads "File is empty or unparseable, fewer than the minimum N lines."
- **AES303** — a source file with no top-level struct/enum/trait/class/interface/type, or an empty class body: one `[HIGH]` mandatory-definition finding on the file, or one `[MED]` dead-inheritance finding on the declaring line.
- **AES304** — `x.unwrap()` on line 42: one `[CRIT]` finding on that line whose message opens with the kind word `UNWRAP_EXPECT` and names the matched token `unwrap`.
- **AES305** — two near-identical validator files: one `[MED]` finding anchored on the alphabetically first participant, listing the other participants and the top duplicated line ranges in a single bounded message.

### Severity Scale

| Code | Severity | Rationale | Gates CI |
|------|----------|-----------|----------|
| AES301 FileTooLarge | HIGH | One bloated file hides responsibilities and blocks review, but does not crash the build or mask a latent defect. | Yes (HIGH) |
| AES302 FileTooShort | HIGH | A stub or empty file scatters logic across call sites and hides where behaviour should live. | Yes (HIGH) |
| AES303 MandatoryDefinition | HIGH | A file that names no primary type keeps its public shape undiscoverable and its responsibilities ungrouped. | Yes (HIGH) |
| AES303 DeadInheritance | MEDIUM | An empty class or struct-without-impl is dead weight that misleads readers, but is often a placeholder mid-refactor. | Yes (MEDIUM) |
| AES304 Bypass/Panic/Shortcut | CRITICAL | `unwrap`/`panic`/`todo!`/bypass comments crash at runtime or mask real defects; zero-tolerance product rule. | Yes (CRITICAL) |
| AES305 CodeDuplication | MEDIUM | Duplicated logic forks on every fix; refactoring debt, not a correctness hazard. | Yes (MEDIUM) |

Severity badges rendered by the text formatter: `[CRIT]`, `[HIGH]`, `[MED]`, `[LOW]`, `[INFO]`.

### Finding Template

Every quality-rules finding is built from one template, in this order:

1. **Message** — the problem, stated once: the kind word, the matched token or measurement, and the limit. No scanner vocabulary ("non-import windows", "non-unique"), no repetition of the limit in the why or fix.
2. **Violation name** — the SCREAMING_SNAKE subtype (e.g. `FILE_TOO_LARGE`, `UNWRAP_EXPECT`, `CODE_DUPLICATION`).
3. **Why** — the rule rationale: why this property matters. No overlap with the message.
4. **Fix** — one concrete action. No parenthetical restatement of the limit.

Whole-file findings carry the line marker `0`, rendered as `file` by the text formatter; line-specific findings (AES304, dead-inheritance) carry the real line number.

### Suppressions

| Code | Exemption scope | Mechanism |
|------|-----------------|-----------|
| AES301 / AES302 | Module-barrel and package-init files, plus the layer's `exceptions` list | Filename matched against the built-in default exception list and the layer's exception list |
| AES303 | `utility` and `surface` layers (no top-level type expected); constant files, language entry-point files, and type-annotation marker files; global and layer exception lists | Layer base-name check plus filename exception lists |
| AES304 | Test-only code blocks and lazy-static initialisers | Skipped during line scan; bypass patterns come from the config `forbidden_bypass` list |
| AES305 | Files whose stem or full name is in the AES305 rule's `exceptions` | Stem/filename match before emission; a file never compares against itself |

### Output Samples

One expected text-formatter line per code (badge, code, file:line, message):

| Code | Sample |
|------|--------|
| AES301 | `[HIGH] AES301 src/agent_orchestrator:file  File has 1420 lines, more than the maximum 1000 lines.` |
| AES302 | `[HIGH] AES302 src/capabilities_stub:file  File is empty or unparseable, fewer than the minimum 10 lines.` |
| AES303 (mandatory) | `[HIGH] AES303 src/capabilities_helpers:file  File declares no struct, enum, trait, class, interface or type.` |
| AES303 (dead inheritance) | `[MED] AES303 src/capabilities_node:14  Unit struct without impl or derive` |
| AES304 (unwrap) | `[CRIT] AES304 src/order_validator:42  UNWRAP_EXPECT: forbidden \`unwrap\` call detected.` |
| AES304 (Python) | `[CRIT] AES304 src/utility_calc:9  Unreachable-body marker \`raise NotImplementedError\` leaves the method without a body.` |
| AES305 | `[MED] AES305 src/order_validator:file  AES305 (similar): 82% of this file's code appears in another file (the limit is 75%). Also duplicated in src/payment_validator. Blocks: lines 12-40 duplicated in src/payment_validator:lines 10-38 (+1 more)` |

### Progress & Cancellation Contract

The quality-rules analysis stage shall run as a single synchronous pass over the pre-fetched file set and shall emit no user-visible progress or lifecycle events. The contract is:

1. **QualityAnalysisStarted** — implicit on `execute(CodeAnalysisRequest::RunAnalysis { files })` entry; no event payload.
2. **QualityAnalysisProgress** — at most one progress tick per 250 files; on fast scans (fewer than 250 files) no progress tick fires at all.
3. **QualityAnalysisFinished** — implicit on `execute` returning `CodeAnalysisResponse::Analysis { violations }`; the violation list is the result.
4. **Cancellation** — the analysis is a single synchronous pass over the pre-fetched file set; it cannot be cancelled mid-pass. If the caller drops the request, the in-flight pass completes and its partial findings are discarded; no partial-finding event is emitted.

## Functional Requirements

### FR-QualityRules-001: File Line Count Limits (AES301 & AES302)

**Description:** The quality-rules feature shall flag source files whose line count exceeds the configured maximum (AES301) or falls below the configured minimum (AES302), and shall allow files at or within both limits.

#### Business Logic

- The AES301 check shall fire a `FileTooLarge` violation when a file's line count is above the configured maximum.
- The violation message shall contain both the observed line count and the limit for diagnosis.
- Files whose line count is exactly at the maximum shall pass without a finding.
- The AES302 check shall fire a `FileTooShort` violation when a file's line count is under the configured minimum.
- Files whose line count is exactly at the minimum shall pass without a finding.

#### Edge Cases

- A file one line over the maximum shall produce a `FileTooLarge` finding; a file at the maximum shall produce none.
- A file one line under the minimum shall produce a `FileTooShort` finding; a file at the minimum shall produce none.
- An empty file counts as below the minimum and shall produce a finding. The orchestrator emits a single `FileTooShort` finding ("File is empty or unparseable, fewer than the minimum N lines.") for a file whose content is empty or that failed to parse, and skips the remaining per-file rules for that file.
- An unreadable file shall yield no finding rather than failing the analysis.
- An empty scan set shall yield no findings.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path plus content; source is `CodeAnalysisRequest::RunAnalysis` via `ICodeAnalysisAggregate::execute`. |
| **Output** | A `FileTooLarge` (AES301) or `FileTooShort` (AES302) violation inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `ILineCheckerProtocol` |
| **Method** | `check_line_counts` |
| **Event** | See the Progress & Cancellation Contract above; the stage is synchronous and fires no per-finding events. |
| **Error** | An unreadable file reports no finding for that path rather than failing the analysis. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A source file exceeds the configured maximum | The quality-rules audit runs AES301 on that file | One `FileTooLarge` finding is produced and its message contains the observed line count and the limit |
| A source file sits exactly at the configured maximum | The quality-rules audit runs AES301 on that file | No finding is produced |
| A source file is under the configured minimum | The quality-rules audit runs AES302 on that file | One `FileTooShort` finding is produced and its message contains the observed line count and the limit |
| A source file sits exactly at the configured minimum | The quality-rules audit runs AES302 on that file | No finding is produced |
| An empty file is scanned | The quality-rules audit runs AES302 on that file | One `FileTooShort` finding is produced |
| The file content cannot be read | The quality-rules audit runs AES301 or AES302 on that path | No finding is produced and the analysis continues |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per oversized file seeded into the scan | Exactly 1 `FileTooLarge` |
| Findings per undersized file seeded into the scan | Exactly 1 `FileTooShort` |
| Findings for an at-limit or at-minimum file | 0 |

### FR-QualityRules-002: Mandatory Definitions & Dead Inheritance (AES303)

**Description:** The quality-rules feature shall require a mandatory top-level definition in every source file and shall flag declared types that are never given a body.

#### Business Logic

- The AES303 mandatory-definition check shall fire when a source file contains no definition of a recognized kind — struct, enum, trait, class, interface, or type — **except for files in the `utility` or `surface` layers** which are skipped.
- The AES303 dead-inheritance check shall fire a `DeadInheritance` violation for a struct or class declared without an impl block, and for empty class or interface variants, per language. Dead-inheritance (sub-check 2) is unchanged and is not layer-scoped.
- A file that carries a definition and non-empty bodies shall pass without a finding.

#### Edge Cases

- A file without any struct or class shall produce a mandatory-definition finding.
- A struct declared without an impl block shall produce a `DeadInheritance` finding.
- An empty Python class, an empty JS class, and an empty JS interface shall each produce a `DeadInheritance` finding (empty class/interface bodies are owned by the dead-inheritance sub-check, not the mandatory-definition sub-check).

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Parsed file entries (definitions, impls) plus file content; source is `CodeAnalysisRequest::RunAnalysis`. |
| **Output** | `MandatoryClassDefinition` and `DeadInheritance` violations inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `IMandatoryClassProtocol`, `IDeadInheritanceProtocol` |
| **Method** | `check_mandatory_class_definition`, `check_dead_inheritance` |
| **Event** | See the Progress & Cancellation Contract above; the stage is synchronous and fires no per-finding events. |
| **Error** | An unparseable file contributes no findings and the analysis continues across files. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A file has no struct or class | The quality-rules audit runs AES303 on that file | One mandatory-definition finding is produced |
| A file in the `utility` or `surface` layer has no struct or class | The quality-rules audit runs AES303 on that file | No mandatory-definition finding is produced |
| A struct is declared without an impl block | The quality-rules audit runs AES303 on that file | One `DeadInheritance` finding is produced |
| An empty JS interface is scanned | The quality-rules audit runs AES303 on that file | One `DeadInheritance` finding is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per definition-miss fixture seeded into the scan | Exactly 1 |
| Findings per struct-without-impl fixture | Exactly 1 `DeadInheritance` |

### FR-QualityRules-003: Bypass Detection (AES304)

**Description:** The quality-rules feature shall detect bypass annotations and panic-style shortcuts that dodge quality checks, and shall count each occurrence as its own finding.

#### Business Logic

- The AES304 check shall fire one finding per occurrence for each bypass kind: `UNWRAP_EXPECT`, `PANIC`, `TODO`, `BYPASS_COMMENT`, and dead-body markers.
- Each finding message shall open with its kind word and name the matched token or phrase, from one shared vocabulary across the five kinds.
- Language-scoped phrase patterns (Python `raise NotImplementedError` / `assert False`, JS/TS `throw new ...`) shall use that language's own vocabulary in the message and fix, not Rust-only terms.
- A file with no bypass markers shall produce no AES304 finding.

#### Edge Cases

- Multiple bypass sites in one file shall each fire a finding.
- A comment-only todo marker shall flag the `TODO` kind; its message opens with `TODO` and names the matched token.
- An unreachable-body marker (`unreachable!`, Python `raise NotImplementedError`) shall fire a dead-body-marker finding; its message and fix use Python vocabulary, not Rust macro wording.
- An unreadable file shall yield no finding rather than failing the analysis.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File content plus path; source is `CodeAnalysisRequest::RunAnalysis`. |
| **Output** | Violations of kind `UNWRAP_EXPECT`, `PANIC`, `TODO`, `BYPASS_COMMENT`, and dead-body markers inside `CodeAnalysisResponse::Analysis`. |
| **Protocol** | `IBypassCheckerProtocol` |
| **Method** | `check_bypass_comments` |
| **Event** | See the Progress & Cancellation Contract above; the stage is synchronous and fires no per-finding events. |
| **Error** | An unreadable file contributes no findings; no analysis failure occurs. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A file contains one panic-style call | The quality-rules audit runs AES304 on that file | One `PANIC` finding is produced; its message opens with `PANIC` and names the matched token |
| A file contains two bypass suppression comments | The quality-rules audit runs AES304 on that file | Two `BYPASS_COMMENT` findings are produced |
| A Python method body is `raise NotImplementedError` | The quality-rules audit runs AES304 on that file | One dead-body-marker finding is produced; its message and fix use Python vocabulary, with no `Result::Err` wording |
| A clean file is scanned | The quality-rules audit runs AES304 on that file | No AES304 finding is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per bypass occurrence seeded into the scan | Exactly 1, of the matching kind |
| Findings for a clean file | 0 |

### FR-QualityRules-004: Duplicate Code Detection (AES305)

**Description:** The quality-rules feature shall detect excessive code duplication across the scanned file set, and shall group near-identical files into a single finding.

#### Business Logic

- The AES305 check shall fire a `CodeDuplication` violation when duplicate regions across scanned files exceed the configured duplication threshold.
- Identical files or near-clones shall group into a single finding anchored on the alphabetically first participant, naming the other participants in the message.
- The finding message shall state the duplicated percentage and the threshold in plain language ("N% of this file's code appears in another file (the limit is M%)") plus a bounded list of the top 5 duplicated blocks; it shall not use scanner vocabulary such as "non-import windows" or "non-unique".
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
| **Event** | See the Progress & Cancellation Contract above; the stage is synchronous and fires no per-finding events. |
| **Error** | An unreadable file drops out of the comparison set rather than failing the run. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A seeded clone pair exceeds the duplication threshold | The quality-rules audit runs AES305 on that set | One `CodeDuplication` finding is produced, anchored on the alphabetically first participant and naming the other |
| A set of fully distinct files is scanned | The quality-rules audit runs AES305 on that set | No `CodeDuplication` finding is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings per seeded clone pair | Exactly 1 `CodeDuplication` |
| Findings for a distinct-file fixture set | 0 |

### FR-QualityRules-005: Per-Rule Enable Flags (UX-S5-002)

**Description:** The quality-rules feature shall honour each rule's own `enabled` flag in addition to the global `config.enabled` switch, and shall name the rules that did not run in the analysis summary.

#### Business Logic

- The orchestrator shall read the `enabled` flag of each `ArchitectureRule` (by its code name `AES301`…`AES305`) before running that rule.
- A rule listed in `config.rules` with `enabled: false` shall be skipped; a rule absent from `config.rules` keeps its built-in default (enabled).
- The analysis summary shall return the disabled rule codes in stable order so a summary can state which rules ran and which did not.

#### Edge Cases

- Disabling a single rule shall suppress only that rule's findings while the remaining rules still run.
- Disabling all rules shall return an empty violation list without failing the analysis.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `ArchitectureConfig.rules` plus `config.enabled`; source is `ICodeAnalysisAggregate::execute`. |
| **Output** | Violations from the enabled rules only; the orchestrator names the disabled rule codes in the analysis summary. |
| **Protocol** | `ICodeAnalysisAggregate` |
| **Method** | `run_analysis_with_entries` |
| **Event** | See the Progress & Cancellation Contract above; the stage is synchronous and fires no per-finding events. |
| **Error** | A disabled rule is skipped, not a failure. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| `AES304` is disabled in config | The quality-rules audit runs on a file with an `unwrap()` call | No AES304 finding is produced and the analysis summary lists `AES304` among the disabled rule codes |
| `AES304` is enabled in config | The quality-rules audit runs on a file with an `unwrap()` call | One AES304 finding is produced and the analysis summary lists no disabled rule codes |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Findings from a rule disabled in config | 0 |
| Disabled rule codes named in the analysis summary | Exactly the ones set to `enabled: false` |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | `CodeAnalysisOrchestrator` — the agent-layer orchestrator behind the quality-rules stage of the check command. |
| Aggregate | `ICodeAnalysisAggregate` — the feature's single front door. |
| Method | `execute(&self, request: CodeAnalysisRequest) -> CodeAnalysisResponse` |
| Request | `CodeAnalysisRequest::{RunAnalysis, Name}` |
| Response | `CodeAnalysisResponse::{Analysis, Name}` |
| Rule summary | The analysis summary returns the disabled rule codes in stable order |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking and epics | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

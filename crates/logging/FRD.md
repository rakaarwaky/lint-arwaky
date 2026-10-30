# FRD — logging

---

## Feature Overview

The logging crate gives a developer running `lint-arwaky scan -v` a real-time feed on stderr of what the scan is doing: which directories it walks, which directories it skips and why, how long each linter phase takes, and what the final result is.

It provides:

- `LogVerbosity` — the value type the `-v` / `--verbose` flag selects. One density level: the flag is always the full per-file trace.
- Four capabilities: filter construction, subscriber installation, phase timing, and walker reporting.
- An orchestrator that composes the four capabilities; a root container that wires them together.

Output goes to stderr so stdout stays clean for piping.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["CLI entry point"] -->|LogVerbosity| B["LoggingOrchestrator"]
    B --> C1["FilterBuild\n(IFilterBuildProtocol)"]
    B --> C2["SubscriberInstall\n(ISubscriberInstallProtocol)"]
    B --> C3["PhaseTimer\n(IPhaseTimerProtocol)"]
    B --> C4["WalkerReporter\n(IWalkerReportProtocol)"]
    C1 --> D["EnvFilter directives"]
    C2 --> E["tracing_subscriber global\n(stderr, real-time)"]
    C3 --> F["phase events: name + elapsed_ms"]
    C4 --> G["walker events: enter / skip(reason) / done(count)"]
```

---

## Functional Requirements

### FR-Logging-001: Build the Filter Directives

#### Description

| No | Detail |
| --- | --- |
| 1 | Map a verbosity level to tracing EnvFilter directives for the root and audit targets. |
| 2 | — |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | Verbosity levels (default, info, debug) map to fixed directives: default yields warn on root plus info on the audit target; info yields info on root plus info on audit; debug yields debug on root plus debug on audit. |
| 2 | Unknown verbosity falls back to default. Directives are returned as an EnvFilter, never installed here. |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Unknown verbosity level falls back to the default mapping rather than panicking; repeated builds with the same level yield identical directives. |
| 2 | Audit target is always at least info. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | FilterBuilder maps verbosity to filter directives. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IFilterBuildProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Build the EnvFilter string/directives for a given verbosity level. |
| 2 | — |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Verbosity enum from LoggingRequest (InstallSubscriber carries it); source is CLI verbosity flags. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | EnvFilter passed to subscriber installation in FR-Logging-002; returned in LoggingResponse::Done. |
| 2 | — |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Invalid level input falls back to default directives; never fails. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None; pure mapping. |
| 2 | — |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | fr_001_default_filter_directive, fr_001_info_filter_directive, fr_001_debug_filter_directive assert exact directive strings. |
| 2 | filter_directive_default_returns_warn_and_audit_info, and filter_directive_debug_returns_debug_and_audit_debug assert exact directive strings. |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Each of the three verbosity levels maps to its documented directive pair with zero mismatch on the filter_directive_* suite. |
| 2 | — |
| ∞ | — |

### FR-Logging-002: Install the Global Subscriber

#### Description

| No | Detail |
| --- | --- |
| 1 | Install the global tracing subscriber exactly once with the requested filter and ANSI setting. |
| 2 | — |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | SubscriberInit installs a tracing-subscriber registry with the EnvFilter from FR-Logging-001. First call wins: a second init call is a noop. |
| 2 | ANSI on/off is honored by the request. Installation never panics for any verbosity level. |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Second init call is a noop (no panic, no double subscriber); init with ANSI disabled works. |
| 2 | Init with debug verbosity works; default verbosity works; container init delegates to the orchestrator. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | SubscriberInit performs global subscriber installation. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ISubscriberInstallProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Install the subscriber with the given filter and ansi flag; returns the EnvFilter used. |
| 2 | — |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Verbosity level plus ansi boolean; source is LoggingRequestKind::InstallSubscriber via ILoggingAggregate::execute. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | LoggingResponseKind::Done carrying install status and the EnvFilter. |
| 2 | — |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Already-installed global subscriber makes the call a noop with success status; no error propagates. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None; subscriber installation is process-global setup. |
| 2 | — |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | fr_002_init_does_not_panic, second_init_call_is_noop, init_debug_with_ansi_does_not_panic assert idempotence and delegation. |
| 2 | install_via_protocol_returns_env_filter_for_each_level, and container_init_delegates_to_orchestrator assert idempotence and delegation. |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Three verbosity levels each return their EnvFilter from one protocol install call. |
| 2 | Repeat install leaves process state unchanged. |
| ∞ | — |

### FR-Logging-003: Time a Scan Phase

#### Description

| No | Detail |
| --- | --- |
| 1 | Time a scan phase and emit a structured duration event when it finishes. |
| 2 | — |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | PhaseTimer starts when PhaseStarted arrives and records the elapsed time on PhaseFinished, emitting a structured tracing event with the phase name and duration. |
| 2 | Measurements use a monotonic clock; nested phases are timed independently. |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | PhaseFinished without a matching PhaseStarted tolerates a zero/unknown duration rather than panicking; two sequential phases each get their own timing. |
| 2 | Dropping a timer without finishing still returns a response without panic. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | PhaseTimerCapability records phase durations. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IPhaseTimerProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Start phase timing, stop it, and return the measured duration. |
| 2 | — |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Phase name from LoggingRequestKind::PhaseStarted and PhaseFinished; source is per-phase calls in the scan pipeline. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Timing data through LoggingResponseKind::PhaseTimer plus a structured trace event. |
| 2 | — |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unmatched finish (no start) yields a zero-duration result instead of an error. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | Structured trace event on phase completion with phase name and elapsed milliseconds. |
| 2 | — |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | PhaseTimer contract tests in the logging suite assert start/stop pairing and response shape (LoggingResponseKind::PhaseTimer). |
| 2 | — |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Measured duration for a seeded busy phase is greater than zero and within a sane bound of wall clock. |
| 2 | Unmatched finish returns zero without panic. |
| ∞ | — |

### FR-Logging-004: Report Walker Progress

#### Description

| No | Detail |
| --- | --- |
| 1 | Emit real-time progress events for files the directory walker enters and skips. |
| 2 | — |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | WalkerEnter reports a path being considered; WalkerSkip reports a path rejected by ignore rules with a reason; FilesDiscovered aggregates the count found in the batch. |
| 2 | Events map to tracing at the verbosity tier from FR-Logging-001 so they appear only when enabled. |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Skipped files still appear as skip events with reason, not as enter events; empty batch emits nothing. |
| 2 | Disabled verbosity suppresses emission without dropping counts; walker continuation is unaffected by reporting failures. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | WalkerReporter emits walker progress events. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IWalkerReportProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Report enter, skip, and discovered-count signals to the tracing backend. |
| 2 | — |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Path plus optional reason and counts; source is LoggingRequestKind::WalkerEnter, WalkerSkip, and FilesDiscovered inside the scan walk. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | LoggingResponseKind::Done plus trace events visible at info/debug. |
| 2 | — |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Reporting never fails the walk; emission is best-effort. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | Trace events for enter, skip, and discovered counts. |
| 2 | — |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | Walker-report contract coverage in the logging tests asserts enter/skip event emission and count reporting through the aggregate. |
| 2 | — |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | On a seeded tree, reported enter plus skip events equal the seeded file count. |
| 2 | Disabled verbosity produces zero emitted events yet identical counts. |
| ∞ | — |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | LoggingOrchestrator — the agent-layer orchestrator behind logger setup and observability requests. |
| Aggregate | ILoggingAggregate — the feature's single front door. |
| Method | execute(&self, request: LoggingRequest) -> LoggingResponse |
| Request | LoggingRequest { kind: LoggingRequestKind::{InstallSubscriber, PhaseStarted, PhaseFinished, WalkerEnter, WalkerSkip, FilesDiscovered} } |
| Response | LoggingResponse { kind: LoggingResponseKind::{Done, PhaseTimer} } |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|

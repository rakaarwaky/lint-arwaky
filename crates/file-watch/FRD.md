# FRD — file-watch

## Feature Overview

[Surface] --> [Watch Orchestrator]
[Watch Orchestrator] --> [Filesystem Watcher Lifecycle]
[Filesystem Watcher Lifecycle] --> [Change Filter]
[Change Filter] --> [Change Lint Handler]
[Change Lint Handler] --> [Code Analysis Aggregate]

The file-watch feature sits in the orchestration layer and is called by surface commands to monitor filesystem changes in real time for linting. It watches configured directory trees, filters and deduplicates change events, and invokes injected code analysis to trigger incremental lint passes. Lint results and lifecycle status flow back to the calling surface.

## Functional Requirements

### FR-FileWatch-001: Watch Filesystem Lifecycle

**Description:**
The file-watch feature shall manage the start, continuous monitoring, and clean teardown of the filesystem watcher service that emits change events for incremental linting. The watcher shall register configured directory paths, report readiness to callers, deliver change notifications over a broadcast stream to subscribers, and release all operating-system watcher resources on shutdown.

#### Business Logic
- The watcher service shall register filesystem watches over the configured target paths, recursive flag, debounce interval, and ignore patterns.
- The watcher shall report whether it is available and ready before callers subscribe to its events.
- The watcher shall deliver filesystem change events to every active subscriber over a broadcast channel.
- Starting an already running watcher service shall be idempotent and keep the existing session alive.
- Stopping the watcher service shall tear down all path monitors and release operating-system watcher resources cleanly.

#### Edge Cases
- Issuing stop when no watcher is running shall succeed without error.
- Requesting an event stream after the service has stopped shall return an empty stream that yields no events.
- Path registration failures due to missing directories or insufficient filesystem permissions shall raise a watcher error scoped to the failed path.
- Reaching operating-system watcher or file-descriptor limits shall be reported as an initialization failure.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | WatchConfig describing target paths, recursive flag, debounce interval, and ignore patterns; supplied via WatchRequest from the calling surface. |
| **Output** | WatchResponse reporting watcher lifecycle status plus a broadcast stream of WatchEvent change notifications delivered to subscribers. |
| **Protocol** | IWatchLifecycleProtocol |
| **Method** | start, is_available, subscribe, stop |
| **Event** | WatchEvent emitted per filesystem change, delivered on the broadcast channel. |
| **Error** | WATCH_SERVICE_ERROR raised when path registration encounters missing targets, permission denials, or resource exhaustion. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A watcher service is constructed over the seeded fixture tree | The service starts with a valid config | The service reports readiness and accepts subscriptions |
| Stop is called on a running watcher | Stop completes | All path monitors are torn down and watcher resources are released within one event-loop turn |
| A subscriber registers after stop | Stop has completed | The subscriber receives an empty stream that yields no events |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Watcher availability | 100% of seeded sessions report ready after start |
| Resource release | All watcher handles closed within one event-loop turn on stop |

### FR-FileWatch-002: Filter and Deduplicate Change Events

**Description:**
The file-watch feature shall keep only lintable file changes from the watcher broadcast and remove duplicate events for the same path and kind, passing a clean ordered stream of unique lintable changes to the lint handler.

#### Business Logic
- Filtering shall drop any change event whose path extension is not in the lintable set.
- Filtering shall remove duplicate events keyed on path and change kind, keeping only the first occurrence.
- Filtering shall preserve the original input order of kept events.
- Filtering shall be total and never fail on any input batch.

#### Edge Cases
- An empty event batch yields an empty output.
- A batch of only non-lintable paths yields an empty output.
- A mixed batch keeps exactly one event per unique path-and-kind key; a batch with no duplicates passes through unchanged.
- Temporary files and ignored paths are treated as non-lintable.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | Vec<WatchEvent> delivered from the watcher broadcast stream. |
| **Output** | Vec<WatchEvent> containing only lintable, deduplicated changes in original order. |
| **Protocol** | IChangeFilterProtocol |
| **Method** | is_lintable, filter_events |
| **Event** | None; this stage only narrows WatchEvents from the watcher. |
| **Error** | None; filtering is total and never fails. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A batch contains lintable and non-lintable change events | Filter processes the batch | Non-lintable events are removed and lintable events pass through unchanged |
| A batch contains duplicate events for the same path and kind | Filter processes the batch | Only the first occurrence of each unique key is kept |
| An empty batch is filtered | Filter runs on empty input | An empty output is returned |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Unique events per key | Exactly one retained event per unique path-and-kind key |
| Non-lintable share passed to lint | Zero non-lintable events reach the lint handler |

### FR-FileWatch-003: Run Lint on Changed Files

**Description:**
The file-watch feature shall invoke the injected linter for every retained change event coming out of the filter, so each lintable change triggers exactly one lint pass and a failure on one file does not stop the remaining files.

#### Business Logic
- Linting shall resolve each filtered change event's path and hand it to the dependency-injected linter.
- Only events that passed the filter reach the lint handler.
- A failure linting one file shall not prevent processing of the remaining events.
- An empty queue is a no-op and produces no error.

#### Edge Cases
- A file deleted between event emission and linting produces a watcher error scoped to that file only.
- A non-lintable event that reaches the handler by mistake is skipped without linting.
- Unreadable or vanished files produce a per-file error without crashing the handler.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | A single WatchEvent carrying path and change kind, sourced from the filtered event stream. |
| **Output** | Lint invoked per event with Ok(()) on success; results surface through the watch response. |
| **Protocol** | IChangeLintProtocol |
| **Method** | lint_changed |
| **Event** | None; lint results flow through the injected analysis dependency rather than as domain events. |
| **Error** | WATCH_SERVICE_ERROR scoped to the affected file when linting an unreadable or vanished file. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A filtered lintable change event reaches the handler | The handler invokes linting | The linter runs exactly once for that file |
| One file in a batch is unreadable | The handler processes remaining events | Other files are still linted and the bad file yields a scoped error only |
| An empty event queue reaches the handler | The handler runs | No linting occurs and no error is raised |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Lint invocations per seeded change | Exactly one per lintable file |
| Vanished-file handling | One scoped error per vanished file and zero panics |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | WatchOrchestrator — the agent-layer orchestrator owning the watcher lifecycle loop. |
| Aggregate | IWatchAggregate — the feature's single front door. |
| Method | execute(&self, request: WatchRequest) -> WatchResponse |
| Request | WatchRequest::{Run { config }, IsLintable { path }} |
| Response | WatchResponse::{Run { status }, IsLintable { lintable }} |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

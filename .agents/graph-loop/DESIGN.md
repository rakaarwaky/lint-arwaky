# Graph Loop Engineering — Design Concept (Revision)

> **Version:** 2.0
> **Revision:** 2026-08-07
> **Previous revision:** 1.0 (2026-08-06)

---

## Vision

A 24/7 automated review pipeline system that processes features sequentially based on priority, with quality gates at every stage. The pipeline uses AI agents acting as Business-Analyst, Tech-Lead, Architect, Developer, and Quality-Analysis to replace the manual review flow.

---

## Core Principle

**Architect as merge gatekeeper.**

Business-Analyst and Tech-Lead analyze in parallel, then Architect merges their results into **one merged plan** for Developer. Developer does not need to reconcile 3 separate reports.

```
Business-Analyst ──┐
                    ├──▶ Architect ──▶ Developer ──▶ Quality-Analysis
Tech-Lead ─────────┘
```

---

## Design Philosophy

### 1. Single Source of Truth

One configuration file (`config.yaml`) for all settings. No scattered configs.

### 2. Full Names, No Abbreviations

All nodes use full names: **Business-Analyst**, **Tech-Lead**, **Quality-Analysis** — not BA, TL, QA. Consistent across all documents, logs, state, prompts, and config.

### 3. Feature Priority Queue

User defines the feature priority order. Core packages are processed first, surface packages last.

### 4. Parallel with Coordination

Business-Analyst and Tech-Lead run concurrently with **tight coordination** to prevent drift:

- **Shared contract:** Both receive identical input/output schema
- **Shared acceptance criteria:** The same checklist is injected into both prompts
- **Shared output schema:** Same report format (Markdown with mandatory sections)
- **Folder boundary:** Skill instructions restrict analysis scope to the feature folder only
- **FRD snapshot:** Both receive the same FRD version at the same timestamp

### 5. Fail Graceful

Pipeline can handle partial completion, timeout, rejection, and escalation without crashing. Every failure has a clear state and recovery path.

### 6. Separation of Concerns — Two Separate Counters

Pipeline has two different counters:

- **Rejection loop counter:** Counts how many times Quality-Analysis rejected (max 3)
- **Pipeline iteration counter:** Counts how many times the pipeline ran fully from trigger to verdict (max 5)

Both are separate and reset independently.

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                         GRAPH ENGINE                                  │
│                                                                       │
│  ┌──────────┐    ┌──────────────┐    ┌──────────┐    ┌────────────┐  │
│  │ TRIGGER  │───▶│    STATE     │───▶│ DISPATCH │───▶│  FEATURE   │  │
│  │  LAYER   │    │   MACHINE    │    │  LAYER   │    │   QUEUE    │  │
│  └──────────┘    └──────────────┘    └──────────┘    └────────────┘  │
│       │               │                  │                │            │
│       ▼               ▼                  ▼                ▼            │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │              AGENT EXECUTION LAYER                           │    │
│  │                                                               │    │
│  │  ┌────────────────┐  ┌────────────┐ ← parallel               │    │
│  │  │Business-Analyst│  │ Tech-Lead  │                           │    │
│  │  └───────┬────────┘  └──────┬─────┘                           │    │
│  │          └────────┬─────────┘                                  │    │
│  │                   ▼                                            │    │
│  │  ┌─────────────────────────┐                                   │    │
│  │  │       Architect         │ ← merge + validate                │    │
│  │  │    (gatekeeper)         │                                   │    │
│  │  └───────────┬─────────────┘                                   │    │
│  │              ▼                                                 │    │
│  │  ┌─────────────────────────┐                                   │    │
│  │  │       Developer         │ ← execute plan                    │    │
│  │  └───────────┬─────────────┘                                   │    │
│  │              ▼                                                 │    │
│  │  ┌─────────────────────────┐                                   │    │
│  │  │    Quality-Analysis     │ ← final gate (NEVER skip)         │    │
│  │  └─────────────────────────┘                                   │    │
│  └──────────────────────────────────────────────────────────────┘    │
│       │                                                              │
│       ▼                                                              │
│  ┌──────────┐    ┌──────────────┐                                    │
│  │  STATE   │    │   RECOVERY   │ ← resume / retry / mark failed    │
│  │  PERSIST │◀──▶│   MANAGER    │                                    │
│  └──────────┘    └──────────────┘                                    │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Node Roles

| Node                       | Role                                                                                   | Input (Absolute Paths)                                          | Output (Absolute Paths)                                  | Skippable?                         |
| -------------------------- | -------------------------------------------------------------------------------------- | --------------------------------------------------------------- | -------------------------------------------------------- | ---------------------------------- |
| **Business-Analyst** | Validate business logic & requirements                                                 | Feature context + FRD snapshot (via prompt)                     | `/home/.../results/business-analyst-<feature>.md`      | Yes (simple fix, hotfix, typo)     |
| **Tech-Lead**        | Pre-implementation technical review: architecture, dependencies, security, performance | Feature context + FRD snapshot (via prompt)                     | `/home/.../results/tech-lead-<feature>.md`             | Yes (doc-only update)              |
| **Architect**        | Merge both reports, validate AES compliance, final certification                       | BA report + TL report (or Skip Report) + Feature context        | `/home/.../plans/merged-<feature>-<correlation_id>.md` | **No** — always required    |
| **Developer**        | Execute merged plan                                                                    | Merged plan (via prompt)                                        | PR +`/home/.../reports/done-<feature>.md`              | **No** — always required    |
| **Quality-Analysis** | Final gatekeeper: validate PR against merged plan + acceptance criteria                | PR + merged plan + FRD + CI/test result + BA report + TL report | Verdict (APPROVE / REJECT / ESCALATE)                    | **No** — always required--- |

## Pipeline Flow

### Happy Path

```
trigger ──[feature-selected]──▶ ┌── Business-Analyst (parallel) ──┐
                                │                                  │
                                └── Tech-Lead (parallel) ─────────┘
                                                     │
                                          both_reports_complete
                                                     ▼
                                             Architect (merge)
                                                     │
                                            merged_plan_written
                                                     ▼
                                          Developer (creates PR)
                                                     │
                                              ci_green
                                                     ▼
                                          Quality-Analysis (review)
                                                     │
                                                 APPROVED
                                                     ▼
                                                  MERGED
                                                     │
                                              next feature
                                                     ▼
                                                  IDLE
```

### Rejection Loop

```
Quality-Analysis ──[REJECT]──▶ Architect ──[re-merge]──▶ Developer ──[fix]──▶ Quality-Analysis
       │                                                                            │
       └────────────────── rejection_loop_counter < 3 ─────────────────────────────┘
       │                                                                            │
       └── rejection_loop_counter >= 3 ──▶ ESCALATED ──▶ WAITING_HUMAN              │
```

### Escalation Flow

```
Developer ──[blocker]──▶ Architect ──[guidance]──▶ Developer
                │
                └──[unresolvable]──▶ WAITING_HUMAN ──[human fix]──▶ RESUMED

Quality-Analysis ──[CRITICAL]──▶ Architect
        │
        ├──[resolvable]──▶ Developer (fix) ──▶ Quality-Analysis
        │
        └──[unresolvable]──▶ WAITING_HUMAN ──[human fix]──▶ RESUMED
```

### Skip Flow

```
trigger ──[doc-only]──▶ ┌──[SKIP]──▶ Skip Report ──┐
                        │                            │
                        └── Tech-Lead (parallel) ───┘
                                            │
                                    Architect (certifies)
                                            │
                                     Developer (trivial change)
                                            │
                                Quality-Analysis (auto-approve if trivial)
                                            │
                                          MERGED
```

---

## State Machine

### Pipeline State Machine

```
                                   ┌─────────────┐
                              ┌───▶│    IDLE     │◀──┐
                              │    └──────┬──────┘   │
                              │           │           │
                    feature completed     │ trigger   │ pipeline completed
                    ┌─────────┘          │ received  │         │
                    │                     ▼           │         │
                    │    ┌───────────────────────┐    │         │
                    │    │    DISPATCHING        │    │         │
                    │    │  (queue claim lock)   │    │         │
                    │    └───────────┬───────────┘    │         │
                    │                │                 │         │
                    │                ▼                 │         │
                    │    ┌───────────────────────┐    │         │
                    │    │    ANALYZING          │    │         │
                    │    │  (parallel)           │    │         │
                    │    │  ┌─────────┐┌───────┐│    │         │
                    │    │  │Business-││Tech-  ││    │         │
                    │    │  │Analyst  ││Lead   ││    │         │
                    │    │  └────┬────┘└───┬───┘│    │         │
                    │    └───────┼─────────┼────┘    │         │
                    │            │ both done│        │         │
                    │            └────┬────┘         │         │
                    │                 ▼              │         │
                    │    ┌───────────────────────┐   │         │
                    │    │      ARCHITECT        │◀──┼──┐      │
                    │    └───────────┬───────────┘   │  │      │
                    │                │ merged plan   │  │      │
                    │                ▼               │  │      │
                    │    ┌───────────────────────┐   │  │      │
                    │    │      DEVELOPER        │───┼──┼──┐   │
                    │    └───────────┬───────────┘   │  │  │   │
                    │                │ complete      │  │  │   │
                    │                ▼               │  │  │   │
                    │    ┌───────────────────────┐   │  │  │   │
                    │    │   QUALITY-ANALYSIS    │───┼──┼──┘   │
                    │    └───────────┬───────────┘   │  │      │
                    │                │               │  │      │
                    │       ┌────────┼────────┐      │  │      │
                    │       │        │        │      │  │      │
                    │       ▼        ▼        ▼      │  │      │
                    │   APPROVE   REJECT  ESCALATE   │  │      │
                    │       │        │        │      │  │      │
                    │       ▼        ▼        ▼      │  │      │
                    │    MERGED  re-merge  escalate  │  │      │
                    │       │     loop     │        │  │      │
                    │       │        │     ▼        │  │      │
                    │       │        │  WAITING_    │  │      │
                    │       │        │   HUMAN      │  │      │
                    │       │        │     │        │  │      │
                    │       │        │  human fix   │  │      │
                    │       │        │     │        │  │      │
                    │       │        │     ▼        │  │      │
                    │       │        └────▶ Developer│  │      │
                    │       │                 │     │  │      │
                    └───────┘                 └─────┘  │      │
                                                        │      │
              ┌─────────────────────────────────────────┘      │
              │                                                │
              │  ┌──────────────────────────────────┐          │
              │  │  FAILURE STATES                   │          │
              │  │                                    │          │
              │  │  ┌─────────┐  retry  ┌─────────┐ │          │
              │  │  │ FAILED  │───────▶│ retrying │ │          │
              │  │  └────┬────┘        └────┬────┘ │          │
              │  │       │                   │      │          │
              │  │       │ max retries       │      │          │
              │  │       ▼                   │      │          │
              │  │  ┌─────────┐              │      │          │
              │  │  │ BLOCKED │              │      │          │
              │  │  └────┬────┘              │      │          │
              │  │       │ skip to next      │      │          │
              │  │       ▼                   │      │          │
              │  │  ┌─────────┐              │      │          │
              │  │  │ TIMEOUT │──────▶┌──────┘      │          │
              │  │  └─────────┘       │             │          │
              │  │                    ▼             │          │
              │  │  ┌──────────────┐  ┌──────────┐  │          │
              │  │  │ WAITING_HUMAN│  │ SKIPPED  │  │          │
              │  │  └──────┬───────┘  └──────────┘  │          │
              │  │         │ human fixes             │          │
              │  │         ▼                         │          │
              │  │  ┌─────────┐                      │          │
              │  │  │ RESUMED │──────────────────────┘          │
              │  │  └─────────┘                                 │
              │  └──────────────────────────────────┘           │
              │                                                 │
              └──▶ IDLE (next feature) ◀────────────────────────┘
```

### State Definitions

| State            | Meaning                                                    | Transitions                       |
| ---------------- | ---------------------------------------------------------- | --------------------------------- |
| IDLE             | No active pipeline, scanning trigger queue                 | → DISPATCHING (trigger received) |
| DISPATCHING      | Claim feature from queue, acquire lock                     | → ANALYZING (lock acquired)      |
| ANALYZING        | Business-Analyst + Tech-Lead parallel                      | → ARCHITECT (both done)          |
| ARCHITECT        | Merge both reports, produce merged plan                    | → DEVELOPER (plan written)       |
| DEVELOPER        | Execute merged plan, create PR                             | → QUALITY-ANALYSIS (PR ready)    |
| QUALITY-ANALYSIS | Review PR against merged plan + acceptance criteria        | → MERGED / ARCHITECT / ESCALATED |
| MERGED           | Pipeline complete, feature reviewed                        | → IDLE (next feature)            |
| FAILED           | Node execution error                                       | → retrying / BLOCKED             |
| BLOCKED          | Max retries exceeded, feature cannot continue              | → IDLE (skip to next feature)    |
| TIMEOUT          | Global timeout reached                                     | → BLOCKED / IDLE                 |
| WAITING_HUMAN    | Requires human intervention                                | → RESUMED (after human fix)      |
| ESCALATED        | Quality-Analysis found a critical issue, escalated         | → WAITING_HUMAN                  |
| SKIPPED          | Feature skipped (cooldown, priority)                       | → IDLE (next feature)            |
| RESUMED          | After human intervention, pipeline restarts from Architect | → ARCHITECT                      |

### Recovery Policy (State Persist Across Restarts)

When the engine restarts (crash, reboot, maintenance):

1. **Auto-resume** if state < 1 hour old: Read `state.json`, validate state, resume from last state
2. **Mark failed** if state > global timeout (180 minutes): State is considered stale, marked FAILED
3. **Wait** if state = WAITING_HUMAN: No action, waiting for human intervention
4. **Skip** if feature is SKIPPED: Proceed to next feature

```
restart detected
    │
    ├─ state.json exists?
    │    ├─ NO → IDLE
    │    └─ YES → read state
    │              │
    │              ├─ current_state == WAITING_HUMAN → stay (wait)
    │              ├─ elapsed < global_timeout → RESUME from current state
    │              ├─ elapsed >= global_timeout → mark BLOCKED → IDLE (next)
    │              └─ state invalid/corrupted → mark FAILED → IDLE (next)
    │
    └─ recovery log written
```

---

## Feature Coordination

### Problem

Business-Analyst and Tech-Lead must analyze the same folder without conflicts.

### Solution

- Both agents receive **identical feature context** (shared contract)
- Feature is locked when entering the pipeline (queue claim lock)
- Folder boundary enforced via skill instructions
- FRD snapshot is taken at dispatch, hashed to ensure the same version
- **Queue claim lock:** Marks a feature as being processed (state file). Prevents duplicate dispatch.
- **File system lock:** Feature folder is locked while the pipeline is active. Business-Analyst and Tech-Lead can still run in parallel because they only read, not write to the same folder.

---

## Feature Priority Queue

User defines the priority order manually:

```
Priority  Category   Features
─────────────────────────────────────────────────────
1-3       CORE       shared → filesystem → config-system
4-8       RULES      naming-rules → import-rules → quality-rules → role-rules → orphan-rules
9         TOOLS      auto-fix
10-12     SURFACE    cli-commands → mcp-server → tui
```

Engine processes from priority 1 to N sequentially.

### Feature Queue State Machine

```
                    ┌──────────┐
                    │ PENDING  │  ← new feature detected
                    └────┬─────┘
                         │ claim
                         ▼
                    ┌──────────┐
                    │ LOCKED   │  ← queue claim lock acquired
                    └────┬─────┘
                         │ dispatch agents
                         ▼
                    ┌──────────┐
                    │ ACTIVE   │  ← pipeline is running
                    └────┬─────┘
                    ┌────┼──────────┬───────────┐
                    ▼    ▼          ▼           ▼
              ┌────────┐ ┌──────┐ ┌─────────┐ ┌──────────┐
              │  DONE  │ │FAILED│ │ BLOCKED │ │SKIPPED   │
              │(merged)│ │      │ │         │ │          │
              └────────┘ └──┬───┘ └────┬────┘ └──────────┘
                            │          │
                            │ cooldown │ skip to next
                            ▼          ▼
                    ┌──────────┐   ┌──────────┐
                    │ WAITING  │   │ PENDING  │ (next feature)
                    └──────────┘   └──────────┘
```

### Feature Failure Behavior

| Status                 | Behavior                             | Next                              |
| ---------------------- | ------------------------------------ | --------------------------------- |
| DONE                   | Feature complete, release lock       | → PENDING (next feature)         |
| FAILED (retryable)     | Transient error, retry with cooldown | → ACTIVE (retry)                 |
| FAILED (non-retryable) | Permanent error, requires human fix  | → WAITING_HUMAN                  |
| BLOCKED                | Max retries exceeded                 | → PENDING (skip to next feature) |
| SKIPPED                | Cooldown or lower priority           | → PENDING (next feature)         |

### Priority Override

If a high-priority feature is BLOCKED:

1. That feature enters WAITING_HUMAN status
2. Engine **proceeds to the next feature** (no stall)
3. When human fix is complete, the BLOCKED feature can be requeued with its original priority

### Dedup

- **Cooldown:** Do not process the same feature within 1 hour
- **Pipeline iteration counter:** Max 5 full pipeline runs per feature (reset for each new feature)
- **Rejection loop counter:** Max 3 rejections per pipeline (reset for each new pipeline)

---

## Two Counter System

### Rejection Loop Counter

- **Counts:** How many times Quality-Analysis sent REJECT to Architect
- **Reset:** Each new pipeline (after dispatch)
- **Max:** 3
- **If max reached:** Pipeline → ESCALATED → WAITING_HUMAN

### Pipeline Iteration Counter

- **Counts:** How many times the pipeline ran fully from IDLE to verdict
- **Reset:** Each new feature
- **Max:** 5
- **If max reached:** Feature → BLOCKED → engine proceeds to next feature

```
Pipeline Run 1: IDLE → ... → QUALITY-ANALYSIS → REJECT → ARCHITECT
  rejection_loop_counter: 1
  pipeline_iteration_counter: 1

Pipeline Run 2: ARCHITECT → DEVELOPER → QUALITY-ANALYSIS → REJECT → ARCHITECT
  rejection_loop_counter: 2
  pipeline_iteration_counter: 2

Pipeline Run 3: ARCHITECT → DEVELOPER → QUALITY-ANALYSIS → REJECT
  rejection_loop_counter: 3 (max) → ESCALATED → WAITING_HUMAN
  pipeline_iteration_counter: 3
```

---

## Quality-Analysis Severity Handling

Quality-Analysis classifies findings into 3 severity levels:

| Severity           | Action                                   | Example                                                         |
| ------------------ | ---------------------------------------- | --------------------------------------------------------------- |
| **Minor**    | Comment only, does not block             | Style inconsistency, naming suggestion                          |
| **Major**    | REJECT, send to Architect for re-merge   | Missing test, logic error, AES violation                        |
| **Critical** | ESCALATE to Architect, Architect decides | Security vulnerability, data loss risk, architectural violation |

### Critical Path

```
Quality-Analysis ──[CRITICAL]──▶ Architect
        │
        ├─ Architect can fix → update merged plan → Developer
        │
        └─ Architect cannot fix → ESCALATED → WAITING_HUMAN
```

### Major Path

```
Quality-Analysis ──[MAJOR/REJECT]──▶ Architect
        │
        └─ Architect re-merge → Developer (fix) → Quality-Analysis
```

### Minor Path

```
Quality-Analysis ──[MINOR/APPROVE]──▶ MERGED
        │
        └─ Comments added to PR, no re-merge needed
```

---

## Trigger Types

| Type              | Source           | When                                   |
| ----------------- | ---------------- | -------------------------------------- |
| Event             | GitHub           | Label changed (e.g., "need review")    |
| Schedule          | Cron             | Quality-Analysis scan every 15 minutes |
| State-Change      | Filesystem       | Report written, CI status              |
| Parallel Complete | Engine           | Both Business-Analyst + Tech-Lead done |
| Rejection         | Quality-Analysis | Verdict = REJECT                       |
| Escalation        | Agent            | CRITICAL issue or unresolved blocker   |

### Trigger Guard (Loop Prevention)

To prevent trigger loops (engine writes report → trigger fires → engine runs again):

| Guard                       | Description                                                                    |
| --------------------------- | ------------------------------------------------------------------------------ |
| **Correlation ID**    | Each pipeline run has a unique ID. Trigger with the same ID is ignored         |
| **Bot Event Filter**  | GitHub events from bots (GitHub Actions, dependabot) are ignored               |
| **Idempotency Check** | Before processing trigger, check if feature is already active/not yet complete |
| **Trigger Debounce**  | Same trigger within 30 seconds is debounced                                    |
| **State Guard**       | Triggers are only processed when state = IDLE                                  |

```
trigger received
    │
    ├─ correlation_id matches active pipeline? → IGNORE (loop detected)
    ├─ sender is bot? → IGNORE
    ├─ feature already ACTIVE/LOCKED? → IGNORE
    ├─ same trigger within 30s? → DEBOUNCE (ignore)
    ├─ state != IDLE? → QUEUE (wait for IDLE)
    └─ all clear → PROCESS trigger
```

---

## Prompt System

### Prompt Assembly — Single Text Before Dispatch

The Dispatcher is responsible for combining **Feature Context** + **Role Prompt** into **one complete text block** before sending to `qwen -p "$prompt" -o text`. The agent receives a single block of text, not multiple separate parts.

```
┌─────────────────────────────────────────────────────┐
│  DISPATCHER                                          │
│                                                      │
│  1. Read config.yaml (node settings, skills)         │
│  2. Read /home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/prompts/<role>.txt (role template)         │
│  3. Read FRD snapshot (hash verified)                │
│  4. Combine:                                        │
│     ┌──────────────────────────────────────┐        │
│     │  Feature Context (header)             │        │
│     │  +-----------------------------------│        │
│     │  Role Prompt (body)                   │        │
│     │  +-----------------------------------│        │
│     │  Shared Acceptance Criteria (footer)  │        │
│     └──────────────────────────────────────┘        │
│  5. Replace placeholders: {{FEATURE}}, {{PATH}},    │
│     {{FRD}}, {{FRD_HASH}}, {{CORRELATION_ID}},      │
│     {{DATE}}, {{ABSOLUTE_PROJECT_ROOT}}             │
│  6. Send 1 text to qwen -p "$prompt" -o text        │
└─────────────────────────────────────────────────────┘
```

### Prompt Structure (Single Complete Text)

```text
## Feature Context
- Feature: shared
- Feature Path: /home/raka/mcp-arwaky/lint-arwaky/crates/shared
- FRD Path: /home/raka/mcp-arwaky/lint-arwaky/crates/shared/FRD.md
- FRD Hash: sha256:abc123def456...
- Project Root: /home/raka/mcp-arwaky/lint-arwaky
- Correlation ID: pipeline-20260807-451
- Pipeline Iteration: 1/5
- Rejection Loop: 0/3
- Rule: Only analyze files within Feature Path

## Role: Business-Analyst
(workflow conversation from /home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/prompts/business-analyst.txt)

## Shared Acceptance Criteria
- [ ] All findings must have evidence (file + line number)
- [ ] All recommendations must be actionable
- [ ] Report must follow the specified output schema
- [ ] FRD snapshot must be consistent with the recorded hash
```

### Path Convention — Always Absolute

All paths in the prompt must use **absolute paths** from the project root:

| Field             | Format                                                                           | Example                                                                                               |
| ----------------- | -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `Project Root`  | `/home/<user>/<workspace>`                                                     | `/home/raka/mcp-arwaky/lint-arwaky`                                                                 |
| `Feature Path`  | `<Project Root>/<category>/<feature>`                                          | `/home/raka/mcp-arwaky/lint-arwaky/crates/shared`                                                   |
| `FRD Path`      | `<Feature Path>/FRD.md`                                                        | `/home/raka/mcp-arwaky/lint-arwaky/crates/shared/FRD.md`                                            |
| `Report Output` | `<Project Root>/.agents/graph-loop/results/<role>-<feature>.md`                | `/home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/results/business-analyst-shared.md`           |
| `Merged Plan`   | `<Project Root>/.agents/graph-loop/plans/merged-<feature>-<correlation_id>.md` | `/home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/plans/merged-shared-pipeline-20260807-451.md` |

**Why absolute paths:**

- Agents run in different working directories (project root)
- No ambiguity about file locations
- Facilitates debugging and logging
- Consistent across all nodes (Business-Analyst, Tech-Lead, Architect, Developer, Quality-Analysis)

### Template Placeholders

| Placeholder                | Value                           | Source                                                                      |
| -------------------------- | ------------------------------- | --------------------------------------------------------------------------- |
| `{{FEATURE}}`            | Feature name                    | config.yaml → features queue                                               |
| `{{FEATURE_PATH}}`       | Absolute path to feature folder | `<Project Root>/<category>/<feature>`                                     |
| `{{FRD_PATH}}`           | Absolute path to FRD            | `<Feature Path>/FRD.md`                                                   |
| `{{FRD_HASH}}`           | SHA256 hash of FRD content      | Computed at dispatch                                                        |
| `{{PROJECT_ROOT}}`       | Absolute path to project root   | config.yaml → project_root                                                 |
| `{{CORRELATION_ID}}`     | Unique pipeline run ID          | Created at dispatch                                                         |
| `{{PIPELINE_ITERATION}}` | Pipeline iteration counter      | state.json → pipeline_iteration_counter                                    |
| `{{REJECTION_LOOP}}`     | Rejection loop counter          | state.json → rejection_loop_counter                                        |
| `{{DATE}}`               | Dispatch timestamp              | `date -Iseconds`                                                          |
| `{{ROLE_TEMPLATE}}`      | Prompt template file content    | `/home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/prompts/<role>.txt` |

### Skip Report Template

```text
## Skip Report — Business-Analyst

- Feature: shared
- Feature Path: /home/raka/mcp-arwaky/lint-arwaky/crates/shared
- Skipped: YES
- Reason: Simple fix (hotfix)
- Skipped at: 2026-08-07T10:00:00+07:00

### Unvalidated Assumptions
- [ ] Business logic correctness: ASSUMED VALID
- [ ] Requirements traceability: ASSUMED VALID
- [ ] Edge case coverage: NOT CHECKED

### Architect Action Required
Architect must explicitly validate the assumptions above before producing merged plan.
```

---

## Role Prompt Workflows (Detail)

Each node has a workflow defined in its skill file. Below are the workflow details injected into the prompt.

### Business-Analyst Workflow

**Skill:** `.agents/skills/role-business-analyst/SKILL.md`

**Prerequisites (must read before analysis):**

1. `.agents/rules/RULES_AES.md` — architectural constraints
2. `ARCHITECTURE.md` — 7-layer context
3. `PRD.md` — product context

**Workflow (sequential, no skips):**

```
1. IDENTIFY
   - Locate: /home/raka/mcp-arwaky/lint-arwaky/<category>/<feature>/
   - Read FRD.md
   - List modules + responsibilities

2. REFERENCE
   - RULES_AES.md Groups 2 & 4 (import + role constraints)
   - Map each FRD requirement to code files
   - Rule: 1 FR = 1 capabilities file + 1 contract protocol

3. ANALYZE (5 dimensions)
   ┌─────────────────────┬────────────────────────────────────┐
   │ Dimension           │ Focus                              │
   ├─────────────────────┼────────────────────────────────────┤
   │ Requirements Clarity│ Unambiguous, complete, consistent  │
   │ Business Flow       │ Matches spec, edge cases handled  │
   │ Logic Implementation│ FRD→code correct, no missing paths│
   │ Testability         │ Verifiable, acceptance criteria   │
   │ Traceability        │ FRD→code/tests/config traceable   │
   └─────────────────────┴────────────────────────────────────┘

4. CHECK HISTORY
   - Check features.json: has the feature already been completed?
   - If DONE → skip, report "Already processed"
   - If not → proceed to Step 5

5. PLAN
   - Save: .agents/plans/todo-<feature>-business-analyst-<timestamp>.md
   - All findings, severity-categorized
   - Include fixed code
```

**Output Template:**

```markdown
# Plan: {feature} — Business Analyst

## Summary
{One paragraph}

## Findings

### Requirements Clarity
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Business Flow
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Logic Implementation
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Testability & Acceptance
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Traceability (FRD→Code)
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

## Violations
{List or "None"}

## Action Items
- [ ] {Priority} {Item}

## Fixed Code
{Grouped by file}
```

**Severity Levels:**

| Level       | Meaning                                                       |
| ----------- | ------------------------------------------------------------- |
| 🔴 CRITICAL | Missing core requirement, wrong logic, data integrity risk    |
| 🟡 WARNING  | Ambiguous requirement, missing edge case, incomplete criteria |
| 🟢 INFO     | Suggestion or optimization, deferrable                        |

---

### Tech-Lead Workflow

**Skill:** `.agents/skills/role-tech-lead/SKILL.md`

**Prerequisites:**

1. `.agents/rules/RULES_AES.md` (Groups 3-4)
2. `ARCHITECTURE.md` — 7-layer spec
3. `PRD.md` — product context

**Workflow (sequential, no skips):**

```
1. IDENTIFY
   - Locate: /home/raka/mcp-arwaky/lint-arwaky/<category>/<feature>/
   - Read FRD.md
   - List affected files

2. REFERENCE
   - RULES_AES.md Groups 3 (AES301-305) & 4 (AES401-406)
   - ARCHITECTURE.md expected patterns

3. ANALYZE (6 dimensions)
   ┌───────────────────┬────────────────────────────────────────┐
   │ Dimension         │ Focus                                  │
   ├───────────────────┼────────────────────────────────────────┤
   │ Security          │ Vuln, injection, auth bypass, leaks    │
   │ Performance       │ Bottleneck, O(n²), unnecessary alloc   │
   │ Error Handling    │ Unwrap, panic, missing error path      │
   │ SOLID             │ SRP, OCP, DIP violations               │
   │ Code Quality      │ Duplication, complexity, naming        │
   │ Maintainability   │ Coupling, readability, documentation   │
   └───────────────────┴────────────────────────────────────────┘

4. CHECK HISTORY
   - Check features.json: has the feature already been completed?
   - If DONE → skip, report "Already processed"
   - If not → proceed to Step 5

5. PLAN
   - Save: .agents/plans/todo-<feature>-tech-lead-<timestamp>.md
```

**Output Template:**

```markdown
# Plan: {feature} — Tech Lead

## Summary
{One paragraph}

## Findings

### Security
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Performance
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Error Handling
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### SOLID
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Code Quality
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Maintainability
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

## Action Items
- [ ] {Priority} {Item}

## Fixed Code
{Grouped by file}
```

**Severity Levels:**

| Level       | Meaning                                          |
| ----------- | ------------------------------------------------ |
| 🔴 CRITICAL | Security vuln, data leak, crash risk             |
| 🟡 WARNING  | Perf bottleneck, SOLID violation, bypass pattern |
| 🟢 INFO     | Nice-to-have, deferrable                         |

---

### Architect Workflow

**Skill:** `.agents/skills/role-architect/SKILL.md`

**Prerequisites:**

1. `.agents/rules/RULES_AES.md` (rules 101-506)
2. `ARCHITECTURE.md` — 7-layer spec
3. `PRD.md` — product context

**Workflow (sequential, no skips):**

```
1. IDENTIFY
   - Locate: /home/raka/mcp-arwaky/lint-arwaky/<category>/<feature>/
   - Read FRD.md
   - List modules

2. REFERENCE
   - RULES_AES.md Groups 1-5
   - Classify files: taxonomy|contract|utility|capabilities|agent|surface|root

3. ANALYZE (7 dimensions)
   ┌───────────────────┬────────────────────────────────────────┐
   │ Dimension         │ Focus                                  │
   ├───────────────────┼────────────────────────────────────────┤
   │ Naming            │ Convention compliance (AES101-102)     │
   │ Boundaries        │ Import rules, dependency direction     │
   │ Capabilities      │ Protocol impl (AES301-305)             │
   │ Agent             │ Aggregate impl (AES401-406)            │
   │ Orphan            │ Dead code (AES501-506)                 │
   │ Scalability       │ SRP, coupling                          │
   │ Data Flow         │ Unidirectional, no cycles              │
   └───────────────────┴────────────────────────────────────────┘

4. CHECK HISTORY
   - Check features.json: has the feature already been completed?
   - If DONE → skip, report "Already processed"
   - If not → proceed to Step 5

5. MERGE (Architect-specific)
   - Read BA report + TL report (or Skip Report)
   - Combine findings into one merged plan
   - Resolve conflicts between BA and TL
   - Validate AES compliance
   - Final certification with timestamp + correlation ID

6. PLAN
   - Save: .agents/plans/todo-<feature>-architect-<timestamp>.md
```

**Architect Certification (additional steps):**

1. Validate merged plan against FRD
2. Validate AES compliance
3. Validate Skip Report (if any nodes were skipped)
4. Validate untested assumptions
5. Sign merged plan with timestamp + correlation ID

**Output Template:**

```markdown
# Plan: {feature} — Architect (Merged Plan)

## Summary
{One paragraph — synthesis of BA + TL findings}

## Merged Findings

### Layer Boundaries
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Naming
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Orphan
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Scalability
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Data Flow
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

## Validation
- [ ] FRD compliance checked
- [ ] AES compliance checked
- [ ] Skip Report validated (if any)
- [ ] Assumptions validated
- [ ] Timestamp + Correlation ID signed

## Action Items
- [ ] {Priority} {Item}

## Fixed Code
{Grouped by file}
```

**Severity Levels:**

| Level       | Meaning                              |
| ----------- | ------------------------------------ |
| 🔴 CRITICAL | Layering breach, security, data leak |
| 🟡 WARNING  | Convention/perf/maintainability      |
| 🟢 INFO     | Suggestion, deferrable               |

---

### Developer Workflow

**Skill:** `.agents/skills/role-fullstack-developer/SKILL.md`

**Prerequisites:**

1. Merged plan from Architect
2. `.agents/rules/RULES_AES.md`
3. `ARCHITECTURE.md`

**Workflow (sequential, no skips):**

```
1. READ MERGED PLAN
   - Read .agents/plans/todo-<feature>-architect-<timestamp>.md
   - Understand all findings and action items

2. IMPLEMENT
   - Execute action items by priority
   - Follow recommended fixed code
   - Create branch: worktree-<feature>
   - Commit with conventional commits

3. SELF-VERIFY
   - Run: cargo check, cargo clippy, cargo fmt
   - Run: cargo nextest run -p <crate>
   - Run: bash scripts/gates.sh
   - Ensure 0 violations

4. CREATE PR
   - Push to remote
   - Create PR to develop
   - Label: "need review"
   - Body: link to merged plan

5. WRITE REPORT
   - Save: .agents/reports/done-<feature>-<timestamp>.md
   - Include: PR number, changes made, test results
```

**Output Template:**

```markdown
# Execution Report: {feature} — Developer

## PR Info
- **PR:** #{number}
- **Branch:** worktree-{feature} → develop
- **Merged Plan:** .agents/plans/todo-{feature}-architect-{timestamp}.md

## Changes Made
{List of files changed and what was done}

## Self-Verification
| Gate | Result |
|------|--------|
| cargo check | ✅/❌ |
| cargo clippy | ✅/❌ |
| cargo fmt | ✅/❌ |
| cargo nextest | ✅/❌ |
| gates.sh | ✅/❌ |

## Test Results
{Summary of test output}
```

---

### Quality-Analysis Workflow

**Skill:** `.agents/skills/role-quality-analysis/SKILL.md`

**Prerequisites:**

1. `.agents/rules/RULES_AES.md`
2. `ARCHITECTURE.md`
3. `TEST.md`
4. `scripts/gates.sh`
5. `CONTRIBUTING.md`

**Workflow (sequential, no skips):**

```
1. IDENTIFY PR
   - Filter: gh pr list --label "need review"
   - Pick oldest PR targeting develop
   - Add "in progress" label

2. VALIDATE REPORT
   - Read .agents/reports/done-*.md
   - Verify accuracy and timestamp consistency

3. VERIFY CI
   - Run: gh pr checks <pr-number>
   - All checks must pass
   - Any fail = REJECT immediately

4. PRE-EXISTING TRIAGE
   - Compare develop vs PR branch
   - Pre-existing: Ignore
   - PR-introduced: Flag (CRITICAL/WARNING)
   - Resolved: Note positively

5. ANALYZE CODE
   Review diff for:
   - AES Compliance
   - Layer Boundaries
   - Quality Rules
   - Role Integrity
   - Orphan Detection
   - Contract Stability
   - Test Coverage
   - Security
   - Convention Adherence

6. VERDICT
   ├── APPROVED → Merge, clean labels, delete report
   └── REJECTED → Comment, write rejection plan
```

**Severity Levels:**

| Level       | Meaning                                                              |
| ----------- | -------------------------------------------------------------------- |
| 🔴 CRITICAL | CI fail, AES violation, layer breach, security risk, test regression |
| 🟡 WARNING  | Convention deviation, missing test, inaccurate report                |
| 🟢 INFO     | Style/optimization, follow-up                                        |

**Verdict Rules:**

| Verdict  | When                                             | Action                  |
| -------- | ------------------------------------------------ | ----------------------- |
| APPROVED | All CI pass, 0 PR-introduced CRITICAL/WARNING    | Merge, delete report    |
| REJECTED | CI fails OR PR-introduced CRITICAL/WARNING exist | Comment, write new plan |

**Rejection Plan Template:**

```markdown
# Review Plan: {feature} — Quality Analysis (Rejection)

## PR Info
- **PR:** #{number} — {title}
- **Branch:** {source} → develop
- **Reason:** {one-line summary}

## CI Gate Results
| Gate | Result | Details |
|------|--------|---------|

## Findings to Fix

### AES Violations
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

### Test Issues
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

### Code Quality
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

### Report Inaccuracies
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

## Action Items & Fixed Code
- [ ] {Priority} {Specific fix}
{Corrected code blocks}
```

---

## Conditional Skip

| Node                       | Skip When                       | Never Skip When                        | Replacement                       |
| -------------------------- | ------------------------------- | -------------------------------------- | --------------------------------- |
| **Business-Analyst** | Simple fix (hotfix, typo, docs) | feature affects business logic         | Skip Report (assumptions flagged) |
| **Tech-Lead**        | Doc-only update                 | Security/auth/token/encryption related | Skip Report (assumptions flagged) |
| **Architect**        | **Never skipped**         | —                                     | —                                |
| **Developer**        | **Never skipped**         | —                                     | —                                |
| **Quality-Analysis** | **Never skipped**         | —                                     | — (auto-approve if doc-only)     |

### Important Note: Quality-Analysis Never Skipped

Quality-Analysis is the final gatekeeper that is never skipped. For doc-only changes:

- Quality-Analysis still runs
- Verdict is automatically: **APPROVE with severity MINOR** (comment only, does not block)
- This ensures every pipeline always has a final gate

### Architect Certification

Architect does not just merge — it performs **final certification**:

1. Validate merged plan against FRD
2. Validate AES compliance
3. **Validate Skip Report** (if any nodes were skipped)
4. **Validate assumptions** that have not been tested (if any)
5. Sign merged plan with timestamp + correlation ID

---

## 24/7 Operation

Engine runs as a background service (`graph-loop.service`), polling every 30 seconds:

1. Scan triggers (GitHub PR, cron, filesystem)
2. Apply trigger guard (correlation ID, bot filter, idempotency)
3. Claim feature based on priority (queue claim lock)
4. Dispatch agents in parallel/sequentially
5. Handle timeout, rejection, escalation
6. Handle failure states (FAILED, BLOCKED, TIMEOUT, ESCALATED, SKIPPED)
7. Log all activity with correlation ID

### Feature Queue Continuity

If a high-priority feature fails (BLOCKED/ESCALATED):

1. That feature enters WAITING_HUMAN
2. Engine immediately proceeds to the next feature
3. No stall — pipeline keeps running
4. When human fix is complete, the feature can be requeued

```
Queue: [shared(1), filesystem(2), config-system(3)]

shared → BLOCKED (max retries)
  → engine proceeds to filesystem
  → filesystem → DONE
  → engine proceeds to config-system
  → shared requeued manually by human → processed after config-system
```

---

## Locking Strategy

### Queue Claim Lock

- **Purpose:** Prevent two pipelines from claiming the same feature
- **Implementation:** State file (`state.json`) — feature ID + timestamp
- **Scope:** Feature-level (one feature at a time)
- **Release:** When feature DONE / BLOCKED / SKIPPED

### File System Lock

- **Purpose:** Prevent concurrent writes to the feature folder
- **Implementation:** Lock file in `locks_dir`
- **Scope:** Folder-level (feature folder is locked)
- **Release:** When pipeline completes

### Parallel Safe

Business-Analyst and Tech-Lead can run in parallel because:

- Both only **read** from the feature folder
- Both **write** reports to different folders (`/home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/results/`)
- Feature folder is not modified during analysis

---

## Notification

| Event                      | Action                         | Severity |
| -------------------------- | ------------------------------ | -------- |
| Pipeline started           | Log                            | INFO     |
| Node completed             | Log                            | INFO     |
| Node skipped               | Log + reason                   | WARN     |
| Pipeline completed         | Log + PR comment               | INFO     |
| Quality-Analysis approved  | Log + PR comment               | INFO     |
| Quality-Analysis rejected  | Log + PR comment               | WARN     |
| Quality-Analysis escalated | Log + PR comment + human alert | ERROR    |
| Timeout                    | Log + human alert              | ERROR    |
| Error (non-retryable)      | Log + human alert              | ERROR    |
| Waiting human              | Log + human alert              | CRITICAL |

---

## Observability

- **Dashboard:** Real-time pipeline status (state, feature, counters)
- **Health Check:** Periodic monitoring (disk, stuck states, errors)
- **Execution Log:** Full audit trail with correlation ID
- **Feature Queue:** Track status of each feature
- **Counter Dashboard:** Rejection loop counter + pipeline iteration counter per feature

---

## Changelog (Revision 2.0)

| #  | Issue                                                      | Change                                                                                                  |
| -- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| 1  | MERGED → ARCHITECT transition wrong                       | MERGED now transitions to IDLE (next feature), not ARCHITECT                                            |
| 2  | Quality-Analysis skippable = contradiction                 | Quality-Analysis is**never skipped**. For doc-only, auto-approve with MINOR                       |
| 3  | Business-Analyst/Tech-Lead skip → Architect without input | Skip Report added: placeholder with assumptions that Architect must validate                            |
| 4  | Tech-Lead validates code quality without code              | Tech-Lead changed to**pre-implementation technical review** (input: Feature + FRD)                |
| 5  | Use of BA, TL, QA abbreviations                            | All changed to full names: Business-Analyst, Tech-Lead, Quality-Analysis                                |
| 6  | Quality-Analysis input only PR                             | Input expanded: PR + merged plan + FRD + CI/test result + BA report + TL report                         |
| 7  | "Identical context" oversimplified                         | Added: shared contract, shared acceptance criteria, shared output schema, folder boundary, FRD snapshot |
| 8  | Ambiguity between rejection loop vs pipeline iteration     | Two counters explicitly defined with names, functions, and reset behavior                               |
| 9  | Fail graceful without supporting states                    | 6 failure states added: FAILED, TIMEOUT, ESCALATED, BLOCKED, SKIPPED, WAITING_HUMAN                     |
| 10 | Escalation only to Architect                               | Human escalation path added: ESCALATED → WAITING_HUMAN → RESUMED                                      |
| 11 | Quality-Analysis critical = guidance only                  | Severity handling added: minor=comment, major=reject, critical=escalate                                 |
| 12 | Feature queue sequential without failure behavior          | Failure behavior added: BLOCKED → skip to next, priority override, requeue                             |
| 13 | Trigger loop without guard                                 | Trigger guard added: correlation ID, bot filter, idempotency, debounce, state guard                     |
| 14 | Feature lock unclear                                       | Two locks defined: queue claim lock + file system lock. Parallel safe explained                         |
| 15 | State persist without recovery policy                      | Recovery policy added: resume (auto), mark failed (stale), wait (human), skip (skipped)                 |

---

## Design Decisions

| #  | Problem                               | Decision                                               | Rationale                                                                                                            | Rejected Alternatives                                                                                        |
| -- | ------------------------------------- | ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| 1  | MERGED → ARCHITECT wrong transition  | MERGED → IDLE (next feature)                          | New features must start from DISPATCHING, not ARCHITECT. ARCHITECT is only for re-merge                              | MERGED → IDLE directly (no intermediate state) — rejected because cleanup logic is needed                  |
| 2  | Quality-Analysis skippable            | Quality-Analysis is**never skipped**             | As the final gate, consistency matters more than optimization. Auto-approve for doc-only                             | Quality-Analysis skippable with Architect certification — rejected because it creates a gap in the pipeline |
| 3  | Architect without BA/TL input         | Skip Report as replacement                             | Architect still needs input. Skip Report flags assumptions that have not been validated                              | Architect guesses on its own — rejected because it violates single source of truth                          |
| 4  | Tech-Lead validates code without code | Pre-implementation technical review                    | Architecture/security/performance review is more useful before implementation than after                             | Tech-Lead reviews after PR — rejected because Developer has already wasted time                             |
| 5  | Node names inconsistent               | Full names everywhere                                  | Consistency prevents ambiguity. Matches skill file names                                                             | Abbreviations with deprecation notice — rejected because it creates dual naming                             |
| 6  | Quality-Analysis insufficient input   | 6 inputs: PR + plan + FRD + CI + BA report + TL report | Final gate needs full context for comprehensive validation                                                           | Only PR + plan — rejected because it cannot validate business logic                                         |
| 7  | "Identical context" oversimplified    | 5 coordination mechanisms                              | Shared contract + acceptance criteria + output schema + folder boundary + FRD snapshot comprehensively prevent drift | Only shared context — rejected because it is insufficient to prevent semantic drift                         |
| 8  | Two counters ambiguous                | Two separate counters with clear names                 | Rejection loop (QA → Architect) and pipeline iteration (trigger → verdict) are different concerns                  | Single counter — rejected because it conflates two independent failure modes                                |
| 9  | Fail graceful without states          | 6 failure states                                       | Each type of failure needs a different recovery path                                                                 | Only FAILED — rejected because it cannot handle timeout/escalation/human wait                               |
| 10 | Limited escalation                    | Human escalation path                                  | Architect cannot always resolve. Human intervention must exist as a last resort                                      | Escalation only loops to Architect — rejected because of infinite loop risk                                 |
| 11 | No severity                           | 3-level severity handling                              | Minor/major/critical have different actions. Not all findings need rejection                                         | All findings = reject — rejected because it is too aggressive for minor issues                              |
| 12 | Queue stall on failure                | Skip to next feature                                   | Pipeline must keep running. A blocked feature must not block everything                                              | Wait until fix — rejected because it can block the entire pipeline                                          |
| 13 | Trigger loop                          | 5 guard mechanisms                                     | Correlation ID + bot filter + idempotency + debounce + state guard prevent all types of loops                        | Single guard — rejected because it is insufficient for all edge cases                                       |
| 14 | Unclear locks                         | Two separate locks                                     | Queue claim lock (feature-level) and file system lock (folder-level) are different concerns                          | Single lock — rejected because it is either too broad or too narrow                                         |
| 15 | State lost on restart                 | Resume + stale detection + wait                        | 4 different policies for 4 restart situations                                                                        | Always resume — rejected because state can be stale. Always reset — rejected because progress is lost      |

---

## Open Questions

| # | Question                                                               | Options                                                           | Recommendation                                                        |
| - | ---------------------------------------------------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------------------- |
| 1 | How does a human provide a fix for WAITING_HUMAN?                      | (a) PR comment, (b) file edit, (c) manual state update            | (a) + (c): PR comment for context, manual state update for resume     |
| 2 | What is the cooldown between retries for FAILED state?                 | (a) Fixed 5 minutes, (b) Exponential backoff                      | (b) Exponential: 2^n minutes (max 30 minutes)                         |
| 3 | Should the pipeline iteration counter be a hard limit or configurable? | (a) Hardcoded 5, (b) In config.yaml                               | (b) In config.yaml for per-project flexibility                        |
| 4 | How to track correlation ID on GitHub PR?                              | (a) PR comment, (b) PR label, (c) Both                            | (c) Both: label for filtering, comment for audit trail                |
| 5 | Should Quality-Analysis auto-approve for doc-only log differently?     | (a) Normal log, (b) Special "auto-approved" log                   | (b) Special log for clear audit trail                                 |
| 6 | How to handle concurrent PRs for the same feature?                     | (a) Queue PRs, (b) Reject duplicates, (c) Merge queue             | (b) Reject duplicates with correlation ID check                       |
| 7 | Should Tech-Lead skip report have minimum validation?                  | (a) No validation, (b) Architect must validate, (c) Auto-validate | (b) Architect must validate — consistent with skip report philosophy |
| 8 | What is the maximum feature queue limit?                               | (a) Unlimited, (b) Max 20, (c) Max 50                             | (b) Max 20 — sufficient for this project, prevents memory bloat      |

---

## Implementation Notes

### State File Schema (Updated)

```json
{
  "version": "2.0",
  "pipeline": {
    "id": "pipeline-20260807-451",
    "feature": "shared",
    "current_state": "ANALYZING",
    "started_at": "2026-08-07T10:00:00+07:00",
    "iteration": 1,
    "rejection_loop_counter": 0,
    "pipeline_iteration_counter": 1,
    "correlation_id": "corr-20260807-shared-abc123",
    "project_root": "/home/raka/mcp-arwaky/lint-arwaky",
    "parallel_nodes": {
      "business-analyst": {
        "status": "completed",
        "task_id": "task-001",
        "report_file": "/home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/results/business-analyst-shared.md",
        "started_at": "2026-08-07T10:00:00+07:00",
        "completed_at": "2026-08-07T10:15:00+07:00"
      },
      "tech-lead": {
        "status": "completed",
        "task_id": "task-002",
        "report_file": "/home/raka/mcp-arwaky/lint-arwaky/.agents/graph-loop/results/tech-lead-shared.md",
        "started_at": "2026-08-07T10:00:00+07:00",
        "completed_at": "2026-08-07T10:20:00+07:00"
      }
    },
    "pending_merge": [],
    "failure": null,
    "escalation": null
  },
  "feature_queue": {
    "shared": { "status": "DONE", "priority": 1 },
    "filesystem": { "status": "ACTIVE", "priority": 2 }
  },
  "history": [],
  "pending_triggers": [],
  "triggers_log": []
}
```

### Config Schema Updates Required

Below are the config.yaml sections that need to be added/updated:

```yaml
# ── Project Root ─────────────────────────────────────────────────────
project_root: /home/raka/mcp-arwaky/lint-arwaky

# ── Counter Limits ──────────────────────────────────────────────────
counters:
  max_rejection_loops: 3
  max_pipeline_iterations: 5

# ── Recovery Policy ─────────────────────────────────────────────────
recovery:
  resume_if_stale_minutes: 60
  max_stale_minutes: 180
  exponential_backoff:
    initial_minutes: 2
    max_minutes: 30

# ── Trigger Guards ──────────────────────────────────────────────────
trigger_guards:
  debounce_seconds: 30
  ignore_bot_events: true
  correlation_id_required: true

# ── Lock Settings ───────────────────────────────────────────────────
locks:
  queue_claim_lock: true
  file_system_lock: true
  lock_timeout_minutes: 180
```

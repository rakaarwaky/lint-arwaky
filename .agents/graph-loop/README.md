# Graph Loop Pipeline (v2.0)

Automated state-machine pipeline that processes features through 5 AI agent roles sequentially.

## Overview

Graph Loop runs a complete pipeline for each feature:

```
BA + TL (parallel) → Architect → Developer → QA
```

Each role runs as a background `qwen` agent. The engine monitors agent status via files (`.result`, `.pid`), then transitions to the next stage automatically.

## Architecture

### State Machine

```
IDLE → DISPATCHING → ANALYZING → ARCHITECT → DEVELOPER → QUALITY-ANALYSIS → MERGED
                                ↑                                                    │
                                │              ← REJECTED (counter < 3) ←────────────┘
                                │
                                └── FAILED → (backoff) → ANALYZING (retry, max 5x)
                                                    └── BLOCKED (max retries exceeded)
```

**All states:**

| State | Description |
|-------|-------------|
| `IDLE` | Waiting for trigger (new PR or PENDING feature) |
| `DISPATCHING` | Spawning BA + TL in parallel |
| `ANALYZING` | Waiting for BA + TL to finish |
| `ARCHITECT` | Merging BA + TL reports into an implementation plan |
| `DEVELOPER` | Implementing changes + creating PR |
| `QUALITY-ANALYSIS` | QA reviews PR, APPROVED or REJECTED |
| `MERGED` | Pipeline complete, feature DONE |
| `FAILED` | Error occurred, auto-retry with backoff |
| `BLOCKED` | Max retries exceeded, requires manual intervention |
| `WAITING_HUMAN` | Escalation, waiting for human |
| `ESCALATED` | Critical issue from QA |

### Agent Roles

| Role | Purpose | Timeout | Retry |
|------|---------|---------|-------|
| **Business-Analyst** | Analyzes business requirements from FRD | 20 min | 2x |
| **Tech-Lead** | Reviews architecture and security | 30 min | 2x |
| **Architect** | Merges BA + TL reports into implementation plan | 30 min | 2x |
| **Developer** | Implements code changes + creates PR | 60 min | 3x |
| **Quality-Analysis** | Reviews PR (code review + test verification) | 30 min | No (escalate to human) |

### Parallel Pipeline

BA and TL run **simultaneously** (parallel) to speed up analysis. Architect waits for both to finish, then merges the results.

```
DISPATCHING
    ├── spawn BA (background)
    └── spawn TL (background)
         ↓
ANALYZING
    ├── BA running → completed
    └── TL running → completed
         ↓
ARCHITECT (merge BA + TL reports)
```

## Installation

### Prerequisites

- Python 3.10+
- `qwen` CLI (Qwen Code) — must be callable from terminal
- `gh` CLI (GitHub CLI) — for PR scanning and commenting
- `notify-send` (optional) — desktop notifications

### Setup

```bash
# Install Python dependencies
pip install -r .agents/graph-loop/requirements.txt

# Verify installation
python3 .agents/graph-loop/src/main.py
```

### Systemd Service (optional)

To run the engine as a background service:

```bash
# Copy service file
sudo cp .agents/graph-loop/graph-loop.service /etc/systemd/system/
sudo systemctl daemon-reload

# Start service
sudo systemctl start graph-loop

# Enable on boot
sudo systemctl enable graph-loop

# Check status
sudo systemctl status graph-loop

# View logs
journalctl -u graph-loop -f
```

## Usage

### CLI

```bash
# Run engine (loops until all features are processed)
python3 .agents/graph-loop/src/main.py engine start

# Run once (no loop)
python3 .agents/graph-loop/src/main.py engine once

# Recover from crash
python3 .agents/graph-loop/src/main.py engine recover

# Check pipeline status
python3 .agents/graph-loop/src/main.py engine status

# Full dashboard
python3 .agents/graph-loop/src/main.py dashboard full

# Partial dashboard
python3 .agents/graph-loop/src/main.py dashboard status
python3 .agents/graph-loop/src/main.py dashboard nodes
python3 .agents/graph-loop/src/main.py dashboard features
python3 .agents/graph-loop/src/main.py dashboard activity
python3 .agents/graph-loop/src/main.py dashboard metrics

# Health check
python3 .agents/graph-loop/src/main.py health report
```

### Dashboard

The dashboard displays pipeline information in a color-coded terminal output:

```
╔══════════════════════════════════════════════════════════════╗
║         GRAPH LOOP PIPELINE DASHBOARD (v2.0 / Python)        ║
╚══════════════════════════════════════════════════════════════╝

━━━ Pipeline Status ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  State:         DEVELOPER
  Feature:       cli-commands
  Rejection:     0/3
  Iteration:     1/5

━━━ Feature Queue ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total features: 12
  DONE: 3
  ACTIVE: 1
  PENDING: 8
```

## Configuration

File: `.agents/graph-loop/config.yaml`

Contains only **project-specific** values. All design constants are hardcoded in `config.py`.

```yaml
# Project root
project_root: "/home/raka/mcp-arwaky/lint-arwaky"

# Feature queue (priority order)
features:
  - name: "shared"
    priority: 1
    path: "/home/raka/mcp-arwaky/lint-arwaky/crates/shared"
  - name: "filesystem"
    priority: 2
    path: "/home/raka/mcp-arwaky/lint-arwaky/crates/filesystem"
  # ... etc

# Paths relative to project_root
paths:
  state_file: ".agents/graph-loop/state.json"
  features_file: ".agents/graph-loop/features.json"
  results_dir: ".agents/graph-loop/results"
  plans_dir: ".agents/graph-loop/plans"
  reports_dir: ".agents/graph-loop/reports"
  prompts_dir: ".agents/graph-loop/prompts"
  locks_dir: ".agents/graph-loop/locks"

# Skip rules (optional)
skip_rules:
  business-analyst:
    skip_when:
      - pr_title_matches: "^(hotfix|fix|typo|docs)"
      - complexity: "simple"
  tech-lead:
    skip_when:
      - pr_title_matches: "^(docs|readme)"
      - code_files_count: 0
    never_skip_when:
      - pr_title_matches: "(security|auth|token)"
```

### Hardcoded Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `POLL_INTERVAL` | 30 seconds | State check interval |
| `GLOBAL_TIMEOUT` | 180 minutes | Pipeline timeout before BLOCKED |
| `MAX_REJECTION_LOOPS` | 3 | QA rejections before escalation |
| `MAX_PIPELINE_ITERATIONS` | 5 | Max retries before BLOCKED |
| `RESUME_STALE_MINUTES` | 60 | Auto-resume if stale < 60 min |
| `BACKOFF_INITIAL` | 2 minutes | Initial backoff after failure |
| `BACKOFF_MAX` | 30 minutes | Maximum backoff |
| `DEBOUNCE_SECONDS` | 30 | Debounce between triggers |
| `COOLDOWN_MINUTES` | 60 | Cooldown between pipelines for the same feature |

## File Structure

```
.agents/graph-loop/
├── config.yaml              # Project-specific configuration
├── src/
│   ├── main.py              # CLI entry point
│   ├── engine.py            # State machine + main loop
│   ├── state.py             # state.json management (atomic I/O)
│   ├── config.py            # config.yaml reader + hardcoded constants
│   ├── dispatcher.py        # Prompt assembly + agent execution
│   ├── parallel.py          # Parallel BA + TL spawn
│   ├── features.py          # Feature queue + lock + cooldown
│   ├── scanner.py           # Trigger scanner (PR + debounce)
│   ├── skip.py              # Conditional skip logic
│   ├── notify.py            # Notifications (log, PR comment, desktop, webhook)
│   ├── health.py            # Health checks (engine, agent, disk, log)
│   ├── dashboard.py         # Terminal dashboard
│   └── common.py            # Shared helpers (time, subprocess, logging)
├── prompts/
│   └── templates/           # Prompt templates for each role
│       ├── context.txt
│       ├── business-analyst.txt
│       ├── tech-lead.txt
│       ├── architect.txt
│       ├── developer.txt
│       └── quality-analysis.txt
├── state.json               # Pipeline state (auto-generated)
├── features.json            # Feature status (auto-generated)
├── engine.pid               # Engine process PID
├── execution.log            # Execution log
├── notifications.log        # Notification log
├── health.log               # Health check log
├── results/                 # Agent output (.result, .pid)
├── plans/                   # Merged plans from Architect
├── reports/                 # Done reports from Developer
├── prompts/generated/       # Generated prompt files
├── locks/                   # Per-feature lock files
├── graph-loop.service       # Systemd service file
├── requirements.txt         # Python dependencies (PyYAML)
└── README.md                # This file
```

## Workflow

### 1. Trigger

Engine starts in `IDLE` state and searches for triggers:

- **PR trigger**: Scans GitHub PRs with label `need review`
- **Feature trigger**: Selects PENDING features by priority order

### 2. Dispatching + Analyzing

BA and TL run in parallel:

```
spawn("business-analyst") → background qwen
spawn("tech-lead")         → background qwen
state → ANALYZING
```

Engine polls every 30 seconds:
- Checks if agent PID is still alive
- Checks if `.result` file exists and is non-empty
- If agent died without output → automatic re-dispatch

### 3. Architect

After BA + TL finish:
- If either was skipped → generates Skip Report
- Architect receives BA report + TL report
- Output: merged plan at `plans/merged-{feature}-{correlation_id}.md`

### 4. Developer

- Reads merged plan
- Creates a new worktree (`git worktree add`)
- Implements changes
- Creates PR on GitHub
- Output: done report at `reports/done-{feature}-{correlation_id}.md`

### 5. Quality-Analysis

- Reviews PR (code review + test verification)
- Verifies acceptance criteria from FRD
- Verifies architecture from merged plan
- Verifies quality from Tech-Lead report

**Verdict:**
- `APPROVED` → PR merged, feature DONE
- `REJECTED` → Counter +1, restart from Architect (max 3x)
- Counter max → ESCALATED to human

### 6. Cleanup

Pipeline automatically cleans up files at each transition:
- Prompt files that have been consumed
- PID files of finished agents
- Result files that have been processed
- Plans/reports that have been merged

## Recovery

Engine supports automatic recovery on restart.

### 3-Tier Recovery

1. **Resume** (< 60 min stale): Continue from last known state
2. **Resume with caution** (60-180 min stale): Continue with warning
3. **BLOCKED** (>= 180 min stale): Mark BLOCKED, requires manual intervention

### Dead Process Detection

If an agent process dies without producing output:
1. Check PID file → `os.kill(pid, 0)` → process dead
2. Check result file → missing or empty (0 bytes)
3. Automatic re-dispatch

### Recovery Output File Check

Before dispatching, engine checks if output already exists:
- Architect: checks `plans/merged-{feature}-*.md`
- Developer: checks `reports/done-{feature}-*.md`

If exists → skip dispatch, advance to next stage.

## Features

### Conditional Skip

Certain nodes can be skipped based on conditions:

- **BA**: Skip for hotfix/fix/typo/docs or simple features (<=3 files, <=200 LOC)
- **TL**: Skip for docs-only updates (unless security/auth)
- **Architect/Developer/QA**: Never skipped

### Debounce & Bot Filter

- **Debounce**: Same trigger not processed within 30 seconds
- **Bot filter**: PRs from bots (dependabot, renovate, etc.) are ignored
- **State guard**: Triggers only processed when state is IDLE

### Cooldown

Completed features (DONE) cannot be re-triggered for 60 minutes.

### Notifications

- **Log**: All events recorded in `execution.log`
- **PR Comment**: QA approve/reject commented on PR
- **Desktop**: Errors and timeouts sent via `notify-send`
- **Webhook**: Configurable for Slack/Discord delivery

## Troubleshooting

### Engine not running

```bash
# Check status
python3 .agents/graph-loop/src/main.py engine status

# Check health
python3 .agents/graph-loop/src/main.py health report

# Check recent logs
tail -50 .agents/graph-loop/execution.log
```

### Agent stuck

```bash
# Check agent PIDs
cat .agents/graph-loop/results/*.pid

# Check if process is alive
ps aux | grep "qwen -p"

# Force kill
kill $(cat .agents/graph-loop/results/feature-business-analyst.pid)
```

### Pipeline stuck in a state

```bash
# Check state
cat .agents/graph-loop/state.json | python3 -m json.tool

# Manual reset
echo '{"version":"2.0","pipeline":{"current_state":"IDLE"}}' > .agents/graph-loop/state.json
```

### Health check

```bash
# Full health check
python3 .agents/graph-loop/src/main.py health report

# Individual checks
python3 .agents/graph-loop/src/main.py health check-engine
python3 .agents/graph-loop/src/main.py health check-state
python3 .agents/graph-loop/src/main.py health check-disk
```

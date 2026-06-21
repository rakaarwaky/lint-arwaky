# Plan: Full Watch Implementation

## Current State

| Component | Status | Problem |
|---|---|---|
| `file-watch` crate | Partial | Snapshot-based polling via `read_dir`, no real FS events |
| `IWatchProviderPort` | Minimal | Only `start`/`stop`/`is_available`, no change detection callback |
| `WatchServiceProvider` | Stub | `start`/`stop` on trait are no-ops, real logic in non-trait methods |
| TUI `watch` | Stub | `sleep(2s)` + full re-lint loop, no change detection |
| CLI `watch` | Same as TUI | Same polling stub |

## Goal

Proper file watcher using Linux `inotify` (via `notify` crate) that:
- Detects file changes (create/modify/delete) in real-time
- Debounces rapid changes (avoid linting 50 times during `git checkout`)
- Runs lint only on changed files (incremental), with optional full re-lint
- Works from both TUI and CLI with identical behavior

---

## Architecture (AES Layers)

```
shared/file-watch/
  contract_provider_port.rs    ← IWatchProviderPort (extended)
  taxonomy_watch_event_vo.rs   ← NEW: WatchEvent VO
  taxonomy_watch_config_vo.rs  ← NEW: WatchConfig VO
  taxonomy_service_error.rs    ← existing

file-watch/
  infrastructure_notify_provider.rs  ← NEW: notify crate adapter
  capabilities_change_analyzer.rs    ← NEW: debounce + filter logic
  agent_watch_orchestrator.rs        ← NEW: orchestrate watch → lint pipeline
  root_file_watch_container.rs       ← UPDATE: wire new components
```

---

## Step 1: Extend Contract Layer (shared)

### 1a. `taxonomy_watch_event_vo.rs` (NEW)

```rust
pub struct WatchEvent {
    pub path: FilePath,
    pub kind: WatchEventKind,  // Created | Modified | Removed
    pub timestamp: SystemTime,
}

pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
}
```

### 1b. `taxonomy_watch_config_vo.rs` (NEW)

```rust
pub struct WatchConfig {
    pub path: FilePath,
    pub recursive: bool,
    pub debounce_ms: u64,           // default 500ms
    pub ignore_patterns: Vec<String>, // ".git", "node_modules", "__pycache__", "target"
    pub lint_changed_only: bool,     // true = incremental, false = full re-lint
}
```

### 1c. Extend `contract_provider_port.rs`

```rust
#[async_trait]
pub trait IWatchProviderPort: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> broadcast::Receiver<WatchEvent>;  // NEW
}
```

---

## Step 2: Infrastructure — `notify` Adapter

### 2a. Add dependency

`file-watch/Cargo.toml`:
```toml
[dependencies]
notify = "6"          # inotify on Linux, FSEvent on macOS
notify-debouncer-mini = "0.4"  # built-in debounce
tokio = { workspace = true, features = ["sync", "rt"] }
```

### 2b. `infrastructure_notify_provider.rs` (NEW)

- Implements `IWatchProviderPort`
- Uses `notify::RecommendedWatcher` (resolves to `inotify` on Linux)
- Uses `notify_debouncer_mini` for debounce
- Broadcasts `WatchEvent` via `tokio::sync::broadcast`
- Respects ignore patterns (skip `.git`, `node_modules`, `target`, `__pycache__`)

Key implementation:
```rust
pub struct NotifyWatchProvider {
    watcher: Mutex<Option<Debouncer<RecommendedWatcher>>>,
    tx: broadcast::Sender<WatchEvent>,
}
```

---

## Step 3: Capabilities — Change Analyzer

### 3a. `capabilities_change_analyzer.rs` (NEW)

- Receives raw `WatchEvent` stream
- Filters by language (`.rs`, `.py`, `.js`, `.ts`)
- Groups rapid changes into batches (debounce window)
- Deduplicates (same file modified 3x in 500ms → 1 event)
- Outputs `Vec<WatchEvent>` batches ready for lint

```rust
pub struct ChangeAnalyzer {
    debounce_window: Duration,
}

impl ChangeAnalyzer {
    pub fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        // dedupe by path, keep latest event per file
    }
}
```

---

## Step 4: Agent — Watch Orchestrator

### 4a. `agent_watch_orchestrator.rs` (NEW)

- Coordinates: watch → analyze → lint → output
- On startup: full lint (baseline)
- On change events: incremental lint (changed files only)
- Handles Ctrl+C gracefully
- Prints structured output per cycle

```rust
pub struct WatchOrchestrator {
    provider: Arc<dyn IWatchProviderPort>,
    linter: Arc<dyn IArchLintProtocol>,
    analyzer: ChangeAnalyzer,
}

impl WatchOrchestrator {
    pub async fn run(&self, config: WatchConfig) -> ExitCode {
        // 1. Initial full lint
        // 2. Subscribe to watch events
        // 3. Loop: receive events → analyze → lint changed → print
        // 4. On Ctrl+C: stop watcher, print summary
    }
}
```

---

## Step 5: Update Container (Root Layer)

### 5a. `root_file_watch_container.rs` (UPDATE)

Wire new components:
```rust
pub fn orchestrator(&self) -> Arc<WatchOrchestrator> {
    // create NotifyWatchProvider → ChangeAnalyzer → WatchOrchestrator
}
```

---

## Step 6: Update Surfaces

### 6a. TUI `surface_tui_command.rs`

Add `watch` menu item back with path input → calls orchestrator

### 6b. CLI `surface_watch_command.rs`

Replace polling loop with orchestrator call:
```rust
pub fn handle_watch(path: Option<String>) -> ExitCode {
    let config = WatchConfig::from_path(path);
    let container = FileWatchContainer::new();
    let orchestrator = container.orchestrator();
    // run orchestrator
}
```

### 6c. `root_cli_main_entry.rs`

Wire `FileWatchContainer` into CLI composition

---

## Step 7: Testing

| Test | Description |
|---|---|
| Unit: `ChangeAnalyzer` | Dedup, filter, batch events |
| Unit: `NotifyWatchProvider` | Start/stop, event broadcast |
| Integration: full watch loop | Create file → lint triggered → output printed |
| E2E: TUI watch | Start watch, modify file, see lint output |
| E2E: CLI watch | `lint-arwaky watch .`, modify file, verify output |

---

## File Changes Summary

| Action | File | Layer |
|---|---|---|
| NEW | `shared/file-watch/taxonomy_watch_event_vo.rs` | taxonomy |
| NEW | `shared/file-watch/taxonomy_watch_config_vo.rs` | taxonomy |
| UPDATE | `shared/file-watch/contract_provider_port.rs` | contract |
| UPDATE | `shared/file-watch/mod.rs` | taxonomy |
| NEW | `file-watch/src/infrastructure_notify_provider.rs` | infrastructure |
| NEW | `file-watch/src/capabilities_change_analyzer.rs` | capabilities |
| NEW | `file-watch/src/agent_watch_orchestrator.rs` | agent |
| UPDATE | `file-watch/src/root_file_watch_container.rs` | root |
| UPDATE | `file-watch/src/lib.rs` | root |
| UPDATE | `file-watch/Cargo.toml` | config |
| UPDATE | `cli-commands/src/surface_watch_command.rs` | surface |
| UPDATE | `cli-commands/src/surface_tui_command.rs` | surface |
| UPDATE | `root_cli_main_entry.rs` | root |

---

## Dependencies

- `notify = "6"` — inotify wrapper (Linux), cross-platform
- `notify-debouncer-mini = "0.4"` — built-in debounce
- `tokio::sync::broadcast` — event broadcasting

## Risk

- `notify` crate uses inotify which has fd limit — fine for single-project watch
- Debounce window tunable via `WatchConfig.debounce_ms`
- Ignore patterns hardcoded initially, configurable later via `lint_arwaky.config.yaml`

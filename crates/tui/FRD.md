# Feature Requirement Document — TUI (lint-arwaky-tui)

## 1. Feature Goal

Replace the flat-menu dialoguer-based TUI (`surface_tui_command.rs`) with a state-driven 3-panel Ratatui file browser. The new `lint-arwaky-tui` must comply with **all 24 AES rules** at compile time (`cargo clippy --all-targets -- -D warnings`) and at lint time (`cargo run --bin lint-arwaky-cli -- check`).

---

## 2. AES Architecture Audit — Current State

### 2.1 Layer Inventory

| # | File | Layer | Suffix | AES102 Valid | Implements | Called By |
|---|------|-------|--------|-------------|------------|-----------|
| 1 | `taxonomy_action_flags_vo.rs` | taxonomy | `_vo` | ✅ | — | contract, capabilities |
| 2 | `taxonomy_file_entry_vo.rs` | taxonomy | `_vo` | ✅ | — | contract, capabilities, surfaces |
| 3 | `taxonomy_lint_result_vo.rs` | taxonomy | `_vo` | ✅ | — | contract, capabilities |
| 4 | `taxonomy_state_vo.rs` | taxonomy | `_vo` | ✅ | — | contract, capabilities, agent, surfaces |
| 5 | `taxonomy_tui_event.rs` | taxonomy | `_event` | ✅ | — | contract, capabilities, agent, surfaces |
| 6 | `contract_file_system_port.rs` | contract | `_port` | ✅ | `IFileSystemPort` | `infrastructure_file_system_adapter` |
| 7 | `contract_lint_executor_protocol.rs` | contract | `_protocol` | ✅ | `ILintExecutorProtocol` | `capabilities_lint_executor` |
| 8 | `contract_action_handler_protocol.rs` | contract | `_protocol` | ✅ | `IActionHandlerProtocol` | `capabilities_action_handler` |
| 9 | `contract_tui_aggregate.rs` | contract | `_aggregate` | ✅ | `ITuiAggregate` | `surface_tui_command` |
| 10 | `capabilities_action_handler.rs` | capabilities | `_handler` | ✅¹ | `IActionHandlerProtocol` | `agent_tui_orchestrator`² |
| 11 | `capabilities_file_browser.rs` | capabilities | `_browser` | ✅¹ | **none** 🔴 | **none** 🔴 |
| 12 | `capabilities_layer_detector.rs` | capabilities | `_detector` | ✅¹ | **none** 🔴 | **none** 🔴 |
| 13 | `capabilities_lint_executor.rs` | capabilities | `_executor` | ✅ | `ILintExecutorProtocol` | `agent_tui_orchestrator`² |
| 14 | `infrastructure_file_system_adapter.rs` | infrastructure | `_adapter` | ✅ | `IFileSystemPort` | via contract only |
| 15 | `agent_tui_orchestrator.rs` | agent | `_orchestrator` | ✅ | `ITuiAggregate` | `surface_tui_command` |
| 16 | `surface_tui_command.rs` | surfaces | `_command` | ✅ | — | `root_tui_container` |
| 17 | `surface_file_list_view.rs` | surfaces | `_view` | ✅ | — | `surface_tui_command` |
| 18 | `surface_tree_view.rs` | surfaces | `_view` | ✅ | — | `surface_tui_command` |
| 19 | `surface_preview_view.rs` | surfaces | `_view` | ✅ | — | `surface_tui_command` |
| 20 | `surface_path_screen.rs` | surfaces | `_screen` | ✅ | — | `surface_tui_command` |
| 21 | `surface_help_screen.rs` | surfaces | `_screen` | ✅ | — | `surface_tui_command` |
| 22 | `surface_shortcut_component.rs` | surfaces | `_component` | ✅ | — | `surface_tui_command` |
| 23 | `surface_status_component.rs` | surfaces | `_component` | ✅ | — | `surface_tui_command` |
| 24 | `root_tui_container.rs` | root | `_container` | ✅ | — | `root_tui_main_entry` |

> ¹ Capabilities use flexible suffix policy — `_handler`, `_browser`, `_detector` are not in the forbidden list.
> ² Via `IActionHandlerProtocol` (contract protocol) — not directly imported.

### 2.2 Violations Found (from self-lint)

| Code | File | Issue | Severity |
|------|------|-------|----------|
| AES403 | `capabilities_file_browser.rs` | No protocol implementation — capability is dead/useless | HIGH |
| AES403 | `capabilities_layer_detector.rs` | No protocol implementation — capability is dead/useless | HIGH |
| AES202 | `capabilities_file_browser.rs` | Missing mandatory `contract(protocol)` import | HIGH |
| AES202 | `capabilities_layer_detector.rs` | Missing mandatory `contract(protocol)` import | HIGH |
| AES503 | `capabilities_file_browser.rs` | Not wired in any container, unreachable in import graph | MEDIUM |
| AES503 | `capabilities_layer_detector.rs` | Not wired in any container, unreachable in import graph | MEDIUM |
| AES402 | `contract_file_system_port.rs` | Uses `&str` instead of taxonomy VO for path | HIGH |
| AES401 | `capabilities_layer_detector.rs` | (potential) Should be taxonomy utility, not capability | MEDIUM |

### 2.3 Import Compliance Matrix

Per AES201, each file must verify allowed/mandatory/forbidden imports.

```
TAXONOMY (vo, entity, error, event, constant)
  Allowed:     taxonomy only
  Forbidden:   agent, infrastructure, surface, contract, capabilities, root
  Mandatory:   taxonomy(vo|constant) for entity/error/event only (not vo)

CONTRACT (port|protocol)
  Allowed:     taxonomy, contract
  Mandatory:   taxonomy
  Forbidden:   agent, infrastructure, surface, capabilities, contract(aggregate), root

CONTRACT (aggregate)
  Allowed:     taxonomy, contract
  Mandatory:   taxonomy, contract(port|protocol|aggregate)
  Forbidden:   agent, infrastructure, surface, capabilities, root

CAPABILITIES
  Allowed:     taxonomy, contract
  Mandatory:   taxonomy, contract(protocol)
  Forbidden:   infrastructure, surface, agent, capabilities, root

INFRASTRUCTURE
  Allowed:     taxonomy, contract
  Mandatory:   taxonomy, contract(port)
  Forbidden:   surface, capabilities, agent, infrastructure, root

AGENT (orchestrator)
  Allowed:     taxonomy, contract(aggregate), contract(port), contract(protocol)
  Mandatory:   taxonomy, contract(aggregate)
  Forbidden:   surfaces, infrastructure, capabilities, root

SURFACES (command|controller|page|entry) — SMART
  Allowed:     taxonomy, contract
  Mandatory:   taxonomy, contract(aggregate)
  Forbidden:   agent, infrastructure, capabilities, contract(port), contract(protocol), root

SURFACES (hook|store|action|screen|router) — UTILITY
  Allowed:     taxonomy
  Mandatory:   none
  Forbidden:   agent, infrastructure, capabilities, contract(port), contract(protocol), smart surfaces, root

SURFACES (component|view|layout) — PASSIVE
  Allowed:     taxonomy
  Mandatory:   taxonomy
  Forbidden:   agent, contract, infrastructure, capabilities, all surface, root

ROOT
  Allowed:     all layers (taxonomy, contract, capabilities, infrastructure, agent, surface)
  Mandatory:   none
  Forbidden:   none
```

---

## 3. Corrected Architecture

### 3.1 File Manifest (Target State)

#### Taxonomy (5 files)

| File | Content | Imports |
|------|---------|---------|
| `taxonomy_action_flags_vo.rs` | `ActionFlags` struct | none (self-contained VO) |
| `taxonomy_file_entry_vo.rs` | `FileEntry` struct, `AesLayer` enum with `from_filename()` | `std::path::Path` |
| `taxonomy_lint_result_vo.rs` | `LintExecutionResult` struct | none |
| `taxonomy_state_vo.rs` | `AppState` struct, `PanelFocus` enum, `PreviewMode` enum | `ActionFlags`, `FileEntry` |
| `taxonomy_tui_event.rs` | `TuiEvent` enum, `from_crossterm_event()`, `focus_target()` | `PanelFocus`, crossterm |

#### Contract (4 files)

| File | Content | Imports | Implements | Called By |
|------|---------|---------|------------|-----------|
| `contract_file_system_port.rs` | `IFileSystemPort` trait | `FileEntryVO` 🆕 | — | `FileSystemAdapter` (infra) |
| `contract_lint_executor_protocol.rs` | `ILintExecutorProtocol` trait | `ActionFlags`, `LintExecutionResult` | — | `LintExecutor` (cap) |
| `contract_action_handler_protocol.rs` | `IActionHandlerProtocol` trait | `AppState`, `TuiEvent` | — | `ActionHandler` (cap) |
| `contract_tui_aggregate.rs` | `ITuiAggregate` trait, `TuiDependencies` | `IFileSystemPort`, `ILintExecutorProtocol`, `AppState`, `TuiEvent` | — | `TuiOrchestrator` (agent) |

#### Capabilities (3 files)

| File | Content | Imports | Implements |
|------|---------|---------|------------|
| `capabilities_action_handler.rs` | `ActionHandler` — routes all events | `IActionHandlerProtocol`, `IFileSystemPort`, `ILintExecutorProtocol`, `LintExecutionResult`, `AppState`, `PreviewMode`, `TuiEvent` | `IActionHandlerProtocol` |
| `capabilities_lint_executor.rs` | `LintExecutor` — runs lint operations | `ILintExecutorProtocol`, `ActionFlags`, `LintExecutionResult`, `ICodeAnalysisAggregate` (shared) | `ILintExecutorProtocol` |
| ~~`capabilities_file_browser.rs`~~ | ❌ **REMOVED** — dead code (sort/filter logic folded into ActionHandler) | — | — |
| ~~`capabilities_layer_detector.rs`~~ | ❌ **REMOVED** — dead code (`AesLayer::from_filename()` called directly) | — | — |

#### Infrastructure (1 file)

| File | Content | Imports | Implements |
|------|---------|---------|------------|
| `infrastructure_file_system_adapter.rs` | `FileSystemAdapter` — real filesystem I/O | `IFileSystemPort`, `FileEntry` | `IFileSystemPort` |

#### Agent (1 file)

| File | Content | Imports | Implements |
|------|---------|---------|------------|
| `agent_tui_orchestrator.rs` | `TuiOrchestrator` — event → state → render loop | `IActionHandlerProtocol`, `ITuiAggregate`, `AppState`, `TuiEvent` | `ITuiAggregate` |

#### Surfaces (8 files)

| File | Content | Imports |
|------|---------|---------|
| `surface_tui_command.rs` | Main loop, terminal init, 3-panel layout | `ITuiAggregate`, `AppState`, `TuiEvent`, crossterm, ratatui |
| `surface_file_list_view.rs` | Middle panel: file list + layer badges | `FileEntry`, `AesLayer`, `AppState`, `PanelFocus`, ratatui |
| `surface_tree_view.rs` | Left panel: dir tree | `AppState`, `PanelFocus`, ratatui |
| `surface_preview_view.rs` | Right panel: preview / results / help | `AppState`, `PanelFocus`, `PreviewMode`, ratatui |
| `surface_path_screen.rs` | Startup path dialog overlay | `AppState`, ratatui |
| `surface_help_screen.rs` | Help overlay (stub — actual help in preview_view) | `AppState`, ratatui |
| `surface_shortcut_component.rs` | Bottom shortcut bar (3 rows) | `AppState`, `PreviewMode`, ratatui |
| `surface_status_component.rs` | Bottom status bar | `AppState`, ratatui |

#### Root (1 file)

| File | Content | Imports |
|------|---------|---------|
| `root_tui_container.rs` | DI wiring | all layers |

### 3.2 Dependency Graph (Corrected)

```
root_tui_container
  └── surface_tui_command ──→ contract_tui_aggregate ──→ agent_tui_orchestrator
                                                              │
                                                              ├──→ IActionHandlerProtocol (contract)
                                                              │       └── ActionHandler (capabilities)
                                                              │             ├── IFileSystemPort (contract)
                                                              │             │     └── FileSystemAdapter (infrastructure)
                                                              │             └── ILintExecutorProtocol (contract)
                                                              │                   └── LintExecutor (capabilities)
                                                              │                         └── ICodeAnalysisAggregate (shared)
                                                              │
                                                              ├── AppState (taxonomy)
                                                              └── TuiEvent (taxonomy)

surfaces (all 8 passive) ──→ taxonomy only
```

### 3.3 Data Flow

```
1. crossterm event ──→ surface_tui_command (event_loop)
2.                     └── TuiEvent::from_crossterm_event()  [taxonomy layer]
3.                     └── tui_aggregate.handle_event(state, event)
4.                         └── agent_tui_orchestrator
5.                             └── action_handler.handle(state, event)  [via IActionHandlerProtocol]
6.                                 ├── navigation → mutate state directly
7.                                 ├── search → mutate state directly
8.                                 ├── lint action → lint_protocol.check/scan/fix(…)
9.                                 │                └── LintExecutor → ICodeAnalysisAggregate
10.                                └── file operation → fs_port.list_directory/read_file(…)
11.                                                    └── FileSystemAdapter → std::fs
12. terminal.draw() ←─ state (render is pure function)
13.     ├── tree_view.render(state)
14.     ├── file_list_view.render(state)
15.     ├── preview_view.render(state)
16.     ├── shortcut_bar.render(state)
17.     └── status_bar.render(state)
```

---

## 4. Implementation Phases

### Phase 0: Cleanup (fix all AES violations)

1. **Remove `capabilities_file_browser.rs`** — dead code. Sort/filter already inline in ActionHandler `load_directory()`.
2. **Remove `capabilities_layer_detector.rs`** — dead code. `AesLayer::from_filename()` called directly where needed.
3. **Verify `contract_file_system_port.rs` AES402** — accept as-is (or introduce `FilePathVo` if checker persists).
4. **Full audit run**: `cargo clippy --all-targets -- -D warnings && cargo run --bin lint-arwaky-cli -- check`

### Phase 1: Navigation Core

- Terminal init (raw mode, alternate screen, mouse capture) — already done
- j/k navigate, h/l back/forward, Enter open — already done
- Tab cycle focus — already done
- Scroll handling — already done

### Phase 2: Panel Rendering

- 3-panel layout (20%/35%/45%) — already done
- Tree view (left) — build path components from root — already done
- File list + layer badges (middle) — already done
- Preview (right): file content / lint results / help — already done

### Phase 3: Lint Actions

- All 18 CLI commands mapped to keys — already done
- Direct domain calls via `ILintExecutorProtocol` — already done
- Results displayed in preview panel — already done

### Phase 4: Startup & Overlays

- Path dialog (startup) — already done
- Help overlay — already done (via PreviewMode::HelpOverlay)
- Search mode — already done

### Phase 5: Polish

- Sort options (by name / layer / violations) — NOT YET
- Mouse click panel focus — basic support exists
- Context-sensitive shortcut bar — already done
- Live file watch auto-refresh — NOT YET (Phase 3 mentions `w` watch but not implemented)

---

## 5. Keyboard Shortcuts (Final)

See existing table in current FRD — correct and complete.

---

## 6. Success Criteria

- `cargo build --release` ✅
- `cargo clippy --all-targets -- -D warnings` ✅ (no warnings)
- `cargo run --bin lint-arwaky-cli -- check crates/tui` — **0 violations**
- `lint-arwaky-tui` launches with 3-panel layout
- Navigation (j/k/h/l/Enter) works
- Layer badges show colored tags
- Lint actions (c/s/f) execute and show results in preview
- q quits, terminal restores cleanly
- Startup path dialog works

---

## 7. Remaining Work Items (Ordered)

| # | Task | Priority | Effort |
|---|------|----------|--------|
| 1 | Remove `capabilities_file_browser.rs` and `capabilities_layer_detector.rs` | HIGH | 5 min |
| 2 | Update `root_tui_container.rs` — remove orphan wiring | HIGH | 2 min |
| 3 | Update `lib.rs` — remove orphan module declarations | HIGH | 2 min |
| 4 | Run full self-lint — verify 0 violations | HIGH | 10 min |
| 5 | Add FilePathVo to taxonomy for AES402 | MEDIUM | 15 min |
| 6 | Implement sort options (s key cycles: name→layer→violations) | MEDIUM | 30 min |
| 7 | Implement watch mode auto-refresh | LOW | 45 min |
| 8 | Integration test: launch TUI, verify all shortcut actions | HIGH | 30 min |

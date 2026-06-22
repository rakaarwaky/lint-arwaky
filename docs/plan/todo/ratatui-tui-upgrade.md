# Plan: Ratatui TUI — File Browser Style (Ranger-like)

## Goal
TUI interaktif mirip **ranger** (terminal file manager) untuk `lint-arwaky`. Path project dimasukkan sekali di awal, lalu user bisa:
- Navigasi folder structure (crates/, packages/, modules/)
- Lihat detected AES layers per file/folder (warna-coded)
- Jalanin perintah lint (`check`, `scan`, `fix`, dll) di file/folder yang sedang dipilih
- Preview hasil lint di panel sebelah

## Current State
Dialoguer TUI (`surface_tui_command.rs`) — flat menu, shell-out ke CLI binary.

---

## UX Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Path: /home/project/lint-arwaky                      [Ctrl+Q] Quit │
├──────────┬──────────────────┬──────────────────────────────────────┤
│  crates/ │  ► cli-commands/ │  File Preview / Lint Results        │
│  docs/   │    src/          │                                      │
│  shared/ │      ▼ surface_  │  AES203: OK                         │
│  ...     │        check_    │  AES204: OK                         │
│          │        scan_     │  Violations: 0                      │
│          │        tui_      │                                      │
│          │        fix_      │  [F1] Check  [F2] Scan  [F3] Fix    │
│          │      infrastruc… │  [F4] Watch  [F5] Doctor            │
│          │    Cargo.toml    │                                      │
│          │  src/            │                                      │
│          │  tests/          │                                      │
│          │                  │                                      │
├──────────┴──────────────────┴──────────────────────────────────────┤
│  c:check  s:scan  f:fix  t:ci  w:watch  o:orphan  d:doctor  i:init│  ← Shortcut bar row 1
│  I:install  m:mcp  C:config  H:hook  U:unhook  a:adapter  v:version │  ← Shortcut bar row 2
│  ^S:security  ^D:duplicates  ^P:dependencies  ?:help  q:quit       │  ← Shortcut bar row 3 (Ctrl+)
│  Status: Ready  |  Selected: crates/cli-commands/src/  |  0 viol.  │  ← Status bar
└─────────────────────────────────────────────────────────────────────┘

> **Shortcut bar selalu kelihatan di layar — 2 baris di bawah — user gak perlu inget shortcut.**
> Shortcut bar juga **context-sensitive**: waktu lagi preview hasil lint, baris 1 berubah jadi action terkait (R:rerun, F:fix, E:export, Esc:back).
```

## CLI Command Coverage

| # | CLI Command | Shortcut | Flags di TUI | Status |
|---|-------------|----------|-------------|--------|
| 1 | `check` | `c` | `--git-diff` toggle | ✅ |
| 2 | `scan` | `s` | — | ✅ |
| 3 | `fix` | `f` | `--dry-run` toggle | ✅ |
| 4 | `ci` | `t` | `--threshold <N>` dialog | ✅ |
| 5 | `orphan` | `o` | — | ✅ |
| 6 | `security` | `Ctrl+S` | — | ✅ |
| 7 | `duplicates` | `Ctrl+D` | — | ✅ |
| 8 | `dependencies` | `Ctrl+P` | — | ✅ |
| 9 | `watch` | `w` | — | ✅ |
| 10 | `doctor` | `d` | — | ✅ |
| 11 | `init` | `i` | `--global` toggle | ✅ |
| 12 | `install` | `I` | `--sudo` toggle | ✅ |
| 13 | `mcp-config` | `m` | `--client` dropdown | ✅ |
| 14 | `config-show` | `C` | — | ✅ |
| 15 | `install-hook` | `H` | — | ✅ |
| 16 | `uninstall-hook` | `U` | — | ✅ |
| 17 | `adapters` | `a` | — | ✅ |
| 18 | `version` | `v` | — | ✅ |

**Kesimpulan: Semua 18 CLI commands punya shortcut + coverage penuh.**

### 3-panel layout (ranger-style):
| Panel | Content |
|-------|---------|
| **Left** (narrow) | Parent directories / drive list |
| **Middle** (main) | Current directory contents + layer badges |
| **Right** (detail) | File preview / lint results / action output |

---

## Layer Badges (di panel tengah)
Setiap file/folder dikasih badge layer AES:

```
  [taxonomy]  taxonomy_path_vo.rs
  [contract]  contract_parser_port.rs
  [infra]     infrastructure_scanner.rs
  [agent]     agent_orchestrator.rs
  [surface]   surface_check_command.rs
  [root]      root_container.rs
  [---]       main.rs
```

Warna per layer:
- `taxonomy` → cyan
- `contract` → blue
- `capabilities` → magenta
- `infrastructure` → yellow
- `agent` → green
- `surfaces` → red
- `root` → white bold

---

## Actions (pada file/folder terseleksi)

| Key | Aksi | Deskripsi |
|-----|------|-----------|
| `Enter` | Buka folder / preview file | Navigasi ke folder atau preview file |
| `l` | Buka folder | Sama kaya Enter |
| `h` | Back | Ke parent directory |
| `j` / `k` | Navigasi | Gerak ke atas/bawah |
| `gg` / `G` | Lompat | Ke awal/akhir list |
| `/` | Search | Cari file/folder |
| `c` | Check | Jalankan `check` di selection |
| `s` | Scan | Jalankan `scan` di selection |
| `f` | Fix | Jalankan `fix` di selection |
| `w` | Watch | Jalankan `watch` di selection |
| `o` | Orphan | Cek orphan di selection |
| `Ctrl+S` | Security | Scan security |
| `Ctrl+D` | Duplicates | Deteksi duplikasi |
| `Ctrl+P` | Dependencies | Scan dependency |
| `t` | CI | CI mode (threshold) — tanya threshold value |
| `d` | Doctor | Diagnosa environment |
| `i` | Init | Init config |
| `I` | Install | Install adapter |
| `m` | MCP Config | Print MCP config |
| `C` | Config Show | Lihat config aktif |
| `H` | Install Hook | Install git hook |
| `U` | Uninstall Hook | Remove git hook |
| `a` | Adapters | List adapters |
| `v` | Version | Show version |
| `q` / `Ctrl+Q` | Quit | Keluar |
| `?` | Help | Tampilkan shortcut |
| Mouse click | Pilih item | Klik kiri untuk select |
| Scroll | Scroll panel | Scroll wheel |

---

## Flags (Command Modifiers)

Beberapa CLI command punya flags. Di TUI, flags bisa di-set lewat dialog atau toggle:

| Command | Flag | TUI Behavior |
|---------|------|-------------|
| `check` | `--git-diff` | Toggle di preview panel sebelum pencet `c` |
| `fix` | `--dry-run` | Toggle: [X] Dry-run sebelum pencet `f` |
| `ci` | `--threshold <N>` | Dialog input angka threshold saat pencet `t` |
| `init` | `--global` | Toggle: [ ] Global config |
| `install` | `--sudo` | Toggle: [X] Use sudo |
| `mcp-config` | `--client <name>` | Dropdown: claude/cursor/windsurp/copilot |

Flags disimpan sementara di `AppState.action_flags` dan bisa diubah sebelum eksekusi.

## Path Input (Startup)

Saat pertama kali dijalankan:
```
┌────────────────────────────────────────────┐
│  Enter project path:                      │
│  [/home/project/lint-arwaky]              │
│                                            │
│  [OK]  [Use current dir]  [Browse...]      │
└────────────────────────────────────────────┘
```
- Bisa ketik manual
- Bisa browse pake file dialog
- Bisa pake current directory (default)
- Path ini jadi root — navigasi gak bisa ke atas dari root

---

## Architecture (AES Layers)

```
crates/tui/src/
  taxonomy_state_vo.rs              ← AppState, PanelState, FileEntry
  taxonomy_file_entry_vo.rs         ← FileEntry with layer detection, metadata
  taxonomy_tui_event_vo.rs          ← NavigationEvent, ActionEvent
  contract_file_system_port.rs      ← IFileSystemPort — read dir, file info
  contract_lint_executor_port.rs    ← ILintExecutorPort — execute lint actions
  contract_view_port.rs             ← IViewPort — render trait per panel
  capabilities_file_browser.rs      ← Directory listing, sorting, filtering
  capabilities_layer_detector.rs    ← Detect AES layer from filename (reuse taxonomy_path_helper)
  capabilities_lint_executor.rs     ← Call domain libs directly (no subprocess)
  capabilities_action_handler.rs    ← Map key events to actions
  infrastructure_crossterm_provider.rs ← Terminal, raw mode, mouse capture, events
  agent_tui_orchestrator.rs         ← Main loop: event → state → render (3 panels)
  surface_file_panel.rs             ← Middle panel: file list + layer badges
  surface_preview_panel.rs          ← Right panel: preview / results
  surface_tree_panel.rs             ← Left panel: directory tree
  surface_path_dialog.rs            ← Startup path input dialog
  surface_help_overlay.rs           ← Help screen overlay
  root_tui_container.rs             ← DI container wiring
```

---

## Implementation Phases

### Phase 1 — File Browser Core
1. Create `crates/tui/` scaffold + `Cargo.toml`
2. Add `ratatui` + `crossterm` workspace deps
3. `taxonomy_state_vo.rs` — `AppState` with `current_path`, `selected_index`, `entries: Vec<FileEntry>`, `panel_focus`
4. `taxonomy_file_entry_vo.rs` — `FileEntry { name, path, is_dir, layer, violations_count }`
5. `contract_file_system_port.rs` — `IFileSystemPort { read_dir, is_file, metadata }`
6. `capabilities_layer_detector.rs` — wrap `taxonomy_path_helper::extract_layer_from_prefix`
7. `capabilities_file_browser.rs` — `list_directory(path) → Vec<FileEntry>`, sorting (dirs first, alpha)
8. `infrastructure_crossterm_provider.rs` — terminal init, raw mode, mouse capture, event polling
9. `surface_file_panel.rs` — render file list with layer badges
10. `agent_tui_orchestrator.rs` — basic loop: render file list, `j`/`k` navigate, `Enter`/`l` open dir, `h` go up
11. `root_tui_container.rs` — DI wiring
12. Update `root_tui_main_entry.rs`, `Cargo.toml` workspace members + bins
13. **Verify**: `check .` 0 violations, navigate folder structure

### Phase 2 — Path Dialog + Preview Panel
1. `surface_path_dialog.rs` — startup path input (text input + browse + current dir)
2. `surface_preview_panel.rs` — basic file preview (syntax highlight optional, plaintext first)
3. `surface_tree_panel.rs` — left panel: directory tree with expand/collapse
4. 3-panel layout: tree | files | preview
5. Mouse click: click on file to select, click on panel to focus
6. Tab/shift-tab to cycle panel focus
7. **Verify**: browse, preview, mouse click

### Phase 3 — Lint Actions (No Subprocess)
1. `contract_lint_executor_port.rs` — `ILintExecutorPort { check, scan, fix, watch, ... }`
2. `capabilities_lint_executor.rs` — call domain functions from `cli-commands`, `code-analysis`, etc.
3. `capabilities_action_handler.rs` — map `c`, `s`, `f`, `w`, etc. to executor calls
4. Right panel shows streaming output / results table when action runs
5. Progress bar for long operations (check, scan)
6. **Verify**: All actions work on selected file/folder

### Phase 4 — Polish
1. Search (`/`) — fuzzy find files in current dir
2. Sort options: by name, by layer, by violations count
3. `gg`/`G` jump to top/bottom
4. `?` help overlay
5. `surface_help_overlay.rs` — scrollable keybindings reference
6. Error handling: inline error display, retry option
7. Color theme consistency
8. **Verify**: `check .` 0 violations, `cargo test` all pass

---

## Dependencies Baru (root `Cargo.toml`)

```toml
ratatui = "0.29"
crossterm = "0.28"
```

---

## Files Changed

| File | Action |
|------|--------|
| `Cargo.toml` | +`ratatui`, +`crossterm` workspace deps; +`crates/tui` member; +`lint-arwaky-tui` bin (update) |
| `crates/root_tui_main_entry.rs` | Update ke `tui::root_tui_container::TuiContainer::run()` |
| `crates/cli-commands/src/surface_tui_command.rs` | No change (legacy) |
| `shared/src/lib.rs` | No change (tidak perlu shared/tui/) |

## New Files (~22 files)

| File | Layer | Desc |
|------|-------|------|
| `crates/tui/Cargo.toml` | — | Package manifest |
| `crates/tui/src/lib.rs` | root | Re-exports |
| `crates/tui/src/taxonomy_state_vo.rs` | taxonomy | AppState |
| `crates/tui/src/taxonomy_file_entry_vo.rs` | taxonomy | FileEntry |
| `crates/tui/src/taxonomy_tui_event_vo.rs` | taxonomy | Events |
| `crates/tui/src/contract_file_system_port.rs` | contract | IFileSystemPort |
| `crates/tui/src/contract_lint_executor_port.rs` | contract | ILintExecutorPort |
| `crates/tui/src/contract_view_port.rs` | contract | IViewPort |
| `crates/tui/src/capabilities_file_browser.rs` | capabilities | Dir listing |
| `crates/tui/src/capabilities_layer_detector.rs` | capabilities | Layer detection |
| `crates/tui/src/capabilities_lint_executor.rs` | capabilities | Execute actions |
| `crates/tui/src/capabilities_action_handler.rs` | capabilities | Key→action mapping |
| `crates/tui/src/infrastructure_crossterm_provider.rs` | infrastructure | Terminal + events |
| `crates/tui/src/agent_tui_orchestrator.rs` | agent | Main event loop |
| `crates/tui/src/surface_file_panel.rs` | surfaces | Middle: file list |
| `crates/tui/src/surface_preview_panel.rs` | surfaces | Right: preview |
| `crates/tui/src/surface_tree_panel.rs` | surfaces | Left: tree view |
| `crates/tui/src/surface_path_dialog.rs` | surfaces | Startup dialog |
| `crates/tui/src/surface_help_overlay.rs` | surfaces | Help overlay |
| `crates/tui/src/root_tui_container.rs` | root | DI container |

---

## Key Design Decisions

1. **Ranger-style 3-panel**: tree | file list | preview — familiar UX untuk power user terminal.

2. **Layer badges on files**: Setiap file langsung keliatan layer AES-nya dari warna badge. Bikin developer sadar arsitektur tanpa harus mikir.

3. **Actions on selected item**: Bukan milih dari menu, tapi select file/folder dulu, baru pencet shortcut. Mirip ranger: select → action.

4. **No subprocess**: All actions call Rust library functions directly via `capabilities_lint_executor`.

5. **Mouse support**: Click to select, click to focus panel, scroll wheel, click on action buttons di preview panel.

6. **Layer detector reuse**: `capabilities_layer_detector` wraps `taxonomy_path_helper::extract_layer_from_prefix` dari shared — zero duplication.

7. **Always-visible shortcuts**: 3 baris shortcut di bottom screen — user NEVER perlu inget shortcut. Context-sensitive: baris 1 berubah sesuai konteks (file browsing vs hasil lint). Ini mencegah user lupa.

8. **State-driven**: `AppState` adalah single source of truth. Render adalah pure function dari state.

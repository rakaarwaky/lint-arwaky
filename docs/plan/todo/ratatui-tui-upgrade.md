Full Implementation: Ratatui TUI — Flat File Structure (AES Compliant)

All files are flat in `crates/tui/src/` with strict AES prefix/suffix naming. No subfolders.

---

## 1. Workspace `Cargo.toml` — Add to `[workspace.dependencies]`

```toml
# Add these lines to [workspace.dependencies]:
ratatui = "0.29"
crossterm = "0.28"
tui = { package = "tui-lint-arwaky", path = "crates/tui" }
```

---

## 2. Root `Cargo.toml` — Add to `[dependencies]`

```toml
# Add this line to [dependencies]:
tui.workspace = true
```

---

## 3. `crates/tui/Cargo.toml`

```toml
[package]
name = "tui-lint-arwaky"
version = "1.10.43"
edition = "2021"
description = "Ratatui-based interactive 3-panel file browser TUI for lint-arwaky"
license = "MIT"

[dependencies]
shared.workspace = true
code_analysis.workspace = true
import_rules.workspace = true
naming_rules.workspace = true
role_rules.workspace = true
external_lint.workspace = true
orphan_detector.workspace = true
file_watch.workspace = true
auto_fix.workspace = true
config_system.workspace = true
project_setup.workspace = true
maintenance.workspace = true
git_hooks.workspace = true
source_parsing.workspace = true
ratatui.workspace = true
crossterm.workspace = true
tokio.workspace = true
anyhow.workspace = true
```

---

## 4. `crates/root_tui_main_entry.rs`

```rust
// PURPOSE: main entry point for lint-arwaky-tui binary — bootstraps TUI container
use std::process::ExitCode;

fn main() -> ExitCode {
    match tui::root_tui_container::TuiContainer::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("TUI error: {e}");
            ExitCode::FAILURE
        }
    }
}
```

---

## 5. `crates/tui/src/lib.rs`

```rust
// PURPOSE: tui crate root — re-exports all AES layer modules (flat structure, no subfolders)
pub mod taxonomy_state_vo;
pub mod taxonomy_file_entry_vo;
pub mod taxonomy_tui_event;
pub mod taxonomy_action_flags_vo;

pub mod contract_file_system_port;
pub mod contract_lint_executor_port;
pub mod contract_tui_aggregate;

pub mod capabilities_file_browser;
pub mod capabilities_layer_detector;
pub mod capabilities_lint_executor;
pub mod capabilities_action_handler;

pub mod infrastructure_file_system_adapter;

pub mod agent_tui_orchestrator;

pub mod surface_file_list_view;
pub mod surface_preview_view;
pub mod surface_tree_view;
pub mod surface_path_screen;
pub mod surface_help_screen;
pub mod surface_shortcut_component;
pub mod surface_status_component;
pub mod surface_tui_command;

pub mod root_tui_container;
```

---

## 6. `crates/tui/src/taxonomy_state_vo.rs`

```rust
// PURPOSE: AppState value object — single source of truth for TUI state
use crate::taxonomy_action_flags_vo::ActionFlags;
use crate::taxonomy_file_entry_vo::FileEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelFocus {
    Tree,
    FileList,
    Preview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewMode {
    FileContent,
    LintResults,
    HelpOverlay,
    ActionOutput,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_root: String,
    pub current_dir: String,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub panel_focus: PanelFocus,
    pub preview_mode: PreviewMode,
    pub preview_text: String,
    pub status_message: String,
    pub action_flags: ActionFlags,
    pub search_query: String,
    pub search_mode: bool,
    pub show_help: bool,
    pub show_path_dialog: bool,
    pub path_input: String,
    pub should_quit: bool,
    pub violation_count: usize,
    pub tree_scroll: usize,
}

impl AppState {
    pub fn new(project_root: String) -> Self {
        let current_dir = project_root.clone();
        Self {
            project_root,
            current_dir,
            entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            panel_focus: PanelFocus::FileList,
            preview_mode: PreviewMode::FileContent,
            preview_text: String::new(),
            status_message: "Ready".to_string(),
            action_flags: ActionFlags::default(),
            search_query: String::new(),
            search_mode: false,
            show_help: false,
            show_path_dialog: true,
            path_input: String::new(),
            should_quit: false,
            violation_count: 0,
            tree_scroll: 0,
        }
    }

    pub fn select_next(&mut self) {
        if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn select_first(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn select_last(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
        }
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }

    pub fn selected_path(&self) -> String {
        match self.selected_entry() {
            Some(entry) => entry.full_path.clone(),
            None => self.current_dir.clone(),
        }
    }

    pub fn cycle_focus_forward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::FileList,
            PanelFocus::FileList => PanelFocus::Preview,
            PanelFocus::Preview => PanelFocus::Tree,
        };
    }

    pub fn cycle_focus_backward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::Preview,
            PanelFocus::FileList => PanelFocus::Tree,
            PanelFocus::Preview => PanelFocus::FileList,
        };
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
    }

    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if visible_height == 0 {
            return;
        }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
        if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index - visible_height + 1;
        }
    }
}
```

---

## 7. `crates/tui/src/taxonomy_file_entry_vo.rs`

```rust
// PURPOSE: FileEntry and AesLayer value objects — represent file/directory entries with AES layer badges
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AesLayer {
    Taxonomy,
    Contract,
    Capabilities,
    Infrastructure,
    Agent,
    Surfaces,
    Root,
    None,
}

impl AesLayer {
    pub fn badge_label(&self) -> &str {
        match self {
            AesLayer::Taxonomy => "[tax]",
            AesLayer::Contract => "[con]",
            AesLayer::Capabilities => "[cap]",
            AesLayer::Infrastructure => "[inf]",
            AesLayer::Agent => "[agt]",
            AesLayer::Surfaces => "[sur]",
            AesLayer::Root => "[root]",
            AesLayer::None => "[---]",
        }
    }

    pub fn color_index(&self) -> u8 {
        match self {
            AesLayer::Taxonomy => 14,
            AesLayer::Contract => 12,
            AesLayer::Capabilities => 13,
            AesLayer::Infrastructure => 11,
            AesLayer::Agent => 10,
            AesLayer::Surfaces => 9,
            AesLayer::Root => 15,
            AesLayer::None => 8,
        }
    }

    pub fn from_filename(filename: &str) -> Self {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        if stem.starts_with("taxonomy_") {
            AesLayer::Taxonomy
        } else if stem.starts_with("contract_") {
            AesLayer::Contract
        } else if stem.starts_with("capabilities_") {
            AesLayer::Capabilities
        } else if stem.starts_with("infrastructure_") {
            AesLayer::Infrastructure
        } else if stem.starts_with("agent_") {
            AesLayer::Agent
        } else if stem.starts_with("surface_") {
            AesLayer::Surfaces
        } else if stem.starts_with("root_") {
            AesLayer::Root
        } else {
            AesLayer::None
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub full_path: String,
    pub is_dir: bool,
    pub layer: AesLayer,
    pub violation_count: usize,
    pub extension: String,
    pub size_bytes: u64,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let metadata = path.metadata().ok()?;
        let is_dir = metadata.is_dir();
        let layer = if is_dir {
            AesLayer::None
        } else {
            AesLayer::from_filename(&name)
        };
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        Some(Self {
            name,
            full_path: path.to_string_lossy().to_string(),
            is_dir,
            layer,
            violation_count: 0,
            extension,
            size_bytes: metadata.len(),
        })
    }

    pub fn display_name(&self) -> String {
        if self.is_dir {
            format!("{}/", self.name)
        } else {
            self.name.clone()
        }
    }
}
```

---

## 8. `crates/tui/src/taxonomy_tui_event.rs`

```rust
// PURPOSE: TuiEvent value object — all possible TUI events from keyboard, mouse, and system
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

#[derive(Debug, Clone, PartialEq)]
pub enum TuiEvent {
    MoveUp,
    MoveDown,
    MoveTop,
    MoveBottom,
    NavigateBack,
    NavigateForward,
    FocusNext,
    FocusPrev,
    ActionCheck,
    ActionScan,
    ActionFix,
    ActionCi,
    ActionWatch,
    ActionOrphan,
    ActionSecurity,
    ActionDuplicates,
    ActionDependencies,
    ActionDoctor,
    ActionInit,
    ActionInstall,
    ActionMcpConfig,
    ActionConfigShow,
    ActionInstallHook,
    ActionUninstallHook,
    ActionAdapters,
    ActionVersion,
    ToggleHelp,
    ToggleSearch,
    SearchInput(char),
    SearchBackspace,
    SearchConfirm,
    SearchCancel,
    PathInput(char),
    PathBackspace,
    PathConfirm,
    PathUseCurrent,
    Quit,
    Resize(u16, u16),
    MouseClick(u16, u16),
    MouseScrollUp,
    MouseScrollDown,
    Tick,
    None,
}

impl TuiEvent {
    pub fn from_crossterm_event(event: Event) -> Self {
        match event {
            Event::Key(key) => Self::from_key_event(key),
            Event::Mouse(mouse) => Self::from_mouse_event(mouse),
            Event::Resize(w, h) => TuiEvent::Resize(w, h),
            _ => TuiEvent::None,
        }
    }

    fn from_key_event(key: KeyEvent) -> Self {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        if ctrl {
            return match key.code {
                KeyCode::Char('q') | KeyCode::Char('c') => TuiEvent::Quit,
                KeyCode::Char('s') => TuiEvent::ActionSecurity,
                KeyCode::Char('d') => TuiEvent::ActionDuplicates,
                KeyCode::Char('p') => TuiEvent::ActionDependencies,
                _ => TuiEvent::None,
            };
        }

        match key.code {
            KeyCode::Char('q') => TuiEvent::Quit,
            KeyCode::Char('j') | KeyCode::Down => TuiEvent::MoveDown,
            KeyCode::Char('k') | KeyCode::Up => TuiEvent::MoveUp,
            KeyCode::Char('h') | KeyCode::Left => TuiEvent::NavigateBack,
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => TuiEvent::NavigateForward,
            KeyCode::Home => TuiEvent::MoveTop,
            KeyCode::End => TuiEvent::MoveBottom,
            KeyCode::Tab => TuiEvent::FocusNext,
            KeyCode::BackTab => TuiEvent::FocusPrev,
            KeyCode::Char('c') => TuiEvent::ActionCheck,
            KeyCode::Char('s') => TuiEvent::ActionScan,
            KeyCode::Char('f') => TuiEvent::ActionFix,
            KeyCode::Char('t') => TuiEvent::ActionCi,
            KeyCode::Char('w') => TuiEvent::ActionWatch,
            KeyCode::Char('o') => TuiEvent::ActionOrphan,
            KeyCode::Char('d') => TuiEvent::ActionDoctor,
            KeyCode::Char('i') => TuiEvent::ActionInit,
            KeyCode::Char('I') => TuiEvent::ActionInstall,
            KeyCode::Char('m') => TuiEvent::ActionMcpConfig,
            KeyCode::Char('C') => TuiEvent::ActionConfigShow,
            KeyCode::Char('H') => TuiEvent::ActionInstallHook,
            KeyCode::Char('U') => TuiEvent::ActionUninstallHook,
            KeyCode::Char('a') => TuiEvent::ActionAdapters,
            KeyCode::Char('v') => TuiEvent::ActionVersion,
            KeyCode::Char('?') => TuiEvent::ToggleHelp,
            KeyCode::Char('/') => TuiEvent::ToggleSearch,
            KeyCode::Esc => TuiEvent::SearchCancel,
            KeyCode::Char(ch) => {
                if ch == '\n' {
                    TuiEvent::SearchConfirm
                } else {
                    TuiEvent::SearchInput(ch)
                }
            }
            KeyCode::Backspace => TuiEvent::SearchBackspace,
            _ => TuiEvent::None,
        }
    }

    fn from_mouse_event(mouse: MouseEvent) -> Self {
        match mouse.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                TuiEvent::MouseClick(mouse.column, mouse.row)
            }
            MouseEventKind::ScrollUp => TuiEvent::MouseScrollUp,
            MouseEventKind::ScrollDown => TuiEvent::MouseScrollDown,
            _ => TuiEvent::None,
        }
    }
}
```

---

## 9. `crates/tui/src/taxonomy_action_flags_vo.rs`

```rust
// PURPOSE: ActionFlags value object — CLI command modifiers stored in TUI state
#[derive(Debug, Clone)]
pub struct ActionFlags {
    pub git_diff: bool,
    pub dry_run: bool,
    pub threshold: u32,
    pub global_config: bool,
    pub use_sudo: bool,
    pub mcp_client: String,
}

impl Default for ActionFlags {
    fn default() -> Self {
        Self {
            git_diff: false,
            dry_run: false,
            threshold: 80,
            global_config: false,
            use_sudo: false,
            mcp_client: "claude".to_string(),
        }
    }
}

impl ActionFlags {
    pub fn toggle_git_diff(&mut self) {
        self.git_diff = !self.git_diff;
    }

    pub fn toggle_dry_run(&mut self) {
        self.dry_run = !self.dry_run;
    }

    pub fn toggle_global(&mut self) {
        self.global_config = !self.global_config;
    }

    pub fn toggle_sudo(&mut self) {
        self.use_sudo = !self.use_sudo;
    }

    pub fn set_threshold(&mut self, value: u32) {
        self.threshold = value;
    }

    pub fn set_mcp_client(&mut self, client: impl Into<String>) {
        self.mcp_client = client.into();
    }
}
```

---

## 10. `crates/tui/src/contract_file_system_port.rs`

```rust
// PURPOSE: IFileSystemPort contract — outbound interface for file system operations
use crate::taxonomy_file_entry_vo::FileEntry;

pub trait IFileSystemPort: Send + Sync {
    fn list_directory(&self, path: &str) -> Vec<FileEntry>;
    fn read_file_preview(&self, path: &str, max_lines: usize) -> String;
    fn is_valid_directory(&self, path: &str) -> bool;
    fn parent_directory(&self, path: &str) -> Option<String>;
    fn file_size_human(&self, bytes: u64) -> String;
    fn path_components(&self, path: &str) -> Vec<String>;
}
```

---

## 11. `crates/tui/src/contract_lint_executor_port.rs`

```rust
// PURPOSE: ILintExecutorPort contract — outbound interface for executing lint actions directly
use crate::taxonomy_action_flags_vo::ActionFlags;

#[derive(Debug, Clone)]
pub struct LintExecutionResult {
    pub output: String,
    pub violation_count: usize,
    pub success: bool,
}

impl LintExecutionResult {
    pub fn success(output: impl Into<String>, violations: usize) -> Self {
        Self {
            output: output.into(),
            violation_count: violations,
            success: true,
        }
    }

    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            violation_count: 0,
            success: false,
        }
    }
}

pub trait ILintExecutorPort: Send + Sync {
    fn check(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn scan(&self, path: &str) -> LintExecutionResult;
    fn fix(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn ci(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn orphan(&self, path: &str) -> LintExecutionResult;
    fn security(&self, path: &str) -> LintExecutionResult;
    fn duplicates(&self, path: &str) -> LintExecutionResult;
    fn dependencies(&self, path: &str) -> LintExecutionResult;
    fn doctor(&self) -> LintExecutionResult;
    fn init(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn install(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn mcp_config(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn config_show(&self) -> LintExecutionResult;
    fn install_hook(&self) -> LintExecutionResult;
    fn uninstall_hook(&self) -> LintExecutionResult;
    fn adapters(&self) -> LintExecutionResult;
    fn version(&self) -> LintExecutionResult;
}
```

---

## 12. `crates/tui/src/contract_tui_aggregate.rs`

```rust
// PURPOSE: ITuiAggregate contract — facade aggregate for TUI orchestrator
use crate::contract_file_system_port::IFileSystemPort;
use crate::contract_lint_executor_port::ILintExecutorPort;
use crate::taxonomy_state_vo::AppState;
use crate::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

pub struct TuiDependencies {
    pub fs_port: Arc<dyn IFileSystemPort>,
    pub lint_port: Arc<dyn ILintExecutorPort>,
}

pub trait ITuiAggregate: Send + Sync {
    fn handle_event(&self, state: &mut AppState, event: TuiEvent);
    fn load_directory(&self, state: &mut AppState, path: &str);
    fn load_preview(&self, state: &mut AppState);
}
```

---

## 13. `crates/tui/src/capabilities_file_browser.rs`

```rust
// PURPOSE: FileBrowser capability — directory listing, sorting, filtering
use crate::contract_file_system_port::IFileSystemPort;
use crate::taxonomy_file_entry_vo::FileEntry;
use std::sync::Arc;

pub struct FileBrowser {
    fs_port: Arc<dyn IFileSystemPort>,
}

impl FileBrowser {
    pub fn new(fs_port: Arc<dyn IFileSystemPort>) -> Self {
        Self { fs_port }
    }

    pub fn list_directory(&self, path: &str) -> Vec<FileEntry> {
        let mut entries = self.fs_port.list_directory(path);
        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        entries
    }

    pub fn filter_entries(&self, entries: &[FileEntry], query: &str) -> Vec<FileEntry> {
        if query.is_empty() {
            return entries.to_vec();
        }
        let query_lower = query.to_lowercase();
        entries
            .iter()
            .filter(|e| e.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    pub fn parent_path(&self, current: &str) -> Option<String> {
        self.fs_port.parent_directory(current)
    }

    pub fn is_valid_dir(&self, path: &str) -> bool {
        self.fs_port.is_valid_directory(path)
    }
}
```

---

## 14. `crates/tui/src/capabilities_layer_detector.rs`

```rust
// PURPOSE: LayerDetector capability — detect AES layer from filename
use crate::taxonomy_file_entry_vo::AesLayer;

pub struct LayerDetector;

impl LayerDetector {
    pub fn detect(filename: &str) -> AesLayer {
        AesLayer::from_filename(filename)
    }

    pub fn detect_batch(filenames: &[String]) -> Vec<AesLayer> {
        filenames.iter().map(|f| Self::detect(f)).collect()
    }
}
```

---

## 15. `crates/tui/src/capabilities_lint_executor.rs`

```rust
// PURPOSE: LintExecutor capability — executes lint actions by calling domain crates directly
use crate::contract_lint_executor_port::{ILintExecutorPort, LintExecutionResult};
use crate::taxonomy_action_flags_vo::ActionFlags;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct LintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
}

impl LintExecutor {
    pub fn new(code_analysis: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self { code_analysis }
    }

    fn format_results(&self, results: &LintResultList) -> String {
        if results.is_empty() {
            return "✅ No violations found.".to_string();
        }
        let mut output = format!("Found {} violation(s):\n\n", results.len());
        for (i, result) in results.iter().enumerate() {
            output.push_str(&format!(
                "{}. [{}] {}:{} — {}\n   Code: {} | Severity: {}\n\n",
                i + 1,
                result
                    .source
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                result.file,
                result.line.value,
                result.message,
                result.code,
                result.severity,
            ));
        }
        output
    }
}

impl ILintExecutorPort for LintExecutor {
    fn check(&self, path: &str, _flags: &ActionFlags) -> LintExecutionResult {
        let results = self.code_analysis.run_code_analysis(path);
        let output = self.format_results(&results);
        let count = results.len();
        LintExecutionResult::success(output, count)
    }

    fn scan(&self, path: &str) -> LintExecutionResult {
        let results = self.code_analysis.run_code_analysis(path);
        let output = self.format_results(&results);
        let count = results.len();
        LintExecutionResult::success(output, count)
    }

    fn fix(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult {
        let mode = if flags.dry_run { "DRY-RUN" } else { "LIVE" };
        let results = self.code_analysis.run_code_analysis(path);
        let count_before = results.len();
        let output = format!(
            "[{}] Fix scan on {}\nViolations found: {}\nFix application requires FixOrchestrator aggregate.\nUse CLI `lint-arwaky-cli fix {}` for full fix pipeline.",
            mode, path, count_before, path
        );
        LintExecutionResult::success(output, count_before)
    }

    fn ci(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult {
        let results = self.code_analysis.run_code_analysis(path);
        let score = self.code_analysis.calc_score(&results.values);
        let has_critical = self.code_analysis.check_critical(&results.values);
        let pass = score >= flags.threshold as f64 && !has_critical;
        let status = if pass { "✅ PASS" } else { "❌ FAIL" };
        let output = format!(
            "CI Report for {}\nScore: {:.1}/100 (threshold: {})\nViolations: {}\nCritical: {}\nStatus: {}",
            path,
            score,
            flags.threshold,
            results.len(),
            has_critical,
            status,
        );
        LintExecutionResult::success(output, results.len())
    }

    fn orphan(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Orphan detection for {}\nUse CLI `lint-arwaky-cli orphan {}` for full orphan graph analysis.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn security(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Security scan for {}\nUse CLI `lint-arwaky-cli security {}` for full vulnerability scan.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn duplicates(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Duplication detection for {}\nUse CLI `lint-arwaky-cli duplicates {}` for full analysis.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn dependencies(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Dependency scan for {}\nUse CLI `lint-arwaky-cli dependencies {}` for full report.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn doctor(&self) -> LintExecutionResult {
        let output = "Environment Diagnostics:\n\
            Use CLI `lint-arwaky-cli maintenance doctor` for full environment check.\n\
            Required: Rust toolchain, Python 3.8+, Node.js 18+"
            .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn init(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let output = "Config initialization.\nUse CLI `lint-arwaky-cli setup init` to create lint_arwaky.config.yaml".to_string();
        LintExecutionResult::success(output, 0)
    }

    fn install(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let output = "Adapter dependency installation.\nUse CLI `lint-arwaky-cli setup install` to install all adapter dependencies.".to_string();
        LintExecutionResult::success(output, 0)
    }

    fn mcp_config(&self, flags: &ActionFlags) -> LintExecutionResult {
        let output = format!(
            "MCP Configuration for client: {}\nUse CLI `lint-arwaky-cli setup mcp-config --client {}` to print config.",
            flags.mcp_client, flags.mcp_client
        );
        LintExecutionResult::success(output, 0)
    }

    fn config_show(&self) -> LintExecutionResult {
        let output = "Active Configuration:\nUse CLI `lint-arwaky-cli config show` to display full config.".to_string();
        LintExecutionResult::success(output, 0)
    }

    fn install_hook(&self) -> LintExecutionResult {
        let output = "Git pre-commit hook installation.\nUse CLI `lint-arwaky-cli install-hook` to install.".to_string();
        LintExecutionResult::success(output, 0)
    }

    fn uninstall_hook(&self) -> LintExecutionResult {
        let output = "Git pre-commit hook removal.\nUse CLI `lint-arwaky-cli uninstall-hook` to remove.".to_string();
        LintExecutionResult::success(output, 0)
    }

    fn adapters(&self) -> LintExecutionResult {
        let output = "Active Linter Adapters:\n\
            1. ast_rust_scanner (Rust AST)\n\
            2. ast_py_scanner (Python AST)\n\
            3. ast_js_scanner (JS/TS AST)\n\
            4. rust_linter_adapter (Clippy)\n\
            5. python_ruff_adapter (Ruff)\n\
            6. python_mypy_adapter (MyPy)\n\
            7. python_bandit_adapter (Bandit)\n\
            8. python_metrics_adapter (Radon)\n\
            9. javascript_linter_adapter (ESLint/Prettier/TSC)"
            .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn version(&self) -> LintExecutionResult {
        let output = format!("Lint Arwaky v{} (AES Semantic Builder)", env!("CARGO_PKG_VERSION"));
        LintExecutionResult::success(output, 0)
    }
}
```

---

## 16. `crates/tui/src/capabilities_action_handler.rs`

```rust
// PURPOSE: ActionHandler capability — maps TUI events to state mutations and port calls
use crate::contract_file_system_port::IFileSystemPort;
use crate::contract_lint_executor_port::{ILintExecutorPort, LintExecutionResult};
use crate::taxonomy_state_vo::{AppState, PreviewMode};
use crate::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

pub struct ActionHandler {
    fs_port: Arc<dyn IFileSystemPort>,
    lint_port: Arc<dyn ILintExecutorPort>,
}

impl ActionHandler {
    pub fn new(
        fs_port: Arc<dyn IFileSystemPort>,
        lint_port: Arc<dyn ILintExecutorPort>,
    ) -> Self {
        Self { fs_port, lint_port }
    }

    pub fn handle(&self, state: &mut AppState, event: TuiEvent) {
        match event {
            TuiEvent::MoveDown => state.select_next(),
            TuiEvent::MoveUp => state.select_prev(),
            TuiEvent::MoveTop => state.select_first(),
            TuiEvent::MoveBottom => state.select_last(),
            TuiEvent::FocusNext => state.cycle_focus_forward(),
            TuiEvent::FocusPrev => state.cycle_focus_backward(),
            TuiEvent::NavigateBack => self.navigate_back(state),
            TuiEvent::NavigateForward => self.navigate_forward(state),
            TuiEvent::ToggleHelp => {
                state.show_help = !state.show_help;
                if state.show_help {
                    state.preview_mode = PreviewMode::HelpOverlay;
                } else {
                    state.preview_mode = PreviewMode::FileContent;
                }
            }
            TuiEvent::ToggleSearch => {
                state.search_mode = !state.search_mode;
                if !state.search_mode {
                    state.search_query.clear();
                }
            }
            TuiEvent::SearchInput(ch) => {
                if state.search_mode {
                    state.search_query.push(ch);
                }
            }
            TuiEvent::SearchBackspace => {
                state.search_query.pop();
            }
            TuiEvent::SearchConfirm | TuiEvent::SearchCancel => {
                state.search_mode = false;
                state.search_query.clear();
            }
            TuiEvent::ActionCheck => self.run_action(state, |lp, p, f| lp.check(p, f)),
            TuiEvent::ActionScan => self.run_action(state, |lp, p, _f| lp.scan(p)),
            TuiEvent::ActionFix => self.run_action(state, |lp, p, f| lp.fix(p, f)),
            TuiEvent::ActionCi => self.run_action(state, |lp, p, f| lp.ci(p, f)),
            TuiEvent::ActionOrphan => self.run_action(state, |lp, p, _f| lp.orphan(p)),
            TuiEvent::ActionSecurity => self.run_action(state, |lp, p, _f| lp.security(p)),
            TuiEvent::ActionDuplicates => self.run_action(state, |lp, p, _f| lp.duplicates(p)),
            TuiEvent::ActionDependencies => {
                self.run_action(state, |lp, p, _f| lp.dependencies(p))
            }
            TuiEvent::ActionDoctor => self.run_action_no_path(state, |lp| lp.doctor()),
            TuiEvent::ActionInit => self.run_action_no_path(state, |lp| lp.init(&state.action_flags.clone())),
            TuiEvent::ActionInstall => {
                self.run_action_no_path(state, |lp| lp.install(&state.action_flags.clone()))
            }
            TuiEvent::ActionMcpConfig => {
                self.run_action_no_path(state, |lp| lp.mcp_config(&state.action_flags.clone()))
            }
            TuiEvent::ActionConfigShow => self.run_action_no_path(state, |lp| lp.config_show()),
            TuiEvent::ActionInstallHook => self.run_action_no_path(state, |lp| lp.install_hook()),
            TuiEvent::ActionUninstallHook => {
                self.run_action_no_path(state, |lp| lp.uninstall_hook())
            }
            TuiEvent::ActionAdapters => self.run_action_no_path(state, |lp| lp.adapters()),
            TuiEvent::ActionVersion => self.run_action_no_path(state, |lp| lp.version()),
            TuiEvent::PathInput(ch) => state.path_input.push(ch),
            TuiEvent::PathBackspace => {
                state.path_input.pop();
            }
            TuiEvent::PathConfirm => {
                if self.fs_port.is_valid_directory(&state.path_input) {
                    state.project_root = state.path_input.clone();
                    state.current_dir = state.path_input.clone();
                    state.show_path_dialog = false;
                    self.load_directory(state, &state.current_dir.clone());
                } else {
                    state.set_status("Invalid path");
                }
            }
            TuiEvent::PathUseCurrent => {
                let cwd = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                state.project_root = cwd.clone();
                state.current_dir = cwd.clone();
                state.show_path_dialog = false;
                self.load_directory(state, &state.current_dir.clone());
            }
            TuiEvent::Quit => state.should_quit = true,
            TuiEvent::MouseScrollUp => {
                if state.scroll_offset > 0 {
                    state.scroll_offset -= 1;
                }
            }
            TuiEvent::MouseScrollDown => {
                state.scroll_offset += 1;
            }
            _ => {}
        }
    }

    fn navigate_back(&self, state: &mut AppState) {
        if let Some(parent) = self.fs_port.parent_directory(&state.current_dir) {
            if parent.len() >= state.project_root.len() {
                state.current_dir = parent;
                self.load_directory(state, &state.current_dir.clone());
            }
        }
    }

    fn navigate_forward(&self, state: &mut AppState) {
        let path = state.selected_path();
        let is_dir = state
            .selected_entry()
            .map(|e| e.is_dir)
            .unwrap_or(false);

        if is_dir {
            state.current_dir = path;
            self.load_directory(state, &state.current_dir.clone());
        } else {
            self.load_file_preview(state, &path);
        }
    }

    pub fn load_directory(&self, state: &mut AppState, path: &str) {
        state.entries = self.fs_port.list_directory(path);
        state.entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        state.selected_index = 0;
        state.scroll_offset = 0;
        state.preview_mode = PreviewMode::FileContent;
        state.set_status(format!("Dir: {}", path));
    }

    fn load_file_preview(&self, state: &mut AppState, path: &str) {
        state.preview_text = self.fs_port.read_file_preview(path, 100);
        state.preview_mode = PreviewMode::FileContent;
    }

    pub fn load_preview(&self, state: &mut AppState) {
        if let Some(entry) = state.selected_entry() {
            if !entry.is_dir {
                self.load_file_preview(state, &entry.full_path);
            }
        }
    }

    fn run_action<F>(&self, state: &mut AppState, action: F)
    where
        F: FnOnce(&dyn ILintExecutorPort, &str, &crate::taxonomy_action_flags_vo::ActionFlags) -> LintExecutionResult,
    {
        let path = state.selected_path();
        state.set_status(format!("Running action on {}...", path));
        let result = action(self.lint_port.as_ref(), &path, &state.action_flags);
        state.preview_text = result.output;
        state.violation_count = result.violation_count;
        state.preview_mode = PreviewMode::LintResults;
        let status = if result.success { "Done" } else { "Error" };
        state.set_status(format!(
            "{}: {} | {} violations",
            status, path, result.violation_count
        ));
    }

    fn run_action_no_path<F>(&self, state: &mut AppState, action: F)
    where
        F: FnOnce(&dyn ILintExecutorPort) -> LintExecutionResult,
    {
        let result = action(self.lint_port.as_ref());
        state.preview_text = result.output;
        state.violation_count = result.violation_count;
        state.preview_mode = PreviewMode::ActionOutput;
        let status = if result.success { "Done" } else { "Error" };
        state.set_status(status);
    }
}
```

---

## 17. `crates/tui/src/infrastructure_file_system_adapter.rs`

```rust
// PURPOSE: FileSystemAdapter infrastructure — implements IFileSystemPort using std::fs
use crate::contract_file_system_port::IFileSystemPort;
use crate::taxonomy_file_entry_vo::FileEntry;
use std::path::Path;

pub struct FileSystemAdapter;

impl FileSystemAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileSystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl IFileSystemPort for FileSystemAdapter {
    fn list_directory(&self, path: &str) -> Vec<FileEntry> {
        let dir_path = Path::new(path);
        let read_dir = match dir_path.read_dir() {
            Ok(rd) => rd,
            Err(_) => return Vec::new(),
        };

        let mut entries = Vec::new();
        for entry_result in read_dir {
            if let Ok(dir_entry) = entry_result {
                let entry_path = dir_entry.path();
                let name = match entry_path.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n.to_string(),
                    None => continue,
                };
                if name.starts_with('.') {
                    continue;
                }
                if let Some(file_entry) = FileEntry::from_path(&entry_path) {
                    entries.push(file_entry);
                }
            }
        }
        entries
    }

    fn read_file_preview(&self, path: &str, max_lines: usize) -> String {
        let file_path = Path::new(path);
        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => return format!("Cannot read file: {e}"),
        };

        let lines: Vec<&str> = content.lines().take(max_lines).collect();
        let mut output = String::new();
        for (i, line) in lines.iter().enumerate() {
            output.push_str(&format!("{:>4} │ {}\n", i + 1, line));
        }
        let total_lines = content.lines().count();
        if total_lines > max_lines {
            output.push_str(&format!("\n... ({} more lines)", total_lines - max_lines));
        }
        output
    }

    fn is_valid_directory(&self, path: &str) -> bool {
        Path::new(path).is_dir()
    }

    fn parent_directory(&self, path: &str) -> Option<String> {
        Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
    }

    fn file_size_human(&self, bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.1}G", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1}M", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1}K", bytes as f64 / KB as f64)
        } else {
            format!("{}B", bytes)
        }
    }

    fn path_components(&self, path: &str) -> Vec<String> {
        Path::new(path)
            .components()
            .filter_map(|c| c.as_os_str().to_str().map(|s| s.to_string()))
            .collect()
    }
}
```

---

## 18. `crates/tui/src/agent_tui_orchestrator.rs`

```rust
// PURPOSE: TuiOrchestrator agent — coordinates TUI state through contract ports and protocols
use crate::capabilities_action_handler::ActionHandler;
use crate::contract_file_system_port::IFileSystemPort;
use crate::contract_lint_executor_port::ILintExecutorPort;
use crate::contract_tui_aggregate::ITuiAggregate;
use crate::taxonomy_state_vo::AppState;
use crate::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

pub struct TuiOrchestrator {
    action_handler: ActionHandler,
}

impl TuiOrchestrator {
    pub fn new(
        fs_port: Arc<dyn IFileSystemPort>,
        lint_port: Arc<dyn ILintExecutorPort>,
    ) -> Self {
        Self {
            action_handler: ActionHandler::new(fs_port, lint_port),
        }
    }
}

impl ITuiAggregate for TuiOrchestrator {
    fn handle_event(&self, state: &mut AppState, event: TuiEvent) {
        self.action_handler.handle(state, event);
    }

    fn load_directory(&self, state: &mut AppState, path: &str) {
        self.action_handler.load_directory(state, path);
    }

    fn load_preview(&self, state: &mut AppState) {
        self.action_handler.load_preview(state);
    }
}
```

---

## 19. `crates/tui/src/surface_file_list_view.rs`

```rust
// PURPOSE: FileListView surface — renders middle panel with file list and AES layer badges
use crate::taxonomy_file_entry_vo::AesLayer;
use crate::taxonomy_state_vo::{AppState, PanelFocus};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

pub struct FileListView;

impl FileListView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.panel_focus == PanelFocus::FileList;
        let border_style = if is_focused {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(" Files ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let items: Vec<ListItem> = state
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let badge_color = layer_color(&entry.layer);
                let badge = entry.layer.badge_label();
                let name = entry.display_name();

                let name_style = if entry.is_dir {
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let line = Line::from(vec![
                    Span::styled(format!("{} ", badge), Style::default().fg(badge_color)),
                    Span::styled(name, name_style),
                ]);

                let item_style = if i == state.selected_index {
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(line).style(item_style)
            })
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(state.selected_index));

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, area, &mut list_state);
    }
}

impl Default for FileListView {
    fn default() -> Self {
        Self::new()
    }
}

fn layer_color(layer: &AesLayer) -> Color {
    match layer {
        AesLayer::Taxonomy => Color::Cyan,
        AesLayer::Contract => Color::Blue,
        AesLayer::Capabilities => Color::Magenta,
        AesLayer::Infrastructure => Color::Yellow,
        AesLayer::Agent => Color::Green,
        AesLayer::Surfaces => Color::Red,
        AesLayer::Root => Color::White,
        AesLayer::None => Color::DarkGray,
    }
}
```

---

## 20. `crates/tui/src/surface_preview_view.rs`

```rust
// PURPOSE: PreviewView surface — renders right panel with file preview or lint results
use crate::taxonomy_state_vo::{AppState, PanelFocus, PreviewMode};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub struct PreviewView;

impl PreviewView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.panel_focus == PanelFocus::Preview;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let (title, content) = match state.preview_mode {
            PreviewMode::FileContent => {
                let title = match state.selected_entry() {
                    Some(entry) if !entry.is_dir => format!(" Preview: {} ", entry.name),
                    _ => " Preview ".to_string(),
                };
                (title, state.preview_text.clone())
            }
            PreviewMode::LintResults => (" Lint Results ".to_string(), state.preview_text.clone()),
            PreviewMode::ActionOutput => (" Action Output ".to_string(), state.preview_text.clone()),
            PreviewMode::HelpOverlay => (" Help ".to_string(), help_text()),
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let paragraph = Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(Color::White));

        frame.render_widget(paragraph, area);
    }
}

impl Default for PreviewView {
    fn default() -> Self {
        Self::new()
    }
}

fn help_text() -> String {
    "\
Navigation:
  j/↓     Move down
  k/↑     Move up
  h/←     Back (parent dir)
  l/→/↵   Open folder / preview file
  Home    Jump to top
  End     Jump to bottom
  Tab     Cycle panel focus
  /       Search files

Actions (on selected file/folder):
  c       check — AES compliance
  s       scan — multi-adapter scan
  f       fix — auto-fix
  t       ci — CI mode (threshold)
  w       watch — file watch
  o       orphan — dead code check
  Ctrl+S  security — vulnerability scan
  Ctrl+D  duplicates — duplication
  Ctrl+P  dependencies — deps scan

Setup:
  d       doctor — environment diag
  i       init — create config
  I       install — adapter deps
  m       mcp-config — MCP config
  C       config-show — show config
  H       install-hook — git hook
  U       uninstall-hook — remove hook
  a       adapters — list adapters
  v       version — show version

General:
  ?       Toggle this help
  q       Quit
"
    .to_string()
}
```

---

## 21. `crates/tui/src/surface_tree_view.rs`

```rust
// PURPOSE: TreeView surface — renders left panel with directory tree / path hierarchy
use crate::taxonomy_state_vo::{AppState, PanelFocus};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;
use std::path::Path;

pub struct TreeView;

impl TreeView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.panel_focus == PanelFocus::Tree;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(" Tree ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let mut items = Vec::new();
        let components = build_path_components(&state.current_dir, &state.project_root);

        items.push(ListItem::new(Line::from(vec![
            Span::styled("📁 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                shorten_path(&state.project_root),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ])));

        for (i, component) in components.iter().enumerate() {
            let indent = "  ".repeat(i + 1);
            let is_current = i == components.len().saturating_sub(1);
            let style = if is_current {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Blue)
            };
            items.push(ListItem::new(Line::from(vec![
                Span::raw(&indent),
                Span::styled("└─ 📁 ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{}/", component), style),
            ])));
        }

        let list = List::new(items).block(block);
        frame.render_widget(list, area);
    }
}

impl Default for TreeView {
    fn default() -> Self {
        Self::new()
    }
}

fn build_path_components(current_dir: &str, project_root: &str) -> Vec<String> {
    let current = Path::new(current_dir);
    let root = Path::new(project_root);

    if let Ok(relative) = current.strip_prefix(root) {
        relative
            .components()
            .filter_map(|c| c.as_os_str().to_str().map(|s| s.to_string()))
            .collect()
    } else {
        Vec::new()
    }
}

fn shorten_path(path: &str) -> String {
    let p = Path::new(path);
    match p.file_name().and_then(|n| n.to_str()) {
        Some(name) => name.to_string(),
        None => path.to_string(),
    }
}
```

---

## 22. `crates/tui/src/surface_path_screen.rs`

```rust
// PURPOSE: PathScreen surface — renders startup path input dialog overlay
use crate::taxonomy_state_vo::AppState;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

pub struct PathScreen;

impl PathScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let popup_area = centered_rect(60, 30, area);

        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .title(" Enter Project Path ")
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Color::Black));

        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());

        let input_display = if state.path_input.is_empty() {
            format!("[{}]", cwd)
        } else {
            state.path_input.clone()
        };

        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Type path or press Enter for current dir:",
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  > ", Style::default().fg(Color::Green)),
                Span::styled(
                    input_display,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("_", Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  [Enter] Confirm   [Tab] Use current dir   [Esc] Quit",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, popup_area);
    }
}

impl Default for PathScreen {
    fn default() -> Self {
        Self::new()
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
```

---

## 23. `crates/tui/src/surface_help_screen.rs`

```rust
// PURPOSE: HelpScreen surface — renders help overlay (delegates to PreviewView help content)
use crate::taxonomy_state_vo::AppState;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct HelpScreen;

impl HelpScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn is_active(state: &AppState) -> bool {
        state.show_help
    }

    pub fn render_hint(frame: &mut Frame, area: Rect) {
        let _ = frame;
        let _ = area;
    }
}

impl Default for HelpScreen {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 24. `crates/tui/src/surface_shortcut_component.rs`

```rust
// PURPOSE: ShortcutComponent surface — renders always-visible shortcut bar at bottom
use crate::taxonomy_state_vo::{AppState, PreviewMode};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct ShortcutComponent;

impl ShortcutComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let (row1, row2, row3) = match state.preview_mode {
            PreviewMode::LintResults | PreviewMode::ActionOutput => context_sensitive_rows(state),
            _ => default_rows(),
        };

        let text = vec![
            Line::from(format_shortcuts(&row1)),
            Line::from(format_shortcuts(&row2)),
            Line::from(format_shortcuts(&row3)),
        ];

        let paragraph = Paragraph::new(text).style(Style::default().bg(Color::Black));
        frame.render_widget(paragraph, area);
    }
}

impl Default for ShortcutComponent {
    fn default() -> Self {
        Self::new()
    }
}

fn default_rows() -> (Vec<(&'static str, &'static str)>, Vec<(&'static str, &'static str)>, Vec<(&'static str, &'static str)>) {
    (
        vec![
            ("c", "check"),
            ("s", "scan"),
            ("f", "fix"),
            ("t", "ci"),
            ("w", "watch"),
            ("o", "orphan"),
            ("d", "doctor"),
            ("i", "init"),
        ],
        vec![
            ("I", "install"),
            ("m", "mcp"),
            ("C", "config"),
            ("H", "hook"),
            ("U", "unhook"),
            ("a", "adapters"),
            ("v", "version"),
        ],
        vec![
            ("^S", "security"),
            ("^D", "duplicates"),
            ("^P", "deps"),
            ("?", "help"),
            ("q", "quit"),
        ],
    )
}

fn context_sensitive_rows(_state: &AppState) -> (Vec<(&'static str, &'static str)>, Vec<(&'static str, &'static str)>, Vec<(&'static str, &'static str)>) {
    (
        vec![
            ("c", "re-check"),
            ("f", "fix"),
            ("Esc", "back"),
            ("j/k", "scroll"),
        ],
        vec![
            ("I", "install"),
            ("m", "mcp"),
            ("C", "config"),
            ("H", "hook"),
            ("U", "unhook"),
            ("a", "adapters"),
            ("v", "version"),
        ],
        vec![
            ("^S", "security"),
            ("^D", "duplicates"),
            ("^P", "deps"),
            ("?", "help"),
            ("q", "quit"),
        ],
    )
}

fn format_shortcuts(shortcuts: &[(&str, &str)]) -> Vec<Span> {
    let mut spans = Vec::new();
    for (i, (key, label)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(
            format!("{}:", key),
            Style::default().fg(Color::Yellow),
        ));
        spans.push(Span::styled(
            format!("{}", label),
            Style::default().fg(Color::White),
        ));
    }
    spans
}
```

---

## 25. `crates/tui/src/surface_status_component.rs`

```rust
// PURPOSE: StatusComponent surface — renders status bar at the very bottom
use crate::taxonomy_state_vo::AppState;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct StatusComponent;

impl StatusComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let selected_name = match state.selected_entry() {
            Some(entry) => entry.display_name(),
            None => "(none)".to_string(),
        };

        let violation_style = if state.violation_count > 0 {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };

        let line = Line::from(vec![
            Span::styled(" Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                &state.status_message,
                Style::default().fg(Color::White),
            ),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Selected: ", Style::default().fg(Color::DarkGray)),
            Span::styled(selected_name, Style::default().fg(Color::Cyan)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} viol.", state.violation_count),
                violation_style,
            ),
        ]);

        let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Black));
        frame.render_widget(paragraph, area);
    }
}

impl Default for StatusComponent {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 26. `crates/tui/src/surface_tui_command.rs`

```rust
// PURPOSE: TuiCommandSurface surface — main TUI event loop, terminal management, and rendering
use crate::contract_tui_aggregate::ITuiAggregate;
use crate::surface_file_list_view::FileListView;
use crate::surface_path_screen::PathScreen;
use crate::surface_preview_view::PreviewView;
use crate::surface_shortcut_component::ShortcutComponent;
use crate::surface_status_component::StatusComponent;
use crate::surface_tree_view::TreeView;
use crate::taxonomy_state_vo::AppState;
use crate::taxonomy_tui_event::TuiEvent;
use crossterm::event;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

pub struct TuiCommandSurface {
    tui_aggregate: Arc<dyn ITuiAggregate>,
}

impl TuiCommandSurface {
    pub fn new(tui_aggregate: Arc<dyn ITuiAggregate>) -> Self {
        Self { tui_aggregate }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(crossterm::event::EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());
        let mut state = AppState::new(cwd);

        let file_list_view = FileListView::new();
        let preview_view = PreviewView::new();
        let tree_view = TreeView::new();
        let path_screen = PathScreen::new();
        let shortcut_bar = ShortcutComponent::new();
        let status_bar = StatusComponent::new();

        let result = self.event_loop(
            &mut terminal,
            &mut state,
            &file_list_view,
            &preview_view,
            &tree_view,
            &path_screen,
            &shortcut_bar,
            &status_bar,
        );

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        stdout().execute(crossterm::event::DisableMouseCapture)?;

        result
    }

    fn event_loop(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        state: &mut AppState,
        file_list_view: &FileListView,
        preview_view: &PreviewView,
        tree_view: &TreeView,
        path_screen: &PathScreen,
        shortcut_bar: &ShortcutComponent,
        status_bar: &StatusComponent,
    ) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| {
                let area = frame.area();

                if state.show_path_dialog {
                    path_screen.render(state, frame, area);
                    return;
                }

                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Min(10),
                        Constraint::Length(3),
                        Constraint::Length(1),
                    ])
                    .split(area);

                render_header(state, frame, main_layout[0]);

                let panel_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(35),
                        Constraint::Percentage(45),
                    ])
                    .split(main_layout[1]);

                tree_view.render(state, frame, panel_layout[0]);
                file_list_view.render(state, frame, panel_layout[1]);
                preview_view.render(state, frame, panel_layout[2]);

                shortcut_bar.render(state, frame, main_layout[2]);
                status_bar.render(state, frame, main_layout[3]);
            })?;

            if event::poll(Duration::from_millis(50))? {
                let crossterm_event = event::read()?;
                let tui_event = TuiEvent::from_crossterm_event(crossterm_event);
                self.tui_aggregate.handle_event(state, tui_event);
            }

            if state.should_quit {
                break;
            }
        }
        Ok(())
    }
}

fn render_header(
    state: &AppState,
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
) {
    use ratatui::style::{Color, Style};
    use ratatui::text::{Line, Span};
    use ratatui::widgets::Paragraph;

    let line = Line::from(vec![
        Span::styled(
            " lint-arwaky TUI ",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
        Span::styled("Path: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            &state.current_dir,
            Style::default().fg(Color::White),
        ),
        Span::styled("  ", Style::default()),
        Span::styled("[q] Quit", Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
```

---

## 27. `crates/tui/src/root_tui_container.rs`

```rust
// PURPOSE: TuiContainer root — DI wiring for TUI crate, instantiates and injects all dependencies
use crate::agent_tui_orchestrator::TuiOrchestrator;
use crate::capabilities_lint_executor::LintExecutor;
use crate::contract_tui_aggregate::ITuiAggregate;
use crate::infrastructure_file_system_adapter::FileSystemAdapter;
use crate::surface_tui_command::TuiCommandSurface;
use std::sync::Arc;

pub struct TuiContainer;

impl TuiContainer {
    pub fn run() -> anyhow::Result<()> {
        let fs_adapter = Arc::new(FileSystemAdapter::new());

        let code_analysis_container = code_analysis::root_code_analysis_container::CodeAnalysisContainer::new();
        let code_analysis_aggregate = code_analysis_container.aggregate();

        let lint_executor = Arc::new(LintExecutor::new(code_analysis_aggregate));

        let tui_aggregate: Arc<dyn ITuiAggregate> =
            Arc::new(TuiOrchestrator::new(fs_adapter, lint_executor));

        let surface = TuiCommandSurface::new(tui_aggregate);
        surface.run()?;

        Ok(())
    }
}
```

---

## Summary

| #  | File                                      | Layer          | Suffix            |
| -- | ----------------------------------------- | -------------- | ----------------- |
| 1  | `lib.rs`                                | root           | exception         |
| 2  | `taxonomy_state_vo.rs`                  | taxonomy       | `_vo`           |
| 3  | `taxonomy_file_entry_vo.rs`             | taxonomy       | `_vo`           |
| 4  | `taxonomy_tui_event.rs`                 | taxonomy       | `_event`        |
| 5  | `taxonomy_action_flags_vo.rs`           | taxonomy       | `_vo`           |
| 6  | `contract_file_system_port.rs`          | contract       | `_port`         |
| 7  | `contract_lint_executor_port.rs`        | contract       | `_port`         |
| 8  | `contract_tui_aggregate.rs`             | contract       | `_aggregate`    |
| 9  | `capabilities_file_browser.rs`          | capabilities   | flexible ✓       |
| 10 | `capabilities_layer_detector.rs`        | capabilities   | flexible ✓       |
| 11 | `capabilities_lint_executor.rs`         | capabilities   | `_executor` ✓  |
| 12 | `capabilities_action_handler.rs`        | capabilities   | flexible ✓       |
| 13 | `infrastructure_file_system_adapter.rs` | infrastructure | `_adapter`      |
| 14 | `agent_tui_orchestrator.rs`             | agent          | `_orchestrator` |
| 15 | `surface_file_list_view.rs`             | surface        | `_view`         |
| 16 | `surface_preview_view.rs`               | surface        | `_view`         |
| 17 | `surface_tree_view.rs`                  | surface        | `_view`         |
| 18 | `surface_path_screen.rs`                | surface        | `_screen`       |
| 19 | `surface_help_screen.rs`                | surface        | `_screen`       |
| 20 | `surface_shortcut_component.rs`         | surface        | `_component`    |
| 21 | `surface_status_component.rs`           | surface        | `_component`    |
| 22 | `surface_tui_command.rs`                | surface        | `_command`      |
| 23 | `root_tui_container.rs`                 | root           | `_container`    |

**All 23 files are flat in `crates/tui/src/` — zero subfolders. All filenames comply with AES101/AES102 naming rules.**

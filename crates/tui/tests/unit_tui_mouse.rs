// PURPOSE: Unit tests for mouse hit-testing (issue #555).
//
// `handle_mouse_click` derives column boundaries from `compute_panel_layout`,
// so a click in the preview panel must land in the preview handler, not the
// file-list handler. These tests pin that contract by driving the public
// `SurfaceActionHandler::handle` entry point with `TuiEvent::MouseClick`.
//
// Mouse clicks never call the lint executor, so the filesystem aggregates
// wired into `SurfaceLintExecutor::new` are inert stubs — they exist only to
// satisfy the constructor.

use std::sync::Arc;

use dispatcher::surface_check_action::FilesystemSeam;
use dispatcher::surface_orphan_action::OrphanFactory;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_config_language_vo::ConfigLanguage;
use shared_common::taxonomy_language_vo::Language;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::{
    IParserProtocol, IToolResolutionProtocol, IWorkspaceProtocol,
};
use shared_filesystem::taxonomy_filesystem_vo::{ParseWarning, ProjectLanguagesVO};
use shared_quality_rules::{CodeAnalysisRequest, CodeAnalysisResponse, ICodeAnalysisAggregate};
use tui_lint_arwaky::surface_event_action::SurfaceActionHandler;
use tui_lint_arwaky::surface_lint_action::SurfaceLintExecutor;
use tui_lint_arwaky::surface_tui_layout::compute_panel_layout;
use tui_lint_arwaky::taxonomy_tui_event::TuiEvent;
use tui_lint_arwaky::{AppState, PanelFocus};

// ─── Stub filesystem aggregates (never touched by a mouse click) ──
#[derive(Debug)]
struct StubWorkspace;

impl IWorkspaceProtocol for StubWorkspace {
    fn workspace_root(&self, _start: &FilePath) -> Option<std::path::PathBuf> {
        None
    }

    fn find_workspace_root_from_path(
        &self,
        _start: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "stub"))
    }

    fn is_member_path(&self, _path: &FilePath) -> bool {
        false
    }

    fn is_leaf_member_path(&self, _path: &FilePath) -> bool {
        false
    }

    fn detect_source_dir(&self, _project_root: &std::path::Path) -> std::path::PathBuf {
        std::path::PathBuf::new()
    }

    fn detect_language_from_path(&self, _path: &str) -> ConfigLanguage {
        ConfigLanguage::Rust
    }

    fn check_wired_in_container(
        &self,
        _workspace_root: &std::path::Path,
        _identifiers: &PatternList,
    ) -> bool {
        false
    }

    fn detect_project_languages(&self, _root: &std::path::Path) -> ProjectLanguagesVO {
        ProjectLanguagesVO::default()
    }

    fn resolve_orphan_module_path(
        &self,
        _root: &std::path::Path,
        _base_dir: &std::path::Path,
        _module_path: &str,
    ) -> Option<std::path::PathBuf> {
        None
    }
}

#[derive(Debug)]
struct StubParser;

impl IParserProtocol for StubParser {
    fn parse_warnings(&self) -> &[ParseWarning] {
        &[]
    }

    fn import_list(&self) -> Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry> {
        Vec::new()
    }

    fn parse_all(&self, _files: &mut [shared_filesystem::taxonomy_filesystem_vo::FileEntry]) {}

    fn imports_for(
        &self,
        _path: &std::path::Path,
    ) -> Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry> {
        Vec::new()
    }

    fn extract(
        &self,
        _path: &std::path::Path,
        _content: &str,
        _language: Language,
    ) -> Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry> {
        Vec::new()
    }

    fn resolve_barrel_imports(&self, _root_dir: &std::path::Path) {}
}

#[derive(Debug)]
struct StubFilesystemAggregate;

impl IFilesystemAggregate for StubFilesystemAggregate {
    fn execute(
        &self,
        _request: shared_filesystem::taxonomy_filesystem_request::FilesystemRequest,
    ) -> shared_filesystem::taxonomy_filesystem_response::FilesystemResponse {
        unimplemented!("mouse click tests never invoke the filesystem aggregate")
    }
}

#[derive(Debug)]
struct StubToolResolution;

impl IToolResolutionProtocol for StubToolResolution {
    fn is_executable_in_path(&self, _executable: &ToolName) -> bool {
        false
    }

    fn is_binary_available(&self, _bin_name: &ToolName) -> bool {
        false
    }

    fn has_local_bin(&self, _working_dir: &std::path::Path, _executable: &ToolName) -> bool {
        false
    }

    fn resolve_js_cmd(
        &self,
        _executable: &ToolName,
        _args: Vec<String>,
        _working_dir: &FilePath,
    ) -> Option<Vec<String>> {
        None
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }

    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }

    fn has_config_file(&self, _dir: &std::path::Path) -> bool {
        false
    }

    fn has_cargo_toml(&self, _path: &FilePath) -> Option<FilePath> {
        None
    }

    fn has_cargo_lock(&self, _path: &FilePath) -> Option<FilePath> {
        None
    }

    fn is_python_file_recursive(&self, _path: &FilePath) -> bool {
        false
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
}

#[derive(Debug)]
struct StubCodeAnalysis;

impl ICodeAnalysisAggregate for StubCodeAnalysis {
    fn execute(&self, _request: CodeAnalysisRequest) -> CodeAnalysisResponse {
        unimplemented!("mouse click tests never invoke code analysis")
    }
}

// ─── Handler wiring ─────────────────────────────────────────────────

fn handler() -> SurfaceActionHandler {
    let workspace = Arc::new(StubWorkspace);
    let parser = Arc::new(StubParser);
    let tool_resolution = Arc::new(StubToolResolution);
    let fs_agg = Arc::new(StubFilesystemAggregate);
    let code_analysis = Arc::new(StubCodeAnalysis);

    let fs_seam = Arc::new(FilesystemSeam {
        workspace: workspace.clone(),
        parser: parser.clone(),
        aggregate: fs_agg.clone(),
    });
    let inner_seam = FilesystemSeam {
        workspace: workspace.clone(),
        parser: parser.clone(),
        aggregate: fs_agg.clone(),
    };
    let fs_factory = Arc::new(move || inner_seam.clone());

    let orphan_factory: Arc<OrphanFactory> = Arc::new(move |_config, _fs, _ws| {
        unimplemented!("mouse click tests never invoke orphan scanning")
    });

    let executor = Arc::new(SurfaceLintExecutor::new(
        code_analysis,
        fs_agg.clone(),
        workspace,
        tool_resolution,
        fs_seam,
        fs_factory,
        orphan_factory,
    ));
    SurfaceActionHandler::new(executor, fs_agg)
}

fn click_state(w: u16, h: u16, selected: usize) -> AppState {
    let mut state = AppState::new("/tmp".to_string());
    state.terminal_width = w;
    state.terminal_height = h;
    state.path_dialog.visible = false;
    state.navigation.selected_index = selected;
    state.navigation.panel_focus = PanelFocus::FileList;
    state
}

/// Populate `state.navigation.entries` with `n` dummy files so a file-list
/// click can select a valid index (the handler only updates selection when
/// the computed index is within `entries.len()`).
fn with_entries(mut state: AppState, n: usize) -> AppState {
    for i in 0..n {
        state.navigation.entries.push(tui_lint_arwaky::FileEntry {
            name: format!("f{i}.rs"),
            full_path: format!("/tmp/f{i}.rs"),
            is_dir: false,
            layer: tui_lint_arwaky::AesLayer::None,
            violation_count: 0,
            extension: "rs".to_string(),
            size_bytes: 0,
        });
    }
    state
}

/// Clicking a row inside the preview panel must focus Preview and must not
/// touch the file-list selection.
#[test]
fn preview_click_keeps_selected_index() {
    // Width 120 > NARROW_BREAKPOINT_WIDTH (100) → three_column mode.
    let layout = compute_panel_layout(120, 24);
    let col = layout.preview.x + layout.preview.width / 2;
    let row = layout.preview.y + layout.preview.height / 2;

    let mut state = click_state(120, 24, 42);
    let h = handler();
    h.handle(&mut state, TuiEvent::MouseClick(col, row));

    assert_eq!(
        state.navigation.panel_focus,
        PanelFocus::Preview,
        "click inside preview rect must focus Preview"
    );
    assert_eq!(
        state.navigation.selected_index, 42,
        "preview click must not mutate file selection"
    );
}

/// Clicking a row inside the tree panel must focus Tree and must not touch
/// the file-list selection.
#[test]
fn tree_click_focuses_tree() {
    let layout = compute_panel_layout(120, 24);
    let col = layout.tree.x + layout.tree.width / 2;
    let row = layout.tree.y + layout.tree.height / 2;

    let mut state = click_state(120, 24, 7);
    let h = handler();
    h.handle(&mut state, TuiEvent::MouseClick(col, row));

    assert_eq!(
        state.navigation.panel_focus,
        PanelFocus::Tree,
        "click inside tree rect must focus Tree"
    );
    assert_eq!(
        state.navigation.selected_index, 7,
        "tree click must not mutate file selection"
    );
}

/// Clicking a row inside the file-list panel selects that entry.
#[test]
fn file_list_click_updates_selection() {
    let layout = compute_panel_layout(120, 24);
    let col = layout.file_list.x + layout.file_list.width / 2;
    let row = layout.file_list.y + 3;

    let mut state = with_entries(click_state(120, 24, 0), 5);
    let h = handler();
    h.handle(&mut state, TuiEvent::MouseClick(col, row));

    assert_eq!(state.navigation.panel_focus, PanelFocus::FileList);
    // selected_index = scroll_offset + panel_row = 0 + 3
    assert_eq!(state.navigation.selected_index, 3);
}

/// A click in the shortcuts/status band below the panels is ignored.
#[test]
fn click_below_panels_is_ignored() {
    let layout = compute_panel_layout(120, 24);
    let col = layout.file_list.x;
    let row = layout.shortcuts.y + 1;

    let mut state = click_state(120, 24, 5);
    let h = handler();
    h.handle(&mut state, TuiEvent::MouseClick(col, row));

    assert_eq!(
        state.navigation.selected_index, 5,
        "click in shortcut band must not change selection"
    );
}

/// Search-mode mouse click (#909): the panel shows `filtered_indices`, so a
/// click on panel row `i` must resolve to `filtered_indices[i]`, not `i`.
/// Before the fix, clicking row 1 with `filtered_indices = [2, 5, 8]`
/// selected entry #1 instead of entry #5.
#[test]
fn file_list_click_in_search_mode_resolves_filtered_indices() {
    let layout = compute_panel_layout(120, 24);
    let col = layout.file_list.x + layout.file_list.width / 2;
    let row = layout.file_list.y + 1; // panel row 1

    let mut state = with_entries(click_state(120, 24, 0), 9);
    state.search.mode = true;
    state.search.query = "f5".to_string();
    state.search.filtered_indices = vec![2, 5, 8];
    state.search.filter_pos = 0;
    let h = handler();
    h.handle(&mut state, TuiEvent::MouseClick(col, row));

    // Click on filtered row 1 → entry 5, not entry 1.
    assert_eq!(state.navigation.selected_index, 5);
    assert_eq!(
        state.search.filter_pos, 1,
        "filter_pos must follow the click"
    );
    assert_eq!(state.navigation.panel_focus, PanelFocus::FileList);
}

/// Search-mode click on a row beyond the filtered list bounds must not
/// change the selection (#909 guard).
#[test]
fn file_list_click_in_search_mode_bounds_check() {
    let layout = compute_panel_layout(120, 24);
    let col = layout.file_list.x + layout.file_list.width / 2;
    let row = layout.file_list.y + 5; // panel row 5, only 3 filtered rows exist

    let mut state = with_entries(click_state(120, 24, 0), 9);
    state.search.mode = true;
    state.search.query = "f5".to_string();
    state.search.filtered_indices = vec![2, 5, 8];
    state.search.filter_pos = 0;
    let h = handler();
    h.handle(&mut state, TuiEvent::MouseClick(col, row));

    assert_eq!(
        state.navigation.selected_index, 0,
        "out-of-bounds click must not change selection"
    );
}

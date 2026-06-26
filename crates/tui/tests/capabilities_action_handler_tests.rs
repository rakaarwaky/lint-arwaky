use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use shared::common::taxonomy_byte_count_vo::ByteCount;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;
use shared::tui::contract_file_system_port::IFileSystemPort;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use shared::tui::taxonomy_file_entry_vo::{AesLayer, FileEntry};
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use shared::tui::taxonomy_state_vo::{AppState, PanelFocus, PreviewMode};
use shared::tui::taxonomy_tui_event::TuiEvent;
use shared::common::taxonomy_line_count_vo::LineCount;
use std::sync::Arc;

struct MockFs;

impl IFileSystemPort for MockFs {
    fn list_directory(&self, _path: &FilePath) -> Vec<FileEntry> {
        vec![
            FileEntry {
                name: "dir_a".to_string(),
                full_path: "/root/dir_a".to_string(),
                is_dir: true,
                layer: AesLayer::None,
                violation_count: 0,
                extension: String::new(),
                size_bytes: 0,
            },
            FileEntry {
                name: "file_b.rs".to_string(),
                full_path: "/root/file_b.rs".to_string(),
                is_dir: false,
                layer: AesLayer::None,
                violation_count: 0,
                extension: "rs".to_string(),
                size_bytes: 100,
            },
        ]
    }
    fn read_file_preview(&self, _path: &FilePath, _max: &LineCount) -> DisplayContent {
        DisplayContent::new("preview content".to_string())
    }
    fn is_valid_directory(&self, path: &FilePath) -> bool {
        path.value == "/valid" || path.value == "/root"
    }
    fn parent_directory(&self, path: &FilePath) -> Option<FilePath> {
        if path.value == "/root" {
            Some(FilePath::new("/".to_string()).unwrap_or_default())
        } else if path.value == "/root/dir_a" {
            Some(FilePath::new("/root".to_string()).unwrap_or_default())
        } else {
            None
        }
    }
    fn file_size_human(&self, _bytes: &ByteCount) -> DisplayContent {
        DisplayContent::new("100B".to_string())
    }
    fn path_components(&self, path: &FilePath) -> Vec<FilePath> {
        vec![path.clone()]
    }
}

struct MockLint;

impl ILintExecutorProtocol for MockLint {
    fn check(&self, _path: &str, _f: &ActionFlags) -> LintExecutionResult {
        LintExecutionResult::success("checked", 3)
    }
    fn scan(&self, _path: &str) -> LintExecutionResult {
        LintExecutionResult::success("scanned", 5)
    }
    fn fix(&self, _path: &str, _f: &ActionFlags) -> LintExecutionResult {
        LintExecutionResult::success("fixed", 0)
    }
    fn ci(&self, _path: &str, _f: &ActionFlags) -> LintExecutionResult {
        LintExecutionResult::success("ci pass", 0)
    }
    fn orphan(&self, _path: &str) -> LintExecutionResult {
        LintExecutionResult::success("orphan ok", 0)
    }
    fn security(&self, _path: &str) -> LintExecutionResult {
        LintExecutionResult::success("secure", 0)
    }
    fn duplicates(&self, _path: &str) -> LintExecutionResult {
        LintExecutionResult::success("no dupes", 0)
    }
    fn dependencies(&self, _path: &str) -> LintExecutionResult {
        LintExecutionResult::success("deps ok", 0)
    }
    fn doctor(&self) -> LintExecutionResult {
        LintExecutionResult::success("healthy", 0)
    }
    fn init(&self, _f: &ActionFlags) -> LintExecutionResult {
        LintExecutionResult::success("init done", 0)
    }
    fn install(&self, _f: &ActionFlags) -> LintExecutionResult {
        LintExecutionResult::success("installed", 0)
    }
    fn mcp_config(&self, _f: &ActionFlags) -> LintExecutionResult {
        LintExecutionResult::success("mcp config", 0)
    }
    fn config_show(&self) -> LintExecutionResult {
        LintExecutionResult::success("config shown", 0)
    }
    fn install_hook(&self) -> LintExecutionResult {
        LintExecutionResult::success("hook installed", 0)
    }
    fn uninstall_hook(&self) -> LintExecutionResult {
        LintExecutionResult::success("hook removed", 0)
    }
    fn adapters(&self) -> LintExecutionResult {
        LintExecutionResult::success("adapters listed", 0)
    }
    fn version(&self) -> LintExecutionResult {
        LintExecutionResult::success("v1.0", 0)
    }
}

fn make_handler() -> ActionHandler {
    ActionHandler::new(Arc::new(MockFs), Arc::new(MockLint))
}

fn make_entries() -> Vec<FileEntry> {
    vec![
        FileEntry {
            name: "file_a.rs".to_string(),
            full_path: "/root/file_a.rs".to_string(),
            is_dir: false,
            layer: AesLayer::None,
            violation_count: 0,
            extension: "rs".to_string(),
            size_bytes: 50,
        },
        FileEntry {
            name: "file_b.rs".to_string(),
            full_path: "/root/file_b.rs".to_string(),
            is_dir: false,
            layer: AesLayer::None,
            violation_count: 0,
            extension: "rs".to_string(),
            size_bytes: 100,
        },
    ]
}

#[test]
fn test_move_down() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::MoveDown);
    assert_eq!(state.selected_index, 1);
}

#[test]
fn test_move_down_stops_at_end() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    state.selected_index = 1;
    h.handle(&mut state, TuiEvent::MoveDown);
    assert_eq!(state.selected_index, 1);
}

#[test]
fn test_move_up() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    state.selected_index = 1;
    h.handle(&mut state, TuiEvent::MoveUp);
    assert_eq!(state.selected_index, 0);
}

#[test]
fn test_move_up_stops_at_zero() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::MoveUp);
    assert_eq!(state.selected_index, 0);
}

#[test]
fn test_move_top_and_bottom() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::MoveBottom);
    assert_eq!(state.selected_index, 1);
    h.handle(&mut state, TuiEvent::MoveTop);
    assert_eq!(state.selected_index, 0);
    assert_eq!(state.scroll_offset, 0);
}

#[test]
fn test_focus_cycle() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    assert_eq!(state.panel_focus, PanelFocus::FileList);
    h.handle(&mut state, TuiEvent::FocusNext);
    assert_eq!(state.panel_focus, PanelFocus::Preview);
    h.handle(&mut state, TuiEvent::FocusNext);
    assert_eq!(state.panel_focus, PanelFocus::Tree);
    h.handle(&mut state, TuiEvent::FocusPrev);
    assert_eq!(state.panel_focus, PanelFocus::Preview);
}

#[test]
fn test_toggle_help() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    assert!(!state.show_help);
    h.handle(&mut state, TuiEvent::ToggleHelp);
    assert!(state.show_help);
    assert_eq!(state.preview_mode, PreviewMode::HelpOverlay);
    h.handle(&mut state, TuiEvent::ToggleHelp);
    assert!(!state.show_help);
    assert_eq!(state.preview_mode, PreviewMode::FileContent);
}

#[test]
fn test_search_mode() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ToggleSearch);
    assert!(state.search_mode);
    h.handle(&mut state, TuiEvent::SearchInput('a'));
    h.handle(&mut state, TuiEvent::SearchInput('b'));
    assert_eq!(state.search_query, "ab");
    h.handle(&mut state, TuiEvent::SearchBackspace);
    assert_eq!(state.search_query, "a");
    h.handle(&mut state, TuiEvent::SearchConfirm);
    assert!(!state.search_mode);
    assert!(state.search_query.is_empty());
}

#[test]
fn test_search_cancel() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.search_mode = true;
    state.search_query = "test".to_string();
    h.handle(&mut state, TuiEvent::SearchCancel);
    assert!(!state.search_mode);
    assert!(state.search_query.is_empty());
}

#[test]
fn test_search_input_ignored_when_not_search_mode() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    assert!(!state.search_mode);
    h.handle(&mut state, TuiEvent::SearchInput('x'));
    assert!(state.search_query.is_empty());
}

#[test]
fn test_quit() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    assert!(!state.should_quit);
    h.handle(&mut state, TuiEvent::Quit);
    assert!(state.should_quit);
}

#[test]
fn test_mouse_scroll() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    assert_eq!(state.scroll_offset, 0);
    h.handle(&mut state, TuiEvent::MouseScrollDown);
    assert_eq!(state.scroll_offset, 1);
    h.handle(&mut state, TuiEvent::MouseScrollUp);
    assert_eq!(state.scroll_offset, 0);
    h.handle(&mut state, TuiEvent::MouseScrollUp);
    assert_eq!(state.scroll_offset, 0);
}

#[test]
fn test_load_directory_sorts_dirs_first() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.load_directory(&mut state, "/root");
    assert!(!state.entries.is_empty());
    assert!(state.entries[0].is_dir);
    assert_eq!(state.selected_index, 0);
    assert!(state.status_message.contains("/root"));
}

#[test]
fn test_action_check_updates_state() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionCheck);
    assert_eq!(state.preview_mode, PreviewMode::LintResults);
    assert!(state.preview_text.contains("checked"));
    assert_eq!(state.violation_count, 3);
}

#[test]
fn test_action_doctor() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionDoctor);
    assert_eq!(state.preview_mode, PreviewMode::ActionOutput);
    assert!(state.preview_text.contains("healthy"));
}

#[test]
fn test_path_input_and_backspace() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::PathInput('/'));
    h.handle(&mut state, TuiEvent::PathInput('v'));
    h.handle(&mut state, TuiEvent::PathInput('a'));
    h.handle(&mut state, TuiEvent::PathInput('l'));
    h.handle(&mut state, TuiEvent::PathInput('i'));
    h.handle(&mut state, TuiEvent::PathInput('d'));
    assert_eq!(state.path_input, "/valid");
    h.handle(&mut state, TuiEvent::PathBackspace);
    assert_eq!(state.path_input, "/vali");
}

#[test]
fn test_path_confirm_invalid() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.path_input = "/nonexistent".to_string();
    h.handle(&mut state, TuiEvent::PathConfirm);
    assert!(state.show_path_dialog);
    assert!(state.status_message.contains("Invalid"));
}

#[test]
fn test_path_confirm_valid() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.path_input = "/root".to_string();
    h.handle(&mut state, TuiEvent::PathConfirm);
    assert!(!state.show_path_dialog);
    assert_eq!(state.project_root, "/root");
}

#[test]
fn test_empty_entries() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    assert_eq!(state.selected_index, 0);
    h.handle(&mut state, TuiEvent::MoveDown);
    assert_eq!(state.selected_index, 0);
    h.handle(&mut state, TuiEvent::MoveBottom);
    assert_eq!(state.selected_index, 0);
}

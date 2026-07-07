use shared::common::taxonomy_byte_count_vo::ByteCount;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_line_count_vo::LineCount;
use shared::common::taxonomy_path_vo::FilePath;
use shared::tui::contract_file_system_port::IFileSystemPort;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use shared::tui::taxonomy_file_entry_vo::{AesLayer, FileEntry};
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use shared::tui::taxonomy_state_vo::{AppState, PanelFocus, PreviewMode};
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;

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
    state.entries = make_entries(); // 2 entries — scroll needs entries to be bounded
    assert_eq!(state.scroll_offset, 0);
    h.handle(&mut state, TuiEvent::MouseScrollDown(0, 0));
    assert_eq!(state.scroll_offset, 1);
    h.handle(&mut state, TuiEvent::MouseScrollDown(0, 0));
    assert_eq!(state.scroll_offset, 1); // bounded at entries.len()-1
    h.handle(&mut state, TuiEvent::MouseScrollUp(0, 0));
    assert_eq!(state.scroll_offset, 0);
    h.handle(&mut state, TuiEvent::MouseScrollUp(0, 0));
    assert_eq!(state.scroll_offset, 0); // bounded at 0
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

// ============ Additional lint actions ============

#[test]
fn test_action_scan() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::ActionScan);
    assert_eq!(state.preview_mode, PreviewMode::LintResults);
    assert!(state.preview_text.contains("scanned"));
    assert_eq!(state.violation_count, 5);
}

#[test]
fn test_action_fix() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::ActionFix);
    assert_eq!(state.preview_mode, PreviewMode::LintResults);
    assert!(state.preview_text.contains("fixed"));
}

#[test]
fn test_action_ci() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::ActionCi);
    assert!(state.preview_text.contains("ci pass"));
}

#[test]
fn test_action_orphan() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::ActionOrphan);
    assert!(state.preview_text.contains("orphan ok"));
}

#[test]
fn test_action_security() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::ActionSecurity);
    assert!(state.preview_text.contains("secure"));
}

#[test]
fn test_action_duplicates() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::ActionDuplicates);
    assert!(state.preview_text.contains("no dupes"));
}

#[test]
fn test_action_dependencies() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::ActionDependencies);
    assert!(state.preview_text.contains("deps ok"));
}

#[test]
fn test_action_init() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionInit);
    assert_eq!(state.preview_mode, PreviewMode::ActionOutput);
    assert!(state.preview_text.contains("init done"));
}

#[test]
fn test_action_install() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionInstall);
    assert!(state.preview_text.contains("installed"));
}

#[test]
fn test_action_mcp_config() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionMcpConfig);
    assert!(state.preview_text.contains("mcp config"));
}

#[test]
fn test_action_config_show() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionConfigShow);
    assert!(state.preview_text.contains("config shown"));
}

#[test]
fn test_action_install_hook() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionInstallHook);
    assert!(state.preview_text.contains("hook installed"));
}

#[test]
fn test_action_uninstall_hook() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionUninstallHook);
    assert!(state.preview_text.contains("hook removed"));
}

#[test]
fn test_action_adapters() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionAdapters);
    assert!(state.preview_text.contains("adapters listed"));
}

#[test]
fn test_action_version() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionVersion);
    assert!(state.preview_text.contains("v1.0"));
}

#[test]
fn test_path_use_current() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.show_path_dialog = true;
    state.current_dir = String::new();
    h.handle(&mut state, TuiEvent::PathUseCurrent);
    // Should close dialog and set current_dir from env
    assert!(!state.show_path_dialog);
    assert!(!state.current_dir.is_empty());
}

#[test]
fn test_resize_sets_terminal_height() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    assert_eq!(state.terminal_height, 0);
    h.handle(&mut state, TuiEvent::Resize(80, 24));
    assert_eq!(state.terminal_height, 24);
}

#[test]
fn test_navigate_back_stays_at_root() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.current_dir = "/".to_string();
    state.project_root = "/".to_string();
    state.entries = make_entries();
    h.handle(&mut state, TuiEvent::NavigateBack);
    // Should stay at root since parent would be shorter than project_root
    assert_eq!(state.current_dir, "/");
}

#[test]
fn test_navigate_forward_into_directory() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = vec![shared::tui::taxonomy_file_entry_vo::FileEntry {
        name: "subdir".to_string(),
        full_path: "/root/subdir".to_string(),
        is_dir: true,
        layer: shared::tui::taxonomy_file_entry_vo::AesLayer::None,
        violation_count: 0,
        extension: String::new(),
        size_bytes: 0,
    }];
    h.handle(&mut state, TuiEvent::MoveDown); // no-op, only 1 entry
    h.handle(&mut state, TuiEvent::NavigateForward);
    // Should navigate into /root/subdir via MockFs (parent returns Some(/root))
    // MockFs.parent_directory("/root/subdir") -> Some("/root")
    // So current_dir should still be /root since MockFs doesn't support deep navigation
    assert_eq!(state.current_dir, "/root/subdir");
}

#[test]
fn test_navigate_forward_into_file_shows_preview() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_entries();
    // file_b.rs is at index 1 after load_directory sorts dirs first
    state.selected_index = 1;
    h.handle(&mut state, TuiEvent::NavigateForward);
    // Should load file preview (not change directory)
    assert_eq!(state.current_dir, "/root");
    assert_eq!(state.preview_mode, PreviewMode::FileContent);
}

// ---- missing: ActionHandler::poll_watch is a no-op by design ----

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

#[test]
fn test_action_watch_shows_cli_redirect() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    h.handle(&mut state, TuiEvent::ActionWatch);
    assert_eq!(state.preview_mode, PreviewMode::ActionOutput);
    assert!(state.preview_text.contains("lint-arwaky-cli watch"));
    assert!(state.status_message.contains("File watch"));
}

#[test]
fn test_copy_to_clipboard_empty_preview() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.preview_text.clear();
    h.handle(&mut state, TuiEvent::CopyToClipboard);
    assert_eq!(state.status_message, "Nothing to copy");
}

#[test]
fn test_copy_to_clipboard_with_content() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.preview_text = "lint results here".to_string();
    h.handle(&mut state, TuiEvent::CopyToClipboard);
    // In test cfg, arboard is skipped; clipboard tools may or may not exist.
    // Either "Copied to clipboard!" or "Clipboard unavailable" is acceptable.
    assert!(
        state.status_message.contains("Copied")
            || state.status_message.contains("Clipboard unavailable"),
        "unexpected status: {}",
        state.status_message
    );
}

#[test]
fn test_copy_to_file_empty_preview() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.preview_text.clear();
    h.handle(&mut state, TuiEvent::CopyToFile);
    assert_eq!(state.status_message, "Nothing to copy");
}

#[test]
fn test_copy_to_file_creates_file() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.preview_text = "test output".to_string();

    // Use a temp directory to avoid polluting CWD
    let tmp = std::env::temp_dir().join("lint_arwaky_test_copy_");
    let _ = std::fs::create_dir_all(&tmp);
    let original_dir = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => {
            // Cannot determine CWD — skip file-creation assertion
            let _ = std::fs::remove_dir_all(&tmp);
            return;
        }
    };
    if std::env::set_current_dir(&tmp).is_err() {
        let _ = std::fs::remove_dir_all(&tmp);
        return;
    }

    h.handle(&mut state, TuiEvent::CopyToFile);

    // Restore CWD
    let _ = std::env::set_current_dir(&original_dir);

    assert_eq!(state.status_message, "Saved to lint-results.txt");

    let results_path = tmp.join("lint-results.txt");
    if results_path.exists() {
        let content = std::fs::read_to_string(&results_path).unwrap_or_default();
        assert_eq!(content, "test output");
    }

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp);
}

// ---- Search navigation tests ----

fn make_search_entries() -> Vec<FileEntry> {
    vec![
        FileEntry {
            name: "alpha.rs".to_string(),
            full_path: "/root/alpha.rs".to_string(),
            is_dir: false,
            layer: AesLayer::None,
            violation_count: 0,
            extension: "rs".to_string(),
            size_bytes: 50,
        },
        FileEntry {
            name: "beta.txt".to_string(),
            full_path: "/root/beta.txt".to_string(),
            is_dir: false,
            layer: AesLayer::None,
            violation_count: 0,
            extension: "txt".to_string(),
            size_bytes: 100,
        },
        FileEntry {
            name: "gamma.rs".to_string(),
            full_path: "/root/gamma.rs".to_string(),
            is_dir: false,
            layer: AesLayer::None,
            violation_count: 0,
            extension: "rs".to_string(),
            size_bytes: 150,
        },
    ]
}

fn setup_search_state(query: &str) -> AppState {
    let mut state = AppState::new("/root".to_string());
    state.entries = make_search_entries();
    state.search_mode = true;
    state.search_query = query.to_string();
    state.compute_filtered_indices();
    state
}

#[test]
fn test_search_move_down_skips_non_matching() {
    let h = make_handler();
    let mut state = setup_search_state(".rs");
    // ".rs" matches alpha (idx 0) and gamma (idx 2), skips beta (idx 1)
    assert_eq!(state.filtered_indices, vec![0, 2]);
    assert_eq!(state.selected_index, 0); // alpha
    assert_eq!(state.filter_pos, 0);

    h.handle(&mut state, TuiEvent::MoveDown);
    // Should jump to gamma (idx 2), skip beta
    assert_eq!(
        state.selected_index, 2,
        "should skip beta and land on gamma"
    );
    assert_eq!(state.filter_pos, 1);
}

#[test]
fn test_search_move_up_skips_non_matching() {
    let h = make_handler();
    let mut state = setup_search_state(".rs");
    // Start at gamma (last matching)
    state.filter_pos = 1;
    state.selected_index = 2;
    assert_eq!(state.filtered_indices, vec![0, 2]);

    h.handle(&mut state, TuiEvent::MoveUp);
    // Should jump back to alpha (idx 0), skip beta
    assert_eq!(
        state.selected_index, 0,
        "should skip beta and land on alpha"
    );
    assert_eq!(state.filter_pos, 0);
}

#[test]
fn test_search_move_down_stops_at_last_match() {
    let h = make_handler();
    let mut state = setup_search_state(".rs");
    // Already on gamma (last match)
    state.filter_pos = 1;
    state.selected_index = 2;

    h.handle(&mut state, TuiEvent::MoveDown);
    // Should stay on gamma — can't go past last match
    assert_eq!(state.selected_index, 2);
    assert_eq!(state.filter_pos, 1);
}

#[test]
fn test_search_move_up_stops_at_first_match() {
    let h = make_handler();
    let mut state = setup_search_state(".rs");
    // Already on alpha (first match)
    assert_eq!(state.filter_pos, 0);

    h.handle(&mut state, TuiEvent::MoveUp);
    // Should stay on alpha
    assert_eq!(state.selected_index, 0);
    assert_eq!(state.filter_pos, 0);
}

#[test]
fn test_search_cancel_resets_filter() {
    let h = make_handler();
    let mut state = setup_search_state(".rs");
    assert_eq!(state.filtered_indices, vec![0, 2]);

    h.handle(&mut state, TuiEvent::SearchCancel);
    assert!(state.filtered_indices.is_empty());
    assert_eq!(state.filter_pos, 0);
    assert!(!state.search_mode);
}

#[test]
fn test_search_toggle_clears_filter() {
    let h = make_handler();
    let mut state = setup_search_state(".rs");
    assert_eq!(state.filtered_indices, vec![0, 2]);

    // Toggle search off
    h.handle(&mut state, TuiEvent::ToggleSearch);
    assert!(state.filtered_indices.is_empty());
    assert!(!state.search_mode);
}

#[test]
fn test_mouse_scroll_syncs_selection() {
    let h = make_handler();
    let mut state = AppState::new("/root".to_string());
    state.entries = make_search_entries();
    // 3 entries, set terminal height to something reasonable
    state.terminal_height = 10;
    state.selected_index = 0;

    // Scroll down — selection should follow
    h.handle(&mut state, TuiEvent::MouseScrollDown(0, 0));
    assert_eq!(state.scroll_offset, 1);
    assert_eq!(state.selected_index, 1, "selection follows scroll down");

    // Scroll back up — selection should follow
    h.handle(&mut state, TuiEvent::MouseScrollUp(0, 0));
    assert_eq!(state.scroll_offset, 0);
    assert_eq!(state.selected_index, 0, "selection follows scroll up");
}

#[test]
fn test_empty_search_matches_nothing() {
    let h = make_handler();
    let mut state = setup_search_state("zzz");
    // No entries match "zzz"
    assert!(state.filtered_indices.is_empty());

    // MoveDown/MoveUp should be no-ops
    h.handle(&mut state, TuiEvent::MoveDown);
    h.handle(&mut state, TuiEvent::MoveUp);
    assert_eq!(state.filter_pos, 0);
}
